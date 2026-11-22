pub fn plan_java_script_property_definition(exists: bool, extensible: bool, current: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), error.clone(), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let base = self::create_java_script_property_descriptor(kind.clone(), false, false);
                                {
                                    let descriptor = self::apply_java_script_property_descriptor_fields(base.clone(), proposed.clone());
                                    self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 101]), self::ml_string::Text::from_units(vec![]), descriptor.clone(), self::read_java_script_property_descriptor_writes(descriptor.clone()))
                                }
                            }
                        }
                    } else {
                        if (((((!(({ let ml_reference_value141 = proposed.borrow().clone(); ml_reference_value141 }).1) && !(({ let ml_reference_value142 = proposed.borrow().clone(); ml_reference_value142 }).3)) && !(({ let ml_reference_value143 = proposed.borrow().clone(); ml_reference_value143 }).9)) && !(({ let ml_reference_value144 = proposed.borrow().clone(); ml_reference_value144 }).11)) && !(({ let ml_reference_value145 = proposed.borrow().clone(); ml_reference_value145 }).5)) && !(({ let ml_reference_value146 = proposed.borrow().clone(); ml_reference_value146 }).7)) {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![107, 101, 101, 112]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source2_s661 = if !(({ let ml_reference_value147 = current.borrow().clone(); ml_reference_value147 }).0) {
                                        if (({ let ml_reference_value148 = proposed.borrow().clone(); ml_reference_value148 }).1 && ({ let ml_reference_value149 = proposed.borrow().clone(); ml_reference_value149 }).0) {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            if (({ let ml_reference_value150 = proposed.borrow().clone(); ml_reference_value150 }).3 && ((({ let ml_reference_value151 = proposed.borrow().clone(); ml_reference_value151 }).2) != (({ let ml_reference_value152 = current.borrow().clone(); ml_reference_value152 }).2))) {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if (({ let ml_reference_value153 = proposed.borrow().clone(); ml_reference_value153 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value154 = proposed.borrow().clone(); ml_reference_value154 }).4, ({ let ml_reference_value155 = current.borrow().clone(); ml_reference_value155 }).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            if (({ let ml_reference_value156 = proposed.borrow().clone(); ml_reference_value156 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value157 = proposed.borrow().clone(); ml_reference_value157 }).6, ({ let ml_reference_value158 = current.borrow().clone(); ml_reference_value158 }).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Next
                                                            }
                                                        }
                                                    } else {
                                                        if !(({ let ml_reference_value159 = current.borrow().clone(); ml_reference_value159 }).10) {
                                                            if (({ let ml_reference_value160 = proposed.borrow().clone(); ml_reference_value160 }).11 && ({ let ml_reference_value161 = proposed.borrow().clone(); ml_reference_value161 }).10) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                if ({ let ml_reference_value162 = proposed.borrow().clone(); ml_reference_value162 }).9 {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(self::create_java_script_property_transition(if self::test_java_script_semantic_same_value(({ let ml_reference_value163 = proposed.borrow().clone(); ml_reference_value163 }).8, ({ let ml_reference_value164 = current.borrow().clone(); ml_reference_value164 }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }, self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Next
                                    };
                                    match source_ml_source2_s661.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(kind.clone(), ({ let ml_reference_value165 = current.borrow().clone(); ml_reference_value165 }).0, ({ let ml_reference_value166 = current.borrow().clone(); ml_reference_value166 }).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2.clone(), proposed.clone());
                                                    {
                                                        let conversion = ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original));
                                                        self::create_java_script_property_transition(if conversion {
                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![117, 112, 100, 97, 116, 101])
                                                        }, self::ml_string::Text::from_units(vec![]), descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
                                                            descriptor_2.clone()
                                                        } else {
                                                            proposed.clone()
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(source_ml_source1_temporary_result60) => {
                                            source_ml_source1_temporary_result60.clone()
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

pub fn read_java_script_literal_property_index(key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> f64 {
    if ((({ let ml_reference_array167 = key.clone(); let ml_reference_projection167 = ml_reference_array167.borrow().len() as f64; ml_reference_projection167 }) == (0f64)) || (({ let ml_reference_array168 = key.clone(); let ml_reference_projection168 = ml_reference_array168.borrow().len() as f64; ml_reference_projection168 }) > (10f64))) {
        (-1f64)
    } else {
        if ((({ let ml_reference_array169 = key.clone(); let ml_reference_projection169 = ml_reference_array169.borrow().len() as f64; ml_reference_projection169 }) > (1f64)) && {
            let source_ml_source2_comparison_operand664 = {
                let source_ml_source2_array_read_index663 = 0f64;
                { let ml_reference_array170 = key.clone(); let ml_reference_position170 = source_ml_source2_array_read_index663; let ml_reference_projection170 = self::ml_array::optional_at(&*ml_reference_array170.borrow(), self::ml_array::number_index(ml_reference_position170)); ml_reference_projection170 }
            };
            {
                let source_ml_source2_comparison_operand665 = 48f64;
                match (source_ml_source2_comparison_operand664.clone(), source_ml_source2_comparison_operand665) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s666 = self::source_ml_source1_temporary_read_java_script_literal_property_index_loop61(key.clone(), value, at);
                        match source_ml_source2_s666.clone() {
                            self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Return(source_ml_source1_temporary_result61) => {
                                source_ml_source1_temporary_result61
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_literal_property_form(kind: self::ml_string::Text, index: f64) -> self::ml_string::Text {
    if ((((kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && (index >= (0f64))) {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

pub fn read_java_script_string_index_number_form(kind: self::ml_string::Text, element: self::ml_string::Text) -> self::ml_string::Text {
    if (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) || ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (element == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) {
                self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
            } else {
                if ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (((element == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (element == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (element == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
                    self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
                } else {
                    self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
                }
            }
        }
    }
}

pub fn find_java_script_heap_directory_object(objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array171 = objects.clone(); let ml_reference_projection171 = ml_reference_array171.borrow().len() as f64; ml_reference_projection171 };
            {
                let source_ml_source2_s667 = self::source_ml_source1_temporary_find_java_script_heap_directory_object_loop62(objects.clone(), identity.clone(), low, high);
                match source_ml_source2_s667.clone() {
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Return(source_ml_source1_temporary_result62) => {
                        source_ml_source1_temporary_result62
                    }
                }
            }
        }
    }
}

pub fn test_java_script_heap_directory_symbol(symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array172 = symbols.clone(); let ml_reference_projection172 = ml_reference_array172.borrow().len() as f64; ml_reference_projection172 };
            {
                let source_ml_source2_s668 = self::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop63(symbols.clone(), identity.clone(), low, high);
                match source_ml_source2_s668.clone() {
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Return(source_ml_source1_temporary_result63) => {
                        source_ml_source1_temporary_result63
                    }
                }
            }
        }
    }
}

pub fn read_java_script_heap_directory_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, _, _, _, _) => {
            {
                let error = self::read_java_script_semantic_value_error(value.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    match value.clone() {
                        self::JavaScriptSemanticValue::Symbol(identity) => {
                            if self::test_java_script_heap_directory_symbol(shared_constructor_field2.clone(), identity.clone()) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f675, source_ml_source2_f676) => {
                            {
                                let at = self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), source_ml_source2_f675.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = {
                                            let source_ml_source2_array_read_value736 = shared_constructor_field1.clone();
                                            { let ml_reference_array173 = source_ml_source2_array_read_value736.clone(); let ml_reference_position173 = at; let ml_reference_projection173 = self::ml_array::optional_at(&*ml_reference_array173.borrow(), self::ml_array::number_index(ml_reference_position173)); ml_reference_projection173 }
                                        };
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value751, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value751
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value752, _, _, _, _) => {
                                                source_ml_source2_data_property_value752
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value753, _) => {
                                                source_ml_source2_data_property_value753
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value754, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value754
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value755, _, _) => {
                                                source_ml_source2_data_property_value755
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value756, _, _) => {
                                                source_ml_source2_data_property_value756
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value757, _, _) => {
                                                source_ml_source2_data_property_value757
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value758, _, _) => {
                                                source_ml_source2_data_property_value758
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value759, _) => {
                                                source_ml_source2_data_property_value759
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value760, _, _, _) => {
                                                source_ml_source2_data_property_value760
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value761, _, _, _) => {
                                                source_ml_source2_data_property_value761
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value762, _, _, _) => {
                                                source_ml_source2_data_property_value762
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value763, _, _, _) => {
                                                source_ml_source2_data_property_value763
                                            }
                                        }) == source_ml_source2_f676)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_heap_directory_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if ((match key.clone() {
        self::JavaScriptSemanticValue::Text(_) => {
            false
        }
        _ => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => {
            false
        }
        _ => {
            true
        }
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => {
            false
        }
        _ => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_heap_directory_value_error(heap.clone(), key.clone())
    }
}

pub fn find_java_script_heap_directory_property(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_property_descriptor_entry(properties.clone(), key.clone())
}

pub fn read_java_script_array_length_key() -> self::JavaScriptSemanticValue {
    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64])))
}

pub fn test_java_script_array_length_key(key: self::JavaScriptSemanticValue) -> bool {
    self::test_java_script_semantic_same_value(key.clone(), self::read_java_script_array_length_key())
}

pub fn read_java_script_array_index(key: self::JavaScriptSemanticValue) -> f64 {
    {
        let length = self::read_java_script_semantic_text_length(key.clone());
        if ((length < (0f64)) || (length > (10f64))) {
            (-1f64)
        } else {
            self::read_java_script_literal_property_index(self::construct_java_script_semantic_text_units(key.clone(), 10f64))
        }
    }
}

pub fn find_java_script_array_property(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_heap_directory_property(properties.clone(), key.clone())
}

pub fn read_java_script_array_length(object: self::JavaScriptOrdinaryObject) -> f64 {
    {
        let property = {
            let source_ml_source2_array_read_value817 = match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value791, _, _, _, _, _) => {
                    source_ml_source2_data_property_value791.clone()
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value792, _, _, _) => {
                    source_ml_source2_data_property_value792.clone()
                }
                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value793) => {
                    source_ml_source2_data_property_value793.clone()
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value794, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value794.clone()
                }
                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value795, _) => {
                    source_ml_source2_data_property_value795.clone()
                }
                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value796, _) => {
                    source_ml_source2_data_property_value796.clone()
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value797, _) => {
                    source_ml_source2_data_property_value797.clone()
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value798, _) => {
                    source_ml_source2_data_property_value798.clone()
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value799) => {
                    source_ml_source2_data_property_value799.clone()
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value800, _, _) => {
                    source_ml_source2_data_property_value800.clone()
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value801, _, _) => {
                    source_ml_source2_data_property_value801.clone()
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value802, _, _) => {
                    source_ml_source2_data_property_value802.clone()
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value803, _, _) => {
                    source_ml_source2_data_property_value803.clone()
                }
            };
            {
                let source_ml_source2_array_read_index818 = self::find_java_script_array_property(match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value804, _, _, _, _, _) => {
                        source_ml_source2_data_property_value804.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value805, _, _, _) => {
                        source_ml_source2_data_property_value805.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value806) => {
                        source_ml_source2_data_property_value806.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value807, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value807.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value808, _) => {
                        source_ml_source2_data_property_value808.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value809, _) => {
                        source_ml_source2_data_property_value809.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value810, _) => {
                        source_ml_source2_data_property_value810.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value811, _) => {
                        source_ml_source2_data_property_value811.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value812) => {
                        source_ml_source2_data_property_value812.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value813, _, _) => {
                        source_ml_source2_data_property_value813.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value814, _, _) => {
                        source_ml_source2_data_property_value814.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value815, _, _) => {
                        source_ml_source2_data_property_value815.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value816, _, _) => {
                        source_ml_source2_data_property_value816.clone()
                    }
                }, self::read_java_script_array_length_key());
                { let ml_reference_array174 = source_ml_source2_array_read_value817.clone(); let ml_reference_position174 = source_ml_source2_array_read_index818; let ml_reference_projection174 = self::ml_array::optional_at(&*ml_reference_array174.borrow(), self::ml_array::number_index(ml_reference_position174)); ml_reference_projection174 }
            }
        };
        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            (-1f64)
        } else {
            self::read_java_script_array_length_value(({ let ml_reference_value175 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value819) => {
                    source_ml_source2_data_property_value819.clone()
                }
            }).borrow().clone(); ml_reference_value175 }).8)
        }
    }
}

pub fn read_java_script_array_length_value(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Number(value_value) => {
            value_value
        }
        _ => {
            (-1f64)
        }
    }
}

pub fn test_java_script_array_length(length: f64) -> bool {
    (((self::ml_math::is_integer(length) && (length >= (0f64))) && (length <= (4294967295f64))) && !({
        let source_ml_source2_same_value_operand829 = (-0f64);
        (((length == source_ml_source2_same_value_operand829) && ((length != (0f64)) || (((1f64 / length)) == ((1f64 / source_ml_source2_same_value_operand829))))) || ((length != length) && (source_ml_source2_same_value_operand829 != source_ml_source2_same_value_operand829)))
    }))
}

pub fn read_java_script_array_object_error(object: self::JavaScriptOrdinaryObject) -> self::ml_string::Text {
    if match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value830, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value830
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value831, _, _, _, _) => {
            source_ml_source2_data_property_value831
        }
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value832, _) => {
            source_ml_source2_data_property_value832
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value833, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value833
        }
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value834, _, _) => {
            source_ml_source2_data_property_value834
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value835, _, _) => {
            source_ml_source2_data_property_value835
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value836, _, _) => {
            source_ml_source2_data_property_value836
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value837, _, _) => {
            source_ml_source2_data_property_value837
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value838, _) => {
            source_ml_source2_data_property_value838
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value839, _, _, _) => {
            source_ml_source2_data_property_value839
        }
        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value840, _, _, _) => {
            source_ml_source2_data_property_value840
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value841, _, _, _) => {
            source_ml_source2_data_property_value841
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value842, _, _, _) => {
            source_ml_source2_data_property_value842
        }
    } {
        self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 101, 120, 111, 116, 105, 99, 32, 111, 98, 106, 101, 99, 116, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])
    } else {
        {
            let property = {
                let source_ml_source2_array_read_value869 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value843, _, _, _, _, _) => {
                        source_ml_source2_data_property_value843.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value844, _, _, _) => {
                        source_ml_source2_data_property_value844.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value845) => {
                        source_ml_source2_data_property_value845.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value846, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value846.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value847, _) => {
                        source_ml_source2_data_property_value847.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value848, _) => {
                        source_ml_source2_data_property_value848.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value849, _) => {
                        source_ml_source2_data_property_value849.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value850, _) => {
                        source_ml_source2_data_property_value850.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value851) => {
                        source_ml_source2_data_property_value851.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value852, _, _) => {
                        source_ml_source2_data_property_value852.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value853, _, _) => {
                        source_ml_source2_data_property_value853.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value854, _, _) => {
                        source_ml_source2_data_property_value854.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value855, _, _) => {
                        source_ml_source2_data_property_value855.clone()
                    }
                };
                {
                    let source_ml_source2_array_read_index870 = self::find_java_script_array_property(match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value856, _, _, _, _, _) => {
                            source_ml_source2_data_property_value856.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value857, _, _, _) => {
                            source_ml_source2_data_property_value857.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value858) => {
                            source_ml_source2_data_property_value858.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value859, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value859.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value860, _) => {
                            source_ml_source2_data_property_value860.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value861, _) => {
                            source_ml_source2_data_property_value861.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value862, _) => {
                            source_ml_source2_data_property_value862.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value863, _) => {
                            source_ml_source2_data_property_value863.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value864) => {
                            source_ml_source2_data_property_value864.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value865, _, _) => {
                            source_ml_source2_data_property_value865.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value866, _, _) => {
                            source_ml_source2_data_property_value866.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value867, _, _) => {
                            source_ml_source2_data_property_value867.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value868, _, _) => {
                            source_ml_source2_data_property_value868.clone()
                        }
                    }, self::read_java_script_array_length_key());
                    { let ml_reference_array176 = source_ml_source2_array_read_value869.clone(); let ml_reference_position176 = source_ml_source2_array_read_index870; let ml_reference_projection176 = self::ml_array::optional_at(&*ml_reference_array176.borrow(), self::ml_array::number_index(ml_reference_position176)); ml_reference_projection176 }
                }
            };
            if (((((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value177 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value871) => {
                    source_ml_source2_data_property_value871.clone()
                }
            }).borrow().clone(); ml_reference_value177 }).9)) || ({ let ml_reference_value178 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value872) => {
                    source_ml_source2_data_property_value872.clone()
                }
            }).borrow().clone(); ml_reference_value178 }).5) || ({ let ml_reference_value179 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value873) => {
                    source_ml_source2_data_property_value873.clone()
                }
            }).borrow().clone(); ml_reference_value179 }).7) || ({ let ml_reference_value180 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value874) => {
                    source_ml_source2_data_property_value874.clone()
                }
            }).borrow().clone(); ml_reference_value180 }).2) || ({ let ml_reference_value181 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value875) => {
                    source_ml_source2_data_property_value875.clone()
                }
            }).borrow().clone(); ml_reference_value181 }).0) {
                self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 110, 117, 109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110, 99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 105, 110, 116, 101, 103, 114, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121])
            } else {
                {
                    let length = self::read_java_script_array_length_value(({ let ml_reference_value182 = (match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value876) => {
                            source_ml_source2_data_property_value876.clone()
                        }
                    }).borrow().clone(); ml_reference_value182 }).8);
                    if !(self::test_java_script_array_length(length)) {
                        self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 110, 117, 109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110, 99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 105, 110, 116, 101, 103, 114, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121])
                    } else {
                        {
                            let source_ml_source1_temporary_values64 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value877, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value877.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value878, _, _, _) => {
                                    source_ml_source2_data_property_value878.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value879) => {
                                    source_ml_source2_data_property_value879.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value880, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value880.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value881, _) => {
                                    source_ml_source2_data_property_value881.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value882, _) => {
                                    source_ml_source2_data_property_value882.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value883, _) => {
                                    source_ml_source2_data_property_value883.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value884, _) => {
                                    source_ml_source2_data_property_value884.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value885) => {
                                    source_ml_source2_data_property_value885.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value886, _, _) => {
                                    source_ml_source2_data_property_value886.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value887, _, _) => {
                                    source_ml_source2_data_property_value887.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value888, _, _) => {
                                    source_ml_source2_data_property_value888.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value889, _, _) => {
                                    source_ml_source2_data_property_value889.clone()
                                }
                            };
                            {
                                let source_ml_source1_temporary_index64 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s890 = self::source_ml_source1_temporary_read_java_script_array_object_error_loop65(length, source_ml_source1_temporary_values64.clone(), source_ml_source1_temporary_index64.clone());
                                    match source_ml_source2_s890.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Done => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Return(source_ml_source1_temporary_result65) => {
                                            source_ml_source1_temporary_result65.clone()
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

pub fn construct_java_script_array_object(identity: self::ml::Big, prototype: self::JavaScriptSemanticValue, length: f64) -> self::JavaScriptOrdinaryObject {
    {
        let descriptor = self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), true, false);
        {
            let source_ml_source1_temporary_construct_java_script_array_object_written66 = { let ml_field_owner183 = descriptor.clone(); let ml_field_value183 = false; ml_field_owner183.borrow_mut().0 = ml_field_value183; () };
            {
                let source_ml_source1_temporary_construct_java_script_array_object_written67 = { let ml_field_owner184 = descriptor.clone(); let ml_field_value184 = true; ml_field_owner184.borrow_mut().10 = ml_field_value184; () };
                {
                    let source_ml_source1_temporary_construct_java_script_array_object_written68 = { let ml_field_owner185 = descriptor.clone(); let ml_field_value185 = self::JavaScriptSemanticValue::Number(length); ml_field_owner185.borrow_mut().8 = ml_field_value185; () };
                    self::JavaScriptOrdinaryObject::ArrayObject(identity.clone(), Box::new(prototype.clone()), true, false, std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(self::read_java_script_array_length_key()), descriptor.clone())])))
                }
            }
        }
    }
}
