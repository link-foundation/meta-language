pub fn read_java_script_ordinary_own_keys(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source1_field13137 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                        (error.clone(), source_ml_source1_field13137.clone())
                    }))
                } else {
                    {
                        let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                        {
                            let object = {
                                let source_ml_source1_array_read_value13141 = shared_constructor_field1.clone();
                                { let ml_reference_array263 = source_ml_source1_array_read_value13141.clone(); let ml_reference_position263 = index; let ml_reference_projection263 = self::ml_array::optional_at(&*ml_reference_array263.borrow(), self::ml_array::number_index(ml_reference_position263)); ml_reference_projection263 }
                            };
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_field13143 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                    (self::ml_string::Text::from_units(vec![79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 75, 101, 121, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]), source_ml_source1_field13143.clone())
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
                                        let source_ml_source1_field14000 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                        (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 111, 119, 110, 32, 107, 101, 121, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), source_ml_source1_field14000.clone())
                                    }))
                                } else {
                                    {
                                        let numeric = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                        {
                                            let positions = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                            {
                                                let text = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                {
                                                    let symbols = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                    {
                                                        let source_ml_temporary_values109 = (((match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value14002, _, _, _, _, _) => {
                                                                Some((source_ml_source1_data_property_value14002.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value14003, _, _, _) => {
                                                                Some((source_ml_source1_data_property_value14003.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value14004) => {
                                                                Some((source_ml_source1_data_property_value14004.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value14005, _, _, _, _, _, _) => {
                                                                Some((source_ml_source1_data_property_value14005.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value14006, _) => {
                                                                Some((source_ml_source1_data_property_value14006.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value14007, _) => {
                                                                Some((source_ml_source1_data_property_value14007.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value14008, _) => {
                                                                Some((source_ml_source1_data_property_value14008.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value14009, _) => {
                                                                Some((source_ml_source1_data_property_value14009.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value14010) => {
                                                                Some((source_ml_source1_data_property_value14010.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone();
                                                        {
                                                            let source_ml_temporary_index109 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source1_s14011 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop110(numeric.clone(), positions.clone(), text.clone(), symbols.clone(), source_ml_temporary_values109.clone(), source_ml_temporary_index109.clone());
                                                                match source_ml_source1_s14011.clone() {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop110Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop110Done(numeric_2, text_2, symbols_2) => {
                                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source1_field14015 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value264 = numeric_2.borrow().clone(); ml_reference_value264 }), { let ml_reference_value265 = text_2.borrow().clone(); ml_reference_value265 }), { let ml_reference_value266 = symbols_2.borrow().clone(); ml_reference_value266 })));
                                                                            (self::ml_string::Text::from_units(vec![]), source_ml_source1_field14015.clone())
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

pub fn insert_java_script_ordinary_object(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, prototype: self::JavaScriptSemanticValue) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    std::rc::Rc::new(std::cell::RefCell::new((heap_error.clone(), heap.clone(), false)))
                } else {
                    if (({ let ml_reference_array267 = shared_constructor_field1.clone(); let ml_reference_projection267 = ml_reference_array267.borrow().len() as f64; ml_reference_projection267 }) >= (4096f64)) {
                        std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]), heap.clone(), false)))
                    } else {
                        if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(shared_constructor_field2.clone(), identity.clone())) {
                            std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]), heap.clone(), false)))
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
                                            let source_ml_source1_s14925 = match prototype.clone() {
                                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                    {
                                                        let parent = {
                                                            let source_ml_source1_array_read_value14936 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source1_array_read_index14937 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                                                { let ml_reference_array268 = source_ml_source1_array_read_value14936.clone(); let ml_reference_position268 = source_ml_source1_array_read_index14937; let ml_reference_projection268 = self::ml_array::optional_at(&*ml_reference_array268.borrow(), self::ml_array::number_index(ml_reference_position268)); ml_reference_projection268 }
                                                            }
                                                        };
                                                        if (((((((((!(match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
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
                                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 119, 105, 116, 104, 32, 97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 104, 111, 115, 116, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), false))))
                                                        } else {
                                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113Next
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113Next
                                                }
                                            };
                                            match source_ml_source1_s14925.clone() {
                                                self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113Next => {
                                                    {
                                                        let at = 0f64;
                                                        {
                                                            let source_ml_temporary_values112 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_temporary_index112 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let at_2 = self::source_ml_temporary_insert_java_script_ordinary_object_loop114(identity.clone(), at, source_ml_temporary_values112.clone(), source_ml_temporary_index112.clone());
                                                                    {
                                                                        let object = {
                                                                            let source_ml_source1_constructor_field15801 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(identity.clone(), Box::new(prototype.clone()), true, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, false, source_ml_source1_constructor_field15801.clone())
                                                                        };
                                                                        {
                                                                            let objects = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                let source_ml_source1_concat_value15806 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value269 = shared_constructor_field1.borrow().clone(); ml_reference_value269 }, 0f64, at_2)));
                                                                                {
                                                                                    let source_ml_source1_concat_value15807 = std::rc::Rc::new(std::cell::RefCell::new(vec![object.clone()]));
                                                                                    {
                                                                                        let source_ml_source1_concat_value15808 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value270 = shared_constructor_field1.borrow().clone(); ml_reference_value270 }, at_2, f64::INFINITY)));
                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { let ml_reference_value271 = source_ml_source1_concat_value15806.borrow().clone(); ml_reference_value271 }), { let ml_reference_value272 = source_ml_source1_concat_value15807.borrow().clone(); ml_reference_value272 }), { let ml_reference_value273 = source_ml_source1_concat_value15808.borrow().clone(); ml_reference_value273 })
                                                                                    }
                                                                                }
                                                                            }));
                                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                                let source_ml_source1_field15823 = {
                                                                                    let source_ml_source1_nominal_field15810 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source1_nominal_field15812 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source1_nominal_field15814 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source1_nominal_field15816 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field15818 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_field15820 = shared_constructor_field6.clone();
                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects.clone(), source_ml_source1_nominal_field15812.clone(), source_ml_source1_nominal_field15814.clone(), source_ml_source1_nominal_field15816.clone(), source_ml_source1_nominal_field15818.clone(), source_ml_source1_nominal_field15820.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                };
                                                                                (self::ml_string::Text::from_units(vec![]), source_ml_source1_field15823.clone(), true)
                                                                            }))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113::MlTemporaryInsertJavaScriptOrdinaryObjectJoin113Return(source_ml_temporary_result114) => {
                                                    source_ml_temporary_result114.clone()
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

pub fn register_java_script_ordinary_symbol(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    std::rc::Rc::new(std::cell::RefCell::new((error.clone(), heap.clone(), false)))
                } else {
                    if (({ let ml_reference_array274 = shared_constructor_field2.clone(); let ml_reference_projection274 = ml_reference_array274.borrow().len() as f64; ml_reference_projection274 }) >= (16384f64)) {
                        std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]), heap.clone(), false)))
                    } else {
                        if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(shared_constructor_field2.clone(), identity.clone())) {
                            std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]), heap.clone(), false)))
                        } else {
                            {
                                let at = 0f64;
                                {
                                    let source_ml_temporary_values115 = shared_constructor_field2.clone();
                                    {
                                        let source_ml_temporary_index115 = self::ml::Big::from_i128(0);
                                        {
                                            let at_2 = self::source_ml_temporary_register_java_script_ordinary_symbol_loop116(identity.clone(), at, source_ml_temporary_values115.clone(), source_ml_temporary_index115.clone());
                                            {
                                                let symbols = std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_concat_value15845 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value275 = shared_constructor_field2.borrow().clone(); ml_reference_value275 }, 0f64, at_2)));
                                                    {
                                                        let source_ml_source1_concat_value15846 = std::rc::Rc::new(std::cell::RefCell::new(vec![identity.clone()]));
                                                        {
                                                            let source_ml_source1_concat_value15847 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value276 = shared_constructor_field2.borrow().clone(); ml_reference_value276 }, at_2, f64::INFINITY)));
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml::Big>::new(), { let ml_reference_value277 = source_ml_source1_concat_value15845.borrow().clone(); ml_reference_value277 }), { let ml_reference_value278 = source_ml_source1_concat_value15846.borrow().clone(); ml_reference_value278 }), { let ml_reference_value279 = source_ml_source1_concat_value15847.borrow().clone(); ml_reference_value279 })
                                                        }
                                                    }
                                                }));
                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_field15862 = {
                                                        let source_ml_source1_nominal_field15849 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source1_nominal_field15851 = shared_constructor_field2.clone();
                                                            {
                                                                let source_ml_source1_nominal_field15853 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source1_nominal_field15855 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source1_nominal_field15857 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field15859 = shared_constructor_field6.clone();
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_nominal_field15849.clone(), symbols.clone(), source_ml_source1_nominal_field15853.clone(), source_ml_source1_nominal_field15855.clone(), source_ml_source1_nominal_field15857.clone(), source_ml_source1_nominal_field15859.clone())
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    (self::ml_string::Text::from_units(vec![]), source_ml_source1_field15862.clone(), true)
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

pub fn read_java_script_descriptor_field(index: f64) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<f64>>>)>> {
    if (index == (0f64)) {
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source1_field15867 = std::rc::Rc::new(std::cell::RefCell::new(vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64]));
            (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]), source_ml_source1_field15867.clone())
        }))
    } else {
        if (index == (1f64)) {
            std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field15870 = std::rc::Rc::new(std::cell::RefCell::new(vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64]));
                (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]), source_ml_source1_field15870.clone())
            }))
        } else {
            if (index == (2f64)) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source1_field15873 = std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64]));
                    (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source1_field15873.clone())
                }))
            } else {
                if (index == (3f64)) {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source1_field15876 = std::rc::Rc::new(std::cell::RefCell::new(vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64]));
                        (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), source_ml_source1_field15876.clone())
                    }))
                } else {
                    if (index == (4f64)) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_field15879 = std::rc::Rc::new(std::cell::RefCell::new(vec![103f64, 101f64, 116f64]));
                            (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 116]), source_ml_source1_field15879.clone())
                        }))
                    } else {
                        if (index == (5f64)) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source1_field15882 = std::rc::Rc::new(std::cell::RefCell::new(vec![115f64, 101f64, 116f64]));
                                (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]), self::ml_string::Text::from_units(vec![115, 101, 116]), source_ml_source1_field15882.clone())
                            }))
                        } else {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source1_field15885 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), self::ml_string::Text::from_units(vec![]), source_ml_source1_field15885.clone())
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn convert_java_script_semantic_boolean(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![84, 111, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                value_value
            }
            self::JavaScriptSemanticValue::Number(value_value_2) => {
                ((value_value_2 != (0f64)) && !(value_value_2.is_nan()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_3) => {
                (value_value_3 != (self::ml::Big::from_i128(0)))
            }
            self::JavaScriptSemanticValue::Text(units) => {
                (({ let ml_reference_array280 = units.clone(); let ml_reference_projection280 = ml_reference_array280.borrow().len() as f64; ml_reference_projection280 }) != (0f64))
            }
            self::JavaScriptSemanticValue::SourceText(_, text) => {
                ((text.length()) != (0f64))
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                true
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                true
            }
            _ => {
                false
            }
        }
    }
}

pub fn create_java_script_empty_descriptor() -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field15895 = self::JavaScriptSemanticValue::Undefined;
        {
            let source_ml_source1_field15899 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_field15901 = self::JavaScriptSemanticValue::Undefined;
                (false, false, false, false, source_ml_source1_field15899.clone(), false, source_ml_source1_field15901.clone(), false, source_ml_source1_field15895.clone(), false, false, false)
            }
        }
    }))
}

pub fn test_java_script_descriptor_field_presence(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, index: f64) -> bool {
    if (index == (0f64)) {
        ({ let ml_reference_value281 = descriptor.borrow().clone(); ml_reference_value281 }).3
    } else {
        if (index == (1f64)) {
            ({ let ml_reference_value282 = descriptor.borrow().clone(); ml_reference_value282 }).1
        } else {
            if (index == (2f64)) {
                ({ let ml_reference_value283 = descriptor.borrow().clone(); ml_reference_value283 }).9
            } else {
                if (index == (3f64)) {
                    ({ let ml_reference_value284 = descriptor.borrow().clone(); ml_reference_value284 }).11
                } else {
                    if (index == (4f64)) {
                        ({ let ml_reference_value285 = descriptor.borrow().clone(); ml_reference_value285 }).5
                    } else {
                        if (index == (5f64)) {
                            ({ let ml_reference_value286 = descriptor.borrow().clone(); ml_reference_value286 }).7
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_descriptor_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if {
                        let source_ml_source1_s15903 = shared_constructor_field1.clone();
                        match source_ml_source1_s15903.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }
                    } {
                        self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((((!(self::ml_math::is_safe_integer(shared_constructor_field2)) || (shared_constructor_field2 < (0f64))) || (shared_constructor_field2 > (6f64))) || ((shared_constructor_field2 == (0f64)) && (((1f64 / shared_constructor_field2)) < (0f64)))) || (shared_constructor_field4 && (shared_constructor_field2 == (6f64)))) {
                                    self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                } else {
                                    {
                                        let index = shared_constructor_field2;
                                        {
                                            let source_ml_source1_s15928 = self::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop117(state.clone(), index);
                                            match source_ml_source1_s15928.clone() {
                                                self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Done => {
                                                    {
                                                        let descriptor = shared_constructor_field3.clone();
                                                        if (({ let ml_reference_value287 = descriptor.borrow().clone(); ml_reference_value287 }).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value288 = descriptor.borrow().clone(); ml_reference_value288 }).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 118, 97, 108, 117, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116])
                                                        } else {
                                                            if (({ let ml_reference_value289 = descriptor.borrow().clone(); ml_reference_value289 }).5 && (((self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value290 = descriptor.borrow().clone(); ml_reference_value290 }).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor(({ let ml_reference_value291 = descriptor.borrow().clone(); ml_reference_value291 }).4)))) {
                                                                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                            } else {
                                                                if (({ let ml_reference_value292 = descriptor.borrow().clone(); ml_reference_value292 }).7 && (((self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value293 = descriptor.borrow().clone(); ml_reference_value293 }).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor(({ let ml_reference_value294 = descriptor.borrow().clone(); ml_reference_value294 }).6)))) {
                                                                    self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop117Return(source_ml_temporary_result117) => {
                                                    source_ml_temporary_result117.clone()
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

pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            if (((shared_constructor_field2 == (4f64)) || (shared_constructor_field2 == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
            } else {
                {
                    let before = shared_constructor_field3.clone();
                    {
                        let enumerable = (shared_constructor_field2 == (0f64));
                        {
                            let configurable = (shared_constructor_field2 == (1f64));
                            {
                                let data_value = (shared_constructor_field2 == (2f64));
                                {
                                    let writable = (shared_constructor_field2 == (3f64));
                                    {
                                        let getter = (shared_constructor_field2 == (4f64));
                                        {
                                            let setter = !(((((enumerable || configurable) || data_value) || writable) || getter));
                                            {
                                                let flag = if ((enumerable || configurable) || writable) {
                                                    self::convert_java_script_semantic_boolean(value.clone())
                                                } else {
                                                    false
                                                };
                                                {
                                                    let descriptor = std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source1_spread15945 = { let ml_reference_value295 = before.borrow().clone(); ml_reference_value295 };
                                                        {
                                                            let source_ml_source1_spread_field15946 = (source_ml_source1_spread15945.clone()).0;
                                                            {
                                                                let source_ml_source1_spread_field15947 = (source_ml_source1_spread15945.clone()).1;
                                                                {
                                                                    let source_ml_source1_spread_field15948 = (source_ml_source1_spread15945.clone()).2;
                                                                    {
                                                                        let source_ml_source1_spread_field15949 = (source_ml_source1_spread15945.clone()).3;
                                                                        {
                                                                            let source_ml_source1_spread_field15950 = (source_ml_source1_spread15945.clone()).4;
                                                                            {
                                                                                let source_ml_source1_spread_field15951 = (source_ml_source1_spread15945.clone()).5;
                                                                                {
                                                                                    let source_ml_source1_spread_field15952 = (source_ml_source1_spread15945.clone()).6;
                                                                                    {
                                                                                        let source_ml_source1_spread_field15953 = (source_ml_source1_spread15945.clone()).7;
                                                                                        {
                                                                                            let source_ml_source1_spread_field15954 = (source_ml_source1_spread15945.clone()).8;
                                                                                            {
                                                                                                let source_ml_source1_spread_field15955 = (source_ml_source1_spread15945.clone()).9;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field15956 = (source_ml_source1_spread15945.clone()).10;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field15957 = (source_ml_source1_spread15945.clone()).11;
                                                                                                        {
                                                                                                            let source_ml_source1_field15958 = (enumerable || ({ let ml_reference_value296 = before.borrow().clone(); ml_reference_value296 }).3);
                                                                                                            {
                                                                                                                let source_ml_source1_field15959 = if enumerable {
                                                                                                                    flag
                                                                                                                } else {
                                                                                                                    ({ let ml_reference_value297 = before.borrow().clone(); ml_reference_value297 }).2
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source1_field15960 = (configurable || ({ let ml_reference_value298 = before.borrow().clone(); ml_reference_value298 }).1);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field15961 = if configurable {
                                                                                                                            flag
                                                                                                                        } else {
                                                                                                                            ({ let ml_reference_value299 = before.borrow().clone(); ml_reference_value299 }).0
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source1_field15962 = (data_value || ({ let ml_reference_value300 = before.borrow().clone(); ml_reference_value300 }).9);
                                                                                                                            {
                                                                                                                                let source_ml_source1_field15963 = if data_value {
                                                                                                                                    value.clone()
                                                                                                                                } else {
                                                                                                                                    ({ let ml_reference_value301 = before.borrow().clone(); ml_reference_value301 }).8
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source1_field15964 = (writable || ({ let ml_reference_value302 = before.borrow().clone(); ml_reference_value302 }).11);
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_field15965 = if writable {
                                                                                                                                            flag
                                                                                                                                        } else {
                                                                                                                                            ({ let ml_reference_value303 = before.borrow().clone(); ml_reference_value303 }).10
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_field15966 = (getter || ({ let ml_reference_value304 = before.borrow().clone(); ml_reference_value304 }).5);
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_field15967 = if getter {
                                                                                                                                                    value.clone()
                                                                                                                                                } else {
                                                                                                                                                    ({ let ml_reference_value305 = before.borrow().clone(); ml_reference_value305 }).4
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_field15968 = (setter || ({ let ml_reference_value306 = before.borrow().clone(); ml_reference_value306 }).7);
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_field15969 = if setter {
                                                                                                                                                            value.clone()
                                                                                                                                                        } else {
                                                                                                                                                            ({ let ml_reference_value307 = before.borrow().clone(); ml_reference_value307 }).6
                                                                                                                                                        };
                                                                                                                                                        (source_ml_source1_field15961, source_ml_source1_field15960, source_ml_source1_field15959, source_ml_source1_field15958, source_ml_source1_field15967.clone(), source_ml_source1_field15966, source_ml_source1_field15969.clone(), source_ml_source1_field15968, source_ml_source1_field15963.clone(), source_ml_source1_field15962, source_ml_source1_field15965, source_ml_source1_field15964)
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
                                                    }));
                                                    self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(shared_constructor_field1.clone()), (shared_constructor_field2 + 1f64), descriptor.clone(), false)))
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

pub fn step_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value15972) => {
                    source_ml_source1_data_property_value15972
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source1_s15973 = self::source_ml_temporary_step_java_script_descriptor_conversion_loop118(heap.clone(), current.clone());
                        match source_ml_source1_s15973.clone() {
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Done(current_2) => {
                                let current_2 = *current_2;
                                match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, shared_constructor_field1, _) => {
                                        if ((self::read_java_script_property_descriptor_error(shared_constructor_field1.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                        } else {
                                            self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(shared_constructor_field1.clone())
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop118Result::MlTemporaryStepJavaScriptDescriptorConversionLoop118Return(source_ml_temporary_result118) => {
                                let source_ml_temporary_result118 = *source_ml_temporary_result118;
                                source_ml_temporary_result118.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn begin_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, receiver: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(heap_error.clone())
        } else {
            {
                let error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::step_java_script_descriptor_conversion(heap.clone(), self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), 0f64, self::create_java_script_empty_descriptor(), false))
                        }
                        _ => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                    }
                }
            }
        }
    }
}

pub fn resume_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState, completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if !(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value15986) => {
                    source_ml_source1_data_property_value15986
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source1_data_property_value15987) => {
                            let source_ml_source1_data_property_value15987 = *source_ml_source1_data_property_value15987;
                            source_ml_source1_data_property_value15987.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source1_data_property_value15988) => {
                            let source_ml_source1_data_property_value15988 = *source_ml_source1_data_property_value15988;
                            source_ml_source1_data_property_value15988.clone()
                        }
                    });
                    if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(value_error.clone())
                    } else {
                        match completion.clone() {
                            self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                                let value = *value;
                                {
                                    let applied = self::apply_java_script_descriptor_field(state.clone(), value.clone());
                                    match applied.clone() {
                                        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state_2) => {
                                            let state_2 = *state_2;
                                            self::step_java_script_descriptor_conversion(heap.clone(), state_2.clone())
                                        }
                                        _ => {
                                            applied.clone()
                                        }
                                    }
                                }
                            }
                            self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                                let value_2 = *value_2;
                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(Box::new(value_2.clone()))
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}
