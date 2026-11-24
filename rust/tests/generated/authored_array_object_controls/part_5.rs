pub fn create_java_script_property_descriptor(kind: self::ml_string::Text, configurable: bool, enumerable: bool) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field408 = true;
        {
            let source_ml_source1_field409 = configurable;
            {
                let source_ml_source1_field410 = true;
                {
                    let source_ml_source1_field411 = enumerable;
                    {
                        let source_ml_source1_field412 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                        {
                            let source_ml_source1_field413 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field414 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                {
                                    let source_ml_source1_field415 = false;
                                    {
                                        let source_ml_source1_field416 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                        {
                                            let source_ml_source1_field417 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field418 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                                                {
                                                    let source_ml_source1_field419 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field409, source_ml_source1_field408, source_ml_source1_field411, source_ml_source1_field410, source_ml_source1_field417.clone(), source_ml_source1_field416, source_ml_source1_field419.clone(), source_ml_source1_field418, source_ml_source1_field413.clone(), source_ml_source1_field412, source_ml_source1_field415, source_ml_source1_field414)
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
        let source_ml_source1_field420 = true;
        {
            let source_ml_source1_field421 = if ({ let ml_reference_value52 = proposed.borrow().clone(); ml_reference_value52 }).1 {
                ({ let ml_reference_value53 = proposed.borrow().clone(); ml_reference_value53 }).0
            } else {
                ({ let ml_reference_value54 = base.borrow().clone(); ml_reference_value54 }).0
            };
            {
                let source_ml_source1_field422 = true;
                {
                    let source_ml_source1_field423 = if ({ let ml_reference_value55 = proposed.borrow().clone(); ml_reference_value55 }).3 {
                        ({ let ml_reference_value56 = proposed.borrow().clone(); ml_reference_value56 }).2
                    } else {
                        ({ let ml_reference_value57 = base.borrow().clone(); ml_reference_value57 }).2
                    };
                    {
                        let source_ml_source1_field424 = ({ let ml_reference_value58 = base.borrow().clone(); ml_reference_value58 }).9;
                        {
                            let source_ml_source1_field425 = if (({ let ml_reference_value59 = base.borrow().clone(); ml_reference_value59 }).9 && ({ let ml_reference_value60 = proposed.borrow().clone(); ml_reference_value60 }).9) {
                                ({ let ml_reference_value61 = proposed.borrow().clone(); ml_reference_value61 }).8
                            } else {
                                ({ let ml_reference_value62 = base.borrow().clone(); ml_reference_value62 }).8
                            };
                            {
                                let source_ml_source1_field426 = ({ let ml_reference_value63 = base.borrow().clone(); ml_reference_value63 }).11;
                                {
                                    let source_ml_source1_field427 = if (({ let ml_reference_value64 = base.borrow().clone(); ml_reference_value64 }).11 && ({ let ml_reference_value65 = proposed.borrow().clone(); ml_reference_value65 }).11) {
                                        ({ let ml_reference_value66 = proposed.borrow().clone(); ml_reference_value66 }).10
                                    } else {
                                        ({ let ml_reference_value67 = base.borrow().clone(); ml_reference_value67 }).10
                                    };
                                    {
                                        let source_ml_source1_field428 = ({ let ml_reference_value68 = base.borrow().clone(); ml_reference_value68 }).5;
                                        {
                                            let source_ml_source1_field429 = if (({ let ml_reference_value69 = base.borrow().clone(); ml_reference_value69 }).5 && ({ let ml_reference_value70 = proposed.borrow().clone(); ml_reference_value70 }).5) {
                                                ({ let ml_reference_value71 = proposed.borrow().clone(); ml_reference_value71 }).4
                                            } else {
                                                ({ let ml_reference_value72 = base.borrow().clone(); ml_reference_value72 }).4
                                            };
                                            {
                                                let source_ml_source1_field430 = ({ let ml_reference_value73 = base.borrow().clone(); ml_reference_value73 }).7;
                                                {
                                                    let source_ml_source1_field431 = if (({ let ml_reference_value74 = base.borrow().clone(); ml_reference_value74 }).7 && ({ let ml_reference_value75 = proposed.borrow().clone(); ml_reference_value75 }).7) {
                                                        ({ let ml_reference_value76 = proposed.borrow().clone(); ml_reference_value76 }).6
                                                    } else {
                                                        ({ let ml_reference_value77 = base.borrow().clone(); ml_reference_value77 }).6
                                                    };
                                                    (source_ml_source1_field421, source_ml_source1_field420, source_ml_source1_field423, source_ml_source1_field422, source_ml_source1_field429.clone(), source_ml_source1_field428, source_ml_source1_field431.clone(), source_ml_source1_field430, source_ml_source1_field425.clone(), source_ml_source1_field424, source_ml_source1_field427, source_ml_source1_field426)
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
                        let source_ml_source1_concat_value432 = writes.clone();
                        {
                            let source_ml_source1_concat_value433 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])]));
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value79 = source_ml_source1_concat_value432.borrow().clone(); ml_reference_value79 }), { let ml_reference_value80 = source_ml_source1_concat_value433.borrow().clone(); ml_reference_value80 })
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
                            let source_ml_source1_concat_value434 = writes_3.clone();
                            {
                                let source_ml_source1_concat_value435 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value82 = source_ml_source1_concat_value434.borrow().clone(); ml_reference_value82 }), { let ml_reference_value83 = source_ml_source1_concat_value435.borrow().clone(); ml_reference_value83 })
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
                                let source_ml_source1_concat_value436 = writes_5.clone();
                                {
                                    let source_ml_source1_concat_value437 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])]));
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value85 = source_ml_source1_concat_value436.borrow().clone(); ml_reference_value85 }), { let ml_reference_value86 = source_ml_source1_concat_value437.borrow().clone(); ml_reference_value86 })
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
                                    let source_ml_source1_concat_value438 = writes_7.clone();
                                    {
                                        let source_ml_source1_concat_value439 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value88 = source_ml_source1_concat_value438.borrow().clone(); ml_reference_value88 }), { let ml_reference_value89 = source_ml_source1_concat_value439.borrow().clone(); ml_reference_value89 })
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
                                        let source_ml_source1_concat_value440 = writes_9.clone();
                                        {
                                            let source_ml_source1_concat_value441 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116])]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value91 = source_ml_source1_concat_value440.borrow().clone(); ml_reference_value91 }), { let ml_reference_value92 = source_ml_source1_concat_value441.borrow().clone(); ml_reference_value92 })
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
                                            let source_ml_source1_concat_value442 = writes_11.clone();
                                            {
                                                let source_ml_source1_concat_value443 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 116])]));
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value94 = source_ml_source1_concat_value442.borrow().clone(); ml_reference_value94 }), { let ml_reference_value95 = source_ml_source1_concat_value443.borrow().clone(); ml_reference_value95 })
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
        let source_ml_source1_field444 = disposition.clone();
        {
            let source_ml_source1_field445 = error.clone();
            {
                let source_ml_source1_field446 = descriptor.clone();
                {
                    let source_ml_source1_field447 = writes.clone();
                    (source_ml_source1_field446.clone(), source_ml_source1_field444.clone(), source_ml_source1_field445.clone(), source_ml_source1_field447.clone())
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
                                    let source_ml_source1_s448 = if !(({ let ml_reference_value102 = current.borrow().clone(); ml_reference_value102 }).0) {
                                        if (({ let ml_reference_value103 = proposed.borrow().clone(); ml_reference_value103 }).1 && ({ let ml_reference_value104 = proposed.borrow().clone(); ml_reference_value104 }).0) {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            if (({ let ml_reference_value105 = proposed.borrow().clone(); ml_reference_value105 }).3 && ((({ let ml_reference_value106 = proposed.borrow().clone(); ml_reference_value106 }).2) != (({ let ml_reference_value107 = current.borrow().clone(); ml_reference_value107 }).2))) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if (({ let ml_reference_value108 = proposed.borrow().clone(); ml_reference_value108 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value109 = proposed.borrow().clone(); ml_reference_value109 }).4, ({ let ml_reference_value110 = current.borrow().clone(); ml_reference_value110 }).4))) {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            if (({ let ml_reference_value111 = proposed.borrow().clone(); ml_reference_value111 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value112 = proposed.borrow().clone(); ml_reference_value112 }).6, ({ let ml_reference_value113 = current.borrow().clone(); ml_reference_value113 }).6))) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next
                                                            }
                                                        }
                                                    } else {
                                                        if !(({ let ml_reference_value114 = current.borrow().clone(); ml_reference_value114 }).10) {
                                                            if (({ let ml_reference_value115 = proposed.borrow().clone(); ml_reference_value115 }).11 && ({ let ml_reference_value116 = proposed.borrow().clone(); ml_reference_value116 }).10) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                if ({ let ml_reference_value117 = proposed.borrow().clone(); ml_reference_value117 }).9 {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(self::create_java_script_property_transition(if self::test_java_script_semantic_same_value(({ let ml_reference_value118 = proposed.borrow().clone(); ml_reference_value118 }).8, ({ let ml_reference_value119 = current.borrow().clone(); ml_reference_value119 }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }, self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                                } else {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next
                                    };
                                    match source_ml_source1_s448.clone() {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next => {
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
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8::MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(source_ml_temporary_result8) => {
                                            source_ml_temporary_result8.clone()
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
    if ((({ let ml_reference_array122 = key.clone(); let ml_reference_projection122 = ml_reference_array122.borrow().len() as f64; ml_reference_projection122 }) == (0f64)) || (({ let ml_reference_array123 = key.clone(); let ml_reference_projection123 = ml_reference_array123.borrow().len() as f64; ml_reference_projection123 }) > (10f64))) {
        (-1f64)
    } else {
        if ((({ let ml_reference_array124 = key.clone(); let ml_reference_projection124 = ml_reference_array124.borrow().len() as f64; ml_reference_projection124 }) > (1f64)) && {
            let source_ml_source1_comparison_operand451 = {
                let source_ml_source1_array_read_value449 = key.clone();
                {
                    let source_ml_source1_array_read_index450 = 0f64;
                    { let ml_reference_array125 = source_ml_source1_array_read_value449.clone(); let ml_reference_position125 = source_ml_source1_array_read_index450; let ml_reference_projection125 = self::ml_array::optional_at(&*ml_reference_array125.borrow(), self::ml_array::number_index(ml_reference_position125)); ml_reference_projection125 }
                }
            };
            {
                let source_ml_source1_comparison_operand452 = 48f64;
                match (source_ml_source1_comparison_operand451.clone(), source_ml_source1_comparison_operand452) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s453 = self::source_ml_temporary_read_java_script_literal_property_index_loop9(key.clone(), value, at);
                        match source_ml_source1_s453.clone() {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Return(source_ml_temporary_result9) => {
                                source_ml_temporary_result9
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
            let high = { let ml_reference_array126 = objects.clone(); let ml_reference_projection126 = ml_reference_array126.borrow().len() as f64; ml_reference_projection126 };
            {
                let source_ml_source1_s454 = self::source_ml_temporary_find_java_script_heap_directory_object_loop10(objects.clone(), identity.clone(), low, high);
                match source_ml_source1_s454.clone() {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Return(source_ml_temporary_result10) => {
                        source_ml_temporary_result10
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
            let high = { let ml_reference_array127 = symbols.clone(); let ml_reference_projection127 = ml_reference_array127.borrow().len() as f64; ml_reference_projection127 };
            {
                let source_ml_source1_s455 = self::source_ml_temporary_test_java_script_heap_directory_symbol_loop11(symbols.clone(), identity.clone(), low, high);
                match source_ml_source1_s455.clone() {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Return(source_ml_temporary_result11) => {
                        source_ml_temporary_result11
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f456) => {
                    {
                        let source_ml_source1_f478 = source_ml_source1_f456;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f457) => {
                    {
                        let source_ml_source1_f486 = source_ml_source1_f457;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f458) => {
                    {
                        let source_ml_source1_f494 = source_ml_source1_f458.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f459) => {
                    {
                        let source_ml_source1_f502 = source_ml_source1_f459.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f460, source_ml_source1_f461) => {
                    {
                        let source_ml_source1_f510 = source_ml_source1_f460.clone();
                        {
                            let source_ml_source1_f511 = source_ml_source1_f461.clone();
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_heap_directory_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value513, _, _, _, _) => {
                            source_ml_source1_data_property_value513.clone()
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f462, source_ml_source1_f463) => {
                    {
                        let identity_2 = source_ml_source1_f462.clone();
                        {
                            let callable = source_ml_source1_f463;
                            {
                                let at = self::find_java_script_heap_directory_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value521, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value521.clone()
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = {
                                            let source_ml_source1_array_read_value523 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value522, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value522.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_array_read_index524 = at;
                                                { let ml_reference_array128 = source_ml_source1_array_read_value523.clone(); let ml_reference_position128 = source_ml_source1_array_read_index524; let ml_reference_projection128 = self::ml_array::optional_at(&*ml_reference_array128.borrow(), self::ml_array::number_index(ml_reference_position128)); ml_reference_projection128 }
                                            }
                                        };
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value538, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value538
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value539, _, _, _, _) => {
                                                source_ml_source1_data_property_value539
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value540, _) => {
                                                source_ml_source1_data_property_value540
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value541, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value541
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value542, _, _) => {
                                                source_ml_source1_data_property_value542
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value543, _, _) => {
                                                source_ml_source1_data_property_value543
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value544, _, _) => {
                                                source_ml_source1_data_property_value544
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value545, _, _) => {
                                                source_ml_source1_data_property_value545
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value546, _) => {
                                                source_ml_source1_data_property_value546
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value547, _, _, _) => {
                                                source_ml_source1_data_property_value547
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value548, _, _, _) => {
                                                source_ml_source1_data_property_value548
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value549, _, _, _) => {
                                                source_ml_source1_data_property_value549
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value550, _, _, _) => {
                                                source_ml_source1_data_property_value550
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f551) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f552) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f553) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f554) => {
            false
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f555, source_ml_source1_f556) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f557) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f558, source_ml_source1_f559) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f560) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f561) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f562) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f563) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f564, source_ml_source1_f565) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f566) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f567, source_ml_source1_f568) => {
            true
        }
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f569) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f570) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f571) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f572) => {
            true
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f573, source_ml_source1_f574) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f575) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f576, source_ml_source1_f577) => {
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
            let source_ml_source1_array_read_value604 = match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value578, _, _, _, _, _) => {
                    source_ml_source1_data_property_value578.clone()
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value579, _, _, _) => {
                    source_ml_source1_data_property_value579.clone()
                }
                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value580) => {
                    source_ml_source1_data_property_value580.clone()
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value581, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value581.clone()
                }
                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value582, _) => {
                    source_ml_source1_data_property_value582.clone()
                }
                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value583, _) => {
                    source_ml_source1_data_property_value583.clone()
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value584, _) => {
                    source_ml_source1_data_property_value584.clone()
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value585, _) => {
                    source_ml_source1_data_property_value585.clone()
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value586) => {
                    source_ml_source1_data_property_value586.clone()
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value587, _, _) => {
                    source_ml_source1_data_property_value587.clone()
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value588, _, _) => {
                    source_ml_source1_data_property_value588.clone()
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value589, _, _) => {
                    source_ml_source1_data_property_value589.clone()
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value590, _, _) => {
                    source_ml_source1_data_property_value590.clone()
                }
            };
            {
                let source_ml_source1_array_read_index605 = self::find_java_script_array_property(match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value591, _, _, _, _, _) => {
                        source_ml_source1_data_property_value591.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value592, _, _, _) => {
                        source_ml_source1_data_property_value592.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value593) => {
                        source_ml_source1_data_property_value593.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value594, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value594.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value595, _) => {
                        source_ml_source1_data_property_value595.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value596, _) => {
                        source_ml_source1_data_property_value596.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value597, _) => {
                        source_ml_source1_data_property_value597.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value598, _) => {
                        source_ml_source1_data_property_value598.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value599) => {
                        source_ml_source1_data_property_value599.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value600, _, _) => {
                        source_ml_source1_data_property_value600.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value601, _, _) => {
                        source_ml_source1_data_property_value601.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value602, _, _) => {
                        source_ml_source1_data_property_value602.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value603, _, _) => {
                        source_ml_source1_data_property_value603.clone()
                    }
                }, self::read_java_script_array_length_key());
                { let ml_reference_array129 = source_ml_source1_array_read_value604.clone(); let ml_reference_position129 = source_ml_source1_array_read_index605; let ml_reference_projection129 = self::ml_array::optional_at(&*ml_reference_array129.borrow(), self::ml_array::number_index(ml_reference_position129)); ml_reference_projection129 }
            }
        };
        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            (-1f64)
        } else {
            self::read_java_script_array_length_value(({ let ml_reference_value130 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value606) => {
                    source_ml_source1_data_property_value606.clone()
                }
            }).borrow().clone(); ml_reference_value130 }).8)
        }
    }
}

pub fn read_java_script_array_length_value(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Null => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f607) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Number(value_value) => {
            value_value
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f608) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f609) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f610, source_ml_source1_f611) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f612) => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f613, source_ml_source1_f614) => {
            (-1f64)
        }
    }
}

pub fn test_java_script_array_length(length: f64) -> bool {
    (((self::ml_math::is_integer(length) && (length >= (0f64))) && (length <= (4294967295f64))) && !({
        let source_ml_source1_same_value_operand615 = length;
        {
            let source_ml_source1_same_value_operand616 = (-0f64);
            (((source_ml_source1_same_value_operand615 == source_ml_source1_same_value_operand616) && ((source_ml_source1_same_value_operand615 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand615)) == ((1f64 / source_ml_source1_same_value_operand616))))) || ((source_ml_source1_same_value_operand615 != source_ml_source1_same_value_operand615) && (source_ml_source1_same_value_operand616 != source_ml_source1_same_value_operand616)))
        }
    }))
}
