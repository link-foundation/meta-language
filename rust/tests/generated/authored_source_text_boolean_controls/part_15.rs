pub fn set_java_script_ordinary_prototype(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, prototype: self::JavaScriptSemanticValue) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    std::rc::Rc::new(std::cell::RefCell::new((heap_error.clone(), heap.clone(), false)))
                } else {
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
                        std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]), heap.clone(), false)))
                    } else {
                        {
                            let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                std::rc::Rc::new(std::cell::RefCell::new((error.clone(), heap.clone(), false)))
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                    {
                                        let object = {
                                            let source_ml_source1_array_read_value9994 = shared_constructor_field1.clone();
                                            { let ml_reference_array248 = source_ml_source1_array_read_value9994.clone(); let ml_reference_position248 = index; let ml_reference_projection248 = self::ml_array::optional_at(&*ml_reference_array248.borrow(), self::ml_array::number_index(ml_reference_position248)); ml_reference_projection248 }
                                        };
                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![83, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]), heap.clone(), false)))
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
                                                std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), false)))
                                            } else {
                                                if self::test_java_script_semantic_same_value((((match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value10857, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value10857 = *source_ml_source1_data_property_value10857;
                                                        Some((source_ml_source1_data_property_value10857.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value10858, _, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value10858 = *source_ml_source1_data_property_value10858;
                                                        Some((source_ml_source1_data_property_value10858.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value10859, _, _, _) => {
                                                        let source_ml_source1_data_property_value10859 = *source_ml_source1_data_property_value10859;
                                                        Some((source_ml_source1_data_property_value10859.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value10860, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value10860 = *source_ml_source1_data_property_value10860;
                                                        Some((source_ml_source1_data_property_value10860.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value10861, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value10861 = *source_ml_source1_data_property_value10861;
                                                        Some((source_ml_source1_data_property_value10861.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value10862, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value10862 = *source_ml_source1_data_property_value10862;
                                                        Some((source_ml_source1_data_property_value10862.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value10863, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value10863 = *source_ml_source1_data_property_value10863;
                                                        Some((source_ml_source1_data_property_value10863.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value10864, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value10864 = *source_ml_source1_data_property_value10864;
                                                        Some((source_ml_source1_data_property_value10864.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value10865, _, _, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value10865 = *source_ml_source1_data_property_value10865;
                                                        Some((source_ml_source1_data_property_value10865.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone(), prototype.clone()) {
                                                    std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), heap.clone(), true)))
                                                } else {
                                                    if (!(((match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source1_data_property_value10869, _, _, _, _, _, _, _) => {
                                                            Some((source_ml_source1_data_property_value10869,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source1_data_property_value10870, _, _, _, _, _) => {
                                                            Some((source_ml_source1_data_property_value10870,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source1_data_property_value10871, _, _) => {
                                                            Some((source_ml_source1_data_property_value10871,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source1_data_property_value10872, _, _, _, _, _, _, _, _) => {
                                                            Some((source_ml_source1_data_property_value10872,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source1_data_property_value10873, _, _, _) => {
                                                            Some((source_ml_source1_data_property_value10873,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source1_data_property_value10874, _, _, _) => {
                                                            Some((source_ml_source1_data_property_value10874,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source1_data_property_value10875, _, _, _) => {
                                                            Some((source_ml_source1_data_property_value10875,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source1_data_property_value10876, _, _, _) => {
                                                            Some((source_ml_source1_data_property_value10876,))
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value10877, _, _, _, _, _, _) => {
                                                            Some((source_ml_source1_data_property_value10877,))
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
                                                        let source_ml_source1_comparison_operand10974 = (((match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value10973, _, _, _, _, _) => {
                                                                Some((((source_ml_source1_data_property_value10973.clone().unwrap_or_else(|| (None::<bool>,))).0).clone(),))
                                                            }
                                                            _ => {
                                                                None::<(Option<bool>,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone();
                                                        match (source_ml_source1_comparison_operand10974.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                    })) {
                                                        std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), heap.clone(), false)))
                                                    } else {
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source1_s10979 = self::source_ml_temporary_set_java_script_ordinary_prototype_loop108(heap.clone(), identity.clone(), prototype.clone(), depth);
                                                                match source_ml_source1_s10979.clone() {
                                                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Done => {
                                                                        {
                                                                            let replacement = match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field10980, _, source_ml_source1_nominal_field10982, source_ml_source1_nominal_field10983, source_ml_source1_nominal_field10984, source_ml_source1_nominal_field10985, source_ml_source1_nominal_field10986, source_ml_source1_nominal_field10987, source_ml_source1_nominal_field10988, source_ml_source1_nominal_field10989) => {
                                                                                    let source_ml_source1_nominal_field10988 = *source_ml_source1_nominal_field10988;
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field10980.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field10982, source_ml_source1_nominal_field10983, source_ml_source1_nominal_field10984.clone(), source_ml_source1_nominal_field10985.clone(), source_ml_source1_nominal_field10986, source_ml_source1_nominal_field10987.clone(), Box::new(source_ml_source1_nominal_field10988.clone()), source_ml_source1_nominal_field10989.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field10991, _, source_ml_source1_nominal_field10993, source_ml_source1_nominal_field10994, source_ml_source1_nominal_field10995, source_ml_source1_nominal_field10996, source_ml_source1_nominal_field10997, source_ml_source1_nominal_field10998) => {
                                                                                    let source_ml_source1_nominal_field10996 = *source_ml_source1_nominal_field10996;
                                                                                    let source_ml_source1_nominal_field10997 = *source_ml_source1_nominal_field10997;
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field10991.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field10993, source_ml_source1_nominal_field10994, source_ml_source1_nominal_field10995.clone(), Box::new(source_ml_source1_nominal_field10996.clone()), Box::new(source_ml_source1_nominal_field10997.clone()), source_ml_source1_nominal_field10998.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field11000, _, source_ml_source1_nominal_field11002, source_ml_source1_nominal_field11003, source_ml_source1_nominal_field11004) => {
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field11000.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11002, source_ml_source1_nominal_field11003, source_ml_source1_nominal_field11004.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field11006, _, source_ml_source1_nominal_field11008, source_ml_source1_nominal_field11009, source_ml_source1_nominal_field11010, source_ml_source1_nominal_field11011, source_ml_source1_nominal_field11012, source_ml_source1_nominal_field11013, source_ml_source1_nominal_field11014, source_ml_source1_nominal_field11015, source_ml_source1_nominal_field11016) => {
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field11006.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11008, source_ml_source1_nominal_field11009, source_ml_source1_nominal_field11010.clone(), source_ml_source1_nominal_field11011.clone(), source_ml_source1_nominal_field11012, source_ml_source1_nominal_field11013.clone(), source_ml_source1_nominal_field11014.clone(), source_ml_source1_nominal_field11015.clone(), source_ml_source1_nominal_field11016.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field11018, _, source_ml_source1_nominal_field11020, source_ml_source1_nominal_field11021, source_ml_source1_nominal_field11022, source_ml_source1_nominal_field11023) => {
                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field11018.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11020, source_ml_source1_nominal_field11021, source_ml_source1_nominal_field11022.clone(), source_ml_source1_nominal_field11023.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field11025, _, source_ml_source1_nominal_field11027, source_ml_source1_nominal_field11028, source_ml_source1_nominal_field11029, source_ml_source1_nominal_field11030) => {
                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field11025.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11027, source_ml_source1_nominal_field11028, source_ml_source1_nominal_field11029.clone(), source_ml_source1_nominal_field11030.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field11032, _, source_ml_source1_nominal_field11034, source_ml_source1_nominal_field11035, source_ml_source1_nominal_field11036, source_ml_source1_nominal_field11037) => {
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field11032.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11034, source_ml_source1_nominal_field11035, source_ml_source1_nominal_field11036.clone(), source_ml_source1_nominal_field11037.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field11039, _, source_ml_source1_nominal_field11041, source_ml_source1_nominal_field11042, source_ml_source1_nominal_field11043, source_ml_source1_nominal_field11044) => {
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field11039.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11041, source_ml_source1_nominal_field11042, source_ml_source1_nominal_field11043.clone(), source_ml_source1_nominal_field11044.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field11046, _, source_ml_source1_nominal_field11048, source_ml_source1_nominal_field11049, source_ml_source1_nominal_field11050, source_ml_source1_nominal_field11051, source_ml_source1_nominal_field11052, source_ml_source1_nominal_field11053, source_ml_source1_nominal_field11054) => {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field11046.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11048, source_ml_source1_nominal_field11049.clone(), source_ml_source1_nominal_field11050.clone(), source_ml_source1_nominal_field11051.clone(), source_ml_source1_nominal_field11052.clone(), source_ml_source1_nominal_field11053, source_ml_source1_nominal_field11054.clone())
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field11056, _, source_ml_source1_nominal_field11058, source_ml_source1_nominal_field11059, source_ml_source1_nominal_field11060, source_ml_source1_nominal_field11061, source_ml_source1_nominal_field11062) => {
                                                                                    let source_ml_source1_nominal_field11061 = *source_ml_source1_nominal_field11061;
                                                                                    let source_ml_source1_nominal_field11062 = *source_ml_source1_nominal_field11062;
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field11056.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11058, source_ml_source1_nominal_field11059, source_ml_source1_nominal_field11060.clone(), Box::new(source_ml_source1_nominal_field11061.clone()), Box::new(source_ml_source1_nominal_field11062.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field11064, _, source_ml_source1_nominal_field11066, source_ml_source1_nominal_field11067, source_ml_source1_nominal_field11068, source_ml_source1_nominal_field11069, source_ml_source1_nominal_field11070) => {
                                                                                    let source_ml_source1_nominal_field11069 = *source_ml_source1_nominal_field11069;
                                                                                    let source_ml_source1_nominal_field11070 = *source_ml_source1_nominal_field11070;
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field11064.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11066, source_ml_source1_nominal_field11067, source_ml_source1_nominal_field11068.clone(), Box::new(source_ml_source1_nominal_field11069.clone()), Box::new(source_ml_source1_nominal_field11070.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field11072, _, source_ml_source1_nominal_field11074, source_ml_source1_nominal_field11075, source_ml_source1_nominal_field11076, source_ml_source1_nominal_field11077, source_ml_source1_nominal_field11078) => {
                                                                                    let source_ml_source1_nominal_field11077 = *source_ml_source1_nominal_field11077;
                                                                                    let source_ml_source1_nominal_field11078 = *source_ml_source1_nominal_field11078;
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field11072.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11074, source_ml_source1_nominal_field11075, source_ml_source1_nominal_field11076.clone(), Box::new(source_ml_source1_nominal_field11077.clone()), Box::new(source_ml_source1_nominal_field11078.clone()))
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field11080, _, source_ml_source1_nominal_field11082, source_ml_source1_nominal_field11083, source_ml_source1_nominal_field11084, source_ml_source1_nominal_field11085, source_ml_source1_nominal_field11086) => {
                                                                                    let source_ml_source1_nominal_field11085 = *source_ml_source1_nominal_field11085;
                                                                                    let source_ml_source1_nominal_field11086 = *source_ml_source1_nominal_field11086;
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field11080.clone(), Box::new(prototype.clone()), source_ml_source1_nominal_field11082, source_ml_source1_nominal_field11083, source_ml_source1_nominal_field11084.clone(), Box::new(source_ml_source1_nominal_field11085.clone()), Box::new(source_ml_source1_nominal_field11086.clone()))
                                                                                }
                                                                            };
                                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                                let source_ml_source1_field11088 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                (self::ml_string::Text::from_units(vec![]), source_ml_source1_field11088.clone(), true)
                                                                            }))
                                                                        }
                                                                    }
                                                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Return(source_ml_temporary_result108) => {
                                                                        source_ml_temporary_result108.clone()
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

pub fn create_java_script_ordinary_set_descriptor(value: self::JavaScriptSemanticValue, creation: bool) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field11100 = self::JavaScriptSemanticValue::Undefined;
        {
            let source_ml_source1_field11102 = self::JavaScriptSemanticValue::Undefined;
            (creation, creation, creation, creation, source_ml_source1_field11100.clone(), false, source_ml_source1_field11102.clone(), false, value.clone(), true, creation, creation)
        }
    }))
}

pub fn plan_java_script_ordinary_set(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, value: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, self::JavaScriptSemanticValue, Option<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text, self::JavaScriptOrdinaryHeap, self::JavaScriptSemanticValue, bool)>> {
    self::plan_java_script_ordinary_set_definition(heap.clone(), identity.clone(), key.clone(), value.clone(), receiver.clone(), false)
}

pub fn plan_java_script_ordinary_set_with_array_definition(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, value: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, self::JavaScriptSemanticValue, Option<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text, self::JavaScriptOrdinaryHeap, self::JavaScriptSemanticValue, bool)>> {
    self::plan_java_script_ordinary_set_definition(heap.clone(), identity.clone(), key.clone(), value.clone(), receiver.clone(), true)
}

pub fn plan_java_script_ordinary_set_definition(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, value: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue, defer_array_length: bool) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, self::JavaScriptSemanticValue, Option<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text, self::JavaScriptOrdinaryHeap, self::JavaScriptSemanticValue, bool)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                {
                    let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    {
                        let value_error = self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                            {
                                let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                                    heap_error.clone()
                                } else {
                                    if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                        key_error.clone()
                                    } else {
                                        if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                            value_error.clone()
                                        } else {
                                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                                receiver_error.clone()
                                            } else {
                                                if ((self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone())) < (0f64)) {
                                                    self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
                                                }
                                            }
                                        }
                                    }
                                };
                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field11109 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source1_field11111 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                            (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), source_ml_source1_field11111.clone(), source_ml_source1_field11109.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, error.clone(), heap.clone(), receiver.clone(), false)
                                        }
                                    }))
                                } else {
                                    {
                                        let target = {
                                            let source_ml_source1_array_read_value11114 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source1_array_read_index11115 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                { let ml_reference_array249 = source_ml_source1_array_read_value11114.clone(); let ml_reference_position249 = source_ml_source1_array_read_index11115; let ml_reference_projection249 = self::ml_array::optional_at(&*ml_reference_array249.borrow(), self::ml_array::number_index(ml_reference_position249)); ml_reference_projection249 }
                                            }
                                        };
                                        if (((((((((!(match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_field11979 = (((match target.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, source_ml_source1_data_property_value11975) => {
                                                        let source_ml_source1_data_property_value11975 = *source_ml_source1_data_property_value11975;
                                                        Some((source_ml_source1_data_property_value11975.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, source_ml_source1_data_property_value11976) => {
                                                        let source_ml_source1_data_property_value11976 = *source_ml_source1_data_property_value11976;
                                                        Some((source_ml_source1_data_property_value11976.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, source_ml_source1_data_property_value11977) => {
                                                        let source_ml_source1_data_property_value11977 = *source_ml_source1_data_property_value11977;
                                                        Some((source_ml_source1_data_property_value11977.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, source_ml_source1_data_property_value11978) => {
                                                        let source_ml_source1_data_property_value11978 = *source_ml_source1_data_property_value11978;
                                                        Some((source_ml_source1_data_property_value11978.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone();
                                                {
                                                    let source_ml_source1_field11980 = self::JavaScriptSemanticValue::Object(identity.clone(), false);
                                                    {
                                                        let source_ml_source1_field11981 = std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone(), value.clone(), receiver.clone()]));
                                                        (self::ml_string::Text::from_units(vec![104, 111, 115, 116, 32, 99, 97, 108, 108]), source_ml_source1_field11981.clone(), source_ml_source1_field11979.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), source_ml_source1_field11980.clone(), false)
                                                    }
                                                }
                                            }))
                                        } else {
                                            {
                                                let located = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                                {
                                                    let own = if ({ let ml_reference_value250 = located.borrow().clone(); ml_reference_value250 }).1 {
                                                        ({ let ml_reference_value251 = located.borrow().clone(); ml_reference_value251 }).0
                                                    } else {
                                                        self::create_java_script_ordinary_set_descriptor(self::JavaScriptSemanticValue::Undefined, true)
                                                    };
                                                    if ({ let ml_reference_value252 = own.borrow().clone(); ml_reference_value252 }).5 {
                                                        {
                                                            let setter = ({ let ml_reference_value253 = own.borrow().clone(); ml_reference_value253 }).6;
                                                            match setter.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field11995 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field11997 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field11997.clone(), source_ml_source1_field11995.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Null => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12004 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field12004.clone(), setter.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12011 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field12011.clone(), setter.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Number(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12018 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field12018.clone(), setter.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12025 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field12025.clone(), setter.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12032 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field12032.clone(), setter.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12039 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field12039.clone(), setter.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12046 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field12046.clone(), setter.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12053 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field12053.clone(), setter.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                    }))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if !(({ let ml_reference_value254 = own.borrow().clone(); ml_reference_value254 }).10) {
                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source1_field12058 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field12060 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                    (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12060.clone(), source_ml_source1_field12058.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                }
                                                            }))
                                                        } else {
                                                            match receiver.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12073 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field12075 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12075.clone(), source_ml_source1_field12073.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Null => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12080 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field12082 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12082.clone(), source_ml_source1_field12080.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12087 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field12089 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12089.clone(), source_ml_source1_field12087.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Number(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12094 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field12096 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12096.clone(), source_ml_source1_field12094.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12101 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field12103 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12103.clone(), source_ml_source1_field12101.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12108 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field12110 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12110.clone(), source_ml_source1_field12108.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12115 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field12117 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12117.clone(), source_ml_source1_field12115.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field12122 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field12124 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field12124.clone(), source_ml_source1_field12122.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                        }
                                                                    }))
                                                                }
                                                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                                    {
                                                                        let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                                                        {
                                                                            let object = {
                                                                                let source_ml_source1_array_read_value12127 = shared_constructor_field1.clone();
                                                                                { let ml_reference_array255 = source_ml_source1_array_read_value12127.clone(); let ml_reference_position255 = index; let ml_reference_projection255 = self::ml_array::optional_at(&*ml_reference_array255.borrow(), self::ml_array::number_index(ml_reference_position255)); ml_reference_projection255 }
                                                                            };
                                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                                    let source_ml_source1_field12133 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source1_field12135 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                        (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), source_ml_source1_field12135.clone(), source_ml_source1_field12133.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]), heap.clone(), receiver.clone(), false)
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
                                                                                        let source_ml_source1_field12995 = self::JavaScriptSemanticValue::Undefined;
                                                                                        {
                                                                                            let source_ml_source1_field12997 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                            (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), source_ml_source1_field12997.clone(), source_ml_source1_field12995.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![97, 32, 98, 111, 114, 114, 111, 119, 101, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), receiver.clone(), false)
                                                                                        }
                                                                                    }))
                                                                                } else {
                                                                                    {
                                                                                        let at = self::find_java_script_ordinary_property((((match object.clone().expect("a proven present value") {
                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value12998, _, _, _, _, _) => {
                                                                                                Some((source_ml_source1_data_property_value12998.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value12999, _, _, _) => {
                                                                                                Some((source_ml_source1_data_property_value12999.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value13000) => {
                                                                                                Some((source_ml_source1_data_property_value13000.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value13001, _, _, _, _, _, _) => {
                                                                                                Some((source_ml_source1_data_property_value13001.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value13002, _) => {
                                                                                                Some((source_ml_source1_data_property_value13002.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value13003, _) => {
                                                                                                Some((source_ml_source1_data_property_value13003.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value13004, _) => {
                                                                                                Some((source_ml_source1_data_property_value13004.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value13005, _) => {
                                                                                                Some((source_ml_source1_data_property_value13005.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value13006) => {
                                                                                                Some((source_ml_source1_data_property_value13006.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                                                        {
                                                                                            let property = {
                                                                                                let source_ml_source1_array_read_value13016 = (((match object.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value13007, _, _, _, _, _) => {
                                                                                                        Some((source_ml_source1_data_property_value13007.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value13008, _, _, _) => {
                                                                                                        Some((source_ml_source1_data_property_value13008.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value13009) => {
                                                                                                        Some((source_ml_source1_data_property_value13009.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value13010, _, _, _, _, _, _) => {
                                                                                                        Some((source_ml_source1_data_property_value13010.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value13011, _) => {
                                                                                                        Some((source_ml_source1_data_property_value13011.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value13012, _) => {
                                                                                                        Some((source_ml_source1_data_property_value13012.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value13013, _) => {
                                                                                                        Some((source_ml_source1_data_property_value13013.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value13014, _) => {
                                                                                                        Some((source_ml_source1_data_property_value13014.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value13015) => {
                                                                                                        Some((source_ml_source1_data_property_value13015.clone(),))
                                                                                                    }
                                                                                                    _ => {
                                                                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                                                    }
                                                                                                }).clone().expect("a proven present value")).0).clone();
                                                                                                { let ml_reference_array256 = source_ml_source1_array_read_value13016.clone(); let ml_reference_position256 = at; let ml_reference_projection256 = self::ml_array::optional_at(&*ml_reference_array256.borrow(), self::ml_array::number_index(ml_reference_position256)); ml_reference_projection256 }
                                                                                            };
                                                                                            if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({ let ml_reference_value257 = (match property.clone().expect("a proven present value") {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value13018) => {
                                                                                                    source_ml_source1_data_property_value13018.clone()
                                                                                                }
                                                                                            }).borrow().clone(); ml_reference_value257 }).9) || !(({ let ml_reference_value258 = (match property.clone().expect("a proven present value") {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value13019) => {
                                                                                                    source_ml_source1_data_property_value13019.clone()
                                                                                                }
                                                                                            }).borrow().clone(); ml_reference_value258 }).10))) {
                                                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                    let source_ml_source1_field13024 = self::JavaScriptSemanticValue::Undefined;
                                                                                                    {
                                                                                                        let source_ml_source1_field13026 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                        (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]), source_ml_source1_field13026.clone(), source_ml_source1_field13024.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                                                    }
                                                                                                }))
                                                                                            } else {
                                                                                                {
                                                                                                    let proposed = self::create_java_script_ordinary_set_descriptor(value.clone(), match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false });
                                                                                                    if ((defer_array_length && match object.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                                            true
                                                                                                        }
                                                                                                        _ => {
                                                                                                            false
                                                                                                        }
                                                                                                    }) && self::test_java_script_array_length_key(key.clone())) {
                                                                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                            let source_ml_source1_field13126 = self::JavaScriptSemanticValue::Undefined;
                                                                                                            {
                                                                                                                let source_ml_source1_field13128 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                {
                                                                                                                    let source_ml_source1_field13129 = Some((Some(proposed.clone()),));
                                                                                                                    (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]), source_ml_source1_field13128.clone(), source_ml_source1_field13126.clone(), source_ml_source1_field13129.clone(), self::ml_string::Text::from_units(vec![]), heap.clone(), receiver.clone(), false)
                                                                                                                }
                                                                                                            }
                                                                                                        }))
                                                                                                    } else {
                                                                                                        {
                                                                                                            let transition = self::define_java_script_ordinary_property(heap.clone(), identity_2.clone(), key.clone(), proposed.clone());
                                                                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                let source_ml_source1_field13130 = ({ let ml_reference_value259 = transition.borrow().clone(); ml_reference_value259 }).1;
                                                                                                                {
                                                                                                                    let source_ml_source1_field13131 = if ((({ let ml_reference_value260 = transition.borrow().clone(); ml_reference_value260 }).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                        self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                                                    } else {
                                                                                                                        self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source1_field13132 = ({ let ml_reference_value261 = transition.borrow().clone(); ml_reference_value261 }).0;
                                                                                                                        {
                                                                                                                            let source_ml_source1_field13133 = ({ let ml_reference_value262 = transition.borrow().clone(); ml_reference_value262 }).3;
                                                                                                                            {
                                                                                                                                let source_ml_source1_field13134 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                {
                                                                                                                                    let source_ml_source1_field13136 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                                    (source_ml_source1_field13131.clone(), source_ml_source1_field13136.clone(), source_ml_source1_field13134.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>>,)>, source_ml_source1_field13132.clone(), source_ml_source1_field13130.clone(), receiver.clone(), source_ml_source1_field13133)
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
                }
            }
        }
    }
}
