// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::construct_java_script_semantic_text_units;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_literal_property_index;
use super::read_java_script_semantic_text_length;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_commit_java_script_ordinary_own_property_loop122(
    mut count: f64,
    mut source_ml_source1_temporary_values121: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index121: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index121
            < ({
                let ml_reference_array2626 = source_ml_source1_temporary_values121.clone();
                let ml_reference_projection2626 =
                    self::ml::Big::from_u128(ml_reference_array2626.borrow().len() as u128);
                ml_reference_projection2626
            })) {
            {
                let entry = {
                    let ml_reference_array2627 = source_ml_source1_temporary_values121.clone();
                    let ml_reference_position2627 = source_ml_source1_temporary_index121.clone();
                    let ml_reference_projection2627 = self::ml_array::at(
                        &*ml_reference_array2627.borrow(),
                        ml_reference_position2627.to_index(),
                    );
                    ml_reference_projection2627
                };
                {
                    let count_2 = (count + {
                        let ml_reference_array2628 = match entry.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39680,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39680.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39681,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39681.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39682,
                            ) => source_ml_source2_data_property_value39682.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39683,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39683.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39684,
                                _,
                            ) => source_ml_source2_data_property_value39684.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39685,
                                _,
                            ) => source_ml_source2_data_property_value39685.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39686,
                                _,
                            ) => source_ml_source2_data_property_value39686.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39687,
                                _,
                            ) => source_ml_source2_data_property_value39687.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39688,
                            ) => source_ml_source2_data_property_value39688.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39689,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39689.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39690,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39690.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39691,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39691.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39692,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39692.clone(),
                        };
                        let ml_reference_projection2628 =
                            ml_reference_array2628.borrow().len() as f64;
                        ml_reference_projection2628
                    });
                    {
                        let source_ml_source1_temporary_index121_2 =
                            source_ml_source1_temporary_index121.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index121) =
                                (count_2, source_ml_source1_temporary_index121_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            count
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result {
    MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Done,
    MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Return(
        std::rc::Rc<
            std::cell::RefCell<(
                (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool),
                Vec<f64>,
            )>,
        >,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_set_java_script_ordinary_prototype_loop123(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut next: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (depth
                    <= ({
                        let ml_reference_array2629 = shared_constructor_field1.clone();
                        let ml_reference_projection2629 =
                            ml_reference_array2629.borrow().len() as f64;
                        ml_reference_projection2629
                    }))
                {
                    match next.clone() {
                        self::JavaScriptSemanticValue::Object(next_identity, _) => {
                            if (next_identity == identity) {
                                self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Return(std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source3_record_field497 = self::ml_string::Text::from_units(vec![]);
                                    {
                                        let source_ml_source3_record_field498 = heap.clone();
                                        {
                                            let source_ml_source3_record_field499 = false;
                                            ((source_ml_source3_record_field497.clone(), source_ml_source3_record_field498.clone(), source_ml_source3_record_field499), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                        }
                                    }
                                })))
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), next_identity.clone());
                                    {
                                        let parent = {
                                            let source_ml_source2_array_read_value39701 = shared_constructor_field1.clone();
                                            { let ml_reference_array2630 = source_ml_source2_array_read_value39701.clone(); let ml_reference_position2630 = at; let ml_reference_projection2630 = self::ml_array::optional_at(&*ml_reference_array2630.borrow(), self::ml_array::number_index(ml_reference_position2630)); ml_reference_projection2630 }
                                        };
                                        if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source3_record_field500 = self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
                                                {
                                                    let source_ml_source3_record_field501 = heap.clone();
                                                    {
                                                        let source_ml_source3_record_field502 = false;
                                                        ((source_ml_source3_record_field500.clone(), source_ml_source3_record_field501.clone(), source_ml_source3_record_field502), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                    }
                                                }
                                            })))
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
                                                self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source3_record_field503 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 116, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 105, 108, 101, 110, 116, 108, 121, 32, 116, 114, 97, 118, 101, 114, 115, 101, 32, 97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114]);
                                                    {
                                                        let source_ml_source3_record_field504 = heap.clone();
                                                        {
                                                            let source_ml_source3_record_field505 = false;
                                                            ((source_ml_source3_record_field503.clone(), source_ml_source3_record_field504.clone(), source_ml_source3_record_field505), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                        }
                                                    }
                                                })))
                                            } else {
                                                {
                                                    let next_2 = (((match parent.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value39776, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39776 = *source_ml_source2_data_property_value39776;
                                                            Some((source_ml_source2_data_property_value39776.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value39777, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39777 = *source_ml_source2_data_property_value39777;
                                                            Some((source_ml_source2_data_property_value39777.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value39778, _, _, _) => {
                                                            let source_ml_source2_data_property_value39778 = *source_ml_source2_data_property_value39778;
                                                            Some((source_ml_source2_data_property_value39778.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value39779, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39779 = *source_ml_source2_data_property_value39779;
                                                            Some((source_ml_source2_data_property_value39779.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value39780, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39780 = *source_ml_source2_data_property_value39780;
                                                            Some((source_ml_source2_data_property_value39780.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value39781, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39781 = *source_ml_source2_data_property_value39781;
                                                            Some((source_ml_source2_data_property_value39781.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value39782, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39782 = *source_ml_source2_data_property_value39782;
                                                            Some((source_ml_source2_data_property_value39782.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value39783, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39783 = *source_ml_source2_data_property_value39783;
                                                            Some((source_ml_source2_data_property_value39783.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value39784, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39784 = *source_ml_source2_data_property_value39784;
                                                            Some((source_ml_source2_data_property_value39784.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::JavaScriptSemanticValue,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone();
                                                    {
                                                        let depth_2 = (depth + 1f64);
                                                        {
                                                            (next, depth) = (next_2.clone(), depth_2);
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
                            self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Done
                        }
                    }
                } else {
                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop125Result {
    MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop125Done(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop126(
    mut positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut position: f64,
    mut at: f64,
) -> f64 {
    loop {
        return if ((at
            < ({
                let ml_reference_array2631 = positions.clone();
                let ml_reference_projection2631 = ml_reference_array2631.borrow().len() as f64;
                ml_reference_projection2631
            }))
            && ((({
                let ml_reference_array2632 = positions.clone();
                let ml_reference_position2632 = at;
                let ml_reference_projection2632 = self::ml_array::optional_at(
                    &*ml_reference_array2632.borrow(),
                    self::ml_array::number_index(ml_reference_position2632),
                );
                ml_reference_projection2632
            })
            .clone()
            .unwrap_or_else(|| (-1f64)))
                < position))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop125(
    mut numeric: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut text: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values124: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_source1_temporary_index124: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop125Result {
    loop {
        return if (source_ml_source1_temporary_index124
            < ({
                let ml_reference_array2633 = source_ml_source1_temporary_values124.clone();
                let ml_reference_projection2633 =
                    self::ml::Big::from_u128(ml_reference_array2633.borrow().len() as u128);
                ml_reference_projection2633
            })) {
            {
                let property = {
                    let ml_reference_array2634 = source_ml_source1_temporary_values124.clone();
                    let ml_reference_position2634 = source_ml_source1_temporary_index124.clone();
                    let ml_reference_projection2634 = self::ml_array::at(
                        &*ml_reference_array2634.borrow(),
                        ml_reference_position2634.to_index(),
                    );
                    ml_reference_projection2634
                };
                {
                    let key = match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source2_data_property_value39791,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value39791 =
                                *source_ml_source2_data_property_value39791;
                            source_ml_source2_data_property_value39791.clone()
                        }
                    };
                    if (match key.clone() {
                        self::JavaScriptSemanticValue::Text(_) => true,
                        _ => false,
                    } || match key.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => true,
                        _ => false,
                    }) {
                        {
                            let position =
                                if ((self::read_java_script_semantic_text_length(key.clone()))
                                    > (10f64))
                                {
                                    (-1f64)
                                } else {
                                    self::read_java_script_literal_property_index(
                                        self::construct_java_script_semantic_text_units(
                                            key.clone(),
                                            10f64,
                                        ),
                                    )
                                };
                            if (position < (0f64)) {
                                {
                                    let text_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value39796 =
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                key.clone(),
                                            ]));
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::JavaScriptSemanticValue>::new(),
                                                {
                                                    let ml_reference_value2635 =
                                                        text.borrow().clone();
                                                    ml_reference_value2635
                                                },
                                            ),
                                            {
                                                let ml_reference_value2636 =
                                                    source_ml_source2_concat_value39796
                                                        .borrow()
                                                        .clone();
                                                ml_reference_value2636
                                            },
                                        )
                                    }));
                                    {
                                        let source_ml_source1_temporary_index124_2 =
                                            source_ml_source1_temporary_index124
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (text, source_ml_source1_temporary_index124) = (
                                                text_2.clone(),
                                                source_ml_source1_temporary_index124_2.clone(),
                                            );
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = 0f64;
                                    {
                                        let at_2 = self::source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop126(positions.clone(), position, at);
                                        {
                                            let numeric_2 = std::rc::Rc::new(
                                                std::cell::RefCell::new({
                                                    let source_ml_source2_concat_value39797 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            self::ml_array::slice(
                                                                &{
                                                                    let ml_reference_value2637 =
                                                                        numeric.borrow().clone();
                                                                    ml_reference_value2637
                                                                },
                                                                0f64,
                                                                at_2,
                                                            ),
                                                        ));
                                                    {
                                                        let source_ml_source2_concat_value39798 =
                                                            std::rc::Rc::new(
                                                                std::cell::RefCell::new(vec![
                                                                    key.clone(),
                                                                ]),
                                                            );
                                                        {
                                                            let source_ml_source2_concat_value39799 =
                                                                std::rc::Rc::new(
                                                                    std::cell::RefCell::new(
                                                                        self::ml_array::slice(
                                                                            &{
                                                                                let ml_reference_value2638 =
                                                                                    numeric
                                                                                        .borrow()
                                                                                        .clone();
                                                                                ml_reference_value2638
                                                                            },
                                                                            at_2,
                                                                            f64::INFINITY,
                                                                        ),
                                                                    ),
                                                                );
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value2639 = source_ml_source2_concat_value39797.borrow().clone(); ml_reference_value2639 }), { let ml_reference_value2640 = source_ml_source2_concat_value39798.borrow().clone(); ml_reference_value2640 }), { let ml_reference_value2641 = source_ml_source2_concat_value39799.borrow().clone(); ml_reference_value2641 })
                                                        }
                                                    }
                                                }),
                                            );
                                            {
                                                let positions_2 = std::rc::Rc::new(
                                                    std::cell::RefCell::new({
                                                        let source_ml_source2_concat_value39800 =
                                                            std::rc::Rc::new(
                                                                std::cell::RefCell::new(
                                                                    self::ml_array::slice(
                                                                        &{
                                                                            let ml_reference_value2642 =
                                                                                positions
                                                                                    .borrow()
                                                                                    .clone();
                                                                            ml_reference_value2642
                                                                        },
                                                                        0f64,
                                                                        at_2,
                                                                    ),
                                                                ),
                                                            );
                                                        {
                                                            let source_ml_source2_concat_value39801 =
                                                                std::rc::Rc::new(
                                                                    std::cell::RefCell::new(vec![
                                                                        position,
                                                                    ]),
                                                                );
                                                            {
                                                                let source_ml_source2_concat_value39802 =
                                                                    std::rc::Rc::new(
                                                                        std::cell::RefCell::new(
                                                                            self::ml_array::slice(
                                                                                &{
                                                                                    let ml_reference_value2643 =
                                                                                        positions
                                                                                            .borrow(
                                                                                            )
                                                                                            .clone();
                                                                                    ml_reference_value2643
                                                                                },
                                                                                at_2,
                                                                                f64::INFINITY,
                                                                            ),
                                                                        ),
                                                                    );
                                                                self::ml_array::append(
                                                                    self::ml_array::append(
                                                                        self::ml_array::append(
                                                                            Vec::<f64>::new(),
                                                                            {
                                                                                let ml_reference_value2644 = source_ml_source2_concat_value39800.borrow().clone();
                                                                                ml_reference_value2644
                                                                            },
                                                                        ),
                                                                        {
                                                                            let ml_reference_value2645 = source_ml_source2_concat_value39801.borrow().clone();
                                                                            ml_reference_value2645
                                                                        },
                                                                    ),
                                                                    {
                                                                        let ml_reference_value2646 = source_ml_source2_concat_value39802.borrow().clone();
                                                                        ml_reference_value2646
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }),
                                                );
                                                {
                                                    let source_ml_source1_temporary_index124_3 =
                                                        source_ml_source1_temporary_index124
                                                            .add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (
                                                            numeric,
                                                            positions,
                                                            source_ml_source1_temporary_index124,
                                                        ) = (
                                                            numeric_2.clone(),
                                                            positions_2.clone(),
                                                            source_ml_source1_temporary_index124_3
                                                                .clone(),
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
                    } else {
                        {
                            let symbols_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_concat_value39804 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone()]));
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::JavaScriptSemanticValue>::new(),
                                        {
                                            let ml_reference_value2647 = symbols.borrow().clone();
                                            ml_reference_value2647
                                        },
                                    ),
                                    {
                                        let ml_reference_value2648 =
                                            source_ml_source2_concat_value39804.borrow().clone();
                                        ml_reference_value2648
                                    },
                                )
                            }));
                            {
                                let source_ml_source1_temporary_index124_4 =
                                    source_ml_source1_temporary_index124
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_source1_temporary_index124) = (
                                        symbols_2.clone(),
                                        source_ml_source1_temporary_index124_4.clone(),
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop125Result::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop125Done(numeric.clone(), text.clone(), symbols.clone())
        };
    }
}
