pub fn create_java_script_property_descriptor(kind: self::ml_string::Text, configurable: bool, enumerable: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field398 = true;
        {
            let source_ml_source1_field399 = configurable;
            {
                let source_ml_source1_field400 = true;
                {
                    let source_ml_source1_field401 = enumerable;
                    {
                        let source_ml_source1_field402 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source1_field403 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field404 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source1_field405 = false;
                                    {
                                        let source_ml_source1_field406 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source1_field407 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field408 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source1_field409 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field399, source_ml_source1_field398, source_ml_source1_field401, source_ml_source1_field400, source_ml_source1_field407.clone(), source_ml_source1_field406, source_ml_source1_field409.clone(), source_ml_source1_field408, source_ml_source1_field403.clone(), source_ml_source1_field402, source_ml_source1_field405, source_ml_source1_field404)
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

pub fn apply_java_script_property_descriptor_fields(base: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field410 = true;
        {
            let source_ml_source1_field411 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source1_field412 = true;
                {
                    let source_ml_source1_field413 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source1_field414 = (base.clone()).9;
                        {
                            let source_ml_source1_field415 = if ((base.clone()).9 && (proposed.clone()).9) {
                                (proposed.clone()).8
                            } else {
                                (base.clone()).8
                            };
                            {
                                let source_ml_source1_field416 = (base.clone()).11;
                                {
                                    let source_ml_source1_field417 = if ((base.clone()).11 && (proposed.clone()).11) {
                                        (proposed.clone()).10
                                    } else {
                                        (base.clone()).10
                                    };
                                    {
                                        let source_ml_source1_field418 = (base.clone()).5;
                                        {
                                            let source_ml_source1_field419 = if ((base.clone()).5 && (proposed.clone()).5) {
                                                (proposed.clone()).4
                                            } else {
                                                (base.clone()).4
                                            };
                                            {
                                                let source_ml_source1_field420 = (base.clone()).7;
                                                {
                                                    let source_ml_source1_field421 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                        (proposed.clone()).6
                                                    } else {
                                                        (base.clone()).6
                                                    };
                                                    (source_ml_source1_field411, source_ml_source1_field410, source_ml_source1_field413, source_ml_source1_field412, source_ml_source1_field419.clone(), source_ml_source1_field418, source_ml_source1_field421.clone(), source_ml_source1_field420, source_ml_source1_field415.clone(), source_ml_source1_field414, source_ml_source1_field417, source_ml_source1_field416)
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

pub fn read_java_script_property_descriptor_writes(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> Vec<self::ml_string::Text> {
    {
        let writes = Vec::<self::ml_string::Text>::new();
        {
            let writes_3 = if (descriptor.clone()).1 {
                {
                    let writes_2 = {
                        let source_ml_source1_concat_value422 = writes;
                        {
                            let source_ml_source1_concat_value423 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value422), source_ml_source1_concat_value423)
                        }
                    };
                    writes_2
                }
            } else {
                writes
            };
            {
                let writes_5 = if (descriptor.clone()).3 {
                    {
                        let writes_4 = {
                            let source_ml_source1_concat_value424 = writes_3;
                            {
                                let source_ml_source1_concat_value425 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value424), source_ml_source1_concat_value425)
                            }
                        };
                        writes_4
                    }
                } else {
                    writes_3
                };
                {
                    let writes_7 = if (descriptor.clone()).9 {
                        {
                            let writes_6 = {
                                let source_ml_source1_concat_value426 = writes_5;
                                {
                                    let source_ml_source1_concat_value427 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value426), source_ml_source1_concat_value427)
                                }
                            };
                            writes_6
                        }
                    } else {
                        writes_5
                    };
                    {
                        let writes_9 = if (descriptor.clone()).11 {
                            {
                                let writes_8 = {
                                    let source_ml_source1_concat_value428 = writes_7;
                                    {
                                        let source_ml_source1_concat_value429 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value428), source_ml_source1_concat_value429)
                                    }
                                };
                                writes_8
                            }
                        } else {
                            writes_7
                        };
                        {
                            let writes_11 = if (descriptor.clone()).5 {
                                {
                                    let writes_10 = {
                                        let source_ml_source1_concat_value430 = writes_9;
                                        {
                                            let source_ml_source1_concat_value431 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value430), source_ml_source1_concat_value431)
                                        }
                                    };
                                    writes_10
                                }
                            } else {
                                writes_9
                            };
                            {
                                let writes_13 = if (descriptor.clone()).7 {
                                    {
                                        let writes_12 = {
                                            let source_ml_source1_concat_value432 = writes_11;
                                            {
                                                let source_ml_source1_concat_value433 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value432), source_ml_source1_concat_value433)
                                            }
                                        };
                                        writes_12
                                    }
                                } else {
                                    writes_11
                                };
                                writes_13
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_java_script_property_transition(disposition: self::ml_string::Text, error: self::ml_string::Text, descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), writes: Vec<self::ml_string::Text>) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let source_ml_source1_field434 = disposition.clone();
        {
            let source_ml_source1_field435 = error.clone();
            {
                let source_ml_source1_field436 = descriptor.clone();
                {
                    let source_ml_source1_field437 = writes;
                    (source_ml_source1_field436.clone(), source_ml_source1_field434.clone(), source_ml_source1_field435.clone(), source_ml_source1_field437)
                }
            }
        }
    }
}

pub fn plan_java_script_property_definition(exists: bool, extensible: bool, current: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), error.clone(), current.clone(), Vec::<self::ml_string::Text>::new())
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]), current.clone(), Vec::<self::ml_string::Text>::new())
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new())
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
                        if (((((!((proposed.clone()).1) && !((proposed.clone()).3)) && !((proposed.clone()).9)) && !((proposed.clone()).11)) && !((proposed.clone()).5)) && !((proposed.clone()).7)) {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![107, 101, 101, 112]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source1_s438 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }, self::ml_string::Text::from_units(vec![]), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                    };
                                    match source_ml_source1_s438.clone() {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(kind.clone(), (current.clone()).0, (current.clone()).2)
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
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(source_ml_temporary_result7) => {
                                            source_ml_temporary_result7.clone()
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

pub fn find_java_script_heap_directory_object(objects: Vec<self::JavaScriptOrdinaryObject>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = (objects.len() as f64);
            {
                let source_ml_source1_s439 = self::source_ml_temporary_find_java_script_heap_directory_object_loop8(objects, identity.clone(), low, high);
                match source_ml_source1_s439.clone() {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(source_ml_temporary_result8) => {
                        source_ml_temporary_result8
                    }
                }
            }
        }
    }
}

pub fn test_java_script_heap_directory_symbol(symbols: Vec<self::ml::Big>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = (symbols.len() as f64);
            {
                let source_ml_source1_s440 = self::source_ml_temporary_test_java_script_heap_directory_symbol_loop9(symbols, identity.clone(), low, high);
                match source_ml_source1_s440.clone() {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(source_ml_temporary_result9) => {
                        source_ml_temporary_result9
                    }
                }
            }
        }
    }
}

pub fn read_java_script_heap_directory_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let error = self::read_java_script_semantic_value_error(value.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Null => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f441) => {
                    {
                        let source_ml_source1_f463 = source_ml_source1_f441;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f442) => {
                    {
                        let source_ml_source1_f471 = source_ml_source1_f442;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f443) => {
                    {
                        let source_ml_source1_f479 = source_ml_source1_f443.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f444) => {
                    {
                        let source_ml_source1_f487 = source_ml_source1_f444;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f445, source_ml_source1_f446) => {
                    {
                        let source_ml_source1_f495 = source_ml_source1_f445.clone();
                        {
                            let source_ml_source1_f496 = source_ml_source1_f446.clone();
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_heap_directory_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value498, _, _, _, _) => {
                            source_ml_source1_data_property_value498
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f447, source_ml_source1_f448) => {
                    {
                        let identity_2 = source_ml_source1_f447.clone();
                        {
                            let callable = source_ml_source1_f448;
                            {
                                let at = self::find_java_script_heap_directory_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value506, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value506
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value507, _, _, _, _, _) => source_ml_source1_data_property_value507.as_slice() }), self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value521, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value521
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value522, _, _, _, _) => {
                                                source_ml_source1_data_property_value522
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value523, _) => {
                                                source_ml_source1_data_property_value523
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value524, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value524
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value525, _, _) => {
                                                source_ml_source1_data_property_value525
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value526, _, _) => {
                                                source_ml_source1_data_property_value526
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value527, _, _) => {
                                                source_ml_source1_data_property_value527
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value528, _, _) => {
                                                source_ml_source1_data_property_value528
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value529, _) => {
                                                source_ml_source1_data_property_value529
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value530, _, _, _) => {
                                                source_ml_source1_data_property_value530
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value531, _, _, _) => {
                                                source_ml_source1_data_property_value531
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value532, _, _, _) => {
                                                source_ml_source1_data_property_value532
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value533, _, _, _) => {
                                                source_ml_source1_data_property_value533
                                            }
                                        }) == callable)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
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

pub fn read_java_script_heap_directory_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if ((match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f534) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f535) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f536) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f537) => {
            false
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f538, source_ml_source1_f539) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f540) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f541, source_ml_source1_f542) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f543) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f544) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f545) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f546) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f547, source_ml_source1_f548) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f549) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f550, source_ml_source1_f551) => {
            true
        }
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f552) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f553) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f554) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f555) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f556, source_ml_source1_f557) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f558) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f559, source_ml_source1_f560) => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_heap_directory_value_error(heap.clone(), key.clone())
    }
}

pub fn find_java_script_heap_directory_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_property_descriptor_entry(properties, key.clone())
}

pub fn test_java_script_ordinary_class_constructor_mode(mode: self::ml_string::Text) -> bool {
    (mode == (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115])))
}

pub fn test_java_script_ordinary_class_receiver_result(mode: self::ml_string::Text, result: self::ml_string::Text) -> bool {
    (self::test_java_script_ordinary_class_constructor_mode(mode.clone()) && (result != (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))))
}

pub fn read_java_script_ordinary_class_call_reason(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> self::ml_string::Text {
    {
        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value561, _, _, _, _, _) => source_ml_source1_data_property_value561.as_slice() }), self::ml_array::number_index(self::find_java_script_heap_directory_object(match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value562, _, _, _, _, _) => {
                source_ml_source1_data_property_value562
            }
        }, identity.clone())));
        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            self::ml_string::Text::from_units(vec![])
        } else {
            match object.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_f563, source_ml_source1_f564, source_ml_source1_f565, source_ml_source1_f566, source_ml_source1_f567, source_ml_source1_f568, source_ml_source1_f569, source_ml_source1_f570, source_ml_source1_f571, source_ml_source1_f572) => {
                    let source_ml_source1_f564 = *source_ml_source1_f564;
                    let source_ml_source1_f571 = *source_ml_source1_f571;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_f573, source_ml_source1_f574, source_ml_source1_f575, source_ml_source1_f576, source_ml_source1_f577, source_ml_source1_f578, source_ml_source1_f579, source_ml_source1_f580) => {
                    let source_ml_source1_f574 = *source_ml_source1_f574;
                    let source_ml_source1_f578 = *source_ml_source1_f578;
                    let source_ml_source1_f579 = *source_ml_source1_f579;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_f581, source_ml_source1_f582, source_ml_source1_f583, source_ml_source1_f584, source_ml_source1_f585) => {
                    let source_ml_source1_f582 = *source_ml_source1_f582;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_f586, source_ml_source1_f587, source_ml_source1_f588, source_ml_source1_f589, source_ml_source1_f590, source_ml_source1_f591, source_ml_source1_f592, source_ml_source1_f593, source_ml_source1_f594, source_ml_source1_f595, source_ml_source1_f596) => {
                    let source_ml_source1_f587 = *source_ml_source1_f587;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_f597, source_ml_source1_f598, source_ml_source1_f599, source_ml_source1_f600, source_ml_source1_f601, source_ml_source1_f602) => {
                    let source_ml_source1_f598 = *source_ml_source1_f598;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_f603, source_ml_source1_f604, source_ml_source1_f605, source_ml_source1_f606, source_ml_source1_f607, source_ml_source1_f608) => {
                    let source_ml_source1_f604 = *source_ml_source1_f604;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_f609, source_ml_source1_f610, source_ml_source1_f611, source_ml_source1_f612, source_ml_source1_f613, source_ml_source1_f614) => {
                    let source_ml_source1_f610 = *source_ml_source1_f610;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_f615, source_ml_source1_f616, source_ml_source1_f617, source_ml_source1_f618, source_ml_source1_f619, source_ml_source1_f620) => {
                    let source_ml_source1_f616 = *source_ml_source1_f616;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_f621, source_ml_source1_f622, source_ml_source1_f623, source_ml_source1_f624, source_ml_source1_f625, source_ml_source1_f626, source_ml_source1_f627, source_ml_source1_f628, properties) => {
                    let source_ml_source1_f622 = *source_ml_source1_f622;
                    {
                        let field = self::ml_array::optional_at(&properties, self::ml_array::number_index(self::find_java_script_heap_directory_property(properties, self::JavaScriptSemanticValue::Text(vec![110f64, 97f64, 109f64, 101f64]))));
                        if (((match (field.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !((match field.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value657) => {
                                source_ml_source1_data_property_value657.clone()
                            }
                        }).9)) || (match field.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value658) => {
                                source_ml_source1_data_property_value658.clone()
                            }
                        }).5) || (match field.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value659) => {
                                source_ml_source1_data_property_value659.clone()
                            }
                        }).7) {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            {
                                let value = (match field.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value660) => {
                                        source_ml_source1_data_property_value660.clone()
                                    }
                                }).8;
                                if (match value.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f661) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source1_f662) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f663) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source1_f664) => {
                                        false
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f665, source_ml_source1_f666) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f667) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source1_f668, source_ml_source1_f669) => {
                                        true
                                    }
                                } && match value.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f670) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source1_f671) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f672) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source1_f673) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f674, source_ml_source1_f675) => {
                                        false
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f676) => {
                                        true
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source1_f677, source_ml_source1_f678) => {
                                        true
                                    }
                                }) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    ((self::ml_string::Text::from_units(vec![67, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32])).concat(&self::read_java_script_semantic_text_string(value.clone()))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 105, 110, 118, 111, 107, 101, 100, 32, 119, 105, 116, 104, 111, 117, 116, 32, 39, 110, 101, 119, 39]))
                                }
                            }
                        }
                    }
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_f629, source_ml_source1_f630, source_ml_source1_f631, source_ml_source1_f632, source_ml_source1_f633, source_ml_source1_f634, source_ml_source1_f635) => {
                    let source_ml_source1_f630 = *source_ml_source1_f630;
                    let source_ml_source1_f634 = *source_ml_source1_f634;
                    let source_ml_source1_f635 = *source_ml_source1_f635;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_f636, source_ml_source1_f637, source_ml_source1_f638, source_ml_source1_f639, source_ml_source1_f640, source_ml_source1_f641, source_ml_source1_f642) => {
                    let source_ml_source1_f637 = *source_ml_source1_f637;
                    let source_ml_source1_f641 = *source_ml_source1_f641;
                    let source_ml_source1_f642 = *source_ml_source1_f642;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_f643, source_ml_source1_f644, source_ml_source1_f645, source_ml_source1_f646, source_ml_source1_f647, source_ml_source1_f648, source_ml_source1_f649) => {
                    let source_ml_source1_f644 = *source_ml_source1_f644;
                    let source_ml_source1_f648 = *source_ml_source1_f648;
                    let source_ml_source1_f649 = *source_ml_source1_f649;
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_f650, source_ml_source1_f651, source_ml_source1_f652, source_ml_source1_f653, source_ml_source1_f654, source_ml_source1_f655, source_ml_source1_f656) => {
                    let source_ml_source1_f651 = *source_ml_source1_f651;
                    let source_ml_source1_f655 = *source_ml_source1_f655;
                    let source_ml_source1_f656 = *source_ml_source1_f656;
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}
