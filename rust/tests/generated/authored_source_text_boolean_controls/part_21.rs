pub fn source_ml_temporary_set_java_script_ordinary_prototype_loop108(mut heap: self::JavaScriptOrdinaryHeap, mut identity: self::ml::Big, mut next: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
                if (depth <= ({ let ml_reference_array420 = shared_constructor_field1.clone(); let ml_reference_projection420 = ml_reference_array420.borrow().len() as f64; ml_reference_projection420 })) {
                    match next.clone() {
                        self::JavaScriptSemanticValue::Object(next_identity, _) => {
                            if (next_identity == identity) {
                                self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), heap.clone(), false))))
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), next_identity.clone());
                                    {
                                        let parent = {
                                            let source_ml_source1_array_read_value18403 = shared_constructor_field1.clone();
                                            { let ml_reference_array421 = source_ml_source1_array_read_value18403.clone(); let ml_reference_position421 = at; let ml_reference_projection421 = self::ml_array::optional_at(&*ml_reference_array421.borrow(), self::ml_array::number_index(ml_reference_position421)); ml_reference_projection421 }
                                        };
                                        if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]), heap.clone(), false))))
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
                                                self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 116, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 105, 108, 101, 110, 116, 108, 121, 32, 116, 114, 97, 118, 101, 114, 115, 101, 32, 97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114]), heap.clone(), false))))
                                            } else {
                                                {
                                                    let next_2 = (((match parent.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value19266, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value19266 = *source_ml_source1_data_property_value19266;
                                                            Some((source_ml_source1_data_property_value19266.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value19267, _, _, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value19267 = *source_ml_source1_data_property_value19267;
                                                            Some((source_ml_source1_data_property_value19267.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value19268, _, _, _) => {
                                                            let source_ml_source1_data_property_value19268 = *source_ml_source1_data_property_value19268;
                                                            Some((source_ml_source1_data_property_value19268.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value19269, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value19269 = *source_ml_source1_data_property_value19269;
                                                            Some((source_ml_source1_data_property_value19269.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value19270, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value19270 = *source_ml_source1_data_property_value19270;
                                                            Some((source_ml_source1_data_property_value19270.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value19271, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value19271 = *source_ml_source1_data_property_value19271;
                                                            Some((source_ml_source1_data_property_value19271.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value19272, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value19272 = *source_ml_source1_data_property_value19272;
                                                            Some((source_ml_source1_data_property_value19272.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value19273, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value19273 = *source_ml_source1_data_property_value19273;
                                                            Some((source_ml_source1_data_property_value19273.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value19274, _, _, _, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value19274 = *source_ml_source1_data_property_value19274;
                                                            Some((source_ml_source1_data_property_value19274.clone(),))
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
                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Done
                        }
                    }
                } else {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop110Result {
    MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop110Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>),
}

pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop111(mut positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut position: f64, mut at: f64) -> f64 {
    loop {
        return if ((at < ({ let ml_reference_array422 = positions.clone(); let ml_reference_projection422 = ml_reference_array422.borrow().len() as f64; ml_reference_projection422 })) && ((({ let ml_reference_array423 = positions.clone(); let ml_reference_position423 = at; let ml_reference_projection423 = self::ml_array::optional_at(&*ml_reference_array423.borrow(), self::ml_array::number_index(ml_reference_position423)); ml_reference_projection423 }).clone().unwrap_or_else(|| (-1f64))) < position)) {
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

pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop110(mut numeric: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut text: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_temporary_values109: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index109: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop110Result {
    loop {
        return if (source_ml_temporary_index109 < ({ let ml_reference_array424 = source_ml_temporary_values109.clone(); let ml_reference_projection424 = self::ml::Big::from_u128(ml_reference_array424.borrow().len() as u128); ml_reference_projection424 })) {
            {
                let property = { let ml_reference_array425 = source_ml_temporary_values109.clone(); let ml_reference_position425 = source_ml_temporary_index109.clone(); let ml_reference_projection425 = self::ml_array::at(&*ml_reference_array425.borrow(), ml_reference_position425.to_index()); ml_reference_projection425 };
                {
                    let key = match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value19281, _) => {
                            let source_ml_source1_data_property_value19281 = *source_ml_source1_data_property_value19281;
                            source_ml_source1_data_property_value19281.clone()
                        }
                    };
                    if (match key.clone() {
                        self::JavaScriptSemanticValue::Text(_) => {
                            true
                        }
                        _ => {
                            false
                        }
                    } || match key.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                            true
                        }
                        _ => {
                            false
                        }
                    }) {
                        {
                            let position = if ((self::read_java_script_semantic_text_length(key.clone())) > (10f64)) {
                                (-1f64)
                            } else {
                                self::read_java_script_literal_property_index(self::construct_java_script_semantic_text_units(key.clone(), 10f64))
                            };
                            if (position < (0f64)) {
                                {
                                    let text_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_concat_value19301 = std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone()]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value426 = text.borrow().clone(); ml_reference_value426 }), { let ml_reference_value427 = source_ml_source1_concat_value19301.borrow().clone(); ml_reference_value427 })
                                    }));
                                    {
                                        let source_ml_temporary_index109_2 = source_ml_temporary_index109.add(&self::ml::Big::from_i128(1));
                                        {
                                            (text, source_ml_temporary_index109) = (text_2.clone(), source_ml_temporary_index109_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = 0f64;
                                    {
                                        let at_2 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop111(positions.clone(), position, at);
                                        {
                                            let numeric_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_concat_value19302 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value428 = numeric.borrow().clone(); ml_reference_value428 }, 0f64, at_2)));
                                                {
                                                    let source_ml_source1_concat_value19303 = std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone()]));
                                                    {
                                                        let source_ml_source1_concat_value19304 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value429 = numeric.borrow().clone(); ml_reference_value429 }, at_2, f64::INFINITY)));
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value430 = source_ml_source1_concat_value19302.borrow().clone(); ml_reference_value430 }), { let ml_reference_value431 = source_ml_source1_concat_value19303.borrow().clone(); ml_reference_value431 }), { let ml_reference_value432 = source_ml_source1_concat_value19304.borrow().clone(); ml_reference_value432 })
                                                    }
                                                }
                                            }));
                                            {
                                                let positions_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_concat_value19305 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value433 = positions.borrow().clone(); ml_reference_value433 }, 0f64, at_2)));
                                                    {
                                                        let source_ml_source1_concat_value19306 = std::rc::Rc::new(std::cell::RefCell::new(vec![position]));
                                                        {
                                                            let source_ml_source1_concat_value19307 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value434 = positions.borrow().clone(); ml_reference_value434 }, at_2, f64::INFINITY)));
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value435 = source_ml_source1_concat_value19305.borrow().clone(); ml_reference_value435 }), { let ml_reference_value436 = source_ml_source1_concat_value19306.borrow().clone(); ml_reference_value436 }), { let ml_reference_value437 = source_ml_source1_concat_value19307.borrow().clone(); ml_reference_value437 })
                                                        }
                                                    }
                                                }));
                                                {
                                                    let source_ml_temporary_index109_3 = source_ml_temporary_index109.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (numeric, positions, source_ml_temporary_index109) = (numeric_2.clone(), positions_2.clone(), source_ml_temporary_index109_3.clone());
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
                                let source_ml_source1_concat_value19309 = std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone()]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value438 = symbols.borrow().clone(); ml_reference_value438 }), { let ml_reference_value439 = source_ml_source1_concat_value19309.borrow().clone(); ml_reference_value439 })
                            }));
                            {
                                let source_ml_temporary_index109_4 = source_ml_temporary_index109.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index109) = (symbols_2.clone(), source_ml_temporary_index109_4.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop110Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop110Done(numeric.clone(), text.clone(), symbols.clone())
        };
    }
}

pub fn source_ml_temporary_insert_java_script_ordinary_object_loop114(mut identity: self::ml::Big, mut at: f64, mut source_ml_temporary_values112: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut source_ml_temporary_index112: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index112 < ({ let ml_reference_array440 = source_ml_temporary_values112.clone(); let ml_reference_projection440 = self::ml::Big::from_u128(ml_reference_array440.borrow().len() as u128); ml_reference_projection440 })) {
            {
                let object = { let ml_reference_array441 = source_ml_temporary_values112.clone(); let ml_reference_position441 = source_ml_temporary_index112.clone(); let ml_reference_projection441 = self::ml_array::at(&*ml_reference_array441.borrow(), ml_reference_position441.to_index()); ml_reference_projection441 };
                if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value19325, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19325.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value19326, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19326.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value19327, _, _, _, _) => {
                        source_ml_source1_data_property_value19327.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value19328, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19328.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value19329, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19329.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value19330, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19330.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value19331, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19331.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value19332, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19332.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value19333, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19333.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value19334, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19334.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value19335, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19335.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value19336, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19336.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value19337, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value19337.clone()
                    }
                }) < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_temporary_index112_2 = source_ml_temporary_index112.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index112) = (at_2, source_ml_temporary_index112_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index112_3 = source_ml_temporary_index112.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index112 = source_ml_temporary_index112_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInsertJavaScriptOrdinaryObjectJoin113 {
    MlTemporaryInsertJavaScriptOrdinaryObjectJoin113Next,
    MlTemporaryInsertJavaScriptOrdinaryObjectJoin113Return(std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>>),
}

pub fn source_ml_temporary_register_java_script_ordinary_symbol_loop116(mut identity: self::ml::Big, mut at: f64, mut source_ml_temporary_values115: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut source_ml_temporary_index115: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index115 < ({ let ml_reference_array442 = source_ml_temporary_values115.clone(); let ml_reference_projection442 = self::ml::Big::from_u128(ml_reference_array442.borrow().len() as u128); ml_reference_projection442 })) {
            {
                let symbol = { let ml_reference_array443 = source_ml_temporary_values115.clone(); let ml_reference_position443 = source_ml_temporary_index115.clone(); let ml_reference_projection443 = self::ml_array::at(&*ml_reference_array443.borrow(), ml_reference_position443.to_index()); ml_reference_projection443 };
                if (symbol < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_temporary_index115_2 = source_ml_temporary_index115.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index115) = (at_2, source_ml_temporary_index115_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index115_3 = source_ml_temporary_index115.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index115 = source_ml_temporary_index115_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Result {
    MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Done,
    MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop117(mut state: self::JavaScriptDescriptorConversionState, mut index: f64) -> self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Result {
    loop {
        return if (index < (6f64)) {
            if self::test_java_script_descriptor_field_presence(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value19340, _) => {
                    source_ml_source1_data_property_value19340.clone()
                }
            }, index) {
                self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Return(self::ml_string::Text::from_units(vec![117, 110, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 102, 97, 98, 114, 105, 99, 97, 116, 101, 100, 32, 105, 110, 32, 97, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
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
            self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryStepJavaScriptDescriptorConversionLoop118Result {
    MlTemporaryStepJavaScriptDescriptorConversionLoop118Done(Box<self::JavaScriptDescriptorConversionState>),
    MlTemporaryStepJavaScriptDescriptorConversionLoop118Return(Box<self::JavaScriptDescriptorConversionTransition>),
}

pub fn source_ml_temporary_step_java_script_descriptor_conversion_loop118(mut heap: self::JavaScriptOrdinaryHeap, mut current: self::JavaScriptDescriptorConversionState) -> self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
                if ((match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value19342, _, _) => {
                        source_ml_source1_data_property_value19342
                    }
                }) < (6f64)) {
                    {
                        let field = self::read_java_script_descriptor_field(match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value19343, _, _) => {
                                source_ml_source1_data_property_value19343
                            }
                        });
                        {
                            let key = self::JavaScriptSemanticValue::Text(({ let ml_reference_value444 = field.borrow().clone(); ml_reference_value444 }).2);
                            {
                                let receiver = match current.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value19344, _, _, _) => {
                                        let source_ml_source1_data_property_value19344 = *source_ml_source1_data_property_value19344;
                                        source_ml_source1_data_property_value19344.clone()
                                    }
                                };
                                match receiver.clone() {
                                    self::JavaScriptSemanticValue::Object(identity, _) => {
                                        {
                                            let actual = {
                                                let source_ml_source1_array_read_value19355 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source1_array_read_index19356 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                    { let ml_reference_array445 = source_ml_source1_array_read_value19355.clone(); let ml_reference_position445 = source_ml_source1_array_read_index19356; let ml_reference_projection445 = self::ml_array::optional_at(&*ml_reference_array445.borrow(), self::ml_array::number_index(ml_reference_position445)); ml_reference_projection445 }
                                                }
                                            };
                                            if (!(match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(self::ml_array::includes(&{ let ml_reference_value446 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value446 }, &match actual.clone().expect("a proven present value") {
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
                                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]))))
                                            } else {
                                                {
                                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                                    if !(({ let ml_reference_value447 = property.borrow().clone(); ml_reference_value447 }).1) {
                                                        {
                                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value19357, _, _) => {
                                                                    source_ml_source1_data_property_value19357
                                                                }
                                                            } + 1f64), match current.clone() {
                                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value19358, _) => {
                                                                    source_ml_source1_data_property_value19358.clone()
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
                                                            if ((({ let ml_reference_value448 = plan.borrow().clone(); ml_reference_value448 }).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(({ let ml_reference_value449 = plan.borrow().clone(); ml_reference_value449 }).3)))
                                                            } else {
                                                                if ((({ let ml_reference_value450 = plan.borrow().clone(); ml_reference_value450 }).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value19359, _, _) => {
                                                                            source_ml_source1_data_property_value19359
                                                                        }
                                                                    }, match current.clone() {
                                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value19360, _) => {
                                                                            source_ml_source1_data_property_value19360.clone()
                                                                        }
                                                                    }, true)), Box::new(({ let ml_reference_value451 = plan.borrow().clone(); ml_reference_value451 }).2), Box::new(({ let ml_reference_value452 = plan.borrow().clone(); ml_reference_value452 }).4), ({ let ml_reference_value453 = plan.borrow().clone(); ml_reference_value453 }).1)))
                                                                } else {
                                                                    if ((({ let ml_reference_value454 = plan.borrow().clone(); ml_reference_value454 }).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                        self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))))
                                                                    } else {
                                                                        {
                                                                            let applied = self::apply_java_script_descriptor_field(current.clone(), ({ let ml_reference_value455 = plan.borrow().clone(); ml_reference_value455 }).5);
                                                                            match applied.clone() {
                                                                                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(current_3) => {
                                                                                    let current_3 = *current_3;
                                                                                    {
                                                                                        current = current_3.clone();
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                                _ => {
                                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Return(Box::new(applied.clone()))
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
                                        self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Done(Box::new(current.clone()))
                }
            }
        };
    }
}

fn ml_main() {
    assert!(((self::convert_java_script_semantic_boolean(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![])))) == (false)), "assertion 1");
    assert!(((self::convert_java_script_semantic_boolean(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![48])))) == (true)), "assertion 2");
    assert!(((self::convert_java_script_semantic_boolean(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![0])))) == (true)), "assertion 3");
    assert!(((self::convert_java_script_semantic_boolean(self::JavaScriptSemanticValue::Number((-0f64)))) == (false)), "assertion 4");
    assert!(((self::convert_java_script_semantic_boolean(self::JavaScriptSemanticValue::Number(f64::NAN))) == (false)), "assertion 5");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_source_text_boolean_controls() { main(); }
