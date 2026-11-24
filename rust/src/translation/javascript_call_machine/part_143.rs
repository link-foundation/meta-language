// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptFunctionNameResult;
use super::JavaScriptObjectIntegrityState;
use super::JavaScriptObjectIntegrityTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Result;
use super::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Result;
use super::construct_java_script_semantic_text_units;
use super::create_java_script_property_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::prevent_java_script_ordinary_extensions;
use super::read_java_script_object_integrity_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_own_keys;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_find_java_script_iterator_region_loop1206;
use super::source_ml_source1_temporary_read_java_script_iterator_regions_error_loop1208;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_object_integrity(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectIntegrityState,
) -> self::JavaScriptObjectIntegrityTransition {
    match state.clone() {
        self::JavaScriptObjectIntegrityState::ObjectIntegrity(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_object_integrity_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        if (shared_constructor_field4
                            == (self::ml_string::Text::from_units(vec![
                                112, 114, 101, 118, 101, 110, 116,
                            ])))
                        {
                            if {
                                let source_ml_source2_s26973 = shared_constructor_field3.clone();
                                match source_ml_source2_s26973.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => false,
                                    _ => true,
                                }
                            } {
                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 99, 104,
                                        97, 110, 103, 101, 115, 32, 114, 101, 116, 97, 105, 110,
                                        32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108,
                                        32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116,
                                        105, 116, 121,
                                    ]),
                                )
                            } else {
                                {
                                    let result = self::prevent_java_script_ordinary_extensions(
                                        heap.clone(),
                                        (((match shared_constructor_field3.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                source_ml_source2_data_property_value26978,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value26978
                                                    .clone(),))
                                            }
                                            _ => None::<(self::ml::Big,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0)
                                            .clone(),
                                    );
                                    if (((({
                                        let ml_reference_value1469 = result.borrow().clone();
                                        ml_reference_value1469
                                    })
                                    .0)
                                        .0)
                                        != (self::ml_string::Text::from_units(vec![])))
                                    {
                                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new((({ let ml_reference_value1470 = result.borrow().clone(); ml_reference_value1470 }).0).1), (({ let ml_reference_value1471 = result.borrow().clone(); ml_reference_value1471 }).0).0)
                                    } else {
                                        if !((({
                                            let ml_reference_value1472 = result.borrow().clone();
                                            ml_reference_value1472
                                        })
                                        .0)
                                            .2)
                                        {
                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(Box::new((({ let ml_reference_value1473 = result.borrow().clone(); ml_reference_value1473 }).0).1), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 112, 114, 101, 118, 101, 110, 116, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 115, 32, 111, 110, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                                        } else {
                                            if (shared_constructor_field2
                                                == (self::ml_string::Text::from_units(vec![
                                                    79, 98, 106, 101, 99, 116, 46, 112, 114, 101,
                                                    118, 101, 110, 116, 69, 120, 116, 101, 110,
                                                    115, 105, 111, 110, 115,
                                                ])))
                                            {
                                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new((({ let ml_reference_value1474 = result.borrow().clone(); ml_reference_value1474 }).0).1), Box::new(shared_constructor_field3.clone()))
                                            } else {
                                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityContinue(Box::new((({ let ml_reference_value1475 = result.borrow().clone(); ml_reference_value1475 }).0).1), Box::new({
                                                        let source_ml_source2_nominal_field26983 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field26985 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field26987 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field26989 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field26991 = shared_constructor_field6;
                                                                        self::JavaScriptObjectIntegrityState::ObjectIntegrity(source_ml_source2_nominal_field26983.clone(), Box::new(source_ml_source2_nominal_field26985.clone()), self::ml_string::Text::from_units(vec![107, 101, 121, 115]), source_ml_source2_nominal_field26989.clone(), source_ml_source2_nominal_field26991)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }))
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if {
                                let source_ml_source2_s26995 = shared_constructor_field3.clone();
                                match source_ml_source2_s26995.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => false,
                                    _ => true,
                                }
                            } {
                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 99, 104,
                                        97, 110, 103, 101, 115, 32, 114, 101, 116, 97, 105, 110,
                                        32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108,
                                        32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116,
                                        105, 116, 121,
                                    ]),
                                )
                            } else {
                                if (shared_constructor_field4
                                    == (self::ml_string::Text::from_units(vec![
                                        107, 101, 121, 115,
                                    ])))
                                {
                                    {
                                        let own = self::read_java_script_ordinary_own_keys(
                                            heap.clone(),
                                            (((match shared_constructor_field3.clone() {
                                                self::JavaScriptSemanticValue::Object(
                                                    source_ml_source2_data_property_value27002,
                                                    _,
                                                ) => Some((
                                                    source_ml_source2_data_property_value27002
                                                        .clone(),
                                                )),
                                                _ => None::<(self::ml::Big,)>,
                                            })
                                            .clone()
                                            .expect("a proven present value"))
                                            .0)
                                                .clone(),
                                        );
                                        if (((({
                                            let ml_reference_value1476 = own.borrow().clone();
                                            ml_reference_value1476
                                        })
                                        .0)
                                            .0)
                                            != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), (({ let ml_reference_value1477 = own.borrow().clone(); ml_reference_value1477 }).0).0)
                                        } else {
                                            if (({
                                                let ml_reference_array1479 = (({
                                                    let ml_reference_value1478 =
                                                        own.borrow().clone();
                                                    ml_reference_value1478
                                                })
                                                .0)
                                                    .1;
                                                let ml_reference_projection1479 =
                                                    ml_reference_array1479.borrow().len() as f64;
                                                ml_reference_projection1479
                                            }) > (4096f64))
                                            {
                                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field27004 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27006 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27008 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27010 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field27012 = shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_override27015 = (({ let ml_reference_value1480 = own.borrow().clone(); ml_reference_value1480 }).0).1;
                                                                            self::JavaScriptObjectIntegrityState::ObjectIntegrity(source_ml_source2_nominal_field27004.clone(), Box::new(source_ml_source2_nominal_field27006.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), source_ml_source2_nominal_override27015.clone(), source_ml_source2_nominal_field27012)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }))
                                            }
                                        }
                                    }
                                } else {
                                    if (shared_constructor_field6
                                        == ({
                                            let ml_reference_array1481 =
                                                shared_constructor_field5.clone();
                                            let ml_reference_projection1481 =
                                                ml_reference_array1481.borrow().len() as f64;
                                            ml_reference_projection1481
                                        }))
                                    {
                                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field3.clone()))
                                    } else {
                                        {
                                            let key = {
                                                let source_ml_source2_array_read_value27023 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_array_read_index27024 =
                                                        shared_constructor_field6;
                                                    {
                                                        let ml_reference_array1482 =
                                                            source_ml_source2_array_read_value27023
                                                                .clone();
                                                        let ml_reference_position1482 =
                                                            source_ml_source2_array_read_index27024;
                                                        let ml_reference_projection1482 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1482.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1482,
                                                                ),
                                                            );
                                                        ml_reference_projection1482
                                                    }
                                                }
                                            };
                                            if match (key.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 99, 104, 97, 110, 103, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 32, 107, 101, 121]))
                                            } else {
                                                {
                                                    let object = {
                                                        let source_ml_source2_array_read_value27029 =
                                                            shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_array_read_index27030 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field3.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value27028, _) => {
                                                                        Some((source_ml_source2_data_property_value27028.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone());
                                                            {
                                                                let ml_reference_array1483 = source_ml_source2_array_read_value27029.clone();
                                                                let ml_reference_position1483 = source_ml_source2_array_read_index27030;
                                                                let ml_reference_projection1483 = self::ml_array::optional_at(&*ml_reference_array1483.borrow(), self::ml_array::number_index(ml_reference_position1483));
                                                                ml_reference_projection1483
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let property = {
                                                            let source_ml_source2_optional_chain_receiver27031 =
                                                                (object.clone(),);
                                                            match if match (source_ml_source2_optional_chain_receiver27031.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    Some({
                                                                        let source_ml_source1_temporary_optional_chain1202 = ((source_ml_source2_optional_chain_receiver27031.clone()).0).clone().expect("a proven present value");
                                                                        {
                                                                            let source_ml_source2_array_read_value27060 = match source_ml_source1_temporary_optional_chain1202.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value27032, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value27032.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value27033, _, _, _) => {
                                                                                    source_ml_source2_data_property_value27033.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value27034) => {
                                                                                    source_ml_source2_data_property_value27034.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value27035, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value27035.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value27036, _) => {
                                                                                    source_ml_source2_data_property_value27036.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value27037, _) => {
                                                                                    source_ml_source2_data_property_value27037.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value27038, _) => {
                                                                                    source_ml_source2_data_property_value27038.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value27039, _) => {
                                                                                    source_ml_source2_data_property_value27039.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value27040) => {
                                                                                    source_ml_source2_data_property_value27040.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value27041, _, _) => {
                                                                                    source_ml_source2_data_property_value27041.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value27042, _, _) => {
                                                                                    source_ml_source2_data_property_value27042.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value27043, _, _) => {
                                                                                    source_ml_source2_data_property_value27043.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value27044, _, _) => {
                                                                                    source_ml_source2_data_property_value27044.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_array_read_index27061 = self::find_java_script_ordinary_property({
                                                                                    let source_ml_source2_nullish_field_receiver27045 = (object.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver27045.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present27046 = ((source_ml_source2_nullish_field_receiver27045.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present27046.clone() {
                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value27047, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value27047.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value27048, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value27048.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value27049) => {
                                                                                                    source_ml_source2_data_property_value27049.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value27050, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value27050.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value27051, _) => {
                                                                                                    source_ml_source2_data_property_value27051.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value27052, _) => {
                                                                                                    source_ml_source2_data_property_value27052.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value27053, _) => {
                                                                                                    source_ml_source2_data_property_value27053.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value27054, _) => {
                                                                                                    source_ml_source2_data_property_value27054.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value27055) => {
                                                                                                    source_ml_source2_data_property_value27055.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value27056, _, _) => {
                                                                                                    source_ml_source2_data_property_value27056.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value27057, _, _) => {
                                                                                                    source_ml_source2_data_property_value27057.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value27058, _, _) => {
                                                                                                    source_ml_source2_data_property_value27058.clone()
                                                                                                }
                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value27059, _, _) => {
                                                                                                    source_ml_source2_data_property_value27059.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                }, key.clone().expect("a proven present value"));
                                                                                { let ml_reference_array1484 = source_ml_source2_array_read_value27060.clone(); let ml_reference_position1484 = source_ml_source2_array_read_index27061; let ml_reference_projection1484 = self::ml_array::optional_at(&*ml_reference_array1484.borrow(), self::ml_array::number_index(ml_reference_position1484)); ml_reference_projection1484 }
                                                                            }
                                                                        }
                                                                    })
                                                                } else {
                                                                    None::<self::JavaScriptOrdinaryProperty>
                                                                } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                        };
                                                        if match (property.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        } {
                                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityContinue(Box::new(heap.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field27064 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field27066 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field27068 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field27070 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field27072 = shared_constructor_field6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_override27074 = (shared_constructor_field6 + 1f64);
                                                                                        self::JavaScriptObjectIntegrityState::ObjectIntegrity(source_ml_source2_nominal_field27064.clone(), Box::new(source_ml_source2_nominal_field27066.clone()), source_ml_source2_nominal_field27068.clone(), source_ml_source2_nominal_field27070.clone(), source_ml_source2_nominal_override27074)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }))
                                                        } else {
                                                            {
                                                                let descriptor = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                                                                {
                                                                    let source_ml_source1_temporary_step_java_script_object_integrity_written1203 = {
                                                                        let ml_field_owner1485 =
                                                                            descriptor.clone();
                                                                        let ml_field_value1485 =
                                                                            false;
                                                                        ml_field_owner1485
                                                                            .borrow_mut()
                                                                            .0
                                                                            .3 = ml_field_value1485;
                                                                        ()
                                                                    };
                                                                    {
                                                                        let source_ml_source1_temporary_step_java_script_object_integrity_written1204 = {
                                                                            let ml_field_owner1486 =
                                                                                descriptor.clone();
                                                                            let ml_field_value1486 =
                                                                                false;
                                                                            ml_field_owner1486
                                                                                .borrow_mut()
                                                                                .0
                                                                                .9 =
                                                                                ml_field_value1486;
                                                                            ()
                                                                        };
                                                                        {
                                                                            let source_ml_source1_temporary_step_java_script_object_integrity_written1205 = {
                                                                                let ml_field_owner1487 =
                                                                                    descriptor
                                                                                        .clone();
                                                                                let ml_field_value1487 = ((shared_constructor_field2 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 101, 101, 122, 101]))) && ((self::read_java_script_property_descriptor_kind(match property.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value27078) => {
                                                                                        source_ml_source2_data_property_value27078.clone()
                                                                                    }
                                                                                })) == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))));
                                                                                ml_field_owner1487.borrow_mut().0.11 = ml_field_value1487;
                                                                                ()
                                                                            };
                                                                            {
                                                                                let defined = self::define_java_script_ordinary_property(heap.clone(), (((match shared_constructor_field3.clone() {
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value27080, _) => {
                                                                                            Some((source_ml_source2_data_property_value27080.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone(), key.clone().expect("a proven present value"), descriptor.clone());
                                                                                if (((({ let ml_reference_value1488 = defined.borrow().clone(); ml_reference_value1488 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new((({ let ml_reference_value1489 = defined.borrow().clone(); ml_reference_value1489 }).0).1), (({ let ml_reference_value1490 = defined.borrow().clone(); ml_reference_value1490 }).0).0)
                                                                                    } else {
                                                                                        if !((({ let ml_reference_value1491 = defined.borrow().clone(); ml_reference_value1491 }).0).3) {
                                                                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(Box::new((({ let ml_reference_value1492 = defined.borrow().clone(); ml_reference_value1492 }).0).1), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 115, 101, 116, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 108, 101, 118, 101, 108]))
                                                                                        } else {
                                                                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityContinue(Box::new((({ let ml_reference_value1493 = defined.borrow().clone(); ml_reference_value1493 }).0).1), Box::new({
                                                                                                let source_ml_source2_nominal_field27083 = shared_constructor_field2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field27085 = shared_constructor_field3.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field27087 = shared_constructor_field4.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field27089 = shared_constructor_field5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field27091 = shared_constructor_field6;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override27093 = (shared_constructor_field6 + 1f64);
                                                                                                                    self::JavaScriptObjectIntegrityState::ObjectIntegrity(source_ml_source2_nominal_field27083.clone(), Box::new(source_ml_source2_nominal_field27085.clone()), source_ml_source2_nominal_field27087.clone(), source_ml_source2_nominal_field27089.clone(), source_ml_source2_nominal_override27093)
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
pub fn construct_java_script_property_function_name(
    key: self::JavaScriptSemanticValue,
    prefix: &self::ml_string::Text,
) -> self::JavaScriptFunctionNameResult {
    {
        let error = self::read_java_script_semantic_value_error(key.clone());
        if ((error != (self::ml_string::Text::from_units(vec![])))
            || !(self::ml_array::includes(
                &{
                    let ml_reference_value1494 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![]),
                        self::ml_string::Text::from_units(vec![103, 101, 116]),
                        self::ml_string::Text::from_units(vec![115, 101, 116]),
                    ])))
                    .borrow()
                    .clone();
                    ml_reference_value1494
                },
                prefix,
                0f64,
                false,
                |a, b| a == b,
            )))
        {
            self::JavaScriptFunctionNameResult::FunctionNameInvalid(if !error.0.is_empty() {
                error.clone()
            } else {
                self::ml_string::Text::from_units(vec![
                    102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101, 115, 32, 114,
                    101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 101, 109, 112, 116,
                    121, 47, 103, 101, 116, 47, 115, 101, 116, 32, 112, 114, 101, 102, 105, 120,
                    101, 115,
                ])
            })
        } else {
            match key.clone() {
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::JavaScriptFunctionNameResult::FunctionNameSymbolDescription(
                        Box::new(key.clone()),
                        prefix.clone(),
                    )
                }
                _ => {
                    if (match key.clone() {
                        self::JavaScriptSemanticValue::Text(_) => false,
                        _ => true,
                    } && match key.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => false,
                        _ => true,
                    }) {
                        self::JavaScriptFunctionNameResult::FunctionNameInvalid(
                            self::ml_string::Text::from_units(vec![
                                102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101, 115,
                                32, 99, 111, 110, 115, 117, 109, 101, 32, 97, 32, 99, 111, 109,
                                112, 108, 101, 116, 101, 100, 32, 116, 101, 120, 116, 32, 111, 114,
                                32, 83, 121, 109, 98, 111, 108, 32, 112, 114, 111, 112, 101, 114,
                                116, 121, 32, 107, 101, 121,
                            ]),
                        )
                    } else {
                        {
                            let size = self::read_java_script_semantic_text_length(key.clone());
                            {
                                let leading = if ((*prefix)
                                    == (self::ml_string::Text::from_units(vec![])))
                                {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                } else {
                                    if ((*prefix)
                                        == (self::ml_string::Text::from_units(vec![103, 101, 116])))
                                    {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            103f64, 101f64, 116f64, 32f64,
                                        ]))
                                    } else {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            115f64, 101f64, 116f64, 32f64,
                                        ]))
                                    }
                                };
                                if ((size + {
                                    let ml_reference_array1495 = leading.clone();
                                    let ml_reference_projection1495 =
                                        ml_reference_array1495.borrow().len() as f64;
                                    ml_reference_projection1495
                                }) > (65536f64))
                                {
                                    self::JavaScriptFunctionNameResult::FunctionNameResourceLimit(
                                        self::ml_string::Text::from_units(vec![
                                            102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97,
                                            109, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32,
                                            105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105,
                                            116, 32, 85, 84, 70, 49, 54, 32, 109, 101, 116, 97,
                                            100, 97, 116, 97, 32, 98, 117, 100, 103, 101, 116,
                                        ]),
                                    )
                                } else {
                                    self::JavaScriptFunctionNameResult::FunctionNameValue(Box::new(
                                        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                            std::cell::RefCell::new({
                                                let source_ml_source2_concat_value27100 =
                                                    self::construct_java_script_semantic_text_units(
                                                        key.clone(),
                                                        65536f64,
                                                    );
                                                self::ml_array::append(
                                                    self::ml_array::append(Vec::<f64>::new(), {
                                                        let ml_reference_value1496 =
                                                            leading.borrow().clone();
                                                        ml_reference_value1496
                                                    }),
                                                    {
                                                        let ml_reference_value1497 =
                                                            source_ml_source2_concat_value27100
                                                                .borrow()
                                                                .clone();
                                                        ml_reference_value1497
                                                    },
                                                )
                                            }),
                                        )),
                                    ))
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
pub fn construct_java_script_symbol_function_name(
    description: self::JavaScriptSemanticValue,
    prefix: &self::ml_string::Text,
) -> self::JavaScriptFunctionNameResult {
    {
        let error = self::read_java_script_semantic_value_error(description.clone());
        if ((error != (self::ml_string::Text::from_units(vec![])))
            || ((match description.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                _ => true,
            } && match description.clone() {
                self::JavaScriptSemanticValue::Text(_) => false,
                _ => true,
            }) && match description.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => false,
                _ => true,
            }))
        {
            self::JavaScriptFunctionNameResult::FunctionNameInvalid(if !error.0.is_empty() {
                error.clone()
            } else {
                self::ml_string::Text::from_units(vec![
                    83, 121, 109, 98, 111, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 110,
                    97, 109, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117,
                    97, 108, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 116, 101, 120, 116, 32,
                    100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 115,
                ])
            })
        } else {
            {
                let length = if match description.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    _ => false,
                } {
                    0f64
                } else {
                    self::read_java_script_semantic_text_length(description.clone())
                };
                if (length > (65534f64)) {
                    self::JavaScriptFunctionNameResult::FunctionNameResourceLimit(
                        self::ml_string::Text::from_units(vec![
                            83, 121, 109, 98, 111, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                            32, 110, 97, 109, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105,
                            116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70,
                            49, 54, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 98, 117, 100, 103,
                            101, 116,
                        ]),
                    )
                } else {
                    {
                        let key = self::JavaScriptSemanticValue::Text(
                            if match description.clone() {
                                self::JavaScriptSemanticValue::Undefined => true,
                                _ => false,
                            } {
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                            } else {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value27105 =
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![91f64]));
                                    {
                                        let source_ml_source2_concat_value27106 =
                                            self::construct_java_script_semantic_text_units(
                                                description.clone(),
                                                65536f64,
                                            );
                                        {
                                            let source_ml_source2_concat_value27107 =
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    93f64,
                                                ]));
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(Vec::<f64>::new(), {
                                                        let ml_reference_value1498 =
                                                            source_ml_source2_concat_value27105
                                                                .borrow()
                                                                .clone();
                                                        ml_reference_value1498
                                                    }),
                                                    {
                                                        let ml_reference_value1499 =
                                                            source_ml_source2_concat_value27106
                                                                .borrow()
                                                                .clone();
                                                        ml_reference_value1499
                                                    },
                                                ),
                                                {
                                                    let ml_reference_value1500 =
                                                        source_ml_source2_concat_value27107
                                                            .borrow()
                                                            .clone();
                                                    ml_reference_value1500
                                                },
                                            )
                                        }
                                    }
                                }))
                            },
                        );
                        self::construct_java_script_property_function_name(key.clone(), prefix)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_iterator_region(
    regions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
    slot: f64,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array1501 = regions.clone();
                let ml_reference_projection1501 = ml_reference_array1501.borrow().len() as f64;
                ml_reference_projection1501
            };
            {
                let source_ml_source2_s27108 =
                    self::source_ml_source1_temporary_find_java_script_iterator_region_loop1206(
                        regions.clone(),
                        slot,
                        low,
                        high,
                    );
                match source_ml_source2_s27108.clone() {
                    self::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Result::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Result::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Return(source_ml_source1_temporary_result1206) => {
                        source_ml_source1_temporary_result1206
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_iterator_region_slot(slot: f64) -> bool {
    (((self::ml_math::is_safe_integer(slot) && (slot >= (0f64))) && (slot < (65536f64)))
        && !({
            let source_ml_source2_same_value_operand27110 = (-0f64);
            (((slot == source_ml_source2_same_value_operand27110)
                && ((slot != (0f64))
                    || ((1f64 / slot) == (1f64 / source_ml_source2_same_value_operand27110))))
                || ((slot != slot)
                    && (source_ml_source2_same_value_operand27110
                        != source_ml_source2_same_value_operand27110)))
        }))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_iterator_regions_error(
    heap: self::JavaScriptOrdinaryHeap,
    regions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if (({
                let ml_reference_array1502 = regions.clone();
                let ml_reference_projection1502 = ml_reference_array1502.borrow().len() as f64;
                ml_reference_projection1502
            }) > (128f64))
            {
                self::ml_string::Text::from_units(vec![
                    105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 103, 105, 111, 110, 115,
                    32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116,
                    117, 97, 108, 32, 112, 101, 114, 45, 102, 114, 97, 109, 101, 32, 110, 101, 115,
                    116, 105, 110, 103, 32, 98, 117, 100, 103, 101, 116,
                ])
            } else {
                {
                    let previous = (-1f64);
                    {
                        let source_ml_source1_temporary_index1207 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s27111 = self::source_ml_source1_temporary_read_java_script_iterator_regions_error_loop1208(heap.clone(), previous, regions.clone(), source_ml_source1_temporary_index1207.clone());
                            match source_ml_source2_s27111.clone() {
                                self::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Result::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Done => {
                                    self::ml_string::Text::from_units(vec![])
                                }
                                self::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Result::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Return(source_ml_source1_temporary_result1208) => {
                                    source_ml_source1_temporary_result1208.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
