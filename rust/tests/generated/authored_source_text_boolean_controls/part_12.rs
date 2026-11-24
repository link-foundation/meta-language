pub fn read_java_script_strong_collection_object_error(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject, budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>) -> self::ml_string::Text {
    {
        let source_ml_source1_s5285 = if (match object.clone() {
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                true
            }
            _ => {
                false
            }
        } || match object.clone() {
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                true
            }
            _ => {
                false
            }
        }) {
            {
                let entries = (((match object.clone() {
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source1_data_property_value5476) => {
                        Some((source_ml_source1_data_property_value5476.clone(),))
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source1_data_property_value5477) => {
                        Some((source_ml_source1_data_property_value5477.clone(),))
                    }
                    _ => {
                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,)>
                    }
                }).clone().expect("a proven present value")).0).clone();
                if ((((match object.clone() {
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value5478, _, _) => {
                        Some((source_ml_source1_data_property_value5478,))
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value5479, _, _) => {
                        Some((source_ml_source1_data_property_value5479,))
                    }
                    _ => {
                        None::<(bool,)>
                    }
                }).clone().expect("a proven present value")).0 || match (entries.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false }) || (({ let ml_reference_array194 = entries.clone(); let ml_reference_projection194 = ml_reference_array194.borrow().len() as f64; ml_reference_projection194 }) > (65536f64))) {
                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_strong_collection_object_error_written54 = { let ml_field_owner196 = budget.clone(); let ml_field_previous196 = ml_field_owner196.borrow().1; let ml_field_value196 = { let ml_reference_array196 = entries.clone(); let ml_reference_projection196 = ml_reference_array196.borrow().len() as f64; ml_reference_projection196 }; ml_field_owner196.borrow_mut().1 = ml_field_previous196 + ml_field_value196; () };
                        if ((({ let ml_reference_value197 = budget.borrow().clone(); ml_reference_value197 }).1) > (1048576f64)) {
                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 121, 32, 104, 105, 115, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                {
                                    let source_ml_temporary_index52 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s5480 = self::source_ml_temporary_read_java_script_strong_collection_object_error_loop55(heap.clone(), object.clone(), budget.clone(), keys.clone(), entries.clone(), source_ml_temporary_index52.clone());
                                        match source_ml_source1_s5480.clone() {
                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Done(keys_2) => {
                                                {
                                                    let ordered_keys = self::order_java_script_collection_keys(keys_2.clone());
                                                    {
                                                        let at = 1f64;
                                                        {
                                                            let source_ml_source1_s5481 = self::source_ml_temporary_read_java_script_strong_collection_object_error_loop56(ordered_keys.clone(), at);
                                                            match source_ml_source1_s5481.clone() {
                                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Done => {
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Next
                                                                }
                                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Return(source_ml_temporary_result56) => {
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Return(source_ml_temporary_result56.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Return(source_ml_temporary_result56_2) => {
                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Return(source_ml_temporary_result56_2.clone())
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Next
        };
        match source_ml_source1_s5285.clone() {
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Return(source_ml_temporary_result53) => {
                source_ml_temporary_result53.clone()
            }
        }
    }
}

pub fn read_java_script_ordinary_descriptor_value_error(heap: self::JavaScriptOrdinaryHeap, descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::ml_string::Text {
    if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115])
    } else {
        {
            let source_ml_source1_s5482 = if ({ let ml_reference_value198 = descriptor.borrow().clone(); ml_reference_value198 }).9 {
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value199 = descriptor.borrow().clone(); ml_reference_value199 }).8);
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60Return(error.clone())
                    } else {
                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60Next
                    }
                }
            } else {
                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60Next
            };
            match source_ml_source1_s5482.clone() {
                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60Next => {
                    {
                        let source_ml_source1_s5483 = if ({ let ml_reference_value200 = descriptor.borrow().clone(); ml_reference_value200 }).5 {
                            {
                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value201 = descriptor.borrow().clone(); ml_reference_value201 }).4);
                                if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61Return(error_2.clone())
                                } else {
                                    self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61Next
                                }
                            }
                        } else {
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61Next
                        };
                        match source_ml_source1_s5483.clone() {
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61Next => {
                                {
                                    let source_ml_source1_s5484 = if ({ let ml_reference_value202 = descriptor.borrow().clone(); ml_reference_value202 }).7 {
                                        {
                                            let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value203 = descriptor.borrow().clone(); ml_reference_value203 }).6);
                                            if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62Return(error_3.clone())
                                            } else {
                                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62Next
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62Next
                                    };
                                    match source_ml_source1_s5484.clone() {
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62Next => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62Return(source_ml_temporary_result62) => {
                                            source_ml_temporary_result62.clone()
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61Return(source_ml_temporary_result62_2) => {
                                source_ml_temporary_result62_2.clone()
                            }
                        }
                    }
                }
                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60Return(source_ml_temporary_result62_3) => {
                    source_ml_temporary_result62_3.clone()
                }
            }
        }
    }
}

pub fn read_java_script_ordinary_object_descriptor_error(heap: self::JavaScriptOrdinaryHeap, prototype: self::JavaScriptSemanticValue, properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>) -> self::ml_string::Text {
    if (match prototype.clone() {
        self::JavaScriptSemanticValue::Null => {
            false
        }
        _ => {
            true
        }
    } && match prototype.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => {
            false
        }
        _ => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116])
    } else {
        {
            let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
            if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                prototype_error.clone()
            } else {
                {
                    let source_ml_temporary_read_java_script_ordinary_object_descriptor_error_written63 = { let ml_field_owner205 = budget.clone(); let ml_field_previous205 = ml_field_owner205.borrow().4; let ml_field_value205 = { let ml_reference_array205 = properties.clone(); let ml_reference_projection205 = ml_reference_array205.borrow().len() as f64; ml_reference_projection205 }; ml_field_owner205.borrow_mut().4 = ml_field_previous205 + ml_field_value205; () };
                    if (((({ let ml_reference_value206 = budget.borrow().clone(); ml_reference_value206 }).4) > (16384f64)) || (({ let ml_reference_array207 = properties.clone(); let ml_reference_projection207 = ml_reference_array207.borrow().len() as f64; ml_reference_projection207 }) > (4096f64))) {
                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116])
                    } else {
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s5503 = self::source_ml_temporary_read_java_script_ordinary_object_descriptor_error_loop64(heap.clone(), properties.clone(), at);
                                match source_ml_source1_s5503.clone() {
                                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Return(source_ml_temporary_result64) => {
                                        source_ml_temporary_result64.clone()
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

pub fn read_java_script_ordinary_prototype_chain_error(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject) -> self::ml_string::Text {
    {
        let prototype = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value5504, _, _, _, _, _, _, _, _) => {
                let source_ml_source1_data_property_value5504 = *source_ml_source1_data_property_value5504;
                source_ml_source1_data_property_value5504.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value5505, _, _, _, _, _, _) => {
                let source_ml_source1_data_property_value5505 = *source_ml_source1_data_property_value5505;
                source_ml_source1_data_property_value5505.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value5506, _, _, _) => {
                let source_ml_source1_data_property_value5506 = *source_ml_source1_data_property_value5506;
                source_ml_source1_data_property_value5506.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value5507, _, _, _, _, _, _, _, _, _) => {
                let source_ml_source1_data_property_value5507 = *source_ml_source1_data_property_value5507;
                source_ml_source1_data_property_value5507.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value5508, _, _, _, _) => {
                let source_ml_source1_data_property_value5508 = *source_ml_source1_data_property_value5508;
                source_ml_source1_data_property_value5508.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value5509, _, _, _, _) => {
                let source_ml_source1_data_property_value5509 = *source_ml_source1_data_property_value5509;
                source_ml_source1_data_property_value5509.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value5510, _, _, _, _) => {
                let source_ml_source1_data_property_value5510 = *source_ml_source1_data_property_value5510;
                source_ml_source1_data_property_value5510.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value5511, _, _, _, _) => {
                let source_ml_source1_data_property_value5511 = *source_ml_source1_data_property_value5511;
                source_ml_source1_data_property_value5511.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value5512, _, _, _, _, _, _, _) => {
                let source_ml_source1_data_property_value5512 = *source_ml_source1_data_property_value5512;
                source_ml_source1_data_property_value5512.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source1_data_property_value5513, _, _, _, _, _) => {
                let source_ml_source1_data_property_value5513 = *source_ml_source1_data_property_value5513;
                source_ml_source1_data_property_value5513.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source1_data_property_value5514, _, _, _, _, _) => {
                let source_ml_source1_data_property_value5514 = *source_ml_source1_data_property_value5514;
                source_ml_source1_data_property_value5514.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source1_data_property_value5515, _, _, _, _, _) => {
                let source_ml_source1_data_property_value5515 = *source_ml_source1_data_property_value5515;
                source_ml_source1_data_property_value5515.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source1_data_property_value5516, _, _, _, _, _) => {
                let source_ml_source1_data_property_value5516 = *source_ml_source1_data_property_value5516;
                source_ml_source1_data_property_value5516.clone()
            }
        };
        {
            let depth = 0f64;
            {
                let source_ml_source1_s5517 = self::source_ml_temporary_read_java_script_ordinary_prototype_chain_error_loop65(heap.clone(), object.clone(), prototype.clone(), depth);
                match source_ml_source1_s5517.clone() {
                    self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Done => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Return(source_ml_temporary_result65) => {
                        source_ml_temporary_result65.clone()
                    }
                }
            }
        }
    }
}

pub fn read_java_script_ordinary_heap_error(heap: self::JavaScriptOrdinaryHeap) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, _, _, _, _) => {
            if self::test_java_script_validated_heap_snapshot(heap.clone()) {
                self::ml_string::Text::from_units(vec![])
            } else {
                if ((({ let ml_reference_array208 = shared_constructor_field1.clone(); let ml_reference_projection208 = ml_reference_array208.borrow().len() as f64; ml_reference_projection208 }) > (4096f64)) || (({ let ml_reference_array209 = shared_constructor_field2.clone(); let ml_reference_projection209 = ml_reference_array209.borrow().len() as f64; ml_reference_projection209 }) > (16384f64))) {
                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
                } else {
                    {
                        let previous = self::ml::Big::from_i128(0);
                        {
                            let source_ml_temporary_values66 = shared_constructor_field1.clone();
                            {
                                let source_ml_temporary_index66 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_s5523 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop82(heap.clone(), previous.clone(), source_ml_temporary_values66.clone(), source_ml_temporary_index66.clone());
                                    match source_ml_source1_s5523.clone() {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Done(previous_2) => {
                                            {
                                                let previous_3 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_temporary_values67 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_temporary_index67 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s5525 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop83(heap.clone(), previous_3.clone(), source_ml_temporary_values67.clone(), source_ml_temporary_index67.clone());
                                                            match source_ml_source1_s5525.clone() {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Done => {
                                                                    {
                                                                        let source_ml_source1_s5544 = if (!(match (match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5545, _, _) => {
                                                                                ((source_ml_source1_data_property_value5545.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                            let source_ml_source1_comparison_operand5547 = match heap.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value5546, _, _, _) => {
                                                                                    ((source_ml_source1_data_property_value5546.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_comparison_operand5548 = None::<()>;
                                                                                !(match (source_ml_source1_comparison_operand5547.clone(), source_ml_source1_comparison_operand5548.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                            }
                                                                        }) {
                                                                            if (({
                                                                                let source_ml_source1_comparison_operand5550 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value5549, _, _, _) => {
                                                                                        ((source_ml_source1_data_property_value5549.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                    }
                                                                                };
                                                                                !(match (source_ml_source1_comparison_operand5550.clone(), self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                            } || !({
                                                                                let source_ml_source1_array_argument5553 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5552, _, _) => {
                                                                                        ((source_ml_source1_data_property_value5552.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                    }
                                                                                };
                                                                                match source_ml_source1_array_argument5553.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                            })) || (({
                                                                                let source_ml_source1_nullish_field_receiver5559 = (match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5558, _, _) => {
                                                                                        ((source_ml_source1_data_property_value5558.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                    }
                                                                                },);
                                                                                if match (source_ml_source1_nullish_field_receiver5559.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source1_nullish_field_present5560 = ((source_ml_source1_nullish_field_receiver5559.clone()).0).clone().expect("a proven present value");
                                                                                        { let ml_reference_array210 = source_ml_source1_nullish_field_present5560.clone(); let ml_reference_projection210 = ml_reference_array210.borrow().len() as f64; ml_reference_projection210 }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            }) > (16384f64))) {
                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                            } else {
                                                                                {
                                                                                    let registrations = (match heap.clone() {
                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5561, _, _) => {
                                                                                            ((source_ml_source1_data_property_value5561.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                    {
                                                                                        let prior = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_temporary_index68 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s5562 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop101(heap.clone(), prior.clone(), registrations.clone(), source_ml_temporary_index68.clone());
                                                                                                match source_ml_source1_s5562.clone() {
                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done => {
                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84Next
                                                                                                    }
                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(source_ml_temporary_result101) => {
                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84Return(source_ml_temporary_result101.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84Next
                                                                        };
                                                                        match source_ml_source1_s5544.clone() {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84Next => {
                                                                                {
                                                                                    let registered_keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                    {
                                                                                        let source_ml_temporary_values70 = (match heap.clone() {
                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5563, _, _) => {
                                                                                                ((source_ml_source1_data_property_value5563.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                            }
                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                        {
                                                                                            let source_ml_temporary_index70 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let registered_keys_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop85(registered_keys.clone(), source_ml_temporary_values70.clone(), source_ml_temporary_index70.clone());
                                                                                                {
                                                                                                    let witnessed_keys = self::order_java_script_collection_keys(registered_keys_2.clone());
                                                                                                    {
                                                                                                        let witnessed_units = 0f64;
                                                                                                        {
                                                                                                            let previous_witness = None::<()>;
                                                                                                            {
                                                                                                                let source_ml_temporary_index71 = self::ml::Big::from_i128(0);
                                                                                                                {
                                                                                                                    let source_ml_source1_s5564 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop86(witnessed_units, None::<self::JavaScriptSemanticValue>, witnessed_keys.clone(), source_ml_temporary_index71.clone());
                                                                                                                    match source_ml_source1_s5564.clone() {
                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Done => {
                                                                                                                            {
                                                                                                                                let source_ml_source1_s5586 = if (!(match (match heap.clone() {
                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5587) => {
                                                                                                                                        ((source_ml_source1_data_property_value5587.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                                                                                    let source_ml_source1_comparison_operand5589 = match heap.clone() {
                                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value5588, _) => {
                                                                                                                                            ((source_ml_source1_data_property_value5588.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_comparison_operand5590 = None::<()>;
                                                                                                                                        !(match (source_ml_source1_comparison_operand5589.clone(), source_ml_source1_comparison_operand5590.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                                    }
                                                                                                                                }) {
                                                                                                                                    if (({
                                                                                                                                        let source_ml_source1_comparison_operand5592 = match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value5591, _) => {
                                                                                                                                                ((source_ml_source1_data_property_value5591.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        !(match (source_ml_source1_comparison_operand5592.clone(), self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                                                                    } || !({
                                                                                                                                        let source_ml_source1_array_argument5595 = match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5594) => {
                                                                                                                                                ((source_ml_source1_data_property_value5594.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        match source_ml_source1_array_argument5595.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                                                                                    })) || (({
                                                                                                                                        let source_ml_source1_nullish_field_receiver5601 = (match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5600) => {
                                                                                                                                                ((source_ml_source1_data_property_value5600.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        },);
                                                                                                                                        if match (source_ml_source1_nullish_field_receiver5601.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_nullish_field_present5602 = ((source_ml_source1_nullish_field_receiver5601.clone()).0).clone().expect("a proven present value");
                                                                                                                                                { let ml_reference_array211 = source_ml_source1_nullish_field_present5602.clone(); let ml_reference_projection211 = ml_reference_array211.borrow().len() as f64; ml_reference_projection211 }
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                                                                                        }
                                                                                                                                    }) > (16384f64))) {
                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Return(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 97, 103, 101, 110, 116, 32, 109, 111, 100, 101, 108]))
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let facts = (match heap.clone() {
                                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5603, _, _) => {
                                                                                                                                                    ((source_ml_source1_data_property_value5603.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                                                                            {
                                                                                                                                                let registry = (match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5604) => {
                                                                                                                                                        ((source_ml_source1_data_property_value5604.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistryEntry>::new())));
                                                                                                                                                {
                                                                                                                                                    let identities = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                                                    {
                                                                                                                                                        let previous_key = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                        {
                                                                                                                                                            let units = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_temporary_index72 = self::ml::Big::from_i128(0);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_s5605 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop96(heap.clone(), facts.clone(), identities.clone(), previous_key.clone(), units, registry.clone(), source_ml_temporary_index72.clone());
                                                                                                                                                                    match source_ml_source1_s5605.clone() {
                                                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done(identities_2) => {
                                                                                                                                                                            {
                                                                                                                                                                                let ordered = self::order_java_script_collection_keys(identities_2.clone());
                                                                                                                                                                                {
                                                                                                                                                                                    let previous_identity = None::<()>;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_temporary_index77 = self::ml::Big::from_i128(0);
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_s5606 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop97(None::<self::JavaScriptSemanticValue>, ordered.clone(), source_ml_temporary_index77.clone());
                                                                                                                                                                                            match source_ml_source1_s5606.clone() {
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done => {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_temporary_index78 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source1_s5607 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop98(registry.clone(), facts.clone(), source_ml_temporary_index78.clone());
                                                                                                                                                                                                            match source_ml_source1_s5607.clone() {
                                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Done => {
                                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Next
                                                                                                                                                                                                                }
                                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Return(source_ml_temporary_result98) => {
                                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Return(source_ml_temporary_result98.clone())
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(source_ml_temporary_result99) => {
                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Return(source_ml_temporary_result99.clone())
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Return(source_ml_temporary_result99_2) => {
                                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Return(source_ml_temporary_result99_2.clone())
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
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Next
                                                                                                                                };
                                                                                                                                match source_ml_source1_s5586.clone() {
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Next => {
                                                                                                                                        {
                                                                                                                                            let budget = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                let source_ml_source1_field5608 = 0f64;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_field5609 = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_field5610 = 0f64;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_field5611 = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_field5612 = 0f64;
                                                                                                                                                                (source_ml_source1_field5608, source_ml_source1_field5610, source_ml_source1_field5611, source_ml_source1_field5612, source_ml_source1_field5609)
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }));
                                                                                                                                            {
                                                                                                                                                let source_ml_temporary_values80 = shared_constructor_field1.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_temporary_index80 = self::ml::Big::from_i128(0);
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_s5614 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop88(heap.clone(), budget.clone(), source_ml_temporary_values80.clone(), source_ml_temporary_index80.clone());
                                                                                                                                                        match source_ml_source1_s5614.clone() {
                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Done => {
                                                                                                                                                                {
                                                                                                                                                                    let marks = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<(f64,)>>>::new()));
                                                                                                                                                                    {
                                                                                                                                                                        let index = 0f64;
                                                                                                                                                                        {
                                                                                                                                                                            let marks_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop89(heap.clone(), marks.clone(), index);
                                                                                                                                                                            {
                                                                                                                                                                                let first = 0f64;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_s5615 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop90(heap.clone(), marks_2.clone(), first);
                                                                                                                                                                                    match source_ml_source1_s5615.clone() {
                                                                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Done => {
                                                                                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                                                                                        }
                                                                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Return(source_ml_temporary_result90) => {
                                                                                                                                                                                            source_ml_temporary_result90.clone()
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(source_ml_temporary_result94) => {
                                                                                                                                                                source_ml_temporary_result94.clone()
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Return(source_ml_temporary_result95) => {
                                                                                                                                        source_ml_temporary_result95.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Return(source_ml_temporary_result100) => {
                                                                                                                            source_ml_temporary_result100.clone()
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
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84Return(source_ml_temporary_result100_2) => {
                                                                                source_ml_temporary_result100_2.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Return(source_ml_temporary_result101_2) => {
                                                                    source_ml_temporary_result101_2.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Return(source_ml_temporary_result101_3) => {
                                            source_ml_temporary_result101_3.clone()
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

pub fn replace_java_script_ordinary_object(heap: self::JavaScriptOrdinaryHeap, replacement: self::JavaScriptOrdinaryObject) -> self::JavaScriptOrdinaryHeap {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            {
                let objects = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values212 = { let ml_reference_value213 = shared_constructor_field1.borrow().clone(); ml_reference_value213 }; let ml_callback212 = { let replacement = replacement.clone(); std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value5643, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5643.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value5644, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5644.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value5645, _, _, _, _) => {
                        source_ml_source1_data_property_value5645.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value5646, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5646.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value5647, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5647.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value5648, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5648.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value5649, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5649.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value5650, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5650.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value5651, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5651.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value5652, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5652.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value5653, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5653.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value5654, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5654.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value5655, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5655.clone()
                    }
                }) == (match replacement.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value5656, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5656.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value5657, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5657.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value5658, _, _, _, _) => {
                        source_ml_source1_data_property_value5658.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value5659, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5659.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value5660, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5660.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value5661, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5661.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value5662, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5662.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value5663, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5663.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value5664, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5664.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value5665, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5665.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value5666, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5666.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value5667, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5667.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value5668, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value5668.clone()
                    }
                })) {
                    replacement.clone()
                } else {
                    object.clone()
                } }) }; ml_values212.iter().cloned().enumerate().map(|(ml_index212, ml_value212)| ml_callback212(ml_value212)).collect::<Vec<_>>() }));
                {
                    let source_ml_source1_nominal_field5670 = shared_constructor_field1.clone();
                    {
                        let source_ml_source1_nominal_field5672 = shared_constructor_field2.clone();
                        {
                            let source_ml_source1_nominal_field5674 = shared_constructor_field3.clone();
                            {
                                let source_ml_source1_nominal_field5676 = shared_constructor_field4.clone();
                                {
                                    let source_ml_source1_nominal_field5678 = shared_constructor_field5.clone();
                                    {
                                        let source_ml_source1_nominal_field5680 = shared_constructor_field6.clone();
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects.clone(), source_ml_source1_nominal_field5672.clone(), source_ml_source1_nominal_field5674.clone(), source_ml_source1_nominal_field5676.clone(), source_ml_source1_nominal_field5678.clone(), source_ml_source1_nominal_field5680.clone())
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

pub fn locate_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, bool, self::ml::Big)>> {
    {
        let depth = 0f64;
        {
            let source_ml_source1_s5683 = self::source_ml_temporary_locate_java_script_ordinary_property_loop103(heap.clone(), key.clone(), identity.clone(), depth);
            match source_ml_source1_s5683.clone() {
                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Done => {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source1_field5685 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_field5686 = self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), false, false);
                            (source_ml_source1_field5686.clone(), false, source_ml_source1_field5685.clone())
                        }
                    }))
                }
                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Return(source_ml_temporary_result103) => {
                    source_ml_temporary_result103.clone()
                }
            }
        }
    }
}
