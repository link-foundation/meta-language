pub fn render_collection_construction(target: self::ml_string::Text, kind: self::ml_string::Text, key_kind: self::ml_string::Text, values: self::ml_string::Text, collection_type: self::ml_string::Text, element_type: self::ml_string::Text, comparison: self::ml_string::Text) -> self::ml_string::Text {
    {
        let set = (kind == (self::ml_string::Text::from_units(vec![115, 101, 116])));
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((((self::ml_string::Text::from_units(vec![110, 101, 119, 32])).concat(&if set {
                self::ml_string::Text::from_units(vec![83, 101, 116])
            } else {
                self::ml_string::Text::from_units(vec![77, 97, 112])
            })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                {
                    let comparator = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        self::ml_string::Text::from_units(vec![124, 97, 58, 32, 38, 102, 54, 52, 44, 32, 98, 58, 32, 38, 102, 54, 52, 124, 32, 97, 32, 61, 61, 32, 98, 32, 124, 124, 32, 40, 97, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 38, 38, 32, 98, 46, 105, 115, 95, 110, 97, 110, 40, 41, 41])
                    } else {
                        self::ml_string::Text::from_units(vec![124, 97, 44, 32, 98, 124, 32, 97, 32, 61, 61, 32, 98])
                    };
                    {
                        let values_3 = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                            {
                                let values_2 = ((((self::ml_string::Text::from_units(vec![40])).concat(&values)).concat(&self::ml_string::Text::from_units(vec![41, 46, 105, 110, 116, 111, 95, 105, 116, 101, 114, 40, 41, 46, 109, 97, 112, 40, 124, 109, 108, 95, 118, 97, 108, 117, 101, 124, 32]))).concat(&if set {
                                    self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 61, 32, 48, 102, 54, 52, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 125])
                                } else {
                                    self::ml_string::Text::from_units(vec![40, 105, 102, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 48, 32, 61, 61, 32, 48, 102, 54, 52, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 48, 32, 125, 44, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 49, 41])
                                })).concat(&self::ml_string::Text::from_units(vec![41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41]));
                                values_2.clone()
                            }
                        } else {
                            values.clone()
                        };
                        ((((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 97, 114, 114, 97, 121, 58, 58])).concat(&kind)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&values_3)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&comparator)).concat(&self::ml_string::Text::from_units(vec![41]))
                    }
                }
            } else {
                {
                    let value = self::read_collection_construction_binding(target.clone(), kind.clone(), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]));
                    {
                        let previous = self::read_collection_construction_binding(target.clone(), kind.clone(), self::ml_string::Text::from_units(vec![112, 114, 101, 118, 105, 111, 117, 115]));
                        if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                            {
                                let equal = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                    ((((((((self::ml_string::Text::from_units(vec![40])).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 124, 124, 32, 40]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 78, 97, 78, 32, 38, 38, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 78, 97, 78, 41, 41]))
                                } else {
                                    ((((self::ml_string::Text::from_units(vec![40])).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                };
                                {
                                    let normalized = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                        if set {
                                            self::ml_string::Text::from_units(vec![40, 105, 102, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 32, 61, 61, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 116, 104, 101, 110, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41])
                                        } else {
                                            self::ml_string::Text::from_units(vec![40, 40, 105, 102, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 46, 49, 32, 61, 61, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 116, 104, 101, 110, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 46, 49, 41, 44, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 46, 50, 41])
                                        }
                                    } else {
                                        self::ml_string::Text::from_units(vec![109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108])
                                    };
                                    {
                                        let existing = if set {
                                            self::ml_string::Text::from_units(vec![109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 32, 61, 62, 32, 105, 102, 32])).concat(&equal)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 40, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 46, 49, 44, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 50, 41, 32, 101, 108, 115, 101, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 41, 41]))
                                        };
                                        ((((((((((self::ml_string::Text::from_units(vec![40])).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 32, 61, 62, 32, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 58, 61, 32]))).concat(&normalized)).concat(&self::ml_string::Text::from_units(vec![59, 32, 105, 102, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 46, 97, 110, 121, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 32, 61, 62, 32]))).concat(&equal)).concat(&self::ml_string::Text::from_units(vec![41, 32, 116, 104, 101, 110, 32]))).concat(&existing)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 46, 112, 117, 115, 104, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41, 32, 40, 35, 91, 93, 32, 58, 32]))).concat(&collection_type)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    }
                                }
                            }
                        } else {
                            {
                                let equal_2 = comparison.clone();
                                {
                                    let equal_4 = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                        {
                                            let equal_3 = ((((((self::ml_string::Text::from_units(vec![40, 111, 114, 98, 32])).concat(&equal_2)).concat(&self::ml_string::Text::from_units(vec![32, 40, 97, 110, 100, 98, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110, 32]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![41, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41, 41]));
                                            equal_3.clone()
                                        }
                                    } else {
                                        equal_2.clone()
                                    };
                                    {
                                        let normalized_2 = if (key_kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                            if set {
                                                self::ml_string::Text::from_units(vec![40, 105, 102, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 101, 113, 98, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 32, 48, 37, 102, 108, 111, 97, 116, 32, 116, 104, 101, 110, 32, 48, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41])
                                            } else {
                                                self::ml_string::Text::from_units(vec![40, 40, 105, 102, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 101, 113, 98, 32, 40, 102, 115, 116, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41, 32, 48, 37, 102, 108, 111, 97, 116, 32, 116, 104, 101, 110, 32, 48, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 102, 115, 116, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41, 44, 32, 115, 110, 100, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41])
                                            }
                                        } else {
                                            self::ml_string::Text::from_units(vec![109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108])
                                        };
                                        {
                                            let existing_2 = if set {
                                                self::ml_string::Text::from_units(vec![109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
                                            } else {
                                                ((self::ml_string::Text::from_units(vec![40, 76, 105, 115, 116, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 32, 61, 62, 32, 105, 102, 32])).concat(&equal_4)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 40, 102, 115, 116, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 44, 32, 115, 110, 100, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41, 32, 101, 108, 115, 101, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 41, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 41]))
                                            };
                                            ((((((((((self::ml_string::Text::from_units(vec![40, 76, 105, 115, 116, 46, 102, 111, 108, 100, 95, 108, 101, 102, 116, 32, 40, 102, 117, 110, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 32, 61, 62, 32, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 58, 61, 32])).concat(&normalized_2)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32, 105, 102, 32, 76, 105, 115, 116, 46, 101, 120, 105, 115, 116, 115, 98, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 32, 61, 62, 32]))).concat(&equal_4)).concat(&self::ml_string::Text::from_units(vec![41, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 116, 104, 101, 110, 32]))).concat(&existing_2)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32, 76, 105, 115, 116, 46, 97, 112, 112, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 40, 109, 108, 95, 118, 97, 108, 117, 101, 32, 58, 58, 32, 110, 105, 108, 41, 41, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 40, 64, 110, 105, 108, 32]))).concat(&element_type)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
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

pub fn read_java_script_closed_class_observation_operands(kind: self::ml_string::Text, proof: self::ml_string::Text) -> self::ml_string::Text {
    if self::ml_array::includes(&{ let ml_reference_value7 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 113]), self::ml_string::Text::from_units(vec![110, 101]), self::ml_string::Text::from_units(vec![108, 116]), self::ml_string::Text::from_units(vec![108, 101]), self::ml_string::Text::from_units(vec![103, 116]), self::ml_string::Text::from_units(vec![103, 101])]))).borrow().clone(); ml_reference_value7 }, &proof.clone(), 0f64, false, |a, b| a == b) {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else {
        if (proof == (self::ml_string::Text::from_units(vec![102, 111, 114, 97, 108, 108]))) {
            self::ml_string::Text::from_units(vec![98, 105, 110, 100, 101, 114, 115])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))) || (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 84, 101, 120, 116])))) {
                self::ml_string::Text::from_units(vec![97, 114, 103])
            } else {
                if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108]))) {
                    self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
                } else {
                    if ((kind == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116])))) {
                        self::ml_string::Text::from_units(vec![101, 120, 112, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_observation_operation_operands(kind: self::ml_string::Text, proof: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((kind == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))) && self::ml_array::includes(&{ let ml_reference_value8 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 113]), self::ml_string::Text::from_units(vec![110, 101])]))).borrow().clone(); ml_reference_value8 }, &operation.clone(), 0f64, false, |a, b| a == b)) {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else {
        self::read_java_script_closed_class_observation_operands(kind.clone(), proof.clone())
    }
}

pub fn contains_java_script_closed_class_brand(texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, root: f64, declarations: f64, brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> bool {
    if (({ let ml_reference_array9 = brands.clone(); let ml_reference_projection9 = ml_reference_array9.borrow().len() as f64; ml_reference_projection9 }) == (0f64)) {
        false
    } else {
        {
            let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![root]));
            {
                let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                {
                    let source_ml_source2_s8 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), pending.clone(), seen.clone());
                    match source_ml_source2_s8.clone() {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done => {
                            false
                        }
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

pub fn has_java_script_closed_class_exported_brand(texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, declarations: f64, exports: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> bool {
    if (({ let ml_reference_array10 = exports.clone(); let ml_reference_projection10 = ml_reference_array10.borrow().len() as f64; ml_reference_projection10 }) == (0f64)) {
        false
    } else {
        if (declarations < (0f64)) {
            true
        } else {
            {
                let source_ml_source1_temporary_values8 = exports.clone();
                {
                    let source_ml_source1_temporary_index8 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s9 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), source_ml_source1_temporary_values8.clone(), source_ml_source1_temporary_index8.clone());
                        match source_ml_source2_s9.clone() {
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done => {
                                false
                            }
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result9) => {
                                source_ml_source1_temporary_result9
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_record_field_assignment_operator(operator: self::ml_string::Text) -> bool {
    (((((((((operator == (self::ml_string::Text::from_units(vec![61]))) || (operator == (self::ml_string::Text::from_units(vec![43, 61])))) || (operator == (self::ml_string::Text::from_units(vec![45, 61])))) || (operator == (self::ml_string::Text::from_units(vec![42, 61])))) || (operator == (self::ml_string::Text::from_units(vec![47, 61])))) || (operator == (self::ml_string::Text::from_units(vec![37, 61])))) || (operator == (self::ml_string::Text::from_units(vec![42, 42, 61])))) || (operator == (self::ml_string::Text::from_units(vec![43, 43])))) || (operator == (self::ml_string::Text::from_units(vec![45, 45]))))
}

pub fn read_record_field_write_identity(operator: self::ml_string::Text, field: self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_record_field_assignment_operator(operator.clone()) {
        (((self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])).concat(&operator)).concat(&self::ml_string::Text::from_units(vec![58]))).concat(&field)
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_record_field_write_operator(operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let operators = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![43, 61]), self::ml_string::Text::from_units(vec![45, 61]), self::ml_string::Text::from_units(vec![42, 61]), self::ml_string::Text::from_units(vec![47, 61]), self::ml_string::Text::from_units(vec![37, 61]), self::ml_string::Text::from_units(vec![42, 42, 61]), self::ml_string::Text::from_units(vec![43, 43]), self::ml_string::Text::from_units(vec![45, 45])]));
        {
            let source_ml_source1_temporary_values12 = operators.clone();
            {
                let source_ml_source1_temporary_index12 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s10 = self::source_ml_source1_temporary_read_record_field_write_operator_loop13(operation.clone(), source_ml_source1_temporary_values12.clone(), source_ml_source1_temporary_index12.clone());
                    match source_ml_source2_s10.clone() {
                        self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(source_ml_source1_temporary_result13) => {
                            source_ml_source1_temporary_result13.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_record_field_write_name(operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let operator = self::read_record_field_write_operator(operation.clone());
        if (operator == (self::ml_string::Text::from_units(vec![]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let source_ml_source2_string_range_operand11 = operation.clone();
                {
                    let source_ml_source2_string_range_operand12 = (17f64 + operator.length());
                    {
                        let source_ml_source2_string_range_position13 = source_ml_source2_string_range_operand12;
                        {
                            let source_ml_source2_string_range_position14 = f64::INFINITY;
                            source_ml_source2_string_range_operand11.slice(source_ml_source2_string_range_position13, source_ml_source2_string_range_position14)
                        }
                    }
                }
            }
        }
    }
}

pub fn read_record_field_write_argument_count(operator: self::ml_string::Text) -> f64 {
    if ((operator == (self::ml_string::Text::from_units(vec![43, 43]))) || (operator == (self::ml_string::Text::from_units(vec![45, 45])))) {
        0f64
    } else {
        1f64
    }
}

pub fn read_record_field_write_suffix(initial: f64, receiver: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> f64 {
    {
        let text = (receiver.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&self::ml_string::join(&{ let ml_reference_value11 = arguments_.borrow().clone(); ml_reference_value11 }, &self::ml_string::Text::from_units(vec![32])));
        {
            let suffix = initial;
            {
                let suffix_2 = self::source_ml_source1_temporary_read_record_field_write_suffix_loop14(text.clone(), suffix);
                suffix_2
            }
        }
    }
}

pub fn accept_linked_record_field_write(expression: self::ml_string::Text, operation: self::ml_string::Text, kind: self::ml_string::Text, reference: bool) -> bool {
    ((((expression == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))) && reference) && ((self::read_record_field_write_operator(operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_linked_ordered_record_snapshot_operation(expression: self::ml_string::Text, kind: self::ml_string::Text, ordered: bool) -> self::ml_string::Text {
    if (((expression == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 82, 101, 97, 100]))) && (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))) && ordered) {
        self::ml_string::Text::from_units(vec![114, 101, 97, 100, 79, 114, 100, 101, 114, 101, 100, 82, 101, 99, 111, 114, 100, 83, 110, 97, 112, 115, 104, 111, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_record_field_write_refusal(operator: self::ml_string::Text, kind: self::ml_string::Text, reference: bool, open: bool, field: bool, value_kind: self::ml_string::Text) -> self::ml_string::Text {
    if !(self::accept_record_field_assignment_operator(operator.clone())) {
        self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 111, 112, 101, 114, 97, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 116, 97, 116, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
    } else {
        if (((kind != (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) || !reference) || open) {
            self::ml_string::Text::from_units(vec![97, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32, 110, 101, 101, 100, 115, 32, 97, 32, 99, 108, 111, 115, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 119, 105, 116, 104, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115])
        } else {
            if !field {
                self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 105, 110, 103, 32, 97, 32, 102, 105, 101, 108, 100, 32, 110, 101, 101, 100, 115, 32, 97, 32, 100, 121, 110, 97, 109, 105, 99, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110])
            } else {
                if ((operator != (self::ml_string::Text::from_units(vec![61]))) && (value_kind != (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))) {
                    self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 111, 109, 112, 111, 117, 110, 100, 32, 102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 101, 114, 99, 105, 111, 110, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101, 32, 100, 111, 109, 97, 105, 110])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_record_compound_binary_operation(language: self::ml_string::Text, operator: self::ml_string::Text, kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) || ((kind != (self::ml_string::Text::from_units(vec![105, 110, 116]))) && (kind != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
            if (operator == (self::ml_string::Text::from_units(vec![43, 61]))) {
                self::ml_string::Text::from_units(vec![112, 108, 117, 115])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        } else {
            self::read_record_numeric_compound_binary_operation(operator.clone())
        }
    }
}

pub fn read_record_numeric_compound_binary_operation(operator: self::ml_string::Text) -> self::ml_string::Text {
    if ((operator == (self::ml_string::Text::from_units(vec![43, 61]))) || (operator == (self::ml_string::Text::from_units(vec![43, 43])))) {
        self::ml_string::Text::from_units(vec![112, 108, 117, 115])
    } else {
        if ((operator == (self::ml_string::Text::from_units(vec![45, 61]))) || (operator == (self::ml_string::Text::from_units(vec![45, 45])))) {
            self::ml_string::Text::from_units(vec![115, 117, 98])
        } else {
            if (operator == (self::ml_string::Text::from_units(vec![42, 61]))) {
                self::ml_string::Text::from_units(vec![109, 117, 108])
            } else {
                if (operator == (self::ml_string::Text::from_units(vec![47, 61]))) {
                    self::ml_string::Text::from_units(vec![100, 105, 118])
                } else {
                    if (operator == (self::ml_string::Text::from_units(vec![37, 61]))) {
                        self::ml_string::Text::from_units(vec![114, 101, 109])
                    } else {
                        if (operator == (self::ml_string::Text::from_units(vec![42, 42, 61]))) {
                            self::ml_string::Text::from_units(vec![101, 120, 112, 111, 110, 101, 110, 116, 105, 97, 116, 105, 111, 110])
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_optional_record_compound_binary_operation(language: self::ml_string::Text, operator: self::ml_string::Text, kind: self::ml_string::Text, absence: self::ml_string::Text) -> self::ml_string::Text {
    if ((language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) || ((absence != (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) && (absence != (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
            self::read_record_numeric_compound_binary_operation(operator.clone())
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::read_record_compound_binary_operation(language.clone(), operator.clone(), kind.clone())
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_record_field_source_write_refusal(language: self::ml_string::Text, operator: self::ml_string::Text, kind: self::ml_string::Text, reference: bool, open: bool, field: bool, value_kind: self::ml_string::Text) -> self::ml_string::Text {
    {
        let operation = self::read_record_compound_binary_operation(language.clone(), operator.clone(), value_kind.clone());
        if (operation != (self::ml_string::Text::from_units(vec![]))) {
            if (((kind != (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) || !reference) || open) {
                self::ml_string::Text::from_units(vec![97, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32, 110, 101, 101, 100, 115, 32, 97, 32, 99, 108, 111, 115, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 119, 105, 116, 104, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115])
            } else {
                if !field {
                    self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 105, 110, 103, 32, 97, 32, 102, 105, 101, 108, 100, 32, 110, 101, 101, 100, 115, 32, 97, 32, 100, 121, 110, 97, 109, 105, 99, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        } else {
            self::read_record_field_write_refusal(operator.clone(), kind.clone(), reference, open, field, value_kind.clone())
        }
    }
}

pub fn render_record_field_write(target: self::ml_string::Text, operator: self::ml_string::Text, receiver: self::ml_string::Text, field: self::ml_string::Text, index: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, suffix: self::ml_string::Text) -> self::ml_string::Text {
    {
        let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 111, 119, 110, 101, 114])).concat(&suffix);
        {
            let value = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101])).concat(&suffix);
            {
                let previous = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114, 101, 118, 105, 111, 117, 115])).concat(&suffix);
                {
                    let unary = ((operator == (self::ml_string::Text::from_units(vec![43, 43]))) || (operator == (self::ml_string::Text::from_units(vec![45, 45]))));
                    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                        (((((((self::ml_string::Text::from_units(vec![40, 118, 111, 105, 100, 32, 40, 40])).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 91]))).concat(&field)).concat(&self::ml_string::Text::from_units(vec![93]))).concat(&operator)).concat(&if unary {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            ((self::ml_string::Text::from_units(vec![40])).concat(&({
                                let source_ml_source2_array_read_value15 = arguments_.clone();
                                {
                                    let source_ml_source2_array_read_index16 = 0f64;
                                    { let ml_reference_array12 = source_ml_source2_array_read_value15.clone(); let ml_reference_position12 = source_ml_source2_array_read_index16; let ml_reference_projection12 = self::ml_array::optional_at(&*ml_reference_array12.borrow(), self::ml_array::number_index(ml_reference_position12)); ml_reference_projection12 }
                                }
                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![41]))
                        })).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                    } else {
                        if (target != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            {
                                let output = ((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                {
                                    let output_3 = if (operator != (self::ml_string::Text::from_units(vec![61]))) {
                                        {
                                            let output_2 = output.concat(&((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 46]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                            output_2.clone()
                                        }
                                    } else {
                                        output.clone()
                                    };
                                    {
                                        let output_5 = if !unary {
                                            {
                                                let output_4 = output_3.concat(&((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&({
                                                    let source_ml_source2_array_read_value17 = arguments_.clone();
                                                    {
                                                        let source_ml_source2_array_read_index18 = 0f64;
                                                        { let ml_reference_array13 = source_ml_source2_array_read_value17.clone(); let ml_reference_position13 = source_ml_source2_array_read_index18; let ml_reference_projection13 = self::ml_array::optional_at(&*ml_reference_array13.borrow(), self::ml_array::number_index(ml_reference_position13)); ml_reference_projection13 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                output_4.clone()
                                            }
                                        } else {
                                            output_3.clone()
                                        };
                                        {
                                            let result = if (operator == (self::ml_string::Text::from_units(vec![61]))) {
                                                value.clone()
                                            } else {
                                                if (operator == (self::ml_string::Text::from_units(vec![43, 43]))) {
                                                    previous.concat(&self::ml_string::Text::from_units(vec![32, 43, 32, 49, 46, 48]))
                                                } else {
                                                    if (operator == (self::ml_string::Text::from_units(vec![45, 45]))) {
                                                        previous.concat(&self::ml_string::Text::from_units(vec![32, 45, 32, 49, 46, 48]))
                                                    } else {
                                                        if (operator == (self::ml_string::Text::from_units(vec![42, 42, 61]))) {
                                                            ((((((((((((((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 48, 46, 48, 32, 123, 32, 49, 46, 48, 32, 125, 32, 101, 108, 115, 101, 32, 105, 102, 32]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 124, 124, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 124, 124, 32, 40]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 97, 98, 115, 40, 41, 32, 61, 61, 32, 49, 46, 48, 32, 38, 38, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 105, 110, 102, 105, 110, 105, 116, 101, 40, 41, 41, 32, 123, 32, 102, 54, 52, 58, 58, 78, 65, 78, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 112, 111, 119, 102, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                                                        } else {
                                                            (((previous.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&{
                                                                let source_ml_source2_string_range_operand19 = operator.clone();
                                                                {
                                                                    let source_ml_source2_string_range_operand20 = 0f64;
                                                                    {
                                                                        let source_ml_source2_string_range_operand21 = 1f64;
                                                                        {
                                                                            let source_ml_source2_string_range_position22 = source_ml_source2_string_range_operand20;
                                                                            {
                                                                                let source_ml_source2_string_range_position23 = source_ml_source2_string_range_operand21;
                                                                                source_ml_source2_string_range_operand19.slice(source_ml_source2_string_range_position22, source_ml_source2_string_range_position23)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&value)
                                                        }
                                                    }
                                                }
                                            };
                                            (((((output_5.concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&result)).concat(&self::ml_string::Text::from_units(vec![59, 32, 40, 41, 32, 125]))
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

pub fn read_optional_record_field_order_followers(numeric_keys: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, field: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let selected = ({
            let source_ml_source2_array_read_value24 = numeric_keys.clone();
            {
                let source_ml_source2_array_read_index25 = field;
                { let ml_reference_array14 = source_ml_source2_array_read_value24.clone(); let ml_reference_position14 = source_ml_source2_array_read_index25; let ml_reference_projection14 = self::ml_array::optional_at(&*ml_reference_array14.borrow(), self::ml_array::number_index(ml_reference_position14)); ml_reference_projection14 }
            }
        }).clone().unwrap_or_else(|| (-1f64));
        {
            let followers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            if (selected < (0f64)) {
                followers.clone()
            } else {
                {
                    let index = 0f64;
                    {
                        let followers_2 = self::source_ml_source1_temporary_read_optional_record_field_order_followers_loop15(numeric_keys.clone(), selected, followers.clone(), index);
                        followers_2.clone()
                    }
                }
            }
        }
    }
}

pub fn render_optional_rust_record_field_write(receiver: self::ml_string::Text, index: self::ml_string::Text, argument: self::ml_string::Text, suffix: self::ml_string::Text, ordered: bool, followers: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> self::ml_string::Text {
    {
        let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 111, 119, 110, 101, 114])).concat(&suffix);
        {
            let value = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101])).concat(&suffix);
            {
                let capture = ((((((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&argument)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                {
                    let storage = ((((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 117, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]));
                    if !ordered {
                        ((((((capture.concat(&storage)).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 83, 111, 109, 101, 40, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![44, 41, 41, 59, 32, 40, 41, 32, 125]))
                    } else {
                        {
                            let field = (owner.concat(&self::ml_string::Text::from_units(vec![46, 48, 46]))).concat(&index);
                            {
                                let order = owner.concat(&self::ml_string::Text::from_units(vec![46, 49]));
                                {
                                    let position = if (({ let ml_reference_array15 = followers.clone(); let ml_reference_projection15 = ml_reference_array15.borrow().len() as f64; ml_reference_projection15 }) == (0f64)) {
                                        order.concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41]))
                                    } else {
                                        ((((order.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 112, 111, 115, 105, 116, 105, 111, 110, 40, 124, 109, 108, 95, 111, 114, 100, 101, 114, 101, 100, 95, 102, 105, 101, 108, 100, 124, 32, 91]))).concat(&self::ml_string::join(&{ let ml_reference_value16 = followers.borrow().clone(); ml_reference_value16 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 46, 99, 111, 110, 116, 97, 105, 110, 115, 40, 109, 108, 95, 111, 114, 100, 101, 114, 101, 100, 95, 102, 105, 101, 108, 100, 41, 41, 46, 117, 110, 119, 114, 97, 112, 95, 111, 114, 40]))).concat(&order)).concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41, 41]))
                                    };
                                    {
                                        let next = (self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114, 101, 118, 105, 111, 117, 115])).concat(&suffix);
                                        (((((((((((((((((capture.concat(&storage)).concat(&self::ml_string::Text::from_units(vec![105, 102, 32]))).concat(&field)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 111, 110, 101, 40, 41, 32, 123, 32, 108, 101, 116, 32]))).concat(&next)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![59, 32]))).concat(&order)).concat(&self::ml_string::Text::from_units(vec![46, 105, 110, 115, 101, 114, 116, 40]))).concat(&next)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![46, 48, 41, 59, 32, 125, 32]))).concat(&field)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 83, 111, 109, 101, 40, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![44, 41, 41, 59, 32, 40, 41, 32, 125]))
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

pub fn read_record_field_write_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![65, 110, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32, 101, 118, 97, 108, 117, 97, 116, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 111, 110, 99, 101, 44, 32, 114, 101, 97, 100, 115, 32, 97, 32, 99, 111, 109, 112, 111, 117, 110, 100, 32, 111, 112, 101, 114, 97, 110, 100, 32, 98, 101, 102, 111, 114, 101, 32, 116, 104, 101, 32, 114, 105, 103, 104, 116, 45, 104, 97, 110, 100, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 44, 32, 101, 118, 97, 108, 117, 97, 116, 101, 115, 32, 116, 104, 97, 116, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 111, 110, 99, 101, 44, 32, 116, 104, 101, 110, 32, 119, 114, 105, 116, 101, 115, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 102, 105, 101, 108, 100, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 97, 102, 116, 101, 114, 32, 114, 101, 108, 101, 97, 115, 105, 110, 103, 32, 101, 118, 101, 114, 121, 32, 101, 97, 114, 108, 105, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 111, 114, 114, 111, 119, 59, 32, 105, 116, 115, 32, 115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 114, 101, 115, 117, 108, 116, 32, 105, 115, 32, 85, 110, 105, 116, 46])
}

pub fn read_java_script_class_owner_model() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 111, 119, 110, 101, 114, 115, 32, 49])
}

pub fn read_java_script_class_owner_inference(left: self::ml_string::Text, right: self::ml_string::Text, left_open: bool, right_open: bool) -> self::ml_string::Text {
    if (left == right) {
        self::ml_string::Text::from_units(vec![115, 97, 109, 101])
    } else {
        if (((left != (self::ml_string::Text::from_units(vec![]))) && (right == (self::ml_string::Text::from_units(vec![])))) && right_open) {
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116])
        } else {
            if (((right != (self::ml_string::Text::from_units(vec![]))) && (left == (self::ml_string::Text::from_units(vec![])))) && left_open) {
                self::ml_string::Text::from_units(vec![108, 101, 102, 116])
            } else {
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
            }
        }
    }
}

pub fn read_java_script_class_record_owner(constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, fields: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> self::ml_string::Text {
    {
        let owner = self::ml_string::Text::from_units(vec![]);
        {
            let at = 0f64;
            {
                let source_ml_source2_s26 = self::source_ml_source1_temporary_read_java_script_class_record_owner_loop16(constructors.clone(), brands.clone(), fields.clone(), owner.clone(), at);
                match source_ml_source2_s26.clone() {
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(owner_2) => {
                        owner_2.clone()
                    }
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(source_ml_source1_temporary_result16) => {
                        source_ml_source1_temporary_result16.clone()
                    }
                }
            }
        }
    }
}

pub fn accept_java_script_class_owner_observation(representation: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((representation == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]))) && (operation == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121]))))
}

pub fn word(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, name: self::ml_string::Text) -> self::ml_string::Text {
    {
        let value = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, name.clone());
        if ((value >= (0f64)) && ((self::dependency_function2(forms.clone(), value)) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
            self::dependency_function2(texts.clone(), value)
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn exact(starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, names_2: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> bool {
    if ((node < (0f64)) || ((self::dependency_function1(counts.clone(), node)) != ({ let ml_reference_array17 = names_2.clone(); let ml_reference_projection17 = ml_reference_array17.borrow().len() as f64; ml_reference_projection17 }))) {
        false
    } else {
        {
            let source_ml_source1_temporary_values17 = names_2.clone();
            {
                let source_ml_source1_temporary_index17 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s27 = self::source_ml_source1_temporary_exact_loop18(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, source_ml_source1_temporary_values17.clone(), source_ml_source1_temporary_index17.clone());
                    match source_ml_source2_s27.clone() {
                        self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Done => {
                            true
                        }
                        self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Return(source_ml_source1_temporary_result18) => {
                            source_ml_source1_temporary_result18
                        }
                    }
                }
            }
        }
    }
}

pub fn strings(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        if ((node < (0f64)) || ((self::dependency_function2(forms.clone(), node)) != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![])]))
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s28 = self::source_ml_source1_temporary_strings_loop19(forms.clone(), texts.clone(), starts.clone(), counts.clone(), targets.clone(), node, result.clone(), at);
                    match source_ml_source2_s28.clone() {
                        self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Done(result_2) => {
                            result_2.clone()
                        }
                        self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Return(source_ml_source1_temporary_result19) => {
                            source_ml_source1_temporary_result19.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn names(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        {
            let fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
            if (fields < (0f64)) {
                std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![])]))
            } else {
                {
                    let at = 0f64;
                    {
                        let result_2 = self::source_ml_source1_temporary_names_loop20(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), result.clone(), fields, at);
                        result_2.clone()
                    }
                }
            }
        }
    }
}

pub fn carries(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, type_: f64, owner: self::ml_string::Text, depth: f64) -> bool {
    if ((type_ < (0f64)) || (depth > (128f64))) {
        false
    } else {
        if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))) == owner) {
            true
        } else {
            if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) != (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) {
                false
            } else {
                {
                    let elements = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115]));
                    if (elements < (0f64)) {
                        false
                    } else {
                        {
                            let at = 0f64;
                            {
                                let source_ml_source2_s29 = self::source_ml_source1_temporary_carries_loop21(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner.clone(), depth, elements, at);
                                match source_ml_source2_s29.clone() {
                                    self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Done => {
                                        false
                                    }
                                    self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Return(source_ml_source1_temporary_result21) => {
                                        source_ml_source1_temporary_result21
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

pub fn payload(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, owner: self::ml_string::Text, depth: f64) -> bool {
    if ((node < (0f64)) || (depth > (256f64))) {
        false
    } else {
        {
            let kind = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107]));
            if (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 121, 112, 101])), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))) == owner)
            } else {
                if (((kind == (self::ml_string::Text::from_units(vec![108, 101, 116]))) || (kind == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101])))) {
                    self::payload(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![98, 111, 100, 121])), owner.clone(), (depth + 1f64))
                } else {
                    if (kind == (self::ml_string::Text::from_units(vec![105, 102]))) {
                        (self::payload(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 104, 101, 110])), owner.clone(), (depth + 1f64)) && self::payload(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![101, 108, 115, 101])), owner.clone(), (depth + 1f64)))
                    } else {
                        false
                    }
                }
            }
        }
    }
}

pub fn fresh(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, owner: self::ml_string::Text, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, depth: f64) -> f64 {
    if ((node < (0f64)) || (depth > (256f64))) {
        (-1f64)
    } else {
        {
            let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
            if !(self::carries(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, owner.clone(), 0f64)) {
                (-2f64)
            } else {
                {
                    let kind = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107]));
                    if (kind == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 65, 108, 108, 111, 99, 97, 116, 101]))) {
                        {
                            let reference = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]));
                            if ((((reference >= (0f64)) && ((self::dependency_function2(forms.clone(), reference)) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) && ((self::dependency_function2(texts.clone(), reference)) == (self::ml_string::Text::from_units(vec![116, 114, 117, 101])))) && self::payload(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])), owner.clone(), 0f64)) {
                                node
                            } else {
                                (-1f64)
                            }
                        }
                    } else {
                        if (kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))) {
                            {
                                let name = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                                {
                                    let at = ({ let ml_reference_array18 = bindings.clone(); let ml_reference_projection18 = ml_reference_array18.borrow().len() as f64; ml_reference_projection18 } - 1f64);
                                    {
                                        let source_ml_source2_s30 = self::source_ml_source1_temporary_fresh_loop22(bindings.clone(), values.clone(), name.clone(), at);
                                        match source_ml_source2_s30.clone() {
                                            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Done => {
                                                (-1f64)
                                            }
                                            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Return(source_ml_source1_temporary_result22) => {
                                                source_ml_source1_temporary_result22
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if (kind == (self::ml_string::Text::from_units(vec![108, 101, 116]))) {
                                {
                                    let value = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]));
                                    {
                                        let origin = self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, owner.clone(), bindings.clone(), values.clone(), (depth + 1f64));
                                        self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![98, 111, 100, 121])), owner.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value31 = bindings.clone();
                                            {
                                                let source_ml_source2_concat_value32 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))]));
                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value19 = source_ml_source2_concat_value31.borrow().clone(); ml_reference_value19 }), { let ml_reference_value20 = source_ml_source2_concat_value32.borrow().clone(); ml_reference_value20 })
                                            }
                                        })), std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value33 = values.clone();
                                            {
                                                let source_ml_source2_concat_value34 = std::rc::Rc::new(std::cell::RefCell::new(vec![origin]));
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value21 = source_ml_source2_concat_value33.borrow().clone(); ml_reference_value21 }), { let ml_reference_value22 = source_ml_source2_concat_value34.borrow().clone(); ml_reference_value22 })
                                            }
                                        })), (depth + 1f64))
                                    }
                                }
                            } else {
                                if ((kind == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101])))) {
                                    self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![98, 111, 100, 121])), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64))
                                } else {
                                    if (kind == (self::ml_string::Text::from_units(vec![105, 102]))) {
                                        {
                                            let left = self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 104, 101, 110])), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64));
                                            {
                                                let right = self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![101, 108, 115, 101])), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64));
                                                if ((left >= (0f64)) && (left == right)) {
                                                    left
                                                } else {
                                                    (-1f64)
                                                }
                                            }
                                        }
                                    } else {
                                        if (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) {
                                            {
                                                let origin_2 = (-2f64);
                                                {
                                                    let items = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![105, 116, 101, 109, 115]));
                                                    if (items < (0f64)) {
                                                        (-1f64)
                                                    } else {
                                                        {
                                                            let at_2 = 0f64;
                                                            {
                                                                let source_ml_source2_s35 = self::source_ml_source1_temporary_fresh_loop23(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner.clone(), bindings.clone(), values.clone(), depth, origin_2, items, at_2);
                                                                match source_ml_source2_s35.clone() {
                                                                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Done(origin_3) => {
                                                                        origin_3
                                                                    }
                                                                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Return(source_ml_source1_temporary_result23) => {
                                                                        source_ml_source1_temporary_result23
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101, 73, 110, 100, 101, 120]))) {
                                                self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64))
                                            } else {
                                                (-1f64)
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
