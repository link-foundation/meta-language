// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_set_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_set_java_script_ordinary_prototype_loop123;
use super::source_ml_source3_record_order;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn set_java_script_ordinary_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source3_record_field202 = heap_error.clone();
                    {
                        let source_ml_source3_record_field203 = heap.clone();
                        {
                            let source_ml_source3_record_field204 = false;
                            ((source_ml_source3_record_field202.clone(), source_ml_source3_record_field203.clone(), source_ml_source3_record_field204), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                        }
                    }
                }))
            } else {
                if (match prototype.clone() {
                    self::JavaScriptSemanticValue::Null => false,
                    _ => true,
                } && match prototype.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                }) {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source3_record_field205 =
                            self::ml_string::Text::from_units(vec![
                                97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114,
                                111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98,
                                101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101,
                                99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116,
                            ]);
                        {
                            let source_ml_source3_record_field206 = heap.clone();
                            {
                                let source_ml_source3_record_field207 = false;
                                ((source_ml_source3_record_field205.clone(), source_ml_source3_record_field206.clone(), source_ml_source3_record_field207), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                            }
                        }
                    }))
                } else {
                    {
                        let error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            prototype.clone(),
                        );
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source3_record_field208 = error.clone();
                                {
                                    let source_ml_source3_record_field209 = heap.clone();
                                    {
                                        let source_ml_source3_record_field210 = false;
                                        ((source_ml_source3_record_field208.clone(), source_ml_source3_record_field209.clone(), source_ml_source3_record_field210), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
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
                                        let source_ml_source2_array_read_value2388 =
                                            shared_constructor_field1.clone();
                                        {
                                            let ml_reference_array248 =
                                                source_ml_source2_array_read_value2388.clone();
                                            let ml_reference_position248 = index;
                                            let ml_reference_projection248 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array248.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position248,
                                                    ),
                                                );
                                            ml_reference_projection248
                                        }
                                    };
                                    if match (object.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field211 =
                                                self::ml_string::Text::from_units(vec![
                                                    83, 101, 116, 80, 114, 111, 116, 111, 116, 121,
                                                    112, 101, 79, 102, 32, 110, 101, 101, 100, 115,
                                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                                    108, 32, 79, 98, 106, 101, 99, 116, 32, 105,
                                                    100, 101, 110, 116, 105, 116, 121,
                                                ]);
                                            {
                                                let source_ml_source3_record_field212 =
                                                    heap.clone();
                                                {
                                                    let source_ml_source3_record_field213 = false;
                                                    ((source_ml_source3_record_field211.clone(), source_ml_source3_record_field212.clone(), source_ml_source3_record_field213), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
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
                                                    let source_ml_source3_record_field214 = self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]);
                                                    {
                                                        let source_ml_source3_record_field215 = heap.clone();
                                                        {
                                                            let source_ml_source3_record_field216 = false;
                                                            ((source_ml_source3_record_field214.clone(), source_ml_source3_record_field215.clone(), source_ml_source3_record_field216), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                        }
                                                    }
                                                }))
                                            } else {
                                                if self::test_java_script_semantic_same_value((((match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value2463, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value2463 = *source_ml_source2_data_property_value2463;
                                                        Some((source_ml_source2_data_property_value2463.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value2464, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value2464 = *source_ml_source2_data_property_value2464;
                                                        Some((source_ml_source2_data_property_value2464.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value2465, _, _, _) => {
                                                        let source_ml_source2_data_property_value2465 = *source_ml_source2_data_property_value2465;
                                                        Some((source_ml_source2_data_property_value2465.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value2466, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value2466 = *source_ml_source2_data_property_value2466;
                                                        Some((source_ml_source2_data_property_value2466.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value2467, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value2467 = *source_ml_source2_data_property_value2467;
                                                        Some((source_ml_source2_data_property_value2467.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value2468, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value2468 = *source_ml_source2_data_property_value2468;
                                                        Some((source_ml_source2_data_property_value2468.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value2469, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value2469 = *source_ml_source2_data_property_value2469;
                                                        Some((source_ml_source2_data_property_value2469.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value2470, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value2470 = *source_ml_source2_data_property_value2470;
                                                        Some((source_ml_source2_data_property_value2470.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2471, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value2471 = *source_ml_source2_data_property_value2471;
                                                        Some((source_ml_source2_data_property_value2471.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone(), prototype.clone()) {
                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source3_record_field217 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source3_record_field218 = heap.clone();
                                                            {
                                                                let source_ml_source3_record_field219 = true;
                                                                ((source_ml_source3_record_field217.clone(), source_ml_source3_record_field218.clone(), source_ml_source3_record_field219), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                            }
                                                        }
                                                    }))
                                                } else {
                                                    if (!(((match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source2_data_property_value2475, _, _, _, _, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value2475,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source2_data_property_value2476, _, _, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value2476,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source2_data_property_value2477, _, _) => {
                                                            Some((source_ml_source2_data_property_value2477,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source2_data_property_value2478, _, _, _, _, _, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value2478,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source2_data_property_value2479, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value2479,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source2_data_property_value2480, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value2480,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source2_data_property_value2481, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value2481,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source2_data_property_value2482, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value2482,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value2483, _, _, _, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value2483,))
                                                        }
                                                        _ => {
                                                            None::<(bool,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0) || (match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } && {
                                                        let source_ml_source2_comparison_operand2494 = (((match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value2493, _, _, _, _, _) => {
                                                                Some((((source_ml_source2_data_property_value2493.clone().unwrap_or_else(|| (None::<bool>,))).0).clone(),))
                                                            }
                                                            _ => {
                                                                None::<(Option<bool>,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone();
                                                        match (source_ml_source2_comparison_operand2494.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                    })) {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source3_record_field220 = self::ml_string::Text::from_units(vec![]);
                                                            {
                                                                let source_ml_source3_record_field221 = heap.clone();
                                                                {
                                                                    let source_ml_source3_record_field222 = false;
                                                                    ((source_ml_source3_record_field220.clone(), source_ml_source3_record_field221.clone(), source_ml_source3_record_field222), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                                }
                                                            }
                                                        }))
                                                    } else {
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source2_s2499 = self::source_ml_source1_temporary_set_java_script_ordinary_prototype_loop123(heap.clone(), identity.clone(), prototype.clone(), depth);
                                                                match source_ml_source2_s2499.clone() {
                                                                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Done => {
                                                                        {
                                                                            let replacement = match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field2500, _, source_ml_source2_nominal_field2502, source_ml_source2_nominal_field2503, source_ml_source2_nominal_field2504, source_ml_source2_nominal_field2505, source_ml_source2_nominal_field2506, source_ml_source2_nominal_field2507, source_ml_source2_nominal_field2508, source_ml_source2_nominal_field2509) => {
                                                                                    let source_ml_source2_nominal_field2508 = *source_ml_source2_nominal_field2508;
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field2500.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2502, source_ml_source2_nominal_field2503, source_ml_source2_nominal_field2504.clone(), source_ml_source2_nominal_field2505.clone(), source_ml_source2_nominal_field2506, source_ml_source2_nominal_field2507.clone(), Box::new(source_ml_source2_nominal_field2508.clone()), source_ml_source2_nominal_field2509.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field2511, _, source_ml_source2_nominal_field2513, source_ml_source2_nominal_field2514, source_ml_source2_nominal_field2515, source_ml_source2_nominal_field2516, source_ml_source2_nominal_field2517, source_ml_source2_nominal_field2518) => {
                                                                                    let source_ml_source2_nominal_field2516 = *source_ml_source2_nominal_field2516;
                                                                                    let source_ml_source2_nominal_field2517 = *source_ml_source2_nominal_field2517;
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field2511.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2513, source_ml_source2_nominal_field2514, source_ml_source2_nominal_field2515.clone(), Box::new(source_ml_source2_nominal_field2516.clone()), Box::new(source_ml_source2_nominal_field2517.clone()), source_ml_source2_nominal_field2518.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field2520, _, source_ml_source2_nominal_field2522, source_ml_source2_nominal_field2523, source_ml_source2_nominal_field2524) => {
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field2520.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2522, source_ml_source2_nominal_field2523, source_ml_source2_nominal_field2524.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field2526, _, source_ml_source2_nominal_field2528, source_ml_source2_nominal_field2529, source_ml_source2_nominal_field2530, source_ml_source2_nominal_field2531, source_ml_source2_nominal_field2532, source_ml_source2_nominal_field2533, source_ml_source2_nominal_field2534, source_ml_source2_nominal_field2535, source_ml_source2_nominal_field2536) => {
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field2526.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2528, source_ml_source2_nominal_field2529, source_ml_source2_nominal_field2530.clone(), source_ml_source2_nominal_field2531.clone(), source_ml_source2_nominal_field2532, source_ml_source2_nominal_field2533.clone(), source_ml_source2_nominal_field2534.clone(), source_ml_source2_nominal_field2535.clone(), source_ml_source2_nominal_field2536.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field2538, _, source_ml_source2_nominal_field2540, source_ml_source2_nominal_field2541, source_ml_source2_nominal_field2542, source_ml_source2_nominal_field2543) => {
                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field2538.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2540, source_ml_source2_nominal_field2541, source_ml_source2_nominal_field2542.clone(), source_ml_source2_nominal_field2543.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field2545, _, source_ml_source2_nominal_field2547, source_ml_source2_nominal_field2548, source_ml_source2_nominal_field2549, source_ml_source2_nominal_field2550) => {
                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field2545.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2547, source_ml_source2_nominal_field2548, source_ml_source2_nominal_field2549.clone(), source_ml_source2_nominal_field2550.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field2552, _, source_ml_source2_nominal_field2554, source_ml_source2_nominal_field2555, source_ml_source2_nominal_field2556, source_ml_source2_nominal_field2557) => {
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field2552.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2554, source_ml_source2_nominal_field2555, source_ml_source2_nominal_field2556.clone(), source_ml_source2_nominal_field2557.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field2559, _, source_ml_source2_nominal_field2561, source_ml_source2_nominal_field2562, source_ml_source2_nominal_field2563, source_ml_source2_nominal_field2564) => {
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field2559.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2561, source_ml_source2_nominal_field2562, source_ml_source2_nominal_field2563.clone(), source_ml_source2_nominal_field2564.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field2566, _, source_ml_source2_nominal_field2568, source_ml_source2_nominal_field2569, source_ml_source2_nominal_field2570, source_ml_source2_nominal_field2571, source_ml_source2_nominal_field2572, source_ml_source2_nominal_field2573, source_ml_source2_nominal_field2574) => {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field2566.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2568, source_ml_source2_nominal_field2569.clone(), source_ml_source2_nominal_field2570.clone(), source_ml_source2_nominal_field2571.clone(), source_ml_source2_nominal_field2572.clone(), source_ml_source2_nominal_field2573, source_ml_source2_nominal_field2574.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field2576, _, source_ml_source2_nominal_field2578, source_ml_source2_nominal_field2579, source_ml_source2_nominal_field2580, source_ml_source2_nominal_field2581, source_ml_source2_nominal_field2582) => {
                                                                                    let source_ml_source2_nominal_field2581 = *source_ml_source2_nominal_field2581;
                                                                                    let source_ml_source2_nominal_field2582 = *source_ml_source2_nominal_field2582;
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field2576.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2578, source_ml_source2_nominal_field2579, source_ml_source2_nominal_field2580.clone(), Box::new(source_ml_source2_nominal_field2581.clone()), Box::new(source_ml_source2_nominal_field2582.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field2584, _, source_ml_source2_nominal_field2586, source_ml_source2_nominal_field2587, source_ml_source2_nominal_field2588, source_ml_source2_nominal_field2589, source_ml_source2_nominal_field2590) => {
                                                                                    let source_ml_source2_nominal_field2589 = *source_ml_source2_nominal_field2589;
                                                                                    let source_ml_source2_nominal_field2590 = *source_ml_source2_nominal_field2590;
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field2584.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2586, source_ml_source2_nominal_field2587, source_ml_source2_nominal_field2588.clone(), Box::new(source_ml_source2_nominal_field2589.clone()), Box::new(source_ml_source2_nominal_field2590.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field2592, _, source_ml_source2_nominal_field2594, source_ml_source2_nominal_field2595, source_ml_source2_nominal_field2596, source_ml_source2_nominal_field2597, source_ml_source2_nominal_field2598) => {
                                                                                    let source_ml_source2_nominal_field2597 = *source_ml_source2_nominal_field2597;
                                                                                    let source_ml_source2_nominal_field2598 = *source_ml_source2_nominal_field2598;
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field2592.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2594, source_ml_source2_nominal_field2595, source_ml_source2_nominal_field2596.clone(), Box::new(source_ml_source2_nominal_field2597.clone()), Box::new(source_ml_source2_nominal_field2598.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field2600, _, source_ml_source2_nominal_field2602, source_ml_source2_nominal_field2603, source_ml_source2_nominal_field2604, source_ml_source2_nominal_field2605, source_ml_source2_nominal_field2606) => {
                                                                                    let source_ml_source2_nominal_field2605 = *source_ml_source2_nominal_field2605;
                                                                                    let source_ml_source2_nominal_field2606 = *source_ml_source2_nominal_field2606;
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field2600.clone(), Box::new(prototype.clone()), source_ml_source2_nominal_field2602, source_ml_source2_nominal_field2603, source_ml_source2_nominal_field2604.clone(), Box::new(source_ml_source2_nominal_field2605.clone()), Box::new(source_ml_source2_nominal_field2606.clone()))
                                                                                }
                                                                            };
                                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                                let source_ml_source2_field2608 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                {
                                                                                    let source_ml_source3_record_field223 = self::ml_string::Text::from_units(vec![]);
                                                                                    {
                                                                                        let source_ml_source3_record_field224 = source_ml_source2_field2608.clone();
                                                                                        {
                                                                                            let source_ml_source3_record_field225 = true;
                                                                                            ((source_ml_source3_record_field223.clone(), source_ml_source3_record_field224.clone(), source_ml_source3_record_field225), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }))
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop123Return(source_ml_source1_temporary_result123) => {
                                                                        source_ml_source1_temporary_result123.clone()
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
pub fn create_java_script_ordinary_set_descriptor(
    value: self::JavaScriptSemanticValue,
    creation: bool,
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
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field2620 = self::JavaScriptSemanticValue::Undefined;
        {
            let source_ml_source2_field2622 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source3_record_field226 = creation;
                {
                    let source_ml_source3_record_field227 = creation;
                    {
                        let source_ml_source3_record_field228 = creation;
                        {
                            let source_ml_source3_record_field229 = creation;
                            {
                                let source_ml_source3_record_field230 =
                                    source_ml_source2_field2620.clone();
                                {
                                    let source_ml_source3_record_field231 = false;
                                    {
                                        let source_ml_source3_record_field232 =
                                            source_ml_source2_field2622.clone();
                                        {
                                            let source_ml_source3_record_field233 = false;
                                            {
                                                let source_ml_source3_record_field234 =
                                                    value.clone();
                                                {
                                                    let source_ml_source3_record_field235 = true;
                                                    {
                                                        let source_ml_source3_record_field236 =
                                                            creation;
                                                        {
                                                            let source_ml_source3_record_field237 =
                                                                creation;
                                                            ((source_ml_source3_record_field226, source_ml_source3_record_field227, source_ml_source3_record_field228, source_ml_source3_record_field229, source_ml_source3_record_field230.clone(), source_ml_source3_record_field231, source_ml_source3_record_field232.clone(), source_ml_source3_record_field233, source_ml_source3_record_field234.clone(), source_ml_source3_record_field235, source_ml_source3_record_field236, source_ml_source3_record_field237), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64, 3f64, 2f64, 9f64, 8f64, 11f64, 10f64, 5f64, 4f64, 7f64, 6f64]))
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
    }))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_set_with_array_definition(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            self::ml_string::Text,
            std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
            self::JavaScriptSemanticValue,
            Option<(
                Option<
                    std::rc::Rc<
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
                    >,
                >,
            )>,
            self::ml_string::Text,
            self::JavaScriptOrdinaryHeap,
            self::JavaScriptSemanticValue,
            bool,
        ),
        Vec<f64>,
    )>,
> {
    self::plan_java_script_ordinary_set_definition(
        heap.clone(),
        identity.clone(),
        key.clone(),
        value.clone(),
        receiver.clone(),
        true,
    )
}
