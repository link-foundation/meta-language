pub fn create_java_script_property_descriptor(kind: self::ml_string::Text, configurable: bool, enumerable: bool) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field400 = true;
        {
            let source_ml_source2_field401 = configurable;
            {
                let source_ml_source2_field402 = true;
                {
                    let source_ml_source2_field403 = enumerable;
                    {
                        let source_ml_source2_field404 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source2_field405 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field406 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source2_field407 = false;
                                    {
                                        let source_ml_source2_field408 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source2_field409 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field410 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source2_field411 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field401, source_ml_source2_field400, source_ml_source2_field403, source_ml_source2_field402, source_ml_source2_field409.clone(), source_ml_source2_field408, source_ml_source2_field411.clone(), source_ml_source2_field410, source_ml_source2_field405.clone(), source_ml_source2_field404, source_ml_source2_field407, source_ml_source2_field406)
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

pub fn apply_java_script_property_descriptor_fields(base: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field412 = true;
        {
            let source_ml_source2_field413 = if ({ let ml_reference_value52 = proposed.borrow().clone(); ml_reference_value52 }).1 {
                ({ let ml_reference_value53 = proposed.borrow().clone(); ml_reference_value53 }).0
            } else {
                ({ let ml_reference_value54 = base.borrow().clone(); ml_reference_value54 }).0
            };
            {
                let source_ml_source2_field414 = true;
                {
                    let source_ml_source2_field415 = if ({ let ml_reference_value55 = proposed.borrow().clone(); ml_reference_value55 }).3 {
                        ({ let ml_reference_value56 = proposed.borrow().clone(); ml_reference_value56 }).2
                    } else {
                        ({ let ml_reference_value57 = base.borrow().clone(); ml_reference_value57 }).2
                    };
                    {
                        let source_ml_source2_field416 = ({ let ml_reference_value58 = base.borrow().clone(); ml_reference_value58 }).9;
                        {
                            let source_ml_source2_field417 = if (({ let ml_reference_value59 = base.borrow().clone(); ml_reference_value59 }).9 && ({ let ml_reference_value60 = proposed.borrow().clone(); ml_reference_value60 }).9) {
                                ({ let ml_reference_value61 = proposed.borrow().clone(); ml_reference_value61 }).8
                            } else {
                                ({ let ml_reference_value62 = base.borrow().clone(); ml_reference_value62 }).8
                            };
                            {
                                let source_ml_source2_field418 = ({ let ml_reference_value63 = base.borrow().clone(); ml_reference_value63 }).11;
                                {
                                    let source_ml_source2_field419 = if (({ let ml_reference_value64 = base.borrow().clone(); ml_reference_value64 }).11 && ({ let ml_reference_value65 = proposed.borrow().clone(); ml_reference_value65 }).11) {
                                        ({ let ml_reference_value66 = proposed.borrow().clone(); ml_reference_value66 }).10
                                    } else {
                                        ({ let ml_reference_value67 = base.borrow().clone(); ml_reference_value67 }).10
                                    };
                                    {
                                        let source_ml_source2_field420 = ({ let ml_reference_value68 = base.borrow().clone(); ml_reference_value68 }).5;
                                        {
                                            let source_ml_source2_field421 = if (({ let ml_reference_value69 = base.borrow().clone(); ml_reference_value69 }).5 && ({ let ml_reference_value70 = proposed.borrow().clone(); ml_reference_value70 }).5) {
                                                ({ let ml_reference_value71 = proposed.borrow().clone(); ml_reference_value71 }).4
                                            } else {
                                                ({ let ml_reference_value72 = base.borrow().clone(); ml_reference_value72 }).4
                                            };
                                            {
                                                let source_ml_source2_field422 = ({ let ml_reference_value73 = base.borrow().clone(); ml_reference_value73 }).7;
                                                {
                                                    let source_ml_source2_field423 = if (({ let ml_reference_value74 = base.borrow().clone(); ml_reference_value74 }).7 && ({ let ml_reference_value75 = proposed.borrow().clone(); ml_reference_value75 }).7) {
                                                        ({ let ml_reference_value76 = proposed.borrow().clone(); ml_reference_value76 }).6
                                                    } else {
                                                        ({ let ml_reference_value77 = base.borrow().clone(); ml_reference_value77 }).6
                                                    };
                                                    (source_ml_source2_field413, source_ml_source2_field412, source_ml_source2_field415, source_ml_source2_field414, source_ml_source2_field421.clone(), source_ml_source2_field420, source_ml_source2_field423.clone(), source_ml_source2_field422, source_ml_source2_field417.clone(), source_ml_source2_field416, source_ml_source2_field419, source_ml_source2_field418)
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

pub fn read_java_script_property_descriptor_writes(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        {
            let writes_3 = if ({ let ml_reference_value78 = descriptor.borrow().clone(); ml_reference_value78 }).1 {
                {
                    let writes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value424 = writes.clone();
                        {
                            let source_ml_source2_concat_value425 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])]));
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value79 = source_ml_source2_concat_value424.borrow().clone(); ml_reference_value79 }), { let ml_reference_value80 = source_ml_source2_concat_value425.borrow().clone(); ml_reference_value80 })
                        }
                    }));
                    writes_2.clone()
                }
            } else {
                writes.clone()
            };
            {
                let writes_5 = if ({ let ml_reference_value81 = descriptor.borrow().clone(); ml_reference_value81 }).3 {
                    {
                        let writes_4 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value426 = writes_3.clone();
                            {
                                let source_ml_source2_concat_value427 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value82 = source_ml_source2_concat_value426.borrow().clone(); ml_reference_value82 }), { let ml_reference_value83 = source_ml_source2_concat_value427.borrow().clone(); ml_reference_value83 })
                            }
                        }));
                        writes_4.clone()
                    }
                } else {
                    writes_3.clone()
                };
                {
                    let writes_7 = if ({ let ml_reference_value84 = descriptor.borrow().clone(); ml_reference_value84 }).9 {
                        {
                            let writes_6 = std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_concat_value428 = writes_5.clone();
                                {
                                    let source_ml_source2_concat_value429 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])]));
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value85 = source_ml_source2_concat_value428.borrow().clone(); ml_reference_value85 }), { let ml_reference_value86 = source_ml_source2_concat_value429.borrow().clone(); ml_reference_value86 })
                                }
                            }));
                            writes_6.clone()
                        }
                    } else {
                        writes_5.clone()
                    };
                    {
                        let writes_9 = if ({ let ml_reference_value87 = descriptor.borrow().clone(); ml_reference_value87 }).11 {
                            {
                                let writes_8 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value430 = writes_7.clone();
                                    {
                                        let source_ml_source2_concat_value431 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value88 = source_ml_source2_concat_value430.borrow().clone(); ml_reference_value88 }), { let ml_reference_value89 = source_ml_source2_concat_value431.borrow().clone(); ml_reference_value89 })
                                    }
                                }));
                                writes_8.clone()
                            }
                        } else {
                            writes_7.clone()
                        };
                        {
                            let writes_11 = if ({ let ml_reference_value90 = descriptor.borrow().clone(); ml_reference_value90 }).5 {
                                {
                                    let writes_10 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value432 = writes_9.clone();
                                        {
                                            let source_ml_source2_concat_value433 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116])]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value91 = source_ml_source2_concat_value432.borrow().clone(); ml_reference_value91 }), { let ml_reference_value92 = source_ml_source2_concat_value433.borrow().clone(); ml_reference_value92 })
                                        }
                                    }));
                                    writes_10.clone()
                                }
                            } else {
                                writes_9.clone()
                            };
                            {
                                let writes_13 = if ({ let ml_reference_value93 = descriptor.borrow().clone(); ml_reference_value93 }).7 {
                                    {
                                        let writes_12 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value434 = writes_11.clone();
                                            {
                                                let source_ml_source2_concat_value435 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 116])]));
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value94 = source_ml_source2_concat_value434.borrow().clone(); ml_reference_value94 }), { let ml_reference_value95 = source_ml_source2_concat_value435.borrow().clone(); ml_reference_value95 })
                                            }
                                        }));
                                        writes_12.clone()
                                    }
                                } else {
                                    writes_11.clone()
                                };
                                writes_13.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_java_script_property_transition(disposition: self::ml_string::Text, error: self::ml_string::Text, descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, writes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field436 = disposition.clone();
        {
            let source_ml_source2_field437 = error.clone();
            {
                let source_ml_source2_field438 = descriptor.clone();
                {
                    let source_ml_source2_field439 = writes.clone();
                    (source_ml_source2_field438.clone(), source_ml_source2_field436.clone(), source_ml_source2_field437.clone(), source_ml_source2_field439.clone())
                }
            }
        }
    }))
}

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
                        if (((((!(({ let ml_reference_value96 = proposed.borrow().clone(); ml_reference_value96 }).1) && !(({ let ml_reference_value97 = proposed.borrow().clone(); ml_reference_value97 }).3)) && !(({ let ml_reference_value98 = proposed.borrow().clone(); ml_reference_value98 }).9)) && !(({ let ml_reference_value99 = proposed.borrow().clone(); ml_reference_value99 }).11)) && !(({ let ml_reference_value100 = proposed.borrow().clone(); ml_reference_value100 }).5)) && !(({ let ml_reference_value101 = proposed.borrow().clone(); ml_reference_value101 }).7)) {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![107, 101, 101, 112]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source2_s440 = if !(({ let ml_reference_value102 = current.borrow().clone(); ml_reference_value102 }).0) {
                                        if (({ let ml_reference_value103 = proposed.borrow().clone(); ml_reference_value103 }).1 && ({ let ml_reference_value104 = proposed.borrow().clone(); ml_reference_value104 }).0) {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            if (({ let ml_reference_value105 = proposed.borrow().clone(); ml_reference_value105 }).3 && ((({ let ml_reference_value106 = proposed.borrow().clone(); ml_reference_value106 }).2) != (({ let ml_reference_value107 = current.borrow().clone(); ml_reference_value107 }).2))) {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if (({ let ml_reference_value108 = proposed.borrow().clone(); ml_reference_value108 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value109 = proposed.borrow().clone(); ml_reference_value109 }).4, ({ let ml_reference_value110 = current.borrow().clone(); ml_reference_value110 }).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            if (({ let ml_reference_value111 = proposed.borrow().clone(); ml_reference_value111 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value112 = proposed.borrow().clone(); ml_reference_value112 }).6, ({ let ml_reference_value113 = current.borrow().clone(); ml_reference_value113 }).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                            }
                                                        }
                                                    } else {
                                                        if !(({ let ml_reference_value114 = current.borrow().clone(); ml_reference_value114 }).10) {
                                                            if (({ let ml_reference_value115 = proposed.borrow().clone(); ml_reference_value115 }).11 && ({ let ml_reference_value116 = proposed.borrow().clone(); ml_reference_value116 }).10) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                if ({ let ml_reference_value117 = proposed.borrow().clone(); ml_reference_value117 }).9 {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(if self::test_java_script_semantic_same_value(({ let ml_reference_value118 = proposed.borrow().clone(); ml_reference_value118 }).8, ({ let ml_reference_value119 = current.borrow().clone(); ml_reference_value119 }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }, self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                    };
                                    match source_ml_source2_s440.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(kind.clone(), ({ let ml_reference_value120 = current.borrow().clone(); ml_reference_value120 }).0, ({ let ml_reference_value121 = current.borrow().clone(); ml_reference_value121 }).2)
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
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(source_ml_source1_temporary_result7) => {
                                            source_ml_source1_temporary_result7.clone()
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

pub fn find_java_script_heap_directory_object(objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array122 = objects.clone(); let ml_reference_projection122 = ml_reference_array122.borrow().len() as f64; ml_reference_projection122 };
            {
                let source_ml_source2_s441 = self::source_ml_source1_temporary_find_java_script_heap_directory_object_loop8(objects.clone(), identity.clone(), low, high);
                match source_ml_source2_s441.clone() {
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(source_ml_source1_temporary_result8) => {
                        source_ml_source1_temporary_result8
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
            let high = { let ml_reference_array123 = symbols.clone(); let ml_reference_projection123 = ml_reference_array123.borrow().len() as f64; ml_reference_projection123 };
            {
                let source_ml_source2_s442 = self::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9(symbols.clone(), identity.clone(), low, high);
                match source_ml_source2_s442.clone() {
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(source_ml_source1_temporary_result9) => {
                        source_ml_source1_temporary_result9
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f443) => {
                    {
                        let source_ml_source2_f465 = source_ml_source2_f443;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f444) => {
                    {
                        let source_ml_source2_f473 = source_ml_source2_f444;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f445) => {
                    {
                        let source_ml_source2_f481 = source_ml_source2_f445.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f446) => {
                    {
                        let source_ml_source2_f489 = source_ml_source2_f446.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f447, source_ml_source2_f448) => {
                    {
                        let source_ml_source2_f497 = source_ml_source2_f447.clone();
                        {
                            let source_ml_source2_f498 = source_ml_source2_f448.clone();
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_heap_directory_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value500, _, _, _, _) => {
                            source_ml_source2_data_property_value500.clone()
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f449, source_ml_source2_f450) => {
                    {
                        let identity_2 = source_ml_source2_f449.clone();
                        {
                            let callable = source_ml_source2_f450;
                            {
                                let at = self::find_java_script_heap_directory_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value508, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value508.clone()
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = {
                                            let source_ml_source2_array_read_value510 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value509, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value509.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_array_read_index511 = at;
                                                { let ml_reference_array124 = source_ml_source2_array_read_value510.clone(); let ml_reference_position124 = source_ml_source2_array_read_index511; let ml_reference_projection124 = self::ml_array::optional_at(&*ml_reference_array124.borrow(), self::ml_array::number_index(ml_reference_position124)); ml_reference_projection124 }
                                            }
                                        };
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value525, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value525
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value526, _, _, _, _) => {
                                                source_ml_source2_data_property_value526
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value527, _) => {
                                                source_ml_source2_data_property_value527
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value528, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value528
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value529, _, _) => {
                                                source_ml_source2_data_property_value529
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value530, _, _) => {
                                                source_ml_source2_data_property_value530
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value531, _, _) => {
                                                source_ml_source2_data_property_value531
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value532, _, _) => {
                                                source_ml_source2_data_property_value532
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value533, _) => {
                                                source_ml_source2_data_property_value533
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value534, _, _, _) => {
                                                source_ml_source2_data_property_value534
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value535, _, _, _) => {
                                                source_ml_source2_data_property_value535
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value536, _, _, _) => {
                                                source_ml_source2_data_property_value536
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value537, _, _, _) => {
                                                source_ml_source2_data_property_value537
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f538) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f539) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f540) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f541) => {
            false
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f542, source_ml_source2_f543) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f544) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f545, source_ml_source2_f546) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f547) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f548) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f549) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f550) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f551, source_ml_source2_f552) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f553) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f554, source_ml_source2_f555) => {
            true
        }
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f556) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f557) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f558) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f559) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f560, source_ml_source2_f561) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f562) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f563, source_ml_source2_f564) => {
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

pub fn read_java_script_source_exception_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if (name == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114])
    } else {
        if (name == (self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if (name == (self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if (name == (self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if (name == (self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if (name == (self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if (name == (self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114])
                            } else {
                                self::ml_string::Text::from_units(vec![])
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_source_exception_class(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114])
                            } else {
                                self::ml_string::Text::from_units(vec![])
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_exception_constructor_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_java_script_source_exception_operation(name.clone())) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116])).concat(&name)
    }
}

pub fn read_java_script_exception_constructor_class(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 65, 110, 100, 82, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114])
                            } else {
                                self::ml_string::Text::from_units(vec![])
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_linked_java_script_source_exception(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((self::read_java_script_source_exception_class(operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_java_script_source_exception_absent_message(absence: self::ml_string::Text) -> self::ml_string::Text {
    if (absence == (self::ml_string::Text::from_units(vec![110, 117, 108, 108]))) {
        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_source_exception_message_form(kind: self::ml_string::Text) -> self::ml_string::Text {
    if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
            self::ml_string::Text::from_units(vec![101, 109, 112, 116, 121])
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) {
                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
            } else {
                if ((((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                    self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                } else {
                    self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])
                }
            }
        }
    }
}

pub fn render_java_script_source_exception_expression(target: self::ml_string::Text, operation: self::ml_string::Text, message: self::ml_string::Text, present: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    {
        let name = self::read_java_script_source_exception_class(operation.clone());
        if (name == (self::ml_string::Text::from_units(vec![]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                ((((((self::ml_string::Text::from_units(vec![40, 40, 109, 108, 95, 109, 101, 115, 115, 97, 103, 101, 44, 109, 108, 95, 112, 114, 101, 115, 101, 110, 116, 41, 61, 62, 123, 116, 104, 114, 111, 119, 32, 110, 101, 119, 32])).concat(&name)).concat(&self::ml_string::Text::from_units(vec![40, 109, 108, 95, 112, 114, 101, 115, 101, 110, 116, 63, 109, 108, 95, 109, 101, 115, 115, 97, 103, 101, 58, 117, 110, 100, 101, 102, 105, 110, 101, 100, 41, 59, 125, 41, 40]))).concat(&message)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    ((((((self::ml_string::Text::from_units(vec![40, 99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 83, 111, 117, 114, 99, 101, 69, 120, 99, 101, 112, 116, 105, 111, 110, 32, 123, 32, 99, 108, 97, 115, 115, 95, 110, 97, 109, 101, 58, 32, 83, 116, 114, 105, 110, 103, 58, 58, 102, 114, 111, 109, 40, 34])).concat(&name)).concat(&self::ml_string::Text::from_units(vec![34, 41, 44, 32, 109, 101, 115, 115, 97, 103, 101, 95, 117, 110, 105, 116, 115, 58, 32]))).concat(&if utf16 {
                        ((self::ml_string::Text::from_units(vec![40])).concat(&message)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48]))
                    } else {
                        ((self::ml_string::Text::from_units(vec![40])).concat(&message)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41, 46, 99, 111, 108, 108, 101, 99, 116, 40, 41]))
                    })).concat(&self::ml_string::Text::from_units(vec![44, 32, 109, 101, 115, 115, 97, 103, 101, 95, 112, 114, 101, 115, 101, 110, 116, 58, 32]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![32, 125, 41, 46, 114, 97, 105, 115, 101, 40, 41]))
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_source_exception_ignored_pattern_fields(fields: f64) -> self::ml_string::Text {
    if (fields == (4f64)) {
        self::ml_string::Text::from_units(vec![32, 95, 32, 95])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn accept_source_exception_payload_fields(output: bool, names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> bool {
    ((((((((((output && (({ let ml_reference_array125 = names.clone(); let ml_reference_projection125 = ml_reference_array125.borrow().len() as f64; ml_reference_projection125 }) == (4f64))) && (({ let ml_reference_array126 = kinds.clone(); let ml_reference_projection126 = ml_reference_array126.borrow().len() as f64; ml_reference_projection126 }) == (4f64))) && {
        let source_ml_source2_comparison_operand567 = {
            let source_ml_source2_array_read_value565 = names.clone();
            {
                let source_ml_source2_array_read_index566 = 0f64;
                { let ml_reference_array127 = source_ml_source2_array_read_value565.clone(); let ml_reference_position127 = source_ml_source2_array_read_index566; let ml_reference_projection127 = self::ml_array::optional_at(&*ml_reference_array127.borrow(), self::ml_array::number_index(ml_reference_position127)); ml_reference_projection127 }
            }
        };
        {
            let source_ml_source2_comparison_operand568 = self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116]);
            match (source_ml_source2_comparison_operand567.clone(), source_ml_source2_comparison_operand568.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand571 = {
            let source_ml_source2_array_read_value569 = kinds.clone();
            {
                let source_ml_source2_array_read_index570 = 0f64;
                { let ml_reference_array128 = source_ml_source2_array_read_value569.clone(); let ml_reference_position128 = source_ml_source2_array_read_index570; let ml_reference_projection128 = self::ml_array::optional_at(&*ml_reference_array128.borrow(), self::ml_array::number_index(ml_reference_position128)); ml_reference_projection128 }
            }
        };
        {
            let source_ml_source2_comparison_operand572 = self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116]);
            match (source_ml_source2_comparison_operand571.clone(), source_ml_source2_comparison_operand572.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand575 = {
            let source_ml_source2_array_read_value573 = names.clone();
            {
                let source_ml_source2_array_read_index574 = 1f64;
                { let ml_reference_array129 = source_ml_source2_array_read_value573.clone(); let ml_reference_position129 = source_ml_source2_array_read_index574; let ml_reference_projection129 = self::ml_array::optional_at(&*ml_reference_array129.borrow(), self::ml_array::number_index(ml_reference_position129)); ml_reference_projection129 }
            }
        };
        {
            let source_ml_source2_comparison_operand576 = self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101]);
            match (source_ml_source2_comparison_operand575.clone(), source_ml_source2_comparison_operand576.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand579 = {
            let source_ml_source2_array_read_value577 = kinds.clone();
            {
                let source_ml_source2_array_read_index578 = 1f64;
                { let ml_reference_array130 = source_ml_source2_array_read_value577.clone(); let ml_reference_position130 = source_ml_source2_array_read_index578; let ml_reference_projection130 = self::ml_array::optional_at(&*ml_reference_array130.borrow(), self::ml_array::number_index(ml_reference_position130)); ml_reference_projection130 }
            }
        };
        {
            let source_ml_source2_comparison_operand580 = self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]);
            match (source_ml_source2_comparison_operand579.clone(), source_ml_source2_comparison_operand580.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand583 = {
            let source_ml_source2_array_read_value581 = names.clone();
            {
                let source_ml_source2_array_read_index582 = 2f64;
                { let ml_reference_array131 = source_ml_source2_array_read_value581.clone(); let ml_reference_position131 = source_ml_source2_array_read_index582; let ml_reference_projection131 = self::ml_array::optional_at(&*ml_reference_array131.borrow(), self::ml_array::number_index(ml_reference_position131)); ml_reference_projection131 }
            }
        };
        {
            let source_ml_source2_comparison_operand584 = self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 78, 97, 109, 101]);
            match (source_ml_source2_comparison_operand583.clone(), source_ml_source2_comparison_operand584.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand587 = {
            let source_ml_source2_array_read_value585 = kinds.clone();
            {
                let source_ml_source2_array_read_index586 = 2f64;
                { let ml_reference_array132 = source_ml_source2_array_read_value585.clone(); let ml_reference_position132 = source_ml_source2_array_read_index586; let ml_reference_projection132 = self::ml_array::optional_at(&*ml_reference_array132.borrow(), self::ml_array::number_index(ml_reference_position132)); ml_reference_projection132 }
            }
        };
        {
            let source_ml_source2_comparison_operand588 = self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]);
            match (source_ml_source2_comparison_operand587.clone(), source_ml_source2_comparison_operand588.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand591 = {
            let source_ml_source2_array_read_value589 = names.clone();
            {
                let source_ml_source2_array_read_index590 = 3f64;
                { let ml_reference_array133 = source_ml_source2_array_read_value589.clone(); let ml_reference_position133 = source_ml_source2_array_read_index590; let ml_reference_projection133 = self::ml_array::optional_at(&*ml_reference_array133.borrow(), self::ml_array::number_index(ml_reference_position133)); ml_reference_projection133 }
            }
        };
        {
            let source_ml_source2_comparison_operand592 = self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101, 80, 114, 101, 115, 101, 110, 116]);
            match (source_ml_source2_comparison_operand591.clone(), source_ml_source2_comparison_operand592.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand595 = {
            let source_ml_source2_array_read_value593 = kinds.clone();
            {
                let source_ml_source2_array_read_index594 = 3f64;
                { let ml_reference_array134 = source_ml_source2_array_read_value593.clone(); let ml_reference_position134 = source_ml_source2_array_read_index594; let ml_reference_projection134 = self::ml_array::optional_at(&*ml_reference_array134.borrow(), self::ml_array::number_index(ml_reference_position134)); ml_reference_projection134 }
            }
        };
        {
            let source_ml_source2_comparison_operand596 = self::ml_string::Text::from_units(vec![98, 111, 111, 108]);
            match (source_ml_source2_comparison_operand595.clone(), source_ml_source2_comparison_operand596.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    })
}

pub fn read_source_exception_observation_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 95, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 108, 97, 115, 115, 44, 32, 101, 120, 97, 99, 116, 32, 85, 84, 70, 45, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 110, 100, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 112, 114, 101, 115, 101, 110, 99, 101, 59, 32, 109, 97, 105, 110, 32, 112, 114, 111, 106, 101, 99, 116, 115, 32, 116, 104, 101, 32, 100, 105, 115, 112, 108, 97, 121, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 116, 32, 105, 116, 115, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 99, 101, 115, 115, 32, 98, 111, 117, 110, 100, 97, 114, 121])
}

pub fn read_source_exception_rocq_output_prelude() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 109, 108, 95, 101, 109, 105, 116, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 123, 69, 32, 58, 32, 84, 121, 112, 101, 125, 32, 40, 108, 105, 110, 101, 115, 32, 58, 32, 108, 105, 115, 116, 32, 115, 116, 114, 105, 110, 103, 41, 32, 40, 110, 101, 120, 116, 32, 58, 32, 108, 105, 115, 116, 32, 115, 116, 114, 105, 110, 103, 32, 42, 32, 111, 112, 116, 105, 111, 110, 32, 69, 41, 32, 58, 32, 108, 105, 115, 116, 32, 115, 116, 114, 105, 110, 103, 32, 42, 32, 111, 112, 116, 105, 111, 110, 32, 69, 32, 58, 61, 32, 40, 76, 105, 115, 116, 46, 97, 112, 112, 32, 108, 105, 110, 101, 115, 32, 40, 102, 115, 116, 32, 110, 101, 120, 116, 41, 44, 32, 115, 110, 100, 32, 110, 101, 120, 116, 41, 46])
}
