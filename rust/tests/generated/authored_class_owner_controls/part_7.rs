#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop45(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut type_keys: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut expected_fields: f64, mut actual: f64, mut name: self::ml_string::Text, mut other: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result {
    loop {
        return if (other < (self::dependency_function1(counts.clone(), expected_fields))) {
            {
                let expected = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), expected_fields, other);
                if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) == name) && ((self::dependency_function2(type_keys.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual, self::ml_string::Text::from_units(vec![116, 121, 112, 101])))) != (self::dependency_function2(type_keys.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, self::ml_string::Text::from_units(vec![116, 121, 112, 101])))))) {
                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 102, 105, 101, 108, 100, 32, 99, 104, 97, 110, 103, 101, 100, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 116, 121, 112, 101]))
                } else {
                    {
                        let other_2 = (other + 1f64);
                        {
                            other = other_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop44(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut type_keys: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut actual_fields: f64, mut expected_fields: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), actual_fields))) {
            {
                let actual = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), actual_fields, at);
                {
                    let name = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                    {
                        let other = 0f64;
                        {
                            let source_ml_source2_s106 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop45(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_keys.clone(), expected_fields, actual, name.clone(), other);
                            match source_ml_source2_s106.clone() {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done => {
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            at = at_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(source_ml_source1_temporary_result45) => {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(source_ml_source1_temporary_result45.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop43(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut position: f64, mut branded_fields: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), branded_fields))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), branded_fields, at);
                if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) == (self::dependency_function2(brands.clone(), position))) && ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![116, 121, 112, 101])), self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) != (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 98, 114, 97, 110, 100, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 85, 110, 105, 116, 32, 102, 105, 101, 108, 100]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop28(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut type_keys: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructor_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result {
    loop {
        return if (node < ({ let ml_reference_array117 = starts.clone(); let ml_reference_projection117 = ml_reference_array117.borrow().len() as f64; ml_reference_projection117 })) {
            if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                {
                    let owner = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]));
                    {
                        let fields = self::names(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node);
                        {
                            let position = self::ml_array::index_of(&{ let ml_reference_value118 = constructors.borrow().clone(); ml_reference_value118 }, &owner.clone(), 0f64, false, |a, b| a == b);
                            {
                                let present = { let ml_values119 = { let ml_reference_value120 = brands.borrow().clone(); ml_reference_value120 }; let ml_callback119 = { let fields = fields.clone(); std::rc::Rc::new(move |brand: self::ml_string::Text| -> bool { self::ml_array::includes(&{ let ml_reference_value121 = fields.borrow().clone(); ml_reference_value121 }, &brand.clone(), 0f64, false, |a, b| a == b) }) }; ml_values119.iter().cloned().enumerate().any(|(ml_index119, ml_value119)| ml_callback119(ml_value119)) };
                                if (((self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))) >= (0f64)) && (owner == (self::ml_string::Text::from_units(vec![])))) {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 116, 121, 112, 101, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32, 110, 111, 110, 101, 109, 112, 116, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
                                } else {
                                    if ((owner == (self::ml_string::Text::from_units(vec![]))) && !present) {
                                        {
                                            let node_2 = (node + 1f64);
                                            {
                                                node = node_2;
                                                continue;
                                            }
                                        }
                                    } else {
                                        if (position < (0f64)) {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 116, 121, 112, 101, 32, 99, 97, 110, 110, 111, 116, 32, 100, 114, 111, 112, 32, 105, 116, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 119, 105, 116, 110, 101, 115, 115]))
                                        } else {
                                            {
                                                let definition = self::dependency_function1(owner_nodes.clone(), position);
                                                {
                                                    let expected = std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source2_concat_value107 = self::strings(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), definition, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                        {
                                                            let source_ml_source2_concat_value108 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function2(brands.clone(), position)]));
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value122 = source_ml_source2_concat_value107.borrow().clone(); ml_reference_value122 }), { let ml_reference_value123 = source_ml_source2_concat_value108.borrow().clone(); ml_reference_value123 })
                                                        }
                                                    }));
                                                    {
                                                        let methods = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), definition, self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115]));
                                                        {
                                                            let complete = expected.clone();
                                                            {
                                                                let complete_3 = if ((self::dependency_function2(representations.clone(), position)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]))) {
                                                                    {
                                                                        let at = 0f64;
                                                                        {
                                                                            let complete_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop42(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), methods, complete.clone(), at);
                                                                            complete_2.clone()
                                                                        }
                                                                    }
                                                                } else {
                                                                    complete.clone()
                                                                };
                                                                if ((({ let ml_reference_array124 = fields.clone(); let ml_reference_projection124 = ml_reference_array124.borrow().len() as f64; ml_reference_projection124 }) != ({ let ml_reference_array125 = complete_3.clone(); let ml_reference_projection125 = ml_reference_array125.borrow().len() as f64; ml_reference_projection125 })) || { let ml_values126 = { let ml_reference_value127 = complete_3.borrow().clone(); ml_reference_value127 }; let ml_callback126 = { let fields = fields.clone(); std::rc::Rc::new(move |name: self::ml_string::Text| -> bool { !(self::ml_array::includes(&{ let ml_reference_value128 = fields.borrow().clone(); ml_reference_value128 }, &name.clone(), 0f64, false, |a, b| a == b)) }) }; ml_values126.iter().cloned().enumerate().any(|(ml_index126, ml_value126)| ml_callback126(ml_value126)) }) {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 114, 101, 99, 111, 114, 100, 32, 99, 111, 110, 116, 114, 97, 100, 105, 99, 116, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115]))
                                                                } else {
                                                                    {
                                                                        let branded_fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let source_ml_source2_s109 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop43(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), brands.clone(), position, branded_fields, at_2);
                                                                                match source_ml_source2_s109.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done => {
                                                                                        {
                                                                                            let reference = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function1(constructor_nodes.clone(), position), self::ml_string::Text::from_units(vec![114, 101, 116]));
                                                                                            if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), reference, self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                                                                                                {
                                                                                                    let actual_fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                                                                    {
                                                                                                        let expected_fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), reference, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                                                                        {
                                                                                                            let at_3 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_s110 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop44(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_keys.clone(), actual_fields, expected_fields, at_3);
                                                                                                                match source_ml_source2_s110.clone() {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done => {
                                                                                                                        {
                                                                                                                            let node_3 = (node + 1f64);
                                                                                                                            {
                                                                                                                                node = node_3;
                                                                                                                                continue;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(source_ml_source1_temporary_result44) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result44.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                {
                                                                                                    let node_4 = (node + 1f64);
                                                                                                    {
                                                                                                        node = node_4;
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(source_ml_source1_temporary_result45) => {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result45.clone())
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
                {
                    let node_5 = (node + 1f64);
                    {
                        node = node_5;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27 {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Done(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop48(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructor: self::ml_string::Text, mut actual: f64, mut declaration: f64) -> f64 {
    loop {
        return if (declaration < (self::dependency_function1(counts.clone(), declarations))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, declaration);
                if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101]))) == constructor) && ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![107]))) == (self::ml_string::Text::from_units(vec![102, 110])))) {
                    {
                        let actual_2 = row;
                        {
                            let declaration_2 = (declaration + 1f64);
                            {
                                (actual, declaration) = (actual_2, declaration_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let declaration_3 = (declaration + 1f64);
                        {
                            declaration = declaration_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            actual
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop50(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructor: self::ml_string::Text, mut helper: self::ml_string::Text, mut found: bool, mut declaration: f64) -> bool {
    loop {
        return if (declaration < (self::dependency_function1(counts.clone(), declarations))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, declaration);
                if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101]))) != helper) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![107]))) != (self::ml_string::Text::from_units(vec![102, 110])))) {
                    {
                        let declaration_2 = (declaration + 1f64);
                        {
                            declaration = declaration_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let params = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115]));
                        {
                            let parameter = if (params < (0f64)) {
                                (-1f64)
                            } else {
                                self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), params, 0f64)
                            };
                            {
                                let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), parameter, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                                if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))) == constructor) {
                                    {
                                        let found_2 = true;
                                        {
                                            let declaration_3 = (declaration + 1f64);
                                            {
                                                (found, declaration) = (found_2, declaration_3);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let declaration_4 = (declaration + 1f64);
                                        {
                                            declaration = declaration_4;
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
            found
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop49(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructor: self::ml_string::Text, mut source_ml_source1_temporary_values24: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index24: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result {
    loop {
        return if (source_ml_source1_temporary_index24 < ({ let ml_reference_array129 = source_ml_source1_temporary_values24.clone(); let ml_reference_projection129 = self::ml::Big::from_u128(ml_reference_array129.borrow().len() as u128); ml_reference_projection129 })) {
            {
                let helper = {
                    let source_ml_source2_array_read_value111 = source_ml_source1_temporary_values24.clone();
                    {
                        let source_ml_source2_array_read_index112 = source_ml_source1_temporary_index24.clone();
                        { let ml_reference_array130 = source_ml_source2_array_read_value111.clone(); let ml_reference_position130 = source_ml_source2_array_read_index112.clone(); let ml_reference_projection130 = self::ml_array::at(&*ml_reference_array130.borrow(), ml_reference_position130.to_index()); ml_reference_projection130 }
                    }
                };
                {
                    let found = false;
                    {
                        let declaration = 0f64;
                        {
                            let found_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop50(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructor.clone(), helper.clone(), found, declaration);
                            if !found_2 {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 104, 101, 108, 112, 101, 114, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index24_2 = source_ml_source1_temporary_index24.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index24 = source_ml_source1_temporary_index24_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Done(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop47(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut fields: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut methods: f64, mut method_names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut helpers: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut method: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result {
    loop {
        return if (method < (self::dependency_function1(counts.clone(), methods))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), methods, method);
                {
                    let name = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                    {
                        let helper = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114]));
                        if (((((!(self::exact(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 97, 109, 101]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114])])))) || (name == (self::ml_string::Text::from_units(vec![])))) || (helper == (self::ml_string::Text::from_units(vec![])))) || self::ml_array::includes(&{ let ml_reference_value131 = fields.borrow().clone(); ml_reference_value131 }, &name.clone(), 0f64, false, |a, b| a == b)) || self::ml_array::includes(&{ let ml_reference_value132 = method_names.borrow().clone(); ml_reference_value132 }, &name.clone(), 0f64, false, |a, b| a == b)) || self::ml_array::includes(&{ let ml_reference_value133 = helpers.borrow().clone(); ml_reference_value133 }, &helper.clone(), 0f64, false, |a, b| a == b)) {
                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 111, 119, 110, 101, 114, 115, 32, 97, 114, 101, 32, 99, 111, 110, 116, 114, 97, 100, 105, 99, 116, 111, 114, 121]))
                        } else {
                            {
                                let method_names_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value113 = method_names.clone();
                                    {
                                        let source_ml_source2_concat_value114 = std::rc::Rc::new(std::cell::RefCell::new(vec![name.clone()]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value134 = source_ml_source2_concat_value113.borrow().clone(); ml_reference_value134 }), { let ml_reference_value135 = source_ml_source2_concat_value114.borrow().clone(); ml_reference_value135 })
                                    }
                                }));
                                {
                                    let helpers_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value115 = helpers.clone();
                                        {
                                            let source_ml_source2_concat_value116 = std::rc::Rc::new(std::cell::RefCell::new(vec![helper.clone()]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value136 = source_ml_source2_concat_value115.borrow().clone(); ml_reference_value136 }), { let ml_reference_value137 = source_ml_source2_concat_value116.borrow().clone(); ml_reference_value137 })
                                        }
                                    }));
                                    {
                                        let method_2 = (method + 1f64);
                                        {
                                            (method_names, helpers, method) = (method_names_2.clone(), helpers_2.clone(), method_2);
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Done(helpers.clone())
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop46(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructor_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owners: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), owners))) {
            {
                let owner = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), owners, at);
                {
                    let constructor = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]));
                    {
                        let brand = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100]));
                        {
                            let representation = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110]));
                            if (((((!(self::exact(starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100]), self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]), self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115])])))) || (constructor == (self::ml_string::Text::from_units(vec![])))) || (brand == (self::ml_string::Text::from_units(vec![])))) || self::ml_array::includes(&{ let ml_reference_value138 = constructors.borrow().clone(); ml_reference_value138 }, &constructor.clone(), 0f64, false, |a, b| a == b)) || self::ml_array::includes(&{ let ml_reference_value139 = brands.borrow().clone(); ml_reference_value139 }, &brand.clone(), 0f64, false, |a, b| a == b)) || !(self::ml_array::includes(&{ let ml_reference_value140 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115])]))).borrow().clone(); ml_reference_value140 }, &representation.clone(), 0f64, false, |a, b| a == b))) {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 98, 114, 97, 110, 100, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                            } else {
                                {
                                    let fields = self::strings(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                    if ((((({ let ml_reference_array141 = fields.clone(); let ml_reference_projection141 = ml_reference_array141.borrow().len() as f64; ml_reference_projection141 }) > (8192f64)) || self::ml_array::includes(&{ let ml_reference_value142 = fields.borrow().clone(); ml_reference_value142 }, &self::ml_string::Text::from_units(vec![]), 0f64, false, |a, b| a == b)) || (((({ let ml_reference_value143 = (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::set({ let ml_reference_value144 = fields.borrow().clone(); ml_reference_value144 }, |a, b| a == b)))).borrow().clone(); ml_reference_value143 }).len() as f64)) != ({ let ml_reference_array145 = fields.clone(); let ml_reference_projection145 = ml_reference_array145.borrow().len() as f64; ml_reference_projection145 }))) || self::ml_array::includes(&{ let ml_reference_value146 = fields.borrow().clone(); ml_reference_value146 }, &brand.clone(), 0f64, false, |a, b| a == b)) {
                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 110, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116]))
                                    } else {
                                        {
                                            let methods = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), owner, self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115]));
                                            if (((methods < (0f64)) || ((self::dependency_function2(forms.clone(), methods)) != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) || ((self::dependency_function1(counts.clone(), methods)) > (8192f64))) {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 104, 101, 108, 112, 101, 114, 115]))
                                            } else {
                                                {
                                                    let method_names = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                    {
                                                        let helpers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                        {
                                                            let method = 0f64;
                                                            {
                                                                let source_ml_source2_s117 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop47(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), fields.clone(), methods, method_names.clone(), helpers.clone(), method);
                                                                match source_ml_source2_s117.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Done(helpers_2) => {
                                                                        {
                                                                            let actual = (-1f64);
                                                                            {
                                                                                let declaration = 0f64;
                                                                                {
                                                                                    let actual_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop48(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructor.clone(), actual, declaration);
                                                                                    if (actual_2 < (0f64)) {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))
                                                                                    } else {
                                                                                        {
                                                                                            let ret = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual_2, self::ml_string::Text::from_units(vec![114, 101, 116]));
                                                                                            {
                                                                                                let body = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual_2, self::ml_string::Text::from_units(vec![98, 111, 100, 121]));
                                                                                                {
                                                                                                    let reference = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]));
                                                                                                    if (((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, self::ml_string::Text::from_units(vec![107, 105, 110, 100]))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) && (((reference < (0f64)) || ((self::dependency_function2(forms.clone(), reference)) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || ((self::dependency_function2(texts.clone(), reference)) != (self::ml_string::Text::from_units(vec![116, 114, 117, 101]))))) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 115, 104, 97, 114, 101, 100, 32, 105, 110, 115, 116, 97, 110, 99, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
                                                                                                    } else {
                                                                                                        if (!(self::carries(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, constructor.clone(), 0f64)) || ((self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), body, constructor.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), 0f64)) < (0f64))) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 105, 116, 115, 32, 111, 119, 110, 32, 102, 114, 101, 115, 104, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                                                                                        } else {
                                                                                                            {
                                                                                                                let source_ml_source1_temporary_values24 = helpers_2.clone();
                                                                                                                {
                                                                                                                    let source_ml_source1_temporary_index24 = self::ml::Big::from_i128(0);
                                                                                                                    {
                                                                                                                        let source_ml_source2_s118 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop49(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructor.clone(), source_ml_source1_temporary_values24.clone(), source_ml_source1_temporary_index24.clone());
                                                                                                                        match source_ml_source2_s118.clone() {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Done => {
                                                                                                                                {
                                                                                                                                    let constructors_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                        let source_ml_source2_concat_value119 = constructors.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_concat_value120 = std::rc::Rc::new(std::cell::RefCell::new(vec![constructor.clone()]));
                                                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value147 = source_ml_source2_concat_value119.borrow().clone(); ml_reference_value147 }), { let ml_reference_value148 = source_ml_source2_concat_value120.borrow().clone(); ml_reference_value148 })
                                                                                                                                        }
                                                                                                                                    }));
                                                                                                                                    {
                                                                                                                                        let brands_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                            let source_ml_source2_concat_value121 = brands.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_concat_value122 = std::rc::Rc::new(std::cell::RefCell::new(vec![brand.clone()]));
                                                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value149 = source_ml_source2_concat_value121.borrow().clone(); ml_reference_value149 }), { let ml_reference_value150 = source_ml_source2_concat_value122.borrow().clone(); ml_reference_value150 })
                                                                                                                                            }
                                                                                                                                        }));
                                                                                                                                        {
                                                                                                                                            let representations_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                let source_ml_source2_concat_value123 = representations.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_concat_value124 = std::rc::Rc::new(std::cell::RefCell::new(vec![representation.clone()]));
                                                                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value151 = source_ml_source2_concat_value123.borrow().clone(); ml_reference_value151 }), { let ml_reference_value152 = source_ml_source2_concat_value124.borrow().clone(); ml_reference_value152 })
                                                                                                                                                }
                                                                                                                                            }));
                                                                                                                                            {
                                                                                                                                                let owner_nodes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                    let source_ml_source2_concat_value125 = owner_nodes.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_concat_value126 = std::rc::Rc::new(std::cell::RefCell::new(vec![owner]));
                                                                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value153 = source_ml_source2_concat_value125.borrow().clone(); ml_reference_value153 }), { let ml_reference_value154 = source_ml_source2_concat_value126.borrow().clone(); ml_reference_value154 })
                                                                                                                                                    }
                                                                                                                                                }));
                                                                                                                                                {
                                                                                                                                                    let constructor_nodes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                        let source_ml_source2_concat_value127 = constructor_nodes.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_concat_value128 = std::rc::Rc::new(std::cell::RefCell::new(vec![actual_2]));
                                                                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value155 = source_ml_source2_concat_value127.borrow().clone(); ml_reference_value155 }), { let ml_reference_value156 = source_ml_source2_concat_value128.borrow().clone(); ml_reference_value156 })
                                                                                                                                                        }
                                                                                                                                                    }));
                                                                                                                                                    {
                                                                                                                                                        let at_2 = (at + 1f64);
                                                                                                                                                        {
                                                                                                                                                            (constructors, brands, representations, owner_nodes, constructor_nodes, at) = (constructors_2.clone(), brands_2.clone(), representations_2.clone(), owner_nodes_2.clone(), constructor_nodes_2.clone(), at_2);
                                                                                                                                                            continue;
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop49Return(source_ml_source1_temporary_result49) => {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(source_ml_source1_temporary_result49.clone())
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
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop47Return(source_ml_source1_temporary_result50) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(source_ml_source1_temporary_result50.clone())
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Done(constructors.clone(), brands.clone(), representations.clone(), owner_nodes.clone(), constructor_nodes.clone())
        };
    }
}

fn ml_main() {
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121]))) == (true)), "assertion 1");
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]), self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108]))) == (false)), "assertion 2");
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121]))) == (false)), "assertion 3");
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115]), self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108]))) == (false)), "assertion 4");
    assert!(((self::accept_java_script_class_owner_observation(self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121]))) == (false)), "assertion 5");
    assert!(((self::read_java_script_class_owner_inference(self::ml_string::Text::from_units(vec![66, 111, 120]), self::ml_string::Text::from_units(vec![]), false, true)) == (self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]))), "assertion 6");
    assert!(((self::read_java_script_class_owner_inference(self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![66, 111, 120]), true, false)) == (self::ml_string::Text::from_units(vec![108, 101, 102, 116]))), "assertion 7");
    assert!(((self::read_java_script_class_owner_inference(self::ml_string::Text::from_units(vec![66, 111, 120]), self::ml_string::Text::from_units(vec![79, 116, 104, 101, 114]), false, false)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))), "assertion 8");
    assert!(((self::read_java_script_class_record_owner(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![66, 111, 120]), self::ml_string::Text::from_units(vec![79, 116, 104, 101, 114])])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49])])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])])))) == (self::ml_string::Text::from_units(vec![66, 111, 120]))), "assertion 9");
    assert!(((self::read_java_script_class_record_owner(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![66, 111, 120]), self::ml_string::Text::from_units(vec![79, 116, 104, 101, 114])])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49])])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49])])))) == (self::ml_string::Text::from_units(vec![35, 97, 109, 98, 105, 103, 117, 111, 117, 115]))), "assertion 10");
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
fn all_literal_class_owner_controls() { main(); }
