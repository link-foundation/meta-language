// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesCall;
use super::JavaScriptArrayTransformCall;
use super::JavaScriptArrayTransformState;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result;
use super::apply_java_script_descriptor_field;
use super::find_java_script_ordinary_object;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_descriptor_field;
use super::read_java_script_ordinary_value_error;
use super::test_java_script_descriptor_field_presence;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_species_directory_error_loop215(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut intrinsic_realms: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut prior: self::ml::Big,
    mut source_ml_source1_temporary_values210: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptArrayConstructorFact>>,
    >,
    mut source_ml_source1_temporary_index210: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field5,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (source_ml_source1_temporary_index210
                    < ({
                        let ml_reference_array2694 = source_ml_source1_temporary_values210.clone();
                        let ml_reference_projection2694 =
                            self::ml::Big::from_u128(ml_reference_array2694.borrow().len() as u128);
                        ml_reference_projection2694
                    }))
                {
                    {
                        let fact = {
                            let ml_reference_array2695 =
                                source_ml_source1_temporary_values210.clone();
                            let ml_reference_position2695 =
                                source_ml_source1_temporary_index210.clone();
                            let ml_reference_projection2695 = self::ml_array::at(
                                &*ml_reference_array2695.borrow(),
                                ml_reference_position2695.to_index(),
                            );
                            ml_reference_projection2695
                        };
                        match fact.clone() {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                                shared_constructor_field4,
                                _,
                            ) => {
                                if (((((((((({
                                    let source_ml_source2_type_query_operand40147 =
                                        shared_constructor_field1.clone();
                                    self::ml_string::Text::from_units(vec![
                                        98, 105, 103, 105, 110, 116,
                                    ])
                                }) != (self::ml_string::Text::from_units(
                                    vec![98, 105, 103, 105, 110, 116],
                                ))) || (shared_constructor_field1 <= prior))
                                    || (shared_constructor_field1
                                        > (self::ml::Big::from_i128(18446744073709551615))))
                                    || (({
                                        let source_ml_source2_type_query_operand40155 =
                                            shared_constructor_field2;
                                        self::ml_string::Text::from_units(vec![
                                            98, 111, 111, 108, 101, 97, 110,
                                        ])
                                    }) != (self::ml_string::Text::from_units(vec![
                                        98, 111, 111, 108, 101, 97, 110,
                                    ]))))
                                    || (({
                                        let source_ml_source2_type_query_operand40159 =
                                            shared_constructor_field4;
                                        self::ml_string::Text::from_units(vec![
                                            98, 111, 111, 108, 101, 97, 110,
                                        ])
                                    }) != (self::ml_string::Text::from_units(vec![
                                        98, 111, 111, 108, 101, 97, 110,
                                    ]))))
                                    || (({
                                        let source_ml_source2_type_query_operand40163 =
                                            shared_constructor_field3.clone();
                                        self::ml_string::Text::from_units(vec![
                                            98, 105, 103, 105, 110, 116,
                                        ])
                                    }) != (self::ml_string::Text::from_units(vec![
                                        98, 105, 103, 105, 110, 116,
                                    ]))))
                                    || (shared_constructor_field3
                                        <= (self::ml::Big::from_i128(0))))
                                    || (shared_constructor_field3
                                        > (self::ml::Big::from_i128(18446744073709551615))))
                                    || (shared_constructor_field4 && !shared_constructor_field2))
                                {
                                    self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 97, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 44, 32, 114, 101, 97, 108, 109, 115, 32, 97, 110, 100, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 115]))
                                } else {
                                    {
                                        let prior_2 = shared_constructor_field1.clone();
                                        if ({
                                            let source_ml_source2_comparison_operand40172 = match fact.clone() {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value40171) => {
                                                    ((source_ml_source2_data_property_value40171.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_comparison_operand40173 = None::<()>;
                                                !(match (source_ml_source2_comparison_operand40172.clone(), source_ml_source2_comparison_operand40173.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                            }
                                        } && ((((((({
                                            let source_ml_source2_type_query_operand40177 = (match fact.clone() {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value40176) => {
                                                    ((source_ml_source2_data_property_value40176.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value");
                                            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                                        }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || ((((match fact.clone() {
                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value40179) => {
                                                ((source_ml_source2_data_property_value40179.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value")).length()) == (0f64))) || ((((match fact.clone() {
                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value40181) => {
                                                ((source_ml_source2_data_property_value40181.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value")).length()) > (128f64))) || !shared_constructor_field2) || (shared_constructor_field4 && (((match fact.clone() {
                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value40185) => {
                                                ((source_ml_source2_data_property_value40185.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value")) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))))) || ((((match fact.clone() {
                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value40187) => {
                                                ((source_ml_source2_data_property_value40187.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value")) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && !shared_constructor_field4))) {
                                            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Return(self::ml_string::Text::from_units(vec![105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 107, 105, 110, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 108, 121, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 98, 108, 101, 32, 111, 119, 110, 101, 114]))
                                        } else {
                                            {
                                                let object = {
                                                    let source_ml_source2_array_read_value40192 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index40193 = self::find_java_script_ordinary_object(shared_constructor_field5.clone(), shared_constructor_field1.clone());
                                                        { let ml_reference_array2696 = source_ml_source2_array_read_value40192.clone(); let ml_reference_position2696 = source_ml_source2_array_read_index40193; let ml_reference_projection2696 = self::ml_array::optional_at(&*ml_reference_array2696.borrow(), self::ml_array::number_index(ml_reference_position2696)); ml_reference_projection2696 }
                                                    }
                                                };
                                                if (((match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value40194, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value40194
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value40195, _, _, _, _) => {
                                                        source_ml_source2_data_property_value40195
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value40196, _) => {
                                                        source_ml_source2_data_property_value40196
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value40197, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value40197
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value40198, _, _) => {
                                                        source_ml_source2_data_property_value40198
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value40199, _, _) => {
                                                        source_ml_source2_data_property_value40199
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value40200, _, _) => {
                                                        source_ml_source2_data_property_value40200
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value40201, _, _) => {
                                                        source_ml_source2_data_property_value40201
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value40202, _) => {
                                                        source_ml_source2_data_property_value40202
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value40203, _, _, _) => {
                                                        source_ml_source2_data_property_value40203
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value40204, _, _, _) => {
                                                        source_ml_source2_data_property_value40204
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value40205, _, _, _) => {
                                                        source_ml_source2_data_property_value40205
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value40206, _, _, _) => {
                                                        source_ml_source2_data_property_value40206
                                                    }
                                                })) || (match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                } && shared_constructor_field2)) || match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                }) {
                                                    self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 97, 99, 116, 115, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 44, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 97, 98, 108, 101, 32, 97, 114, 114, 111, 119, 32, 99, 108, 111, 115, 117, 114, 101, 115, 32, 111, 114, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 32, 98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                } else {
                                                    if shared_constructor_field4 {
                                                        {
                                                            let source_ml_source1_temporary_read_java_script_array_species_directory_error_ignored220 = { let ml_method_owner2697 = intrinsic_realms.clone(); let ml_method_argument2697_0 = self::JavaScriptSemanticValue::Integer(shared_constructor_field3.clone()); let ml_inserted = vec![ml_method_argument2697_0]; let mut ml_method_values2697 = ml_method_owner2697.borrow_mut(); let ml_length = ml_method_values2697.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2697.extend(ml_inserted); ml_length as f64 };
                                                            {
                                                                let source_ml_source1_temporary_index210_2 = source_ml_source1_temporary_index210.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    (prior, source_ml_source1_temporary_index210) = (prior_2.clone(), source_ml_source1_temporary_index210_2.clone());
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let source_ml_source1_temporary_index210_3 = source_ml_source1_temporary_index210.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                (prior, source_ml_source1_temporary_index210) = (prior_2.clone(), source_ml_source1_temporary_index210_3.clone());
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
                } else {
                    self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Result {
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Done,
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_species_directory_error_loop214(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values209: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index209: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Result {
    loop {
        return if (source_ml_source1_temporary_index209
            < ({
                let ml_reference_array2698 = source_ml_source1_temporary_values209.clone();
                let ml_reference_projection2698 =
                    self::ml::Big::from_u128(ml_reference_array2698.borrow().len() as u128);
                ml_reference_projection2698
            })) {
            {
                let value = {
                    let ml_reference_array2699 = source_ml_source1_temporary_values209.clone();
                    let ml_reference_position2699 = source_ml_source1_temporary_index209.clone();
                    let ml_reference_projection2699 = self::ml_array::at(
                        &*ml_reference_array2699.borrow(),
                        ml_reference_position2699.to_index(),
                    );
                    ml_reference_projection2699
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index209_2 =
                                source_ml_source1_temporary_index209
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index209 =
                                    source_ml_source1_temporary_index209_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224 {
    MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224Next,
    MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Result {
    MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Done,
    MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_species_state_error_loop225(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values222: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index222: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Result {
    loop {
        return if (source_ml_source1_temporary_index222
            < ({
                let ml_reference_array2700 = source_ml_source1_temporary_values222.clone();
                let ml_reference_projection2700 =
                    self::ml::Big::from_u128(ml_reference_array2700.borrow().len() as u128);
                ml_reference_projection2700
            })) {
            {
                let value = {
                    let ml_reference_array2701 = source_ml_source1_temporary_values222.clone();
                    let ml_reference_position2701 = source_ml_source1_temporary_index222.clone();
                    let ml_reference_projection2701 = self::ml_array::at(
                        &*ml_reference_array2701.borrow(),
                        ml_reference_position2701.to_index(),
                    );
                    ml_reference_projection2701
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Result::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index222_2 =
                                source_ml_source1_temporary_index222
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index222 =
                                    source_ml_source1_temporary_index222_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Result::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Result {
    MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Done,
    MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_species_state_error_loop223(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values221: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index221: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Result {
    loop {
        return if (source_ml_source1_temporary_index221
            < ({
                let ml_reference_array2702 = source_ml_source1_temporary_values221.clone();
                let ml_reference_projection2702 =
                    self::ml::Big::from_u128(ml_reference_array2702.borrow().len() as u128);
                ml_reference_projection2702
            })) {
            {
                let value = {
                    let ml_reference_array2703 = source_ml_source1_temporary_values221.clone();
                    let ml_reference_position2703 = source_ml_source1_temporary_index221.clone();
                    let ml_reference_projection2703 = self::ml_array::at(
                        &*ml_reference_array2703.borrow(),
                        ml_reference_position2703.to_index(),
                    );
                    ml_reference_projection2703
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Result::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index221_2 =
                                source_ml_source1_temporary_index221
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index221 =
                                    source_ml_source1_temporary_index221_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Result::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result {
    MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Done,
    MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Return(
        Option<self::JavaScriptSemanticValue>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_constructor_authority_target_loop226(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut target: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result {
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
                        let ml_reference_array2704 = shared_constructor_field1.clone();
                        let ml_reference_projection2704 =
                            ml_reference_array2704.borrow().len() as f64;
                        ml_reference_projection2704
                    }))
                {
                    if (match target.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => false,
                        _ => true,
                    } || !(((match target.clone() {
                        self::JavaScriptSemanticValue::Object(
                            _,
                            source_ml_source2_data_property_value40238,
                        ) => Some((source_ml_source2_data_property_value40238,)),
                        _ => None::<(bool,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0))
                    {
                        self::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Return(None::<self::JavaScriptSemanticValue>)
                    } else {
                        {
                            let object = {
                                let source_ml_source2_array_read_value40242 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_array_read_index40243 =
                                        self::find_java_script_ordinary_object(
                                            shared_constructor_field1.clone(),
                                            (((match target.clone() {
                                                self::JavaScriptSemanticValue::Object(
                                                    source_ml_source2_data_property_value40241,
                                                    _,
                                                ) => Some((
                                                    source_ml_source2_data_property_value40241
                                                        .clone(),
                                                )),
                                                _ => None::<(self::ml::Big,)>,
                                            })
                                            .clone()
                                            .expect("a proven present value"))
                                            .0)
                                                .clone(),
                                        );
                                    {
                                        let ml_reference_array2705 =
                                            source_ml_source2_array_read_value40242.clone();
                                        let ml_reference_position2705 =
                                            source_ml_source2_array_read_index40243;
                                        let ml_reference_projection2705 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array2705.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position2705,
                                                ),
                                            );
                                        ml_reference_projection2705
                                    }
                                }
                            };
                            if match (object.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Return(None::<self::JavaScriptSemanticValue>)
                            } else {
                                match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, target_2, _, _) => {
                                        let target_2 = *target_2;
                                        {
                                            let depth_2 = (depth + 1f64);
                                            {
                                                (target, depth) = (target_2.clone(), depth_2);
                                                continue;
                                            }
                                        }
                                    }
                                    _ => {
                                        self::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Return(Some(target.clone()))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Result {
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Done,
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop227(
    mut state: self::JavaScriptDescriptorConversionState,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Result {
    loop {
        return if (index < (6f64)) {
            if self::test_java_script_descriptor_field_presence(
                match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        _,
                        source_ml_source2_data_property_value40251,
                        _,
                    ) => source_ml_source2_data_property_value40251.clone(),
                },
                index,
            ) {
                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Return(self::ml_string::Text::from_units(vec![117, 110, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 102, 97, 98, 114, 105, 99, 97, 116, 101, 100, 32, 105, 110, 32, 97, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result {
    MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Done(
        Box<self::JavaScriptDescriptorConversionState>,
    ),
    MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Return(
        Box<self::JavaScriptDescriptorConversionTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_descriptor_conversion_loop228(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut current: self::JavaScriptDescriptorConversionState,
) -> self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result {
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
                if ((match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source2_data_property_value40253,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value40253,
                }) < (6f64))
                {
                    {
                        let field =
                            self::read_java_script_descriptor_field(match current.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                    _,
                                    source_ml_source2_data_property_value40254,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value40254,
                            });
                        {
                            let key = self::JavaScriptSemanticValue::Text(
                                (({
                                    let ml_reference_value2706 = field.borrow().clone();
                                    ml_reference_value2706
                                })
                                .0)
                                    .2,
                            );
                            {
                                let receiver = match current.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value40255, _, _, _) => {
                                        let source_ml_source2_data_property_value40255 = *source_ml_source2_data_property_value40255;
                                        source_ml_source2_data_property_value40255.clone()
                                    }
                                };
                                match receiver.clone() {
                                    self::JavaScriptSemanticValue::Object(identity, _) => {
                                        {
                                            let actual = {
                                                let source_ml_source2_array_read_value40259 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_array_read_index40260 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                    { let ml_reference_array2707 = source_ml_source2_array_read_value40259.clone(); let ml_reference_position2707 = source_ml_source2_array_read_index40260; let ml_reference_projection2707 = self::ml_array::optional_at(&*ml_reference_array2707.borrow(), self::ml_array::number_index(ml_reference_position2707)); ml_reference_projection2707 }
                                                }
                                            };
                                            if (!(match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(self::ml_array::includes(&{ let ml_reference_value2708 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value2708 }, &match actual.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                            }, 0f64, false, |a, b| a == b))) {
                                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]))))
                                            } else {
                                                {
                                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                                    if !((({ let ml_reference_value2709 = property.borrow().clone(); ml_reference_value2709 }).0).1) {
                                                        {
                                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value40261, _, _) => {
                                                                    source_ml_source2_data_property_value40261
                                                                }
                                                            } + 1f64), match current.clone() {
                                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value40262, _) => {
                                                                    source_ml_source2_data_property_value40262.clone()
                                                                }
                                                            }, false);
                                                            {
                                                                current = current_2.clone();
                                                                continue;
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let plan = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), key.clone(), receiver.clone());
                                                            if (((({ let ml_reference_value2710 = plan.borrow().clone(); ml_reference_value2710 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid((({ let ml_reference_value2711 = plan.borrow().clone(); ml_reference_value2711 }).0).3)))
                                                            } else {
                                                                if (((({ let ml_reference_value2712 = plan.borrow().clone(); ml_reference_value2712 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value40263, _, _) => {
                                                                            source_ml_source2_data_property_value40263
                                                                        }
                                                                    }, match current.clone() {
                                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value40264, _) => {
                                                                            source_ml_source2_data_property_value40264.clone()
                                                                        }
                                                                    }, true)), Box::new((({ let ml_reference_value2713 = plan.borrow().clone(); ml_reference_value2713 }).0).2), Box::new((({ let ml_reference_value2714 = plan.borrow().clone(); ml_reference_value2714 }).0).4), (({ let ml_reference_value2715 = plan.borrow().clone(); ml_reference_value2715 }).0).1)))
                                                                } else {
                                                                    if (((({ let ml_reference_value2716 = plan.borrow().clone(); ml_reference_value2716 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                        self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))))
                                                                    } else {
                                                                        {
                                                                            let applied = self::apply_java_script_descriptor_field(current.clone(), (({ let ml_reference_value2717 = plan.borrow().clone(); ml_reference_value2717 }).0).5);
                                                                            match applied.clone() {
                                                                                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(current_3) => {
                                                                                    let current_3 = *current_3;
                                                                                    {
                                                                                        current = current_3.clone();
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                                _ => {
                                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Return(Box::new(applied.clone()))
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
                                    _ => {
                                        self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Done(Box::new(current.clone()))
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237 {
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Next,
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Result {
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Done,
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_transform_state_error_loop240(
    mut state: self::JavaScriptArrayTransformState,
    mut call: Option<self::JavaScriptArraySpeciesCall>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Result {
    loop {
        return if (at
            < ({
                let ml_reference_array2718 = {
                    let source_ml_source2_nullish_field_receiver40268 = (call.clone(),);
                    if match (source_ml_source2_nullish_field_receiver40268.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present40269 =
                                ((source_ml_source2_nullish_field_receiver40268.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_source2_nullish_field_present40269.clone() {
                                self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value40270,
                                ) => source_ml_source2_data_property_value40270.clone(),
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114,
                                111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110,
                                100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                110, 103, 32, 39, 97, 114, 103, 117, 109, 101, 110, 116, 115, 39,
                                41,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    }
                };
                let ml_reference_projection2718 = ml_reference_array2718.borrow().len() as f64;
                ml_reference_projection2718
            })) {
            if !(self::test_java_script_semantic_same_value(({
                let source_ml_source2_array_read_value40274 = {
                    let source_ml_source2_nullish_field_receiver40271 = (call.clone(),);
                    if match (source_ml_source2_nullish_field_receiver40271.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present40272 = ((source_ml_source2_nullish_field_receiver40271.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present40272.clone() {
                                self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, _, source_ml_source2_data_property_value40273) => {
                                    source_ml_source2_data_property_value40273.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 97, 114, 103, 117, 109, 101, 110, 116, 115, 39, 41])).0, message_present: true }).raise()
                    }
                };
                { let ml_reference_array2719 = source_ml_source2_array_read_value40274.clone(); let ml_reference_position2719 = at; let ml_reference_projection2719 = self::ml_array::optional_at(&*ml_reference_array2719.borrow(), self::ml_array::number_index(ml_reference_position2719)); ml_reference_projection2719 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                let source_ml_source2_array_read_value40280 = {
                    let source_ml_source2_nullish_field_receiver40277 = (match state.clone() {
                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value40276) => {
                            ((source_ml_source2_data_property_value40276.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                        }
                    },);
                    if match (source_ml_source2_nullish_field_receiver40277.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present40278 = ((source_ml_source2_nullish_field_receiver40277.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present40278.clone() {
                                self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, _, source_ml_source2_data_property_value40279) => {
                                    source_ml_source2_data_property_value40279.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 97, 114, 103, 117, 109, 101, 110, 116, 115, 39, 41])).0, message_present: true }).raise()
                    }
                };
                { let ml_reference_array2720 = source_ml_source2_array_read_value40280.clone(); let ml_reference_position2720 = at; let ml_reference_projection2720 = self::ml_array::optional_at(&*ml_reference_array2720.borrow(), self::ml_array::number_index(ml_reference_position2720)); ml_reference_projection2720 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) {
                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 99, 97, 110, 110, 111, 116, 32, 99, 104, 97, 110, 103, 101, 32, 98, 101, 116, 119, 101, 101, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]))
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
            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239 {
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239Next,
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Result {
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Done,
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Return(self::ml_string::Text),
}
