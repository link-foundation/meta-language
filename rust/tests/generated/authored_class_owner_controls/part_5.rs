pub fn read_java_script_class_owner_error(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, type_keys: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, root: f64) -> self::ml_string::Text {
    if ((((((({ let ml_reference_array23 = starts.clone(); let ml_reference_projection23 = ml_reference_array23.borrow().len() as f64; ml_reference_projection23 }) > (1048576f64)) || (({ let ml_reference_array24 = starts.clone(); let ml_reference_projection24 = ml_reference_array24.borrow().len() as f64; ml_reference_projection24 }) != ({ let ml_reference_array25 = forms.clone(); let ml_reference_projection25 = ml_reference_array25.borrow().len() as f64; ml_reference_projection25 }))) || (({ let ml_reference_array26 = starts.clone(); let ml_reference_projection26 = ml_reference_array26.borrow().len() as f64; ml_reference_projection26 }) != ({ let ml_reference_array27 = texts.clone(); let ml_reference_projection27 = ml_reference_array27.borrow().len() as f64; ml_reference_projection27 }))) || (({ let ml_reference_array28 = starts.clone(); let ml_reference_projection28 = ml_reference_array28.borrow().len() as f64; ml_reference_projection28 }) != ({ let ml_reference_array29 = counts.clone(); let ml_reference_projection29 = ml_reference_array29.borrow().len() as f64; ml_reference_projection29 }))) || (({ let ml_reference_array30 = starts.clone(); let ml_reference_projection30 = ml_reference_array30.borrow().len() as f64; ml_reference_projection30 }) != ({ let ml_reference_array31 = type_keys.clone(); let ml_reference_projection31 = ml_reference_array31.borrow().len() as f64; ml_reference_projection31 }))) || (({ let ml_reference_array32 = labels.clone(); let ml_reference_projection32 = ml_reference_array32.borrow().len() as f64; ml_reference_projection32 }) != ({ let ml_reference_array33 = targets.clone(); let ml_reference_projection33 = ml_reference_array33.borrow().len() as f64; ml_reference_projection33 }))) {
        self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 114, 97, 112, 104, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let witness = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), root, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114, 115]));
            {
                let declarations = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), root, self::ml_string::Text::from_units(vec![100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115]));
                {
                    let constructors = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                    {
                        let brands = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                        {
                            let representations = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                            {
                                let owner_nodes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let constructor_nodes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source2_s37 = if (witness >= (0f64)) {
                                            if ((((((((self::dependency_function2(forms.clone(), witness)) != (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) || !(self::exact(starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 99, 104, 101, 109, 97, 86, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![108, 97, 110, 103, 117, 97, 103, 101]), self::ml_string::Text::from_units(vec![109, 111, 100, 101, 108]), self::ml_string::Text::from_units(vec![111, 119, 110, 101, 114, 115])]))))) || ((self::dependency_function2(forms.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![115, 99, 104, 101, 109, 97, 86, 101, 114, 115, 105, 111, 110])))) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || ((self::dependency_function2(texts.clone(), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![115, 99, 104, 101, 109, 97, 86, 101, 114, 115, 105, 111, 110])))) != (self::ml_string::Text::from_units(vec![49])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![108, 97, 110, 103, 117, 97, 103, 101]))) != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), root, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 76, 97, 110, 103, 117, 97, 103, 101]))) != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![109, 111, 100, 101, 108]))) != (self::read_java_script_class_owner_model()))) {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text::from_units(vec![117, 110, 114, 101, 99, 111, 103, 110, 105, 122, 101, 100, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 119, 105, 116, 110, 101, 115, 115, 32, 104, 101, 97, 100, 101, 114]))
                                            } else {
                                                {
                                                    let owners = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), witness, self::ml_string::Text::from_units(vec![111, 119, 110, 101, 114, 115]));
                                                    if (((((owners < (0f64)) || ((self::dependency_function2(forms.clone(), owners)) != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) || ((self::dependency_function1(counts.clone(), owners)) < (1f64))) || ((self::dependency_function1(counts.clone(), owners)) > (64f64))) || (declarations < (0f64))) {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 97, 114, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100]))
                                                    } else {
                                                        {
                                                            let at = 0f64;
                                                            {
                                                                let source_ml_source2_s38 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop46(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors.clone(), brands.clone(), representations.clone(), owner_nodes.clone(), constructor_nodes.clone(), owners, at);
                                                                match source_ml_source2_s38.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Done(constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors_2.clone(), brands_2.clone(), representations_2.clone(), owner_nodes_2.clone(), constructor_nodes_2.clone())
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(source_ml_source1_temporary_result46) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(source_ml_source1_temporary_result46.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors.clone(), brands.clone(), representations.clone(), owner_nodes.clone(), constructor_nodes.clone())
                                        };
                                        match source_ml_source2_s37.clone() {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors_3, brands_3, representations_3, owner_nodes_3, constructor_nodes_3) => {
                                                {
                                                    let node = 0f64;
                                                    {
                                                        let source_ml_source2_s39 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop28(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_keys.clone(), constructors_3.clone(), brands_3.clone(), representations_3.clone(), owner_nodes_3.clone(), constructor_nodes_3.clone(), node);
                                                        match source_ml_source2_s39.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done => {
                                                                if (({ let ml_reference_array34 = constructors_3.clone(); let ml_reference_projection34 = ml_reference_array34.borrow().len() as f64; ml_reference_projection34 }) == (0f64)) {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                } else {
                                                                    {
                                                                        let roots = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), root, self::ml_string::Text::from_units(vec![109, 97, 105, 110]))]));
                                                                        {
                                                                            let scopes = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![])]));
                                                                            {
                                                                                let declaration = 0f64;
                                                                                {
                                                                                    let source_ml_source2_s40 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop29(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, roots.clone(), scopes.clone(), declaration);
                                                                                    match source_ml_source2_s40.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(roots_2, scopes_2) => {
                                                                                            {
                                                                                                let scope = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s41 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop30(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), constructors_3.clone(), brands_3.clone(), representations_3.clone(), owner_nodes_3.clone(), roots_2.clone(), scopes_2.clone(), scope);
                                                                                                    match source_ml_source2_s41.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done => {
                                                                                                            {
                                                                                                                let node_2 = 0f64;
                                                                                                                {
                                                                                                                    let source_ml_source2_s42 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop31(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors_3.clone(), brands_3.clone(), representations_3.clone(), node_2);
                                                                                                                    match source_ml_source2_s42.clone() {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done => {
                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(source_ml_source1_temporary_result31) => {
                                                                                                                            source_ml_source1_temporary_result31.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(source_ml_source1_temporary_result37) => {
                                                                                                            source_ml_source1_temporary_result37.clone()
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
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result41) => {
                                                                source_ml_source1_temporary_result41.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(source_ml_source1_temporary_result45) => {
                                                source_ml_source1_temporary_result45.clone()
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction3Loop1Result {
    MlSource1TemporaryDependencyFunction3Loop1Done,
    MlSource1TemporaryDependencyFunction3Loop1Return(f64),
}

pub fn source_ml_source1_temporary_dependency_function3_loop1(mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut field: self::ml_string::Text, mut start: f64, mut count: f64, mut at: f64) -> self::MlSource1TemporaryDependencyFunction3Loop1Result {
    loop {
        return if (at < ((start + count))) {
            if ((self::dependency_function2(labels.clone(), at)) == field) {
                self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Return(self::dependency_function1(targets.clone(), at))
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done,
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(bool),
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut fields: f64, mut at: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), fields))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), fields, at);
                {
                    let name = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                    if (name < (0f64)) {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                    } else {
                        if self::ml_array::includes(&{ let ml_reference_value35 = brands.borrow().clone(); ml_reference_value35 }, &self::dependency_function2(texts.clone(), name), 0f64, false, |a, b| a == b) {
                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                        } else {
                            {
                                let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value43 = pending.clone();
                                    {
                                        let source_ml_source2_concat_value44 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![116, 121, 112, 101]))]));
                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value36 = source_ml_source2_concat_value43.borrow().clone(); ml_reference_value36 }), { let ml_reference_value37 = source_ml_source2_concat_value44.borrow().clone(); ml_reference_value37 })
                                    }
                                }));
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (pending, at) = (pending_2.clone(), at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending.clone())
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut children: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), children))) {
            {
                let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value45 = pending.clone();
                    {
                        let source_ml_source2_concat_value46 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), children, at)]));
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value38 = source_ml_source2_concat_value45.borrow().clone(); ml_reference_value38 }), { let ml_reference_value39 = source_ml_source2_concat_value46.borrow().clone(); ml_reference_value39 })
                    }
                }));
                {
                    let at_2 = (at + 1f64);
                    {
                        (pending, at) = (pending_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            pending.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(bool),
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut fields: f64, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < (self::dependency_function1(counts.clone(), fields))) {
            {
                let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value47 = pending.clone();
                    {
                        let source_ml_source2_concat_value48 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), fields, index), self::ml_string::Text::from_units(vec![116, 121, 112, 101]))]));
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value40 = source_ml_source2_concat_value47.borrow().clone(); ml_reference_value40 }), { let ml_reference_value41 = source_ml_source2_concat_value48.borrow().clone(); ml_reference_value41 })
                    }
                }));
                {
                    let index_2 = (index + 1f64);
                    {
                        (pending, index) = (pending_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            pending.clone()
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: f64, mut ctor: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    loop {
        return if (ctor < (self::dependency_function1(counts.clone(), constructors))) {
            {
                let fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), constructors, ctor), self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                if (fields < (0f64)) {
                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(true)
                } else {
                    {
                        let index = 0f64;
                        {
                            let pending_2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending.clone(), fields, index);
                            {
                                let ctor_2 = (ctor + 1f64);
                                {
                                    (pending, ctor) = (pending_2.clone(), ctor_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending.clone())
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut name: f64, mut found: bool, mut at: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), declarations))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, at);
                {
                    let full_name = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101]));
                    if ((full_name < (0f64)) || ((self::dependency_function2(texts.clone(), full_name)) != (self::dependency_function2(texts.clone(), name)))) {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let constructors = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![99, 116, 111, 114, 115]));
                            if (constructors < (0f64)) {
                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(true)
                            } else {
                                {
                                    let found_2 = true;
                                    {
                                        let ctor = 0f64;
                                        {
                                            let source_ml_source2_s49 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending.clone(), constructors, ctor);
                                            match source_ml_source2_s49.clone() {
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending_2) => {
                                                    {
                                                        let at_3 = (at + 1f64);
                                                        {
                                                            (pending, found, at) = (pending_2.clone(), found_2, at_3);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(source_ml_source1_temporary_result6) => {
                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result6)
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
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending.clone(), found)
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    loop {
        return if { let ml_value = { let ml_reference_array43 = pending.clone(); let ml_reference_projection43 = ml_reference_array43.borrow().len() as f64; ml_reference_projection43 }; ml_value != 0f64 && !ml_value.is_nan() } {
            if ((({ let ml_reference_array44 = seen.clone(); let ml_reference_projection44 = ml_reference_array44.borrow().len() as f64; ml_reference_projection44 }) >= (100000f64)) || (({ let ml_reference_array45 = pending.clone(); let ml_reference_projection45 = ml_reference_array45.borrow().len() as f64; ml_reference_projection45 }) > (100000f64))) {
                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
            } else {
                {
                    let node = self::dependency_function1(pending.clone(), ({ let ml_reference_array46 = pending.clone(); let ml_reference_projection46 = ml_reference_array46.borrow().len() as f64; ml_reference_projection46 } - 1f64));
                    {
                        let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value47 = pending.borrow().clone(); ml_reference_value47 }, 0f64, (-1f64))));
                        if ((node < (0f64)) || (node >= ({ let ml_reference_array48 = starts.clone(); let ml_reference_projection48 = ml_reference_array48.borrow().len() as f64; ml_reference_projection48 }))) {
                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                        } else {
                            if self::ml_array::includes(&{ let ml_reference_value49 = seen.borrow().clone(); ml_reference_value49 }, &node, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                                {
                                    pending = pending_2.clone();
                                    continue;
                                }
                            } else {
                                {
                                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value50 = seen.clone();
                                        {
                                            let source_ml_source2_concat_value51 = std::rc::Rc::new(std::cell::RefCell::new(vec![node]));
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value50 = source_ml_source2_concat_value50.borrow().clone(); ml_reference_value50 }), { let ml_reference_value51 = source_ml_source2_concat_value51.borrow().clone(); ml_reference_value51 })
                                        }
                                    }));
                                    {
                                        let owner = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107, 105, 110, 100]));
                                        if (owner < (0f64)) {
                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                        } else {
                                            {
                                                let kind = self::dependency_function2(texts.clone(), owner);
                                                if (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                                                    {
                                                        let fields = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                        if (fields < (0f64)) {
                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                        } else {
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let source_ml_source2_s52 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), brands.clone(), pending_2.clone(), fields, at);
                                                                    match source_ml_source2_s52.clone() {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending_3) => {
                                                                            {
                                                                                (pending, seen) = (pending_3.clone(), seen_2.clone());
                                                                                continue;
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(source_ml_source1_temporary_result3) => {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if self::ml_array::includes(&{ let ml_reference_value52 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![99, 101, 108, 108])]))).borrow().clone(); ml_reference_value52 }, &kind.clone(), 0f64, false, |a, b| a == b) {
                                                        {
                                                            let pending_4 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source2_concat_value53 = pending_2.clone();
                                                                {
                                                                    let source_ml_source2_concat_value54 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116]))]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value53 = source_ml_source2_concat_value53.borrow().clone(); ml_reference_value53 }), { let ml_reference_value54 = source_ml_source2_concat_value54.borrow().clone(); ml_reference_value54 })
                                                                }
                                                            }));
                                                            {
                                                                (pending, seen) = (pending_4.clone(), seen_2.clone());
                                                                continue;
                                                            }
                                                        }
                                                    } else {
                                                        if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                            {
                                                                let pending_5 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value55 = pending_2.clone();
                                                                    {
                                                                        let source_ml_source2_concat_value56 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107, 101, 121])), self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))]));
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value55 = source_ml_source2_concat_value55.borrow().clone(); ml_reference_value55 }), { let ml_reference_value56 = source_ml_source2_concat_value56.borrow().clone(); ml_reference_value56 })
                                                                    }
                                                                }));
                                                                {
                                                                    (pending, seen) = (pending_5.clone(), seen_2.clone());
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            if (((kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110])))) || (kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) {
                                                                {
                                                                    let children = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, if (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) {
                                                                        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115])
                                                                    } else {
                                                                        if (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110]))) {
                                                                            self::ml_string::Text::from_units(vec![97, 108, 116, 101, 114, 110, 97, 116, 105, 118, 101, 115])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115])
                                                                        }
                                                                    });
                                                                    if (children < (0f64)) {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let pending_6 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(starts.clone(), counts.clone(), targets.clone(), pending_2.clone(), children, at_2);
                                                                                if (kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                                                    {
                                                                                        let pending_7 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value57 = pending_6.clone();
                                                                                            {
                                                                                                let source_ml_source2_concat_value58 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![114, 101, 116]))]));
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value57 = source_ml_source2_concat_value57.borrow().clone(); ml_reference_value57 }), { let ml_reference_value58 = source_ml_source2_concat_value58.borrow().clone(); ml_reference_value58 })
                                                                                            }
                                                                                        }));
                                                                                        {
                                                                                            (pending, seen) = (pending_7.clone(), seen_2.clone());
                                                                                            continue;
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        (pending, seen) = (pending_6.clone(), seen_2.clone());
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                                                                    {
                                                                        let name = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                                                                        if ((name < (0f64)) || (declarations < (0f64))) {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                        } else {
                                                                            {
                                                                                let found = false;
                                                                                {
                                                                                    let at_3 = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s59 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, pending_2.clone(), name, found, at_3);
                                                                                        match source_ml_source2_s59.clone() {
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending_8, found_2) => {
                                                                                                if !found_2 {
                                                                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                                                } else {
                                                                                                    {
                                                                                                        (pending, seen) = (pending_8.clone(), seen_2.clone());
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result5) => {
                                                                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result5)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if !(self::ml_array::includes(&{ let ml_reference_value59 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![117, 110, 105, 116]), self::ml_string::Text::from_units(vec![98, 111, 111, 108]), self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]), self::ml_string::Text::from_units(vec![105, 110, 116]), self::ml_string::Text::from_units(vec![110, 97, 116]), self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value59 }, &kind.clone(), 0f64, false, |a, b| a == b)) {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            (pending, seen) = (pending_2.clone(), seen_2.clone());
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
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(bool),
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(bool),
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut params: f64, mut at: f64) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), params))) {
            {
                let parameter = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), params, at);
                {
                    let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), parameter, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                    if self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, declarations, brands.clone()) {
                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(true)
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
            }
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done
        };
    }
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut exported: self::ml_string::Text, mut found: bool, mut index: f64) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    loop {
        return if (index < (self::dependency_function1(counts.clone(), declarations))) {
            {
                let declaration = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, index);
                {
                    let name = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, self::ml_string::Text::from_units(vec![110, 97, 109, 101]));
                    if ((name < (0f64)) || ((self::dependency_function2(texts.clone(), name)) != exported)) {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let kind = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, self::ml_string::Text::from_units(vec![107]));
                            if ((kind < (0f64)) || ((self::dependency_function2(texts.clone(), kind)) != (self::ml_string::Text::from_units(vec![102, 110])))) {
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let params = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115]));
                                    {
                                        let ret = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, self::ml_string::Text::from_units(vec![114, 101, 116]));
                                        if ((found || (params < (0f64))) || self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, declarations, brands.clone())) {
                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(true)
                                        } else {
                                            {
                                                let found_2 = true;
                                                {
                                                    let at = 0f64;
                                                    {
                                                        let source_ml_source2_s60 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), params, at);
                                                        match source_ml_source2_s60.clone() {
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done => {
                                                                {
                                                                    let index_4 = (index + 1f64);
                                                                    {
                                                                        (found, index) = (found_2, index_4);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(source_ml_source1_temporary_result11) => {
                                                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result11)
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
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found)
        };
    }
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_values8: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index8: self::ml::Big) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    loop {
        return if (source_ml_source1_temporary_index8 < ({ let ml_reference_array60 = source_ml_source1_temporary_values8.clone(); let ml_reference_projection60 = self::ml::Big::from_u128(ml_reference_array60.borrow().len() as u128); ml_reference_projection60 })) {
            {
                let exported = {
                    let source_ml_source2_array_read_value61 = source_ml_source1_temporary_values8.clone();
                    {
                        let source_ml_source2_array_read_index62 = source_ml_source1_temporary_index8.clone();
                        { let ml_reference_array61 = source_ml_source2_array_read_value61.clone(); let ml_reference_position61 = source_ml_source2_array_read_index62.clone(); let ml_reference_projection61 = self::ml_array::at(&*ml_reference_array61.borrow(), ml_reference_position61.to_index()); ml_reference_projection61 }
                    }
                };
                {
                    let found = false;
                    {
                        let index = 0f64;
                        {
                            let source_ml_source2_s63 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), exported.clone(), found, index);
                            match source_ml_source2_s63.clone() {
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found_2) => {
                                    if !found_2 {
                                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(true)
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index8_2 = source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index8 = source_ml_source1_temporary_index8_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result10) => {
                                    self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result10)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result {
    MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done,
    MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_record_field_write_operator_loop13(mut operation: self::ml_string::Text, mut source_ml_source1_temporary_values12: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index12: self::ml::Big) -> self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index12 < ({ let ml_reference_array62 = source_ml_source1_temporary_values12.clone(); let ml_reference_projection62 = self::ml::Big::from_u128(ml_reference_array62.borrow().len() as u128); ml_reference_projection62 })) {
            {
                let operator = {
                    let source_ml_source2_array_read_value64 = source_ml_source1_temporary_values12.clone();
                    {
                        let source_ml_source2_array_read_index65 = source_ml_source1_temporary_index12.clone();
                        { let ml_reference_array63 = source_ml_source2_array_read_value64.clone(); let ml_reference_position63 = source_ml_source2_array_read_index65.clone(); let ml_reference_projection63 = self::ml_array::at(&*ml_reference_array63.borrow(), ml_reference_position63.to_index()); ml_reference_projection63 }
                    }
                };
                if operation.starts_with(&((self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])).concat(&operator)).concat(&self::ml_string::Text::from_units(vec![58]))) {
                    self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(operator.clone())
                } else {
                    {
                        let source_ml_source1_temporary_index12_2 = source_ml_source1_temporary_index12.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index12 = source_ml_source1_temporary_index12_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done
        };
    }
}

pub fn source_ml_source1_temporary_read_record_field_write_suffix_loop14(mut text: self::ml_string::Text, mut suffix: f64) -> f64 {
    loop {
        return if ((text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 111, 119, 110, 101, 114])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))) || text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix))))) || text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114, 101, 118, 105, 111, 117, 115])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix))))) {
            {
                let suffix_2 = (suffix + 1f64);
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        } else {
            suffix
        };
    }
}

pub fn source_ml_source1_temporary_read_optional_record_field_order_followers_loop15(mut numeric_keys: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut selected: f64, mut followers: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < ({ let ml_reference_array64 = numeric_keys.clone(); let ml_reference_projection64 = ml_reference_array64.borrow().len() as f64; ml_reference_projection64 })) {
            {
                let key = ({
                    let source_ml_source2_array_read_value66 = numeric_keys.clone();
                    {
                        let source_ml_source2_array_read_index67 = index;
                        { let ml_reference_array65 = source_ml_source2_array_read_value66.clone(); let ml_reference_position65 = source_ml_source2_array_read_index67; let ml_reference_projection65 = self::ml_array::optional_at(&*ml_reference_array65.borrow(), self::ml_array::number_index(ml_reference_position65)); ml_reference_projection65 }
                    }
                }).clone().unwrap_or_else(|| (-1f64));
                if ((key < (0f64)) || (key > selected)) {
                    {
                        let followers_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value66 = followers.borrow().clone(); ml_reference_value66 }, vec![index])));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (followers, index) = (followers_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            followers.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result {
    MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(self::ml_string::Text),
    MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_record_owner_loop16(mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut fields: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result {
    loop {
        return if (at < ({ let ml_reference_array67 = brands.clone(); let ml_reference_projection67 = ml_reference_array67.borrow().len() as f64; ml_reference_projection67 })) {
            if self::ml_array::includes(&{ let ml_reference_value68 = fields.borrow().clone(); ml_reference_value68 }, &self::dependency_function2(brands.clone(), at), 0f64, false, |a, b| a == b) {
                if (owner != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(self::ml_string::Text::from_units(vec![35, 97, 109, 98, 105, 103, 117, 111, 117, 115]))
                } else {
                    {
                        let owner_2 = self::dependency_function2(constructors.clone(), at);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (owner, at) = (owner_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(owner.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExactLoop18Result {
    MlSource1TemporaryExactLoop18Done,
    MlSource1TemporaryExactLoop18Return(bool),
}

pub fn source_ml_source1_temporary_exact_loop18(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut source_ml_source1_temporary_values17: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index17: self::ml::Big) -> self::MlSource1TemporaryExactLoop18Result {
    loop {
        return if (source_ml_source1_temporary_index17 < ({ let ml_reference_array69 = source_ml_source1_temporary_values17.clone(); let ml_reference_projection69 = self::ml::Big::from_u128(ml_reference_array69.borrow().len() as u128); ml_reference_projection69 })) {
            {
                let name = {
                    let source_ml_source2_array_read_value68 = source_ml_source1_temporary_values17.clone();
                    {
                        let source_ml_source2_array_read_index69 = source_ml_source1_temporary_index17.clone();
                        { let ml_reference_array70 = source_ml_source2_array_read_value68.clone(); let ml_reference_position70 = source_ml_source2_array_read_index69.clone(); let ml_reference_projection70 = self::ml_array::at(&*ml_reference_array70.borrow(), ml_reference_position70.to_index()); ml_reference_projection70 }
                    }
                };
                if ((self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, name.clone())) < (0f64)) {
                    self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Return(false)
                } else {
                    {
                        let source_ml_source1_temporary_index17_2 = source_ml_source1_temporary_index17.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index17 = source_ml_source1_temporary_index17_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStringsLoop19Result {
    MlSource1TemporaryStringsLoop19Done(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
    MlSource1TemporaryStringsLoop19Return(std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
}
