pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop85(mut registered_keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_temporary_values70: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>, mut source_ml_temporary_index70: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if (source_ml_temporary_index70 < ({ let ml_reference_array370 = source_ml_temporary_values70.clone(); let ml_reference_projection370 = self::ml::Big::from_u128(ml_reference_array370.borrow().len() as u128); ml_reference_projection370 })) {
            {
                let fact = { let ml_reference_array371 = source_ml_temporary_values70.clone(); let ml_reference_position371 = source_ml_temporary_index70.clone(); let ml_reference_projection371 = self::ml_array::at(&*ml_reference_array371.borrow(), ml_reference_position371.to_index()); ml_reference_projection371 };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source1_data_property_value17520) => {
                            ((source_ml_source1_data_property_value17520.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    };
                    if !(match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                        {
                            let registered_keys_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value372 = registered_keys.borrow().clone(); ml_reference_value372 }, vec![key.clone().expect("a proven present value")])));
                            {
                                let source_ml_temporary_index70_2 = source_ml_temporary_index70.add(&self::ml::Big::from_i128(1));
                                {
                                    (registered_keys, source_ml_temporary_index70) = (registered_keys_2.clone(), source_ml_temporary_index70_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_index70_3 = source_ml_temporary_index70.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index70 = source_ml_temporary_index70_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            registered_keys.clone()
        };
    }
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop89(mut heap: self::JavaScriptOrdinaryHeap, mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>> {
    loop {
        return if (index < ({ let ml_reference_array373 = match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17522, _, _, _, _, _) => {
                source_ml_source1_data_property_value17522.clone()
            }
        }; let ml_reference_projection373 = ml_reference_array373.borrow().len() as f64; ml_reference_projection373 })) {
            {
                let marks_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value374 = marks.borrow().clone(); ml_reference_value374 }, vec![std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source1_field17523 = 0f64;
                    (source_ml_source1_field17523,)
                }))])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (marks, index) = (marks_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            marks.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop92Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop92Done,
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop92(mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>, mut source_ml_temporary_values81: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index81: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop92Result {
    loop {
        return if (source_ml_temporary_index81 < ({ let ml_reference_array375 = source_ml_temporary_values81.clone(); let ml_reference_projection375 = self::ml::Big::from_u128(ml_reference_array375.borrow().len() as u128); ml_reference_projection375 })) {
            {
                let selected = { let ml_reference_array376 = source_ml_temporary_values81.clone(); let ml_reference_position376 = source_ml_temporary_index81.clone(); let ml_reference_projection376 = self::ml_array::at(&*ml_reference_array376.borrow(), ml_reference_position376.to_index()); ml_reference_projection376 };
                {
                    let mark = { let ml_reference_array377 = marks.clone(); let ml_reference_position377 = selected; let ml_reference_projection377 = self::ml_array::optional_at(&*ml_reference_array377.borrow(), self::ml_array::number_index(ml_reference_position377)); ml_reference_projection377 };
                    if !(match (mark.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                        {
                            let source_ml_temporary_read_java_script_ordinary_heap_error_written93 = { let ml_field_owner378 = mark.clone().expect("a proven present value"); let ml_field_value378 = 2f64; ml_field_owner378.borrow_mut().0 = ml_field_value378; () };
                            {
                                let source_ml_temporary_index81_2 = source_ml_temporary_index81.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index81 = source_ml_temporary_index81_2.clone();
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_index81_3 = source_ml_temporary_index81.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index81 = source_ml_temporary_index81_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop92Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop92Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop91(mut heap: self::JavaScriptOrdinaryHeap, mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>, mut index: f64, mut path: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
                if (index >= (0f64)) {
                    {
                        let current = {
                            let source_ml_source1_array_read_value17529 = shared_constructor_field1.clone();
                            { let ml_reference_array379 = source_ml_source1_array_read_value17529.clone(); let ml_reference_position379 = index; let ml_reference_projection379 = self::ml_array::optional_at(&*ml_reference_array379.borrow(), self::ml_array::number_index(ml_reference_position379)); ml_reference_projection379 }
                        };
                        {
                            let mark = { let ml_reference_array380 = marks.clone(); let ml_reference_position380 = index; let ml_reference_projection380 = self::ml_array::optional_at(&*ml_reference_array380.borrow(), self::ml_array::number_index(ml_reference_position380)); ml_reference_projection380 };
                            if ((match (current.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (mark.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((({ let ml_reference_value381 = (mark.clone().expect("a proven present value")).borrow().clone(); ml_reference_value381 }).0) == (2f64))) {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Done(path.clone())
                            } else {
                                match current.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, bound_target, _, _) => {
                                        let bound_target = *bound_target;
                                        if ((({ let ml_reference_value382 = (mark.clone().expect("a proven present value")).borrow().clone(); ml_reference_value382 }).0) == (1f64)) {
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 32, 99, 104, 97, 105, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 121, 99, 108, 105, 99, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114]))
                                        } else {
                                            {
                                                let source_ml_temporary_read_java_script_ordinary_heap_error_written94 = { let ml_field_owner383 = mark.clone().expect("a proven present value"); let ml_field_value383 = 1f64; ml_field_owner383.borrow_mut().0 = ml_field_value383; () };
                                                {
                                                    let path_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value384 = path.borrow().clone(); ml_reference_value384 }, vec![index])));
                                                    match bound_target.clone() {
                                                        self::JavaScriptSemanticValue::Object(identity, _) => {
                                                            {
                                                                let index_2 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                                {
                                                                    (index, path) = (index_2, path_2.clone());
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Done(path.clone())
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Done(path.clone())
                }
            }
        };
    }
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop90(mut heap: self::JavaScriptOrdinaryHeap, mut marks: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64,)>>>>>, mut first: f64) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Result {
    loop {
        return if (first < ({ let ml_reference_array385 = match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17637, _, _, _, _, _) => {
                source_ml_source1_data_property_value17637.clone()
            }
        }; let ml_reference_projection385 = ml_reference_array385.borrow().len() as f64; ml_reference_projection385 })) {
            {
                let index = first;
                {
                    let path = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let source_ml_source1_s17638 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop91(heap.clone(), marks.clone(), index, path.clone());
                        match source_ml_source1_s17638.clone() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Done(path_2) => {
                                {
                                    let source_ml_temporary_index81 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s17639 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop92(marks.clone(), path_2.clone(), source_ml_temporary_index81.clone());
                                        match source_ml_source1_s17639.clone() {
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop92Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop92Done => {
                                                {
                                                    let first_2 = (first + 1f64);
                                                    {
                                                        first = first_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop91Return(source_ml_temporary_result93) => {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Return(source_ml_temporary_result93.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop90Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop88(mut heap: self::JavaScriptOrdinaryHeap, mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>, mut source_ml_temporary_values80: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut source_ml_temporary_index80: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result {
    loop {
        return if (source_ml_temporary_index80 < ({ let ml_reference_array386 = source_ml_temporary_values80.clone(); let ml_reference_projection386 = self::ml::Big::from_u128(ml_reference_array386.borrow().len() as u128); ml_reference_projection386 })) {
            {
                let object = { let ml_reference_array387 = source_ml_temporary_values80.clone(); let ml_reference_position387 = source_ml_temporary_index80.clone(); let ml_reference_projection387 = self::ml_array::at(&*ml_reference_array387.borrow(), ml_reference_position387.to_index()); ml_reference_projection387 };
                {
                    let opaque_object_error = self::read_java_script_opaque_object_error(heap.clone(), object.clone());
                    if (opaque_object_error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(opaque_object_error.clone())
                    } else {
                        {
                            let closure_object_error = self::read_java_script_closure_object_error(heap.clone(), object.clone(), budget.clone());
                            if (closure_object_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(closure_object_error.clone())
                            } else {
                                {
                                    let bound_object_error = self::read_java_script_bound_object_error(heap.clone(), object.clone(), budget.clone());
                                    if (bound_object_error != (self::ml_string::Text::from_units(vec![]))) {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(bound_object_error.clone())
                                    } else {
                                        {
                                            let collection_iterator_object_error = self::read_java_script_collection_iterator_object_error(heap.clone(), object.clone(), budget.clone());
                                            if (collection_iterator_object_error != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(collection_iterator_object_error.clone())
                                            } else {
                                                {
                                                    let weak_collection_object_error = self::read_java_script_weak_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                    if (weak_collection_object_error != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(weak_collection_object_error.clone())
                                                    } else {
                                                        {
                                                            let strong_collection_object_error = self::read_java_script_strong_collection_object_error(heap.clone(), object.clone(), budget.clone());
                                                            if (strong_collection_object_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(strong_collection_object_error.clone())
                                                            } else {
                                                                {
                                                                    let ordinary_object_descriptor_error = self::read_java_script_ordinary_object_descriptor_error(heap.clone(), match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value17642, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17642 = *source_ml_source1_data_property_value17642;
                                                                            source_ml_source1_data_property_value17642.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value17643, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17643 = *source_ml_source1_data_property_value17643;
                                                                            source_ml_source1_data_property_value17643.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value17644, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17644 = *source_ml_source1_data_property_value17644;
                                                                            source_ml_source1_data_property_value17644.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value17645, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17645 = *source_ml_source1_data_property_value17645;
                                                                            source_ml_source1_data_property_value17645.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value17646, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17646 = *source_ml_source1_data_property_value17646;
                                                                            source_ml_source1_data_property_value17646.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value17647, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17647 = *source_ml_source1_data_property_value17647;
                                                                            source_ml_source1_data_property_value17647.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value17648, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17648 = *source_ml_source1_data_property_value17648;
                                                                            source_ml_source1_data_property_value17648.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value17649, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17649 = *source_ml_source1_data_property_value17649;
                                                                            source_ml_source1_data_property_value17649.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value17650, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17650 = *source_ml_source1_data_property_value17650;
                                                                            source_ml_source1_data_property_value17650.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source1_data_property_value17651, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17651 = *source_ml_source1_data_property_value17651;
                                                                            source_ml_source1_data_property_value17651.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source1_data_property_value17652, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17652 = *source_ml_source1_data_property_value17652;
                                                                            source_ml_source1_data_property_value17652.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source1_data_property_value17653, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17653 = *source_ml_source1_data_property_value17653;
                                                                            source_ml_source1_data_property_value17653.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source1_data_property_value17654, _, _, _, _, _) => {
                                                                            let source_ml_source1_data_property_value17654 = *source_ml_source1_data_property_value17654;
                                                                            source_ml_source1_data_property_value17654.clone()
                                                                        }
                                                                    }, match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value17655, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value17655.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value17656, _, _, _) => {
                                                                            source_ml_source1_data_property_value17656.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value17657) => {
                                                                            source_ml_source1_data_property_value17657.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value17658, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value17658.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value17659, _) => {
                                                                            source_ml_source1_data_property_value17659.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value17660, _) => {
                                                                            source_ml_source1_data_property_value17660.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value17661, _) => {
                                                                            source_ml_source1_data_property_value17661.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value17662, _) => {
                                                                            source_ml_source1_data_property_value17662.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value17663) => {
                                                                            source_ml_source1_data_property_value17663.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value17664, _, _) => {
                                                                            source_ml_source1_data_property_value17664.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value17665, _, _) => {
                                                                            source_ml_source1_data_property_value17665.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value17666, _, _) => {
                                                                            source_ml_source1_data_property_value17666.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value17667, _, _) => {
                                                                            source_ml_source1_data_property_value17667.clone()
                                                                        }
                                                                    }, budget.clone());
                                                                    if (ordinary_object_descriptor_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(ordinary_object_descriptor_error.clone())
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_s17763 = match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                    {
                                                                                        let array_error = self::read_java_script_array_object_error(object.clone());
                                                                                        if (array_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(array_error.clone())
                                                                                        } else {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next
                                                                                        }
                                                                                    }
                                                                                }
                                                                                _ => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next
                                                                                }
                                                                            };
                                                                            match source_ml_source1_s17763.clone() {
                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next => {
                                                                                    {
                                                                                        let ordinary_prototype_chain_error = self::read_java_script_ordinary_prototype_chain_error(heap.clone(), object.clone());
                                                                                        if (ordinary_prototype_chain_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(ordinary_prototype_chain_error.clone())
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_temporary_index80_2 = source_ml_temporary_index80.add(&self::ml::Big::from_i128(1));
                                                                                                {
                                                                                                    source_ml_temporary_index80 = source_ml_temporary_index80_2.clone();
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(source_ml_temporary_result95) => {
                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Return(source_ml_temporary_result95.clone())
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
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop88Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin87Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop99(mut registry: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>, mut key: self::JavaScriptSemanticValue, mut low: f64, mut high: f64) -> f64 {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = { let ml_reference_array388 = registry.clone(); let ml_reference_position388 = middle; let ml_reference_projection388 = self::ml_array::optional_at(&*ml_reference_array388.borrow(), self::ml_array::number_index(ml_reference_position388)); ml_reference_projection388 };
                    if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((self::compare_java_script_collection_keys(match entry.clone().expect("a proven present value") {
                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(_, source_ml_source1_data_property_value17862) => {
                            let source_ml_source1_data_property_value17862 = *source_ml_source1_data_property_value17862;
                            source_ml_source1_data_property_value17862.clone()
                        }
                    }, key.clone())) < (0f64))) {
                        {
                            let low_2 = (middle + 1f64);
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            high = middle;
                            continue;
                        }
                    }
                }
            }
        } else {
            low
        };
    }
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop98(mut registry: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>, mut source_ml_temporary_values78: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>, mut source_ml_temporary_index78: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Result {
    loop {
        return if (source_ml_temporary_index78 < ({ let ml_reference_array389 = source_ml_temporary_values78.clone(); let ml_reference_projection389 = self::ml::Big::from_u128(ml_reference_array389.borrow().len() as u128); ml_reference_projection389 })) {
            {
                let fact = { let ml_reference_array390 = source_ml_temporary_values78.clone(); let ml_reference_position390 = source_ml_temporary_index78.clone(); let ml_reference_projection390 = self::ml_array::at(&*ml_reference_array390.borrow(), ml_reference_position390.to_index()); ml_reference_projection390 };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source1_data_property_value17865) => {
                            ((source_ml_source1_data_property_value17865.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    };
                    if !(match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                        {
                            let low = 0f64;
                            {
                                let high = { let ml_reference_array391 = registry.clone(); let ml_reference_projection391 = ml_reference_array391.borrow().len() as f64; ml_reference_projection391 };
                                {
                                    let low_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop99(registry.clone(), key.clone().expect("a proven present value"), low, high);
                                    if {
                                        let source_ml_source1_comparison_operand17871 = {
                                            let source_ml_source1_optional_chain_receiver17868 = ({ let ml_reference_array392 = registry.clone(); let ml_reference_position392 = low_2; let ml_reference_projection392 = self::ml_array::optional_at(&*ml_reference_array392.borrow(), self::ml_array::number_index(ml_reference_position392)); ml_reference_projection392 },);
                                            if match (source_ml_source1_optional_chain_receiver17868.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_temporary_optional_chain79 = ((source_ml_source1_optional_chain_receiver17868.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_temporary_optional_chain79.clone() {
                                                        self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value17869, _) => {
                                                            source_ml_source1_data_property_value17869.clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::ml::Big>
                                            }
                                        };
                                        {
                                            let source_ml_source1_comparison_operand17872 = match fact.clone() {
                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value17870, _, _) => {
                                                    source_ml_source1_data_property_value17870.clone()
                                                }
                                            };
                                            !(match (source_ml_source1_comparison_operand17871.clone(), source_ml_source1_comparison_operand17872.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                        }
                                    } {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Return(self::ml_string::Text::from_units(vec![107, 110, 111, 119, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 109, 111, 110, 111, 116, 111, 110, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121]))
                                    } else {
                                        {
                                            let source_ml_temporary_index78_2 = source_ml_temporary_index78.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_temporary_index78 = source_ml_temporary_index78_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_index78_3 = source_ml_temporary_index78.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index78 = source_ml_temporary_index78_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop98Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop97(mut previous_identity: Option<self::JavaScriptSemanticValue>, mut source_ml_temporary_values77: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_temporary_index77: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result {
    loop {
        return if (source_ml_temporary_index77 < ({ let ml_reference_array393 = source_ml_temporary_values77.clone(); let ml_reference_projection393 = self::ml::Big::from_u128(ml_reference_array393.borrow().len() as u128); ml_reference_projection393 })) {
            {
                let current = { let ml_reference_array394 = source_ml_temporary_values77.clone(); let ml_reference_position394 = source_ml_temporary_index77.clone(); let ml_reference_projection394 = self::ml_array::at(&*ml_reference_array394.borrow(), ml_reference_position394.to_index()); ml_reference_projection394 };
                if (!(match (previous_identity.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(previous_identity.clone().expect("a proven present value"), current.clone())) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(self::ml_string::Text::from_units(vec![111, 110, 101, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121]))
                } else {
                    {
                        let source_ml_temporary_index77_2 = source_ml_temporary_index77.add(&self::ml::Big::from_i128(1));
                        {
                            (previous_identity, source_ml_temporary_index77) = (Some(current.clone()), source_ml_temporary_index77_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop100(mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>, mut entry: self::MlOptional<self::JavaScriptSymbolRegistryEntry>, mut low: f64, mut high: f64) -> f64 {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                if ((({
                    let source_ml_source1_optional_chain_receiver17887 = ({ let ml_reference_array395 = facts.clone(); let ml_reference_position395 = middle; let ml_reference_projection395 = self::ml_array::optional_at(&*ml_reference_array395.borrow(), self::ml_array::number_index(ml_reference_position395)); ml_reference_projection395 },);
                    if match (source_ml_source1_optional_chain_receiver17887.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        Some({
                            let source_ml_temporary_optional_chain76 = ((source_ml_source1_optional_chain_receiver17887.clone()).0).clone().expect("a proven present value");
                            match source_ml_temporary_optional_chain76.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value17888, _, _) => {
                                    source_ml_source1_data_property_value17888.clone()
                                }
                            }
                        })
                    } else {
                        None::<self::ml::Big>
                    }
                }).clone().unwrap_or_else(|| {
                    let source_ml_source1_nullish_field_receiver17889 = (entry.clone(),);
                    if match (source_ml_source1_nullish_field_receiver17889.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                        {
                            let source_ml_source1_nullish_field_present17890 = match (source_ml_source1_nullish_field_receiver17889.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                            match source_ml_source1_nullish_field_present17890.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value17891, _) => {
                                    source_ml_source1_data_property_value17891.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source1_nullish_field_receiver17889.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                            self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])
                        } else {
                            self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])
                        }).0, message_present: true }).raise()
                    }
                })) < ({
                    let source_ml_source1_nullish_field_receiver17892 = (entry.clone(),);
                    if match (source_ml_source1_nullish_field_receiver17892.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                        {
                            let source_ml_source1_nullish_field_present17893 = match (source_ml_source1_nullish_field_receiver17892.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                            match source_ml_source1_nullish_field_present17893.clone() {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(source_ml_source1_data_property_value17894, _) => {
                                    source_ml_source1_data_property_value17894.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source1_nullish_field_receiver17892.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                            self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])
                        } else {
                            self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])
                        }).0, message_present: true }).raise()
                    }
                })) {
                    {
                        let low_2 = (middle + 1f64);
                        {
                            low = low_2;
                            continue;
                        }
                    }
                } else {
                    {
                        high = middle;
                        continue;
                    }
                }
            }
        } else {
            low
        };
    }
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop96(mut heap: self::JavaScriptOrdinaryHeap, mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut previous_key: self::JavaScriptSemanticValue, mut units: f64, mut source_ml_temporary_values72: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>, mut source_ml_temporary_index72: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result {
    loop {
        return if (source_ml_temporary_index72 < ({ let ml_reference_array396 = source_ml_temporary_values72.clone(); let ml_reference_projection396 = self::ml::Big::from_u128(ml_reference_array396.borrow().len() as u128); ml_reference_projection396 })) {
            {
                let entry = { let ml_reference_array397 = source_ml_temporary_values72.clone(); let ml_reference_position397 = source_ml_temporary_index72.clone(); let ml_reference_projection397 = self::ml_array::at(&*ml_reference_array397.borrow(), ml_reference_position397.to_index()); ml_reference_projection397 };
                match entry.clone() {
                    self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(shared_constructor_field1, shared_constructor_field2) => {
                        let shared_constructor_field2 = *shared_constructor_field2;
                        if ((((({
                            let source_ml_source1_optional_chain_receiver17898 = (entry.clone(),);
                            {
                                let source_ml_temporary_optional_chain73 = (source_ml_source1_optional_chain_receiver17898.clone()).0;
                                match source_ml_temporary_optional_chain73.clone() {
                                    self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(_, _) => {
                                        self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 121, 69, 110, 116, 114, 121])
                                    }
                                }
                            }
                        }) != (self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 121, 69, 110, 116, 114, 121]))) || !(self::test_java_script_ordinary_symbol(match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value17899, _, _, _, _) => {
                                source_ml_source1_data_property_value17899.clone()
                            }
                        }, shared_constructor_field1.clone()))) || ((({
                            let source_ml_source1_optional_chain_receiver17904 = (shared_constructor_field2.clone(),);
                            {
                                let source_ml_temporary_optional_chain74 = (source_ml_source1_optional_chain_receiver17904.clone()).0;
                                match source_ml_temporary_optional_chain74.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                    }
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                    }
                                }
                            }
                        }) != (self::ml_string::Text::from_units(vec![116, 101, 120, 116]))) && (({
                            let source_ml_source1_optional_chain_receiver17908 = (shared_constructor_field2.clone(),);
                            {
                                let source_ml_temporary_optional_chain75 = (source_ml_source1_optional_chain_receiver17908.clone()).0;
                                match source_ml_temporary_optional_chain75.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                    }
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                    }
                                }
                            }
                        }) != (self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116]))))) || ((self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field2.clone())) != (self::ml_string::Text::from_units(vec![])))) {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 107, 101, 121, 115]))
                        } else {
                            {
                                let units_2 = (units + self::read_java_script_semantic_text_length(shared_constructor_field2.clone()));
                                if (((self::read_java_script_semantic_text_length(shared_constructor_field2.clone())) > (1048576f64)) || (units_2 > (16777216f64))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                                } else {
                                    if (match previous_key.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    } && ((self::compare_java_script_collection_keys(previous_key.clone(), shared_constructor_field2.clone())) >= (0f64))) {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 115, 116, 114, 105, 99, 116, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114]))
                                    } else {
                                        {
                                            let previous_key_2 = shared_constructor_field2.clone();
                                            {
                                                let low = 0f64;
                                                {
                                                    let high = { let ml_reference_array398 = facts.clone(); let ml_reference_projection398 = ml_reference_array398.borrow().len() as f64; ml_reference_projection398 };
                                                    {
                                                        let low_2 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop100(facts.clone(), self::MlOptional::Value(entry.clone()), low, high);
                                                        {
                                                            let fact = { let ml_reference_array399 = facts.clone(); let ml_reference_position399 = low_2; let ml_reference_projection399 = self::ml_array::optional_at(&*ml_reference_array399.borrow(), self::ml_array::number_index(ml_reference_position399)); ml_reference_projection399 };
                                                            if ((match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value17930, _, _) => {
                                                                    source_ml_source1_data_property_value17930.clone()
                                                                }
                                                            }) != shared_constructor_field1)) || !(match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source1_data_property_value17932, _) => {
                                                                    source_ml_source1_data_property_value17932
                                                                }
                                                            })) {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                            } else {
                                                                {
                                                                    let key = match fact.clone().expect("a proven present value") {
                                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source1_data_property_value17933) => {
                                                                            ((source_ml_source1_data_property_value17933.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    };
                                                                    if (match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(key.clone().expect("a proven present value"), shared_constructor_field2.clone()))) {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 101, 120, 97, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 110, 101, 115, 115]))
                                                                    } else {
                                                                        {
                                                                            let identities_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value400 = identities.borrow().clone(); ml_reference_value400 }, vec![self::JavaScriptSemanticValue::Symbol(shared_constructor_field1.clone())])));
                                                                            {
                                                                                let source_ml_temporary_index72_2 = source_ml_temporary_index72.add(&self::ml::Big::from_i128(1));
                                                                                {
                                                                                    (identities, previous_key, units, source_ml_temporary_index72) = (identities_2.clone(), previous_key_2.clone(), units_2, source_ml_temporary_index72_2.clone());
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done(identities.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop86(mut witnessed_units: f64, mut previous_witness: Option<self::JavaScriptSemanticValue>, mut source_ml_temporary_values71: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_temporary_index71: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Result {
    loop {
        return if (source_ml_temporary_index71 < ({ let ml_reference_array401 = source_ml_temporary_values71.clone(); let ml_reference_projection401 = self::ml::Big::from_u128(ml_reference_array401.borrow().len() as u128); ml_reference_projection401 })) {
            {
                let key = { let ml_reference_array402 = source_ml_temporary_values71.clone(); let ml_reference_position402 = source_ml_temporary_index71.clone(); let ml_reference_projection402 = self::ml_array::at(&*ml_reference_array402.borrow(), ml_reference_position402.to_index()); ml_reference_projection402 };
                {
                    let witnessed_units_2 = (witnessed_units + self::read_java_script_semantic_text_length(key.clone()));
                    if (witnessed_units_2 > (16777216f64)) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else {
                        if (!(match (previous_witness.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(previous_witness.clone().expect("a proven present value"), key.clone())) {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 104, 97, 118, 101, 32, 117, 110, 105, 113, 117, 101, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 115]))
                        } else {
                            {
                                let source_ml_temporary_index71_2 = source_ml_temporary_index71.add(&self::ml::Big::from_i128(1));
                                {
                                    (witnessed_units, previous_witness, source_ml_temporary_index71) = (witnessed_units_2, Some(key.clone()), source_ml_temporary_index71_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop86Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin84Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(self::ml_string::Text),
}
