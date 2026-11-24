// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn delete_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
            {
                let key_error =
                    self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                {
                    let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                        heap_error.clone()
                    } else {
                        key_error.clone()
                    };
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source3_record_field172 = error.clone();
                            {
                                let source_ml_source3_record_field173 = heap.clone();
                                {
                                    let source_ml_source3_record_field174 = false;
                                    ((source_ml_source3_record_field172.clone(), source_ml_source3_record_field173.clone(), source_ml_source3_record_field174), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                }
                            }
                        }))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(
                                shared_constructor_field1.clone(),
                                identity.clone(),
                            );
                            {
                                let object = {
                                    let source_ml_source2_array_read_value1962 =
                                        shared_constructor_field1.clone();
                                    {
                                        let ml_reference_array242 =
                                            source_ml_source2_array_read_value1962.clone();
                                        let ml_reference_position242 = index;
                                        let ml_reference_projection242 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array242.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position242,
                                                ),
                                            );
                                        ml_reference_projection242
                                    }
                                };
                                if match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source3_record_field175 =
                                            self::ml_string::Text::from_units(vec![
                                                68, 101, 108, 101, 116, 101, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105,
                                                100, 101, 110, 116, 105, 116, 121,
                                            ]);
                                        {
                                            let source_ml_source3_record_field176 = heap.clone();
                                            {
                                                let source_ml_source3_record_field177 = false;
                                                ((source_ml_source3_record_field175.clone(), source_ml_source3_record_field176.clone(), source_ml_source3_record_field177), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                            }
                                        }
                                    }))
                                } else {
                                    if ((((((((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source3_record_field178 = self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]);
                                                {
                                                    let source_ml_source3_record_field179 = heap.clone();
                                                    {
                                                        let source_ml_source3_record_field180 = false;
                                                        ((source_ml_source3_record_field178.clone(), source_ml_source3_record_field179.clone(), source_ml_source3_record_field180), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                    }
                                                }
                                            }))
                                        } else {
                                            {
                                                let at = self::find_java_script_ordinary_property((((match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value2037, _, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value2037.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value2038, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value2038.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value2039) => {
                                                        Some((source_ml_source2_data_property_value2039.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value2040, _, _, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value2040.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value2041, _) => {
                                                        Some((source_ml_source2_data_property_value2041.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value2042, _) => {
                                                        Some((source_ml_source2_data_property_value2042.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value2043, _) => {
                                                        Some((source_ml_source2_data_property_value2043.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value2044, _) => {
                                                        Some((source_ml_source2_data_property_value2044.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value2045) => {
                                                        Some((source_ml_source2_data_property_value2045.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                {
                                                    let property = {
                                                        let source_ml_source2_array_read_value2055 = (((match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value2046, _, _, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value2046.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value2047, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value2047.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value2048) => {
                                                                Some((source_ml_source2_data_property_value2048.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value2049, _, _, _, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value2049.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value2050, _) => {
                                                                Some((source_ml_source2_data_property_value2050.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value2051, _) => {
                                                                Some((source_ml_source2_data_property_value2051.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value2052, _) => {
                                                                Some((source_ml_source2_data_property_value2052.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value2053, _) => {
                                                                Some((source_ml_source2_data_property_value2053.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value2054) => {
                                                                Some((source_ml_source2_data_property_value2054.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone();
                                                        { let ml_reference_array243 = source_ml_source2_array_read_value2055.clone(); let ml_reference_position243 = at; let ml_reference_projection243 = self::ml_array::optional_at(&*ml_reference_array243.borrow(), self::ml_array::number_index(ml_reference_position243)); ml_reference_projection243 }
                                                    };
                                                    if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source3_record_field181 = self::ml_string::Text::from_units(vec![]);
                                                            {
                                                                let source_ml_source3_record_field182 = heap.clone();
                                                                {
                                                                    let source_ml_source3_record_field183 = true;
                                                                    ((source_ml_source3_record_field181.clone(), source_ml_source3_record_field182.clone(), source_ml_source3_record_field183), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                                }
                                                            }
                                                        }))
                                                    } else {
                                                        if !((({ let ml_reference_value244 = (match property.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2060) => {
                                                                source_ml_source2_data_property_value2060.clone()
                                                            }
                                                        }).borrow().clone(); ml_reference_value244 }).0).0) {
                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source3_record_field184 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source3_record_field185 = heap.clone();
                                                                    {
                                                                        let source_ml_source3_record_field186 = false;
                                                                        ((source_ml_source3_record_field184.clone(), source_ml_source3_record_field185.clone(), source_ml_source3_record_field186), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                                    }
                                                                }
                                                            }))
                                                        } else {
                                                            {
                                                                let properties = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values245 = { let ml_reference_value246 = ((((match object.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value2064, _, _, _, _, _) => {
                                                                        Some((source_ml_source2_data_property_value2064.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value2065, _, _, _) => {
                                                                        Some((source_ml_source2_data_property_value2065.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value2066) => {
                                                                        Some((source_ml_source2_data_property_value2066.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value2067, _, _, _, _, _, _) => {
                                                                        Some((source_ml_source2_data_property_value2067.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value2068, _) => {
                                                                        Some((source_ml_source2_data_property_value2068.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value2069, _) => {
                                                                        Some((source_ml_source2_data_property_value2069.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value2070, _) => {
                                                                        Some((source_ml_source2_data_property_value2070.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value2071, _) => {
                                                                        Some((source_ml_source2_data_property_value2071.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value2072) => {
                                                                        Some((source_ml_source2_data_property_value2072.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone()).borrow().clone(); ml_reference_value246 }; let ml_callback245 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { (position != at) }) }; ml_values245.iter().cloned().enumerate().filter_map(|(ml_index245, ml_value245)| if ml_callback245(ml_value245.clone(), (ml_index245 as f64)) { Some(ml_value245) } else { None }).collect::<Vec<_>>() }));
                                                                {
                                                                    let replacement = match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field2073, source_ml_source2_nominal_field2074, source_ml_source2_nominal_field2075, source_ml_source2_nominal_field2076, _, source_ml_source2_nominal_field2078, source_ml_source2_nominal_field2079, source_ml_source2_nominal_field2080, source_ml_source2_nominal_field2081, source_ml_source2_nominal_field2082) => {
                                                                            let source_ml_source2_nominal_field2074 = *source_ml_source2_nominal_field2074;
                                                                            let source_ml_source2_nominal_field2081 = *source_ml_source2_nominal_field2081;
                                                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field2073.clone(), Box::new(source_ml_source2_nominal_field2074.clone()), source_ml_source2_nominal_field2075, source_ml_source2_nominal_field2076, properties.clone(), source_ml_source2_nominal_field2078.clone(), source_ml_source2_nominal_field2079, source_ml_source2_nominal_field2080.clone(), Box::new(source_ml_source2_nominal_field2081.clone()), source_ml_source2_nominal_field2082.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field2084, source_ml_source2_nominal_field2085, source_ml_source2_nominal_field2086, source_ml_source2_nominal_field2087, _, source_ml_source2_nominal_field2089, source_ml_source2_nominal_field2090, source_ml_source2_nominal_field2091) => {
                                                                            let source_ml_source2_nominal_field2085 = *source_ml_source2_nominal_field2085;
                                                                            let source_ml_source2_nominal_field2089 = *source_ml_source2_nominal_field2089;
                                                                            let source_ml_source2_nominal_field2090 = *source_ml_source2_nominal_field2090;
                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field2084.clone(), Box::new(source_ml_source2_nominal_field2085.clone()), source_ml_source2_nominal_field2086, source_ml_source2_nominal_field2087, properties.clone(), Box::new(source_ml_source2_nominal_field2089.clone()), Box::new(source_ml_source2_nominal_field2090.clone()), source_ml_source2_nominal_field2091.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field2093, source_ml_source2_nominal_field2094, source_ml_source2_nominal_field2095, source_ml_source2_nominal_field2096, _) => {
                                                                            let source_ml_source2_nominal_field2094 = *source_ml_source2_nominal_field2094;
                                                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field2093.clone(), Box::new(source_ml_source2_nominal_field2094.clone()), source_ml_source2_nominal_field2095, source_ml_source2_nominal_field2096, properties.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field2099, source_ml_source2_nominal_field2100, source_ml_source2_nominal_field2101, source_ml_source2_nominal_field2102, _, source_ml_source2_nominal_field2104, source_ml_source2_nominal_field2105, source_ml_source2_nominal_field2106, source_ml_source2_nominal_field2107, source_ml_source2_nominal_field2108, source_ml_source2_nominal_field2109) => {
                                                                            let source_ml_source2_nominal_field2100 = *source_ml_source2_nominal_field2100;
                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field2099.clone(), Box::new(source_ml_source2_nominal_field2100.clone()), source_ml_source2_nominal_field2101, source_ml_source2_nominal_field2102, properties.clone(), source_ml_source2_nominal_field2104.clone(), source_ml_source2_nominal_field2105, source_ml_source2_nominal_field2106.clone(), source_ml_source2_nominal_field2107.clone(), source_ml_source2_nominal_field2108.clone(), source_ml_source2_nominal_field2109.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field2111, source_ml_source2_nominal_field2112, source_ml_source2_nominal_field2113, source_ml_source2_nominal_field2114, _, source_ml_source2_nominal_field2116) => {
                                                                            let source_ml_source2_nominal_field2112 = *source_ml_source2_nominal_field2112;
                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field2111.clone(), Box::new(source_ml_source2_nominal_field2112.clone()), source_ml_source2_nominal_field2113, source_ml_source2_nominal_field2114, properties.clone(), source_ml_source2_nominal_field2116.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field2118, source_ml_source2_nominal_field2119, source_ml_source2_nominal_field2120, source_ml_source2_nominal_field2121, _, source_ml_source2_nominal_field2123) => {
                                                                            let source_ml_source2_nominal_field2119 = *source_ml_source2_nominal_field2119;
                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field2118.clone(), Box::new(source_ml_source2_nominal_field2119.clone()), source_ml_source2_nominal_field2120, source_ml_source2_nominal_field2121, properties.clone(), source_ml_source2_nominal_field2123.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field2125, source_ml_source2_nominal_field2126, source_ml_source2_nominal_field2127, source_ml_source2_nominal_field2128, _, source_ml_source2_nominal_field2130) => {
                                                                            let source_ml_source2_nominal_field2126 = *source_ml_source2_nominal_field2126;
                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field2125.clone(), Box::new(source_ml_source2_nominal_field2126.clone()), source_ml_source2_nominal_field2127, source_ml_source2_nominal_field2128, properties.clone(), source_ml_source2_nominal_field2130.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field2132, source_ml_source2_nominal_field2133, source_ml_source2_nominal_field2134, source_ml_source2_nominal_field2135, _, source_ml_source2_nominal_field2137) => {
                                                                            let source_ml_source2_nominal_field2133 = *source_ml_source2_nominal_field2133;
                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field2132.clone(), Box::new(source_ml_source2_nominal_field2133.clone()), source_ml_source2_nominal_field2134, source_ml_source2_nominal_field2135, properties.clone(), source_ml_source2_nominal_field2137.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field2139, source_ml_source2_nominal_field2140, source_ml_source2_nominal_field2141, source_ml_source2_nominal_field2142, source_ml_source2_nominal_field2143, source_ml_source2_nominal_field2144, source_ml_source2_nominal_field2145, source_ml_source2_nominal_field2146, _) => {
                                                                            let source_ml_source2_nominal_field2140 = *source_ml_source2_nominal_field2140;
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field2139.clone(), Box::new(source_ml_source2_nominal_field2140.clone()), source_ml_source2_nominal_field2141, source_ml_source2_nominal_field2142.clone(), source_ml_source2_nominal_field2143.clone(), source_ml_source2_nominal_field2144.clone(), source_ml_source2_nominal_field2145.clone(), source_ml_source2_nominal_field2146, properties.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field2149, source_ml_source2_nominal_field2150, source_ml_source2_nominal_field2151, source_ml_source2_nominal_field2152, _, source_ml_source2_nominal_field2154, source_ml_source2_nominal_field2155) => {
                                                                            let source_ml_source2_nominal_field2150 = *source_ml_source2_nominal_field2150;
                                                                            let source_ml_source2_nominal_field2154 = *source_ml_source2_nominal_field2154;
                                                                            let source_ml_source2_nominal_field2155 = *source_ml_source2_nominal_field2155;
                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field2149.clone(), Box::new(source_ml_source2_nominal_field2150.clone()), source_ml_source2_nominal_field2151, source_ml_source2_nominal_field2152, properties.clone(), Box::new(source_ml_source2_nominal_field2154.clone()), Box::new(source_ml_source2_nominal_field2155.clone()))
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field2157, source_ml_source2_nominal_field2158, source_ml_source2_nominal_field2159, source_ml_source2_nominal_field2160, _, source_ml_source2_nominal_field2162, source_ml_source2_nominal_field2163) => {
                                                                            let source_ml_source2_nominal_field2158 = *source_ml_source2_nominal_field2158;
                                                                            let source_ml_source2_nominal_field2162 = *source_ml_source2_nominal_field2162;
                                                                            let source_ml_source2_nominal_field2163 = *source_ml_source2_nominal_field2163;
                                                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field2157.clone(), Box::new(source_ml_source2_nominal_field2158.clone()), source_ml_source2_nominal_field2159, source_ml_source2_nominal_field2160, properties.clone(), Box::new(source_ml_source2_nominal_field2162.clone()), Box::new(source_ml_source2_nominal_field2163.clone()))
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field2165, source_ml_source2_nominal_field2166, source_ml_source2_nominal_field2167, source_ml_source2_nominal_field2168, _, source_ml_source2_nominal_field2170, source_ml_source2_nominal_field2171) => {
                                                                            let source_ml_source2_nominal_field2166 = *source_ml_source2_nominal_field2166;
                                                                            let source_ml_source2_nominal_field2170 = *source_ml_source2_nominal_field2170;
                                                                            let source_ml_source2_nominal_field2171 = *source_ml_source2_nominal_field2171;
                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field2165.clone(), Box::new(source_ml_source2_nominal_field2166.clone()), source_ml_source2_nominal_field2167, source_ml_source2_nominal_field2168, properties.clone(), Box::new(source_ml_source2_nominal_field2170.clone()), Box::new(source_ml_source2_nominal_field2171.clone()))
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field2173, source_ml_source2_nominal_field2174, source_ml_source2_nominal_field2175, source_ml_source2_nominal_field2176, _, source_ml_source2_nominal_field2178, source_ml_source2_nominal_field2179) => {
                                                                            let source_ml_source2_nominal_field2174 = *source_ml_source2_nominal_field2174;
                                                                            let source_ml_source2_nominal_field2178 = *source_ml_source2_nominal_field2178;
                                                                            let source_ml_source2_nominal_field2179 = *source_ml_source2_nominal_field2179;
                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field2173.clone(), Box::new(source_ml_source2_nominal_field2174.clone()), source_ml_source2_nominal_field2175, source_ml_source2_nominal_field2176, properties.clone(), Box::new(source_ml_source2_nominal_field2178.clone()), Box::new(source_ml_source2_nominal_field2179.clone()))
                                                                        }
                                                                    };
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source2_field2181 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                        {
                                                                            let source_ml_source3_record_field187 = self::ml_string::Text::from_units(vec![]);
                                                                            {
                                                                                let source_ml_source3_record_field188 = source_ml_source2_field2181.clone();
                                                                                {
                                                                                    let source_ml_source3_record_field189 = true;
                                                                                    ((source_ml_source3_record_field187.clone(), source_ml_source3_record_field188.clone(), source_ml_source3_record_field189), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn prevent_java_script_ordinary_extensions(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source3_record_field190 = error.clone();
                    {
                        let source_ml_source3_record_field191 = heap.clone();
                        {
                            let source_ml_source3_record_field192 = false;
                            ((source_ml_source3_record_field190.clone(), source_ml_source3_record_field191.clone(), source_ml_source3_record_field192), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                        }
                    }
                }))
            } else {
                {
                    let index = self::find_java_script_ordinary_object(
                        shared_constructor_field1.clone(),
                        identity.clone(),
                    );
                    {
                        let object = {
                            let source_ml_source2_array_read_value2189 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array247 =
                                    source_ml_source2_array_read_value2189.clone();
                                let ml_reference_position247 = index;
                                let ml_reference_projection247 = self::ml_array::optional_at(
                                    &*ml_reference_array247.borrow(),
                                    self::ml_array::number_index(ml_reference_position247),
                                );
                                ml_reference_projection247
                            }
                        };
                        if match (object.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source3_record_field193 =
                                    self::ml_string::Text::from_units(vec![
                                        80, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110,
                                        115, 105, 111, 110, 115, 32, 110, 101, 101, 100, 115, 32,
                                        105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98,
                                        106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116,
                                        121,
                                    ]);
                                {
                                    let source_ml_source3_record_field194 = heap.clone();
                                    {
                                        let source_ml_source3_record_field195 = false;
                                        ((source_ml_source3_record_field193.clone(), source_ml_source3_record_field194.clone(), source_ml_source3_record_field195), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                    }
                                }
                            }))
                        } else {
                            if ((((((((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
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
                            } && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(
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
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
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
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
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
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => false,
                                _ => true,
                            }) {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source3_record_field196 =
                                        self::ml_string::Text::from_units(vec![
                                            111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111,
                                            114, 32, 101, 120, 116, 101, 110, 115, 105, 98, 105,
                                            108, 105, 116, 121, 32, 114, 101, 113, 117, 105, 114,
                                            101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                            108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116,
                                            111, 99, 111, 108,
                                        ]);
                                    {
                                        let source_ml_source3_record_field197 = heap.clone();
                                        {
                                            let source_ml_source3_record_field198 = false;
                                            ((source_ml_source3_record_field196.clone(), source_ml_source3_record_field197.clone(), source_ml_source3_record_field198), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                        }
                                    }
                                }))
                            } else {
                                {
                                    let replacement = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field2264, source_ml_source2_nominal_field2265, _, source_ml_source2_nominal_field2267, source_ml_source2_nominal_field2268, source_ml_source2_nominal_field2269, source_ml_source2_nominal_field2270, source_ml_source2_nominal_field2271, source_ml_source2_nominal_field2272, source_ml_source2_nominal_field2273) => {
                                                let source_ml_source2_nominal_field2265 = *source_ml_source2_nominal_field2265;
                                                let source_ml_source2_nominal_field2272 = *source_ml_source2_nominal_field2272;
                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field2264.clone(), Box::new(source_ml_source2_nominal_field2265.clone()), false, source_ml_source2_nominal_field2267, source_ml_source2_nominal_field2268.clone(), source_ml_source2_nominal_field2269.clone(), source_ml_source2_nominal_field2270, source_ml_source2_nominal_field2271.clone(), Box::new(source_ml_source2_nominal_field2272.clone()), source_ml_source2_nominal_field2273.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field2275, source_ml_source2_nominal_field2276, _, source_ml_source2_nominal_field2278, source_ml_source2_nominal_field2279, source_ml_source2_nominal_field2280, source_ml_source2_nominal_field2281, source_ml_source2_nominal_field2282) => {
                                                let source_ml_source2_nominal_field2276 = *source_ml_source2_nominal_field2276;
                                                let source_ml_source2_nominal_field2280 = *source_ml_source2_nominal_field2280;
                                                let source_ml_source2_nominal_field2281 = *source_ml_source2_nominal_field2281;
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field2275.clone(), Box::new(source_ml_source2_nominal_field2276.clone()), false, source_ml_source2_nominal_field2278, source_ml_source2_nominal_field2279.clone(), Box::new(source_ml_source2_nominal_field2280.clone()), Box::new(source_ml_source2_nominal_field2281.clone()), source_ml_source2_nominal_field2282.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field2284, source_ml_source2_nominal_field2285, _, source_ml_source2_nominal_field2287, source_ml_source2_nominal_field2288) => {
                                                let source_ml_source2_nominal_field2285 = *source_ml_source2_nominal_field2285;
                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field2284.clone(), Box::new(source_ml_source2_nominal_field2285.clone()), false, source_ml_source2_nominal_field2287, source_ml_source2_nominal_field2288.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field2290, source_ml_source2_nominal_field2291, _, source_ml_source2_nominal_field2293, source_ml_source2_nominal_field2294, source_ml_source2_nominal_field2295, source_ml_source2_nominal_field2296, source_ml_source2_nominal_field2297, source_ml_source2_nominal_field2298, source_ml_source2_nominal_field2299, source_ml_source2_nominal_field2300) => {
                                                let source_ml_source2_nominal_field2291 = *source_ml_source2_nominal_field2291;
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field2290.clone(), Box::new(source_ml_source2_nominal_field2291.clone()), false, source_ml_source2_nominal_field2293, source_ml_source2_nominal_field2294.clone(), source_ml_source2_nominal_field2295.clone(), source_ml_source2_nominal_field2296, source_ml_source2_nominal_field2297.clone(), source_ml_source2_nominal_field2298.clone(), source_ml_source2_nominal_field2299.clone(), source_ml_source2_nominal_field2300.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field2302, source_ml_source2_nominal_field2303, _, source_ml_source2_nominal_field2305, source_ml_source2_nominal_field2306, source_ml_source2_nominal_field2307) => {
                                                let source_ml_source2_nominal_field2303 = *source_ml_source2_nominal_field2303;
                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field2302.clone(), Box::new(source_ml_source2_nominal_field2303.clone()), false, source_ml_source2_nominal_field2305, source_ml_source2_nominal_field2306.clone(), source_ml_source2_nominal_field2307.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field2309, source_ml_source2_nominal_field2310, _, source_ml_source2_nominal_field2312, source_ml_source2_nominal_field2313, source_ml_source2_nominal_field2314) => {
                                                let source_ml_source2_nominal_field2310 = *source_ml_source2_nominal_field2310;
                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field2309.clone(), Box::new(source_ml_source2_nominal_field2310.clone()), false, source_ml_source2_nominal_field2312, source_ml_source2_nominal_field2313.clone(), source_ml_source2_nominal_field2314.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field2316, source_ml_source2_nominal_field2317, _, source_ml_source2_nominal_field2319, source_ml_source2_nominal_field2320, source_ml_source2_nominal_field2321) => {
                                                let source_ml_source2_nominal_field2317 = *source_ml_source2_nominal_field2317;
                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field2316.clone(), Box::new(source_ml_source2_nominal_field2317.clone()), false, source_ml_source2_nominal_field2319, source_ml_source2_nominal_field2320.clone(), source_ml_source2_nominal_field2321.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field2323, source_ml_source2_nominal_field2324, _, source_ml_source2_nominal_field2326, source_ml_source2_nominal_field2327, source_ml_source2_nominal_field2328) => {
                                                let source_ml_source2_nominal_field2324 = *source_ml_source2_nominal_field2324;
                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field2323.clone(), Box::new(source_ml_source2_nominal_field2324.clone()), false, source_ml_source2_nominal_field2326, source_ml_source2_nominal_field2327.clone(), source_ml_source2_nominal_field2328.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field2330, source_ml_source2_nominal_field2331, _, source_ml_source2_nominal_field2333, source_ml_source2_nominal_field2334, source_ml_source2_nominal_field2335, source_ml_source2_nominal_field2336, source_ml_source2_nominal_field2337, source_ml_source2_nominal_field2338) => {
                                                let source_ml_source2_nominal_field2331 = *source_ml_source2_nominal_field2331;
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field2330.clone(), Box::new(source_ml_source2_nominal_field2331.clone()), false, source_ml_source2_nominal_field2333.clone(), source_ml_source2_nominal_field2334.clone(), source_ml_source2_nominal_field2335.clone(), source_ml_source2_nominal_field2336.clone(), source_ml_source2_nominal_field2337, source_ml_source2_nominal_field2338.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field2340, source_ml_source2_nominal_field2341, _, source_ml_source2_nominal_field2343, source_ml_source2_nominal_field2344, source_ml_source2_nominal_field2345, source_ml_source2_nominal_field2346) => {
                                                let source_ml_source2_nominal_field2341 = *source_ml_source2_nominal_field2341;
                                                let source_ml_source2_nominal_field2345 = *source_ml_source2_nominal_field2345;
                                                let source_ml_source2_nominal_field2346 = *source_ml_source2_nominal_field2346;
                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field2340.clone(), Box::new(source_ml_source2_nominal_field2341.clone()), false, source_ml_source2_nominal_field2343, source_ml_source2_nominal_field2344.clone(), Box::new(source_ml_source2_nominal_field2345.clone()), Box::new(source_ml_source2_nominal_field2346.clone()))
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field2348, source_ml_source2_nominal_field2349, _, source_ml_source2_nominal_field2351, source_ml_source2_nominal_field2352, source_ml_source2_nominal_field2353, source_ml_source2_nominal_field2354) => {
                                                let source_ml_source2_nominal_field2349 = *source_ml_source2_nominal_field2349;
                                                let source_ml_source2_nominal_field2353 = *source_ml_source2_nominal_field2353;
                                                let source_ml_source2_nominal_field2354 = *source_ml_source2_nominal_field2354;
                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field2348.clone(), Box::new(source_ml_source2_nominal_field2349.clone()), false, source_ml_source2_nominal_field2351, source_ml_source2_nominal_field2352.clone(), Box::new(source_ml_source2_nominal_field2353.clone()), Box::new(source_ml_source2_nominal_field2354.clone()))
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field2356, source_ml_source2_nominal_field2357, _, source_ml_source2_nominal_field2359, source_ml_source2_nominal_field2360, source_ml_source2_nominal_field2361, source_ml_source2_nominal_field2362) => {
                                                let source_ml_source2_nominal_field2357 = *source_ml_source2_nominal_field2357;
                                                let source_ml_source2_nominal_field2361 = *source_ml_source2_nominal_field2361;
                                                let source_ml_source2_nominal_field2362 = *source_ml_source2_nominal_field2362;
                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field2356.clone(), Box::new(source_ml_source2_nominal_field2357.clone()), false, source_ml_source2_nominal_field2359, source_ml_source2_nominal_field2360.clone(), Box::new(source_ml_source2_nominal_field2361.clone()), Box::new(source_ml_source2_nominal_field2362.clone()))
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field2364, source_ml_source2_nominal_field2365, _, source_ml_source2_nominal_field2367, source_ml_source2_nominal_field2368, source_ml_source2_nominal_field2369, source_ml_source2_nominal_field2370) => {
                                                let source_ml_source2_nominal_field2365 = *source_ml_source2_nominal_field2365;
                                                let source_ml_source2_nominal_field2369 = *source_ml_source2_nominal_field2369;
                                                let source_ml_source2_nominal_field2370 = *source_ml_source2_nominal_field2370;
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field2364.clone(), Box::new(source_ml_source2_nominal_field2365.clone()), false, source_ml_source2_nominal_field2367, source_ml_source2_nominal_field2368.clone(), Box::new(source_ml_source2_nominal_field2369.clone()), Box::new(source_ml_source2_nominal_field2370.clone()))
                                            }
                                        };
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_field2372 =
                                            self::replace_java_script_ordinary_object(
                                                heap.clone(),
                                                replacement.clone(),
                                            );
                                        {
                                            let source_ml_source3_record_field199 =
                                                self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field200 =
                                                    source_ml_source2_field2372.clone();
                                                {
                                                    let source_ml_source3_record_field201 = true;
                                                    ((source_ml_source3_record_field199.clone(), source_ml_source3_record_field200.clone(), source_ml_source3_record_field201), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
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
