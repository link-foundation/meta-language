pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop101(mut heap: self::JavaScriptOrdinaryHeap, mut prior: self::ml::Big, mut source_ml_temporary_values68: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>, mut source_ml_temporary_index68: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result {
    loop {
        return if (source_ml_temporary_index68 < ({ let ml_reference_array403 = source_ml_temporary_values68.clone(); let ml_reference_projection403 = self::ml::Big::from_u128(ml_reference_array403.borrow().len() as u128); ml_reference_projection403 })) {
            {
                let fact = { let ml_reference_array404 = source_ml_temporary_values68.clone(); let ml_reference_position404 = source_ml_temporary_index68.clone(); let ml_reference_projection404 = self::ml_array::at(&*ml_reference_array404.borrow(), ml_reference_position404.to_index()); ml_reference_projection404 };
                match fact.clone() {
                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(shared_constructor_field1, shared_constructor_field2, _) => {
                        if (((((({
                            let source_ml_source1_optional_chain_receiver17941 = (fact.clone(),);
                            {
                                let source_ml_temporary_optional_chain69 = (source_ml_source1_optional_chain_receiver17941.clone()).0;
                                match source_ml_temporary_optional_chain69.clone() {
                                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, _) => {
                                        self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 70, 97, 99, 116])
                                    }
                                }
                            }
                        }) != (self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 70, 97, 99, 116]))) || (({
                            let source_ml_source1_type_query_operand17945 = shared_constructor_field2;
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                        }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || (({
                            let source_ml_source1_type_query_operand17949 = shared_constructor_field1.clone();
                            self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                        }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])))) || (shared_constructor_field1 <= prior)) || !(self::test_java_script_ordinary_symbol(match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value17952, _, _, _, _) => {
                                source_ml_source1_data_property_value17952.clone()
                            }
                        }, shared_constructor_field1.clone()))) {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 102, 97, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let prior_2 = shared_constructor_field1.clone();
                                {
                                    let key = match fact.clone() {
                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source1_data_property_value17955) => {
                                            ((source_ml_source1_data_property_value17955.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                        }
                                    };
                                    if (!(match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (((!shared_constructor_field2 || (match key.clone().expect("a proven present value") {
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    } && match key.clone().expect("a proven present value") {
                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    })) || ((self::read_java_script_ordinary_value_error(heap.clone(), key.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![])))) || ((self::read_java_script_semantic_text_length(key.clone().expect("a proven present value"))) > (1048576f64)))) {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(self::ml_string::Text::from_units(vec![114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 32, 119, 105, 116, 110, 101, 115, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116]))
                                    } else {
                                        {
                                            let source_ml_temporary_index68_2 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                            {
                                                (prior, source_ml_temporary_index68) = (prior_2.clone(), source_ml_temporary_index68_2.clone());
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
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop83(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_temporary_values67: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut source_ml_temporary_index67: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Result {
    loop {
        return if (source_ml_temporary_index67 < ({ let ml_reference_array405 = source_ml_temporary_values67.clone(); let ml_reference_projection405 = self::ml::Big::from_u128(ml_reference_array405.borrow().len() as u128); ml_reference_projection405 })) {
            {
                let symbol = { let ml_reference_array406 = source_ml_temporary_values67.clone(); let ml_reference_position406 = source_ml_temporary_index67.clone(); let ml_reference_projection406 = self::ml_array::at(&*ml_reference_array406.borrow(), ml_reference_position406.to_index()); ml_reference_projection406 };
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17978, _, _, _, _, _) => {
                        source_ml_source1_data_property_value17978.clone()
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_index67_2 = source_ml_temporary_index67.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_temporary_index67) = (symbol.clone(), source_ml_temporary_index67_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop83Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Done(self::ml::Big),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop82(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_temporary_values66: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut source_ml_temporary_index66: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Result {
    loop {
        return if (source_ml_temporary_index66 < ({ let ml_reference_array407 = source_ml_temporary_values66.clone(); let ml_reference_projection407 = self::ml::Big::from_u128(ml_reference_array407.borrow().len() as u128); ml_reference_projection407 })) {
            {
                let object = { let ml_reference_array408 = source_ml_temporary_values66.clone(); let ml_reference_position408 = source_ml_temporary_index66.clone(); let ml_reference_projection408 = self::ml_array::at(&*ml_reference_array408.borrow(), ml_reference_position408.to_index()); ml_reference_projection408 };
                {
                    let source_ml_source1_s18093 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, immutable_prototype, regular_expression, string_data, primitive_data, callable, _) => {
                            if ({
                                let source_ml_source1_comparison_operand18184 = ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                {
                                    let source_ml_source1_comparison_operand18185 = None::<()>;
                                    !(match (source_ml_source1_comparison_operand18184.clone(), source_ml_source1_comparison_operand18185.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                }
                            } && ((match ((regular_expression.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Return(self::ml_string::Text::from_units(vec![114, 101, 103, 117, 108, 97, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 66, 111, 111, 108, 101, 97, 110]))
                            } else {
                                {
                                    let primitive = ((primitive_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                    {
                                        let text = ((string_data.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                        if (!(match (primitive.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (((callable || !(self::ml_array::includes(&{ let ml_reference_value409 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])]))).borrow().clone(); ml_reference_value409 }, &match primitive.clone().expect("a proven present value") {
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
                                        }, 0f64, false, |a, b| a == b))) || ((self::read_java_script_ordinary_value_error(heap.clone(), primitive.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![])))) || !(match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Return(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 66, 111, 111, 108, 101, 97, 110, 32, 66, 105, 103, 73, 110, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116]))
                                        } else {
                                            if (!(match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((callable || (match text.clone().expect("a proven present value") {
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } && match text.clone().expect("a proven present value") {
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            })) || ((self::read_java_script_semantic_value_error(text.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![]))))) {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 83, 116, 114, 105, 110, 103, 68, 97, 116, 97]))
                                            } else {
                                                if ({
                                                    let source_ml_source1_comparison_operand18204 = ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                    {
                                                        let source_ml_source1_comparison_operand18205 = None::<()>;
                                                        !(match (source_ml_source1_comparison_operand18204.clone(), source_ml_source1_comparison_operand18205.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                    }
                                                } && ((match ((immutable_prototype.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Return(self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 115, 108, 111, 116]))
                                                } else {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Next
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Next
                        }
                    };
                    match source_ml_source1_s18093.clone() {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Next => {
                            if !(self::ml_array::includes(&{ let ml_reference_value410 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value410 }, &match object.clone() {
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
                            }, 0f64, false, |a, b| a == b)) {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 107, 105, 110, 100]))
                            } else {
                                if (((({
                                    let source_ml_source1_type_query_operand18233 = match object.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18220, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18220.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18221, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18221.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18222, _, _, _, _) => {
                                            source_ml_source1_data_property_value18222.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18223, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18223.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18224, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18224.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18225, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18225.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18226, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18226.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18227, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18227.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18228, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18228.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18229, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18229.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18230, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18230.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18231, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18231.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18232, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18232.clone()
                                        }
                                    };
                                    self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]))) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18247, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18247.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18248, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18248.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18249, _, _, _, _) => {
                                        source_ml_source1_data_property_value18249.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18250, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18250.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18251, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18251.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18252, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18252.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18253, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18253.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18254, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18254.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18255, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18255.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18256, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18256.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18257, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18257.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18258, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18258.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18259, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18259.clone()
                                    }
                                }) <= previous)) || ((match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18273, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18273.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18274, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18274.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18275, _, _, _, _) => {
                                        source_ml_source1_data_property_value18275.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18276, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18276.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18277, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18277.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18278, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18278.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18279, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18279.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18280, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18280.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18281, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18281.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18282, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18282.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18283, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18283.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18284, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18284.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18285, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18285.clone()
                                    }
                                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_2 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value18286, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18286.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value18287, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18287.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value18288, _, _, _, _) => {
                                                source_ml_source1_data_property_value18288.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value18289, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18289.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value18290, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18290.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value18291, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18291.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value18292, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18292.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value18293, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18293.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value18294, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18294.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value18295, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18295.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value18296, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18296.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value18297, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18297.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value18298, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value18298.clone()
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index66_2 = source_ml_temporary_index66.add(&self::ml::Big::from_i128(1));
                                            {
                                                (previous, source_ml_temporary_index66) = (previous_2.clone(), source_ml_temporary_index66_2.clone());
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin102Return(source_ml_temporary_result102) => {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Return(source_ml_temporary_result102.clone())
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop82Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Result {
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Done,
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Return(std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, bool, self::ml::Big)>>),
}

pub fn source_ml_temporary_locate_java_script_ordinary_property_loop103(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
                if (depth < ({ let ml_reference_array411 = shared_constructor_field1.clone(); let ml_reference_projection411 = ml_reference_array411.borrow().len() as f64; ml_reference_projection411 })) {
                    {
                        let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), current.clone());
                        {
                            let object = {
                                let source_ml_source1_array_read_value18303 = shared_constructor_field1.clone();
                                { let ml_reference_array412 = source_ml_source1_array_read_value18303.clone(); let ml_reference_position412 = index; let ml_reference_projection412 = self::ml_array::optional_at(&*ml_reference_array412.borrow(), self::ml_array::number_index(ml_reference_position412)); ml_reference_projection412 }
                            };
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Done
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value18305, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18305.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value18306, _, _, _) => {
                                            source_ml_source1_data_property_value18306.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value18307) => {
                                            source_ml_source1_data_property_value18307.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value18308, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value18308.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value18309, _) => {
                                            source_ml_source1_data_property_value18309.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value18310, _) => {
                                            source_ml_source1_data_property_value18310.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value18311, _) => {
                                            source_ml_source1_data_property_value18311.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value18312, _) => {
                                            source_ml_source1_data_property_value18312.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value18313) => {
                                            source_ml_source1_data_property_value18313.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value18314, _, _) => {
                                            source_ml_source1_data_property_value18314.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value18315, _, _) => {
                                            source_ml_source1_data_property_value18315.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value18316, _, _) => {
                                            source_ml_source1_data_property_value18316.clone()
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value18317, _, _) => {
                                            source_ml_source1_data_property_value18317.clone()
                                        }
                                    }, key.clone());
                                    {
                                        let property = {
                                            let source_ml_source1_array_read_value18331 = match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value18318, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value18318.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value18319, _, _, _) => {
                                                    source_ml_source1_data_property_value18319.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value18320) => {
                                                    source_ml_source1_data_property_value18320.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value18321, _, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value18321.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value18322, _) => {
                                                    source_ml_source1_data_property_value18322.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value18323, _) => {
                                                    source_ml_source1_data_property_value18323.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value18324, _) => {
                                                    source_ml_source1_data_property_value18324.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value18325, _) => {
                                                    source_ml_source1_data_property_value18325.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value18326) => {
                                                    source_ml_source1_data_property_value18326.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value18327, _, _) => {
                                                    source_ml_source1_data_property_value18327.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value18328, _, _) => {
                                                    source_ml_source1_data_property_value18328.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value18329, _, _) => {
                                                    source_ml_source1_data_property_value18329.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value18330, _, _) => {
                                                    source_ml_source1_data_property_value18330.clone()
                                                }
                                            };
                                            { let ml_reference_array413 = source_ml_source1_array_read_value18331.clone(); let ml_reference_position413 = at; let ml_reference_projection413 = self::ml_array::optional_at(&*ml_reference_array413.borrow(), self::ml_array::number_index(ml_reference_position413)); ml_reference_projection413 }
                                        };
                                        if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                            self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_field18336 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value18335) => {
                                                        source_ml_source1_data_property_value18335.clone()
                                                    }
                                                };
                                                (source_ml_source1_field18336.clone(), true, current.clone())
                                            })))
                                        } else {
                                            {
                                                let prototype = match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value18337, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18337 = *source_ml_source1_data_property_value18337;
                                                        source_ml_source1_data_property_value18337.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value18338, _, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18338 = *source_ml_source1_data_property_value18338;
                                                        source_ml_source1_data_property_value18338.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value18339, _, _, _) => {
                                                        let source_ml_source1_data_property_value18339 = *source_ml_source1_data_property_value18339;
                                                        source_ml_source1_data_property_value18339.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value18340, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18340 = *source_ml_source1_data_property_value18340;
                                                        source_ml_source1_data_property_value18340.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value18341, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18341 = *source_ml_source1_data_property_value18341;
                                                        source_ml_source1_data_property_value18341.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value18342, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18342 = *source_ml_source1_data_property_value18342;
                                                        source_ml_source1_data_property_value18342.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value18343, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18343 = *source_ml_source1_data_property_value18343;
                                                        source_ml_source1_data_property_value18343.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value18344, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18344 = *source_ml_source1_data_property_value18344;
                                                        source_ml_source1_data_property_value18344.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value18345, _, _, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18345 = *source_ml_source1_data_property_value18345;
                                                        source_ml_source1_data_property_value18345.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source1_data_property_value18346, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18346 = *source_ml_source1_data_property_value18346;
                                                        source_ml_source1_data_property_value18346.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source1_data_property_value18347, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18347 = *source_ml_source1_data_property_value18347;
                                                        source_ml_source1_data_property_value18347.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source1_data_property_value18348, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18348 = *source_ml_source1_data_property_value18348;
                                                        source_ml_source1_data_property_value18348.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source1_data_property_value18349, _, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value18349 = *source_ml_source1_data_property_value18349;
                                                        source_ml_source1_data_property_value18349.clone()
                                                    }
                                                };
                                                match prototype.clone() {
                                                    self::JavaScriptSemanticValue::Object(current_2, _) => {
                                                        {
                                                            let depth_2 = (depth + 1f64);
                                                            {
                                                                (current, depth) = (current_2.clone(), depth_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Done
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
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop103Done
                }
            }
        };
    }
}

pub fn source_ml_temporary_define_java_script_array_own_property_loop105(mut count: f64, mut source_ml_temporary_values104: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut source_ml_temporary_index104: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index104 < ({ let ml_reference_array414 = source_ml_temporary_values104.clone(); let ml_reference_projection414 = self::ml::Big::from_u128(ml_reference_array414.borrow().len() as u128); ml_reference_projection414 })) {
            {
                let entry = { let ml_reference_array415 = source_ml_temporary_values104.clone(); let ml_reference_position415 = source_ml_temporary_index104.clone(); let ml_reference_projection415 = self::ml_array::at(&*ml_reference_array415.borrow(), ml_reference_position415.to_index()); ml_reference_projection415 };
                {
                    let count_2 = (count + { let ml_reference_array416 = match entry.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value18360, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18360.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value18361, _, _, _) => {
                            source_ml_source1_data_property_value18361.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value18362) => {
                            source_ml_source1_data_property_value18362.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value18363, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18363.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value18364, _) => {
                            source_ml_source1_data_property_value18364.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value18365, _) => {
                            source_ml_source1_data_property_value18365.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value18366, _) => {
                            source_ml_source1_data_property_value18366.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value18367, _) => {
                            source_ml_source1_data_property_value18367.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value18368) => {
                            source_ml_source1_data_property_value18368.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value18369, _, _) => {
                            source_ml_source1_data_property_value18369.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value18370, _, _) => {
                            source_ml_source1_data_property_value18370.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value18371, _, _) => {
                            source_ml_source1_data_property_value18371.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value18372, _, _) => {
                            source_ml_source1_data_property_value18372.clone()
                        }
                    }; let ml_reference_projection416 = ml_reference_array416.borrow().len() as f64; ml_reference_projection416 });
                    {
                        let source_ml_temporary_index104_2 = source_ml_temporary_index104.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index104) = (count_2, source_ml_temporary_index104_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            count
        };
    }
}

pub fn source_ml_temporary_commit_java_script_ordinary_own_property_loop107(mut count: f64, mut source_ml_temporary_values106: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut source_ml_temporary_index106: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index106 < ({ let ml_reference_array417 = source_ml_temporary_values106.clone(); let ml_reference_projection417 = self::ml::Big::from_u128(ml_reference_array417.borrow().len() as u128); ml_reference_projection417 })) {
            {
                let entry = { let ml_reference_array418 = source_ml_temporary_values106.clone(); let ml_reference_position418 = source_ml_temporary_index106.clone(); let ml_reference_projection418 = self::ml_array::at(&*ml_reference_array418.borrow(), ml_reference_position418.to_index()); ml_reference_projection418 };
                {
                    let count_2 = (count + { let ml_reference_array419 = match entry.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value18375, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18375.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value18376, _, _, _) => {
                            source_ml_source1_data_property_value18376.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value18377) => {
                            source_ml_source1_data_property_value18377.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value18378, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18378.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value18379, _) => {
                            source_ml_source1_data_property_value18379.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value18380, _) => {
                            source_ml_source1_data_property_value18380.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value18381, _) => {
                            source_ml_source1_data_property_value18381.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value18382, _) => {
                            source_ml_source1_data_property_value18382.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value18383) => {
                            source_ml_source1_data_property_value18383.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value18384, _, _) => {
                            source_ml_source1_data_property_value18384.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value18385, _, _) => {
                            source_ml_source1_data_property_value18385.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value18386, _, _) => {
                            source_ml_source1_data_property_value18386.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value18387, _, _) => {
                            source_ml_source1_data_property_value18387.clone()
                        }
                    }; let ml_reference_projection419 = ml_reference_array419.borrow().len() as f64; ml_reference_projection419 });
                    {
                        let source_ml_temporary_index106_2 = source_ml_temporary_index106.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index106) = (count_2, source_ml_temporary_index106_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            count
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Result {
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Done,
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop108Return(std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>>),
}
