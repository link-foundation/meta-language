// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptCollectionEntry;
use super::JavaScriptCollectionIteratorResultCompletion;
use super::JavaScriptCollectionIteratorResultState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptStringIterationTransition;
use super::allocate_java_script_array;
use super::create_java_script_property_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_text_unit;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_allocate_java_script_string_iterator_loop1080;
use super::source_ml_source1_temporary_read_java_script_string_iterator_entry_units_loop1082;
use super::step_java_script_collection_iterator_result;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_iterator_result_descriptor(
    value: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
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
> {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            true,
            true,
        );
        {
            let source_ml_source1_temporary_create_java_script_iterator_result_descriptor_written1077 = {
                let ml_field_owner1214 = descriptor.clone();
                let ml_field_value1214 = value.clone();
                ml_field_owner1214.borrow_mut().0.8 = ml_field_value1214;
                ()
            };
            {
                let source_ml_source1_temporary_create_java_script_iterator_result_descriptor_written1078 = {
                    let ml_field_owner1215 = descriptor.clone();
                    let ml_field_value1215 = true;
                    ml_field_owner1215.borrow_mut().0.10 = ml_field_value1215;
                    ()
                };
                descriptor.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_collection_iterator_result_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionIteratorResultState,
    identity: self::ml::Big,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    match state.clone() {
        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            {
                let request =
                    self::step_java_script_collection_iterator_result(heap.clone(), state.clone());
                match request.clone() {
                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultAllocate(
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        if ((identity <= (self::ml::Big::from_i128(0)))
                            || (identity > (self::ml::Big::from_i128(18446744073709551615))))
                        {
                            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                        } else {
                            if (shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![
                                    101, 110, 116, 114, 121,
                                ])))
                            {
                                {
                                    let allocated = self::allocate_java_script_array(
                                        heap.clone(),
                                        identity.clone(),
                                        shared_constructor_field4.clone(),
                                        2f64,
                                    );
                                    match allocated.clone() {
                                        self::JavaScriptArrayCompletion::ArrayResourceLimit(_, reason, _) => {
                                            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), reason.clone())
                                        }
                                        _ => {
                                            match allocated.clone() {
                                                self::JavaScriptArrayCompletion::ArrayNormal(allocated_heap, value) => {
                                                    let allocated_heap = *allocated_heap;
                                                    let value = *value;
                                                    {
                                                        let first = self::define_java_script_ordinary_property(allocated_heap.clone(), identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![48f64]))), self::create_java_script_iterator_result_descriptor(shared_constructor_field6.clone()));
                                                        {
                                                            let second = if (({ let ml_reference_value1216 = first.borrow().clone(); ml_reference_value1216 }).0).3 {
                                                                self::define_java_script_ordinary_property((({ let ml_reference_value1217 = first.borrow().clone(); ml_reference_value1217 }).0).1, identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![49f64]))), self::create_java_script_iterator_result_descriptor(shared_constructor_field7.clone()))
                                                            } else {
                                                                first.clone()
                                                            };
                                                            if (((!((({ let ml_reference_value1218 = first.borrow().clone(); ml_reference_value1218 }).0).3) || !((({ let ml_reference_value1219 = second.borrow().clone(); ml_reference_value1219 }).0).3)) || (((({ let ml_reference_value1220 = first.borrow().clone(); ml_reference_value1220 }).0).0) != (self::ml_string::Text::from_units(vec![])))) || (((({ let ml_reference_value1221 = second.borrow().clone(); ml_reference_value1221 }).0).0) != (self::ml_string::Text::from_units(vec![])))) {
                                                                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104, 32, 101, 110, 116, 114, 121, 32, 65, 114, 114, 97, 121, 32, 102, 105, 101, 108, 100, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                                            } else {
                                                                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(Box::new((({ let ml_reference_value1222 = second.borrow().clone(); ml_reference_value1222 }).0).1), Box::new({
                                                                    let source_ml_source2_nominal_field23720 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field23722 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field23724 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field23726 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field23728 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field23730 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field23732 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field23734 = shared_constructor_field8;
                                                                                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116]), Box::new(source_ml_source2_nominal_field23722.clone()), source_ml_source2_nominal_field23724.clone(), Box::new(source_ml_source2_nominal_field23726.clone()), Box::new(source_ml_source2_nominal_field23728.clone()), Box::new(source_ml_source2_nominal_field23730.clone()), Box::new(value.clone()), source_ml_source2_nominal_field23734)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }))
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 112, 97, 105, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 103, 108, 111, 98, 97, 108, 32, 65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let allocated_2 = self::insert_java_script_ordinary_object(
                                        heap.clone(),
                                        identity.clone(),
                                        shared_constructor_field5.clone(),
                                    );
                                    if !((({
                                        let ml_reference_value1223 = allocated_2.borrow().clone();
                                        ml_reference_value1223
                                    })
                                    .0)
                                        .2)
                                    {
                                        self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), (({ let ml_reference_value1224 = allocated_2.borrow().clone(); ml_reference_value1224 }).0).0)
                                    } else {
                                        {
                                            let value_2 = self::define_java_script_ordinary_property((({ let ml_reference_value1225 = allocated_2.borrow().clone(); ml_reference_value1225 }).0).1, identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64]))), self::create_java_script_iterator_result_descriptor(shared_constructor_field7.clone()));
                                            {
                                                let done = if (({
                                                    let ml_reference_value1226 =
                                                        value_2.borrow().clone();
                                                    ml_reference_value1226
                                                })
                                                .0)
                                                    .3
                                                {
                                                    self::define_java_script_ordinary_property((({ let ml_reference_value1227 = value_2.borrow().clone(); ml_reference_value1227 }).0).1, identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![100f64, 111f64, 110f64, 101f64]))), self::create_java_script_iterator_result_descriptor(self::JavaScriptSemanticValue::Boolean(shared_constructor_field8)))
                                                } else {
                                                    value_2.clone()
                                                };
                                                if (((!((({
                                                    let ml_reference_value1228 =
                                                        value_2.borrow().clone();
                                                    ml_reference_value1228
                                                })
                                                .0)
                                                    .3)
                                                    || !((({
                                                        let ml_reference_value1229 =
                                                            done.borrow().clone();
                                                        ml_reference_value1229
                                                    })
                                                    .0)
                                                        .3))
                                                    || (((({
                                                        let ml_reference_value1230 =
                                                            value_2.borrow().clone();
                                                        ml_reference_value1230
                                                    })
                                                    .0)
                                                        .0)
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        ))))
                                                    || (((({
                                                        let ml_reference_value1231 =
                                                            done.borrow().clone();
                                                        ml_reference_value1231
                                                    })
                                                    .0)
                                                        .0)
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        ))))
                                                {
                                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 118, 97, 108, 117, 101, 47, 100, 111, 110, 101, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115]))
                                                } else {
                                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultNormal(Box::new((({ let ml_reference_value1232 = done.borrow().clone(); ml_reference_value1232 }).0).1), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => request.clone(),
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_string_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptStringIterationTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptStringIterationTransition::StringIteratorInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if ((invalid != (self::ml_string::Text::from_units(vec![])))
                        || ((self::read_java_script_semantic_text_length(value.clone())) < (0f64)))
                    {
                        self::JavaScriptStringIterationTransition::StringIteratorInvalid(
                            Box::new(heap.clone()),
                            if !invalid.0.is_empty() {
                                invalid.clone()
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116,
                                    111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                                    32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
                                    116, 101, 100, 32, 84, 111, 83, 116, 114, 105, 110, 103, 32,
                                    118, 97, 108, 117, 101,
                                ])
                            },
                        )
                    } else {
                        if (({
                            let ml_reference_array1233 = shared_constructor_field1.clone();
                            let ml_reference_projection1233 =
                                ml_reference_array1233.borrow().len() as f64;
                            ml_reference_projection1233
                        }) >= (4096f64))
                        {
                            self::JavaScriptStringIterationTransition::StringIteratorResourceLimit(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116,
                                    111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                                    32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32,
                                    79, 98, 106, 101, 99, 116, 32, 98, 117, 100, 103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let units =
                                    self::read_java_script_semantic_text_length(value.clone());
                                {
                                    let source_ml_source1_temporary_values1079 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source1_temporary_index1079 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let units_2 = self::source_ml_source1_temporary_allocate_java_script_string_iterator_loop1080(units, source_ml_source1_temporary_values1079.clone(), source_ml_source1_temporary_index1079.clone());
                                            if (units_2 > (67108864f64)) {
                                                self::JavaScriptStringIterationTransition::StringIteratorResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                {
                                                    let allocated =
                                                        self::insert_java_script_ordinary_object(
                                                            heap.clone(),
                                                            identity.clone(),
                                                            prototype.clone(),
                                                        );
                                                    if !((({
                                                        let ml_reference_value1234 =
                                                            allocated.borrow().clone();
                                                        ml_reference_value1234
                                                    })
                                                    .0)
                                                        .2)
                                                    {
                                                        self::JavaScriptStringIterationTransition::StringIteratorInvalid(Box::new(heap.clone()), (({ let ml_reference_value1235 = allocated.borrow().clone(); ml_reference_value1235 }).0).0)
                                                    } else {
                                                        {
                                                            let row = {
                                                                let source_ml_source2_array_read_value23747 = match (({ let ml_reference_value1236 = allocated.borrow().clone(); ml_reference_value1236 }).0).1 {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value23745, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value23745.clone()
                                                                        }
                                                                    };
                                                                {
                                                                    let source_ml_source2_array_read_index23748 = self::find_java_script_ordinary_object(match (({ let ml_reference_value1237 = allocated.borrow().clone(); ml_reference_value1237 }).0).1 {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value23746, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value23746.clone()
                                                                            }
                                                                        }, identity.clone());
                                                                    {
                                                                        let ml_reference_array1238 = source_ml_source2_array_read_value23747.clone();
                                                                        let ml_reference_position1238 = source_ml_source2_array_read_index23748;
                                                                        let ml_reference_projection1238 = self::ml_array::optional_at(&*ml_reference_array1238.borrow(), self::ml_array::number_index(ml_reference_position1238));
                                                                        ml_reference_projection1238
                                                                    }
                                                                }
                                                            };
                                                            if (match (row.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match row.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }) {
                                                                    self::JavaScriptStringIterationTransition::StringIteratorInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 79, 98, 106, 101, 99, 116]))
                                                                } else {
                                                                    {
                                                                        let iterator = {
                                                                            let source_ml_source2_constructor_field23759 = (((match row.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value23758, _, _, _, _, _, _, _, _) => {
                                                                                    Some((source_ml_source2_data_property_value23758.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone();
                                                                            {
                                                                                let source_ml_source2_constructor_field23761 = (((match row.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value23760, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value23760 = *source_ml_source2_data_property_value23760;
                                                                                        Some((source_ml_source2_data_property_value23760.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::JavaScriptSemanticValue,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone();
                                                                                {
                                                                                    let source_ml_source2_constructor_field23763 = ((match row.clone().expect("a proven present value") {
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value23762, _, _, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value23762,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(bool,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0;
                                                                                    {
                                                                                        let source_ml_source2_constructor_field23765 = ((match row.clone().expect("a proven present value") {
                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value23764, _) => {
                                                                                                Some((source_ml_source2_data_property_value23764,))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(bool,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0;
                                                                                        {
                                                                                            let source_ml_source2_constructor_field23767 = (((match row.clone().expect("a proven present value") {
                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23766) => {
                                                                                                    Some((source_ml_source2_data_property_value23766.clone(),))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0).clone();
                                                                                            {
                                                                                                let source_ml_source2_constructor_field23768 = self::ml::Big::from_i128(0);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field23769 = 0f64;
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field23772 = Some(value.clone());
                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_constructor_field23759.clone(), Box::new(source_ml_source2_constructor_field23761.clone()), source_ml_source2_constructor_field23763, source_ml_source2_constructor_field23765, source_ml_source2_constructor_field23767.clone(), source_ml_source2_constructor_field23768.clone(), source_ml_source2_constructor_field23769, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]), None::<(Option<bool>,)>, Some((source_ml_source2_constructor_field23772.clone(),)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        };
                                                                        {
                                                                            let updated = self::replace_java_script_ordinary_object((({ let ml_reference_value1239 = allocated.borrow().clone(); ml_reference_value1239 }).0).1, iterator.clone());
                                                                            {
                                                                                let validation = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                                if (validation == (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::JavaScriptStringIterationTransition::StringIteratorAllocated(Box::new(updated.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
                                                                                } else {
                                                                                    self::JavaScriptStringIterationTransition::StringIteratorInvalid(Box::new(heap.clone()), validation.clone())
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
pub fn read_java_script_string_iterator_entry_units(
    entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,
) -> f64 {
    {
        let units = 0f64;
        {
            let source_ml_source1_temporary_index1081 = self::ml::Big::from_i128(0);
            {
                let units_2 = self::source_ml_source1_temporary_read_java_script_string_iterator_entry_units_loop1082(units, entries.clone(), source_ml_source1_temporary_index1081.clone());
                units_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn advance_java_script_string_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
) -> self::JavaScriptStringIterationTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptStringIterationTransition::StringIteratorInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptStringIterationTransition::StringIteratorInvalid(
                            Box::new(heap.clone()),
                            invalid.clone(),
                        )
                    } else {
                        {
                            let iterator = match receiver.clone() {
                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                    let source_ml_source2_array_read_value23776 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index23777 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                identity.clone(),
                                            );
                                        {
                                            let ml_reference_array1240 =
                                                source_ml_source2_array_read_value23776.clone();
                                            let ml_reference_position1240 =
                                                source_ml_source2_array_read_index23777;
                                            let ml_reference_projection1240 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1240.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1240,
                                                    ),
                                                );
                                            ml_reference_projection1240
                                        }
                                    }
                                }
                                _ => None::<self::JavaScriptOrdinaryObject>,
                            };
                            if ((match (iterator.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || match iterator.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
                                    _,
                                ) => false,
                                _ => true,
                            })
                                || (((((match iterator.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value23790,
                                        _,
                                        _,
                                    ) => {
                                        Some((source_ml_source2_data_property_value23790.clone(),))
                                    }
                                    _ => None::<(self::ml_string::Text,)>,
                                })
                                .clone()
                                .expect("a proven present value"))
                                .0)
                                    .clone())
                                    != (self::ml_string::Text::from_units(vec![
                                        83, 116, 114, 105, 110, 103,
                                    ]))))
                            {
                                self::JavaScriptStringIterationTransition::StringIteratorTypeError(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        83, 116, 114, 105, 110, 103, 32, 73, 116, 101, 114, 97,
                                        116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113,
                                        117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                        116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97,
                                        108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105,
                                        118, 101, 114,
                                    ]),
                                )
                            } else {
                                {
                                    let value = (((match iterator.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value23791) => {
                                                Some((((source_ml_source2_data_property_value23791.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone(),))
                                            }
                                            _ => {
                                                None::<(Option<self::JavaScriptSemanticValue>,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone();
                                    if match (value.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptStringIterationTransition::StringIteratorInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 116, 101, 120, 116, 32, 115, 108, 111, 116]))
                                    } else {
                                        match value.clone().expect("a proven present value") {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    self::JavaScriptStringIterationTransition::StringIteratorComponents(Box::new(heap.clone()), Box::new(receiver.clone()), true, Box::new(self::JavaScriptSemanticValue::Undefined))
                                                }
                                                _ => {
                                                    {
                                                        let length = self::read_java_script_semantic_text_length(value.clone().expect("a proven present value"));
                                                        {
                                                            let index = ((match iterator.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, source_ml_source2_data_property_value23792, _, _, _, _) => {
                                                                    Some((source_ml_source2_data_property_value23792,))
                                                                }
                                                                _ => {
                                                                    None::<(f64,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0;
                                                            if (index >= length) {
                                                                self::JavaScriptStringIterationTransition::StringIteratorComponents(Box::new(self::replace_java_script_ordinary_object(heap.clone(), {
                                                                    let source_ml_source2_constructor_field23794 = (((match iterator.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value23793, _, _, _, _, _, _, _, _, _, _) => {
                                                                            Some((source_ml_source2_data_property_value23793.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::ml::Big,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                    {
                                                                        let source_ml_source2_constructor_field23796 = (((match iterator.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value23795, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value23795 = *source_ml_source2_data_property_value23795;
                                                                                Some((source_ml_source2_data_property_value23795.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::JavaScriptSemanticValue,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                        {
                                                                            let source_ml_source2_constructor_field23798 = ((match iterator.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source2_data_property_value23797, _, _, _, _, _, _, _, _) => {
                                                                                    Some((source_ml_source2_data_property_value23797,))
                                                                                }
                                                                                _ => {
                                                                                    None::<(bool,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0;
                                                                            {
                                                                                let source_ml_source2_constructor_field23800 = ((match iterator.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value23799, _, _, _, _, _, _, _) => {
                                                                                        Some((source_ml_source2_data_property_value23799,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(bool,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0;
                                                                                {
                                                                                    let source_ml_source2_constructor_field23802 = (((match iterator.clone().expect("a proven present value") {
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value23801, _, _, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value23801.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                                    {
                                                                                        let source_ml_source2_constructor_field23804 = (((match iterator.clone().expect("a proven present value") {
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, source_ml_source2_data_property_value23803, _, _, _, _, _) => {
                                                                                                Some((source_ml_source2_data_property_value23803.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::ml::Big,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                                        {
                                                                                            let source_ml_source2_constructor_field23805 = 0f64;
                                                                                            {
                                                                                                let source_ml_source2_constructor_field23807 = (((match iterator.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value23806, _, _, _) => {
                                                                                                        Some((source_ml_source2_data_property_value23806.clone(),))
                                                                                                    }
                                                                                                    _ => {
                                                                                                        None::<(self::ml_string::Text,)>
                                                                                                    }
                                                                                                }).clone().expect("a proven present value")).0).clone();
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field23809 = (((match iterator.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23808, _, _) => {
                                                                                                            Some((source_ml_source2_data_property_value23808.clone(),))
                                                                                                        }
                                                                                                        _ => {
                                                                                                            None::<(self::ml_string::Text,)>
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field23810 = Some(self::JavaScriptSemanticValue::Undefined);
                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_constructor_field23794.clone(), Box::new(source_ml_source2_constructor_field23796.clone()), source_ml_source2_constructor_field23798, source_ml_source2_constructor_field23800, source_ml_source2_constructor_field23802.clone(), source_ml_source2_constructor_field23804.clone(), source_ml_source2_constructor_field23805, source_ml_source2_constructor_field23807.clone(), source_ml_source2_constructor_field23809.clone(), None::<(Option<bool>,)>, Some((source_ml_source2_constructor_field23810.clone(),)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                })), Box::new(receiver.clone()), true, Box::new(self::JavaScriptSemanticValue::Undefined))
                                                            } else {
                                                                {
                                                                    let first = self::read_java_script_semantic_text_unit(value.clone().expect("a proven present value"), index);
                                                                    {
                                                                        let units = std::rc::Rc::new(std::cell::RefCell::new(vec![first]));
                                                                        {
                                                                            let count = 1f64;
                                                                            {
                                                                                let count_3 = if (((first >= (55296f64)) && (first <= (56319f64))) && (((index + 1f64)) < length)) {
                                                                                    {
                                                                                        let second = self::read_java_script_semantic_text_unit(value.clone().expect("a proven present value"), (index + 1f64));
                                                                                        if ((second >= (56320f64)) && (second <= (57343f64))) {
                                                                                            {
                                                                                                let source_ml_source1_temporary_advance_java_script_string_iterator_ignored1083 = { let ml_method_owner1241 = units.clone(); let ml_method_argument1241_0 = second; let ml_inserted = vec![ml_method_argument1241_0]; let mut ml_method_values1241 = ml_method_owner1241.borrow_mut(); let ml_length = ml_method_values1241.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values1241.extend(ml_inserted); ml_length as f64 };
                                                                                                {
                                                                                                    let count_2 = 2f64;
                                                                                                    count_2
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            count
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    count
                                                                                };
                                                                                self::JavaScriptStringIterationTransition::StringIteratorComponents(Box::new(self::replace_java_script_ordinary_object(heap.clone(), {
                                                                                    let source_ml_source2_constructor_field23812 = (((match iterator.clone().expect("a proven present value") {
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value23811, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value23811.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                                    {
                                                                                        let source_ml_source2_constructor_field23814 = (((match iterator.clone().expect("a proven present value") {
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value23813, _, _, _, _, _, _, _, _, _) => {
                                                                                                let source_ml_source2_data_property_value23813 = *source_ml_source2_data_property_value23813;
                                                                                                Some((source_ml_source2_data_property_value23813.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::JavaScriptSemanticValue,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                                        {
                                                                                            let source_ml_source2_constructor_field23816 = ((match iterator.clone().expect("a proven present value") {
                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source2_data_property_value23815, _, _, _, _, _, _, _, _) => {
                                                                                                    Some((source_ml_source2_data_property_value23815,))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(bool,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0;
                                                                                            {
                                                                                                let source_ml_source2_constructor_field23818 = ((match iterator.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value23817, _, _, _, _, _, _, _) => {
                                                                                                        Some((source_ml_source2_data_property_value23817,))
                                                                                                    }
                                                                                                    _ => {
                                                                                                        None::<(bool,)>
                                                                                                    }
                                                                                                }).clone().expect("a proven present value")).0;
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field23820 = (((match iterator.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value23819, _, _, _, _, _, _) => {
                                                                                                            Some((source_ml_source2_data_property_value23819.clone(),))
                                                                                                        }
                                                                                                        _ => {
                                                                                                            None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field23822 = (((match iterator.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, source_ml_source2_data_property_value23821, _, _, _, _, _) => {
                                                                                                                Some((source_ml_source2_data_property_value23821.clone(),))
                                                                                                            }
                                                                                                            _ => {
                                                                                                                None::<(self::ml::Big,)>
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                                                        {
                                                                                                            let source_ml_source2_constructor_field23823 = (index + count_3);
                                                                                                            {
                                                                                                                let source_ml_source2_constructor_field23825 = (((match iterator.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value23824, _, _, _) => {
                                                                                                                        Some((source_ml_source2_data_property_value23824.clone(),))
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        None::<(self::ml_string::Text,)>
                                                                                                                    }
                                                                                                                }).clone().expect("a proven present value")).0).clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_constructor_field23827 = (((match iterator.clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23826, _, _) => {
                                                                                                                            Some((source_ml_source2_data_property_value23826.clone(),))
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            None::<(self::ml_string::Text,)>
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_constructor_field23828 = Some(value.clone().expect("a proven present value"));
                                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_constructor_field23812.clone(), Box::new(source_ml_source2_constructor_field23814.clone()), source_ml_source2_constructor_field23816, source_ml_source2_constructor_field23818, source_ml_source2_constructor_field23820.clone(), source_ml_source2_constructor_field23822.clone(), source_ml_source2_constructor_field23823, source_ml_source2_constructor_field23825.clone(), source_ml_source2_constructor_field23827.clone(), None::<(Option<bool>,)>, Some((source_ml_source2_constructor_field23828.clone(),)))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                })), Box::new(receiver.clone()), false, Box::new(self::JavaScriptSemanticValue::Text(units.clone())))
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
