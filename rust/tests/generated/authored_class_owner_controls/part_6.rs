pub fn source_ml_source1_temporary_strings_loop19(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut at: f64) -> self::MlSource1TemporaryStringsLoop19Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), node))) {
            {
                let value = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), node, at);
                if (((self::dependency_function2(forms.clone(), value)) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || ((self::dependency_function2(texts.clone(), value)) == (self::ml_string::Text::from_units(vec![])))) {
                    self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Return(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![])])))
                } else {
                    {
                        let result_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value70 = result.clone();
                            {
                                let source_ml_source2_concat_value71 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function2(texts.clone(), value)]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value71 = source_ml_source2_concat_value70.borrow().clone(); ml_reference_value71 }), { let ml_reference_value72 = source_ml_source2_concat_value71.borrow().clone(); ml_reference_value72 })
                            }
                        }));
                        {
                            let at_2 = (at + 1f64);
                            {
                                (result, at) = (result_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Done(result.clone())
        };
    }
}

pub fn source_ml_source1_temporary_names_loop20(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut fields: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), fields))) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value72 = result.clone();
                    {
                        let source_ml_source2_concat_value73 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), fields, at), self::ml_string::Text::from_units(vec![110, 97, 109, 101]))]));
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value73 = source_ml_source2_concat_value72.borrow().clone(); ml_reference_value73 }), { let ml_reference_value74 = source_ml_source2_concat_value73.borrow().clone(); ml_reference_value74 })
                    }
                }));
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCarriesLoop21Result {
    MlSource1TemporaryCarriesLoop21Done,
    MlSource1TemporaryCarriesLoop21Return(bool),
}

pub fn source_ml_source1_temporary_carries_loop21(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owner: self::ml_string::Text, mut depth: f64, mut elements: f64, mut at: f64) -> self::MlSource1TemporaryCarriesLoop21Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), elements))) {
            if self::carries(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), elements, at), owner.clone(), (depth + 1f64)) {
                self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Return(true)
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
            self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFreshLoop22Result {
    MlSource1TemporaryFreshLoop22Done,
    MlSource1TemporaryFreshLoop22Return(f64),
}

pub fn source_ml_source1_temporary_fresh_loop22(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut name: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryFreshLoop22Result {
    loop {
        return if (at >= (0f64)) {
            if ((self::dependency_function2(bindings.clone(), at)) == name) {
                self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Return(self::dependency_function1(values.clone(), at))
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFreshLoop23Result {
    MlSource1TemporaryFreshLoop23Done(f64),
    MlSource1TemporaryFreshLoop23Return(f64),
}

pub fn source_ml_source1_temporary_fresh_loop23(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owner: self::ml_string::Text, mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut depth: f64, mut origin: f64, mut items: f64, mut at: f64) -> self::MlSource1TemporaryFreshLoop23Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), items))) {
            {
                let value = self::fresh(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), items, at), owner.clone(), bindings.clone(), values.clone(), (depth + 1f64));
                if ((value == ((-1f64))) || (((value >= (0f64)) && (origin >= (0f64))) && (value != origin))) {
                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Return((-1f64))
                } else {
                    if (value >= (0f64)) {
                        {
                            let origin_2 = value;
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (origin, at) = (origin_2, at_2);
                                    continue;
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
                }
            }
        } else {
            self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Done(origin)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop33(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut operand: f64, mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), operand))) {
            {
                let values_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value74 = values.clone();
                    {
                        let source_ml_source2_concat_value75 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), operand, at)]));
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value75 = source_ml_source2_concat_value74.borrow().clone(); ml_reference_value75 }), { let ml_reference_value76 = source_ml_source2_concat_value75.borrow().clone(); ml_reference_value76 })
                    }
                }));
                {
                    let at_2 = (at + 1f64);
                    {
                        (values, at) = (values_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            values.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop37(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut actual: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ((self::dependency_function1(starts.clone(), actual) + self::dependency_function1(counts.clone(), actual)))) {
            if !(self::ml_array::includes(&{ let ml_reference_value77 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 112, 97, 110]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114])]))).borrow().clone(); ml_reference_value77 }, &self::dependency_function2(labels.clone(), at), 0f64, false, |a, b| a == b)) {
                {
                    let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value76 = pending.clone();
                        {
                            let source_ml_source2_concat_value77 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function1(targets.clone(), at)]));
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value78 = source_ml_source2_concat_value76.borrow().clone(); ml_reference_value78 }), { let ml_reference_value79 = source_ml_source2_concat_value77.borrow().clone(); ml_reference_value79 })
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
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            pending.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop36(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut identity: bool, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result {
    loop {
        return if { let ml_value = { let ml_reference_array81 = pending.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }; ml_value != 0f64 && !ml_value.is_nan() } {
            {
                let actual = self::dependency_function1(pending.clone(), ({ let ml_reference_array82 = pending.clone(); let ml_reference_projection82 = ml_reference_array82.borrow().len() as f64; ml_reference_projection82 } - 1f64));
                {
                    let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value83 = pending.borrow().clone(); ml_reference_value83 }, 0f64, (-1f64))));
                    if ((actual < (0f64)) || self::ml_array::includes(&{ let ml_reference_value84 = seen.borrow().clone(); ml_reference_value84 }, &actual, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                        {
                            pending = pending_2.clone();
                            continue;
                        }
                    } else {
                        if (({ let ml_reference_array85 = seen.clone(); let ml_reference_projection85 = ml_reference_array85.borrow().len() as f64; ml_reference_projection85 }) > (100000f64)) {
                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let seen_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value78 = seen.clone();
                                    {
                                        let source_ml_source2_concat_value79 = std::rc::Rc::new(std::cell::RefCell::new(vec![actual]));
                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value86 = source_ml_source2_concat_value78.borrow().clone(); ml_reference_value86 }), { let ml_reference_value87 = source_ml_source2_concat_value79.borrow().clone(); ml_reference_value87 })
                                    }
                                }));
                                {
                                    let owner = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), actual, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]));
                                    {
                                        let position = self::ml_array::index_of(&{ let ml_reference_value88 = constructors.borrow().clone(); ml_reference_value88 }, &owner.clone(), 0f64, false, |a, b| a == b);
                                        if ((position >= (0f64)) && !(self::accept_java_script_class_owner_observation(self::dependency_function2(representations.clone(), position), if identity {
                                            self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
                                        } else {
                                            self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108])
                                        }))) {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 97, 100, 109, 105, 116, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 100, 101, 98, 117, 103, 32, 111, 114, 32, 112, 114, 111, 111, 102, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110]))
                                        } else {
                                            {
                                                let at = self::dependency_function1(starts.clone(), actual);
                                                {
                                                    let pending_3 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop37(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending_2.clone(), actual, at);
                                                    {
                                                        (pending, seen) = (pending_3.clone(), seen_2.clone());
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
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop35(mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut identity: bool, mut type_: f64, mut owner: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result {
    loop {
        return if (owner < ({ let ml_reference_array89 = constructors.clone(); let ml_reference_projection89 = ml_reference_array89.borrow().len() as f64; ml_reference_projection89 })) {
            if (!(self::accept_java_script_class_owner_observation(self::dependency_function2(representations.clone(), owner), if identity {
                self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
            } else {
                self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108])
            })) && self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, declarations, std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function2(brands.clone(), owner)])))) {
                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 97, 100, 109, 105, 116, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 100, 101, 98, 117, 103, 32, 111, 114, 32, 112, 114, 111, 111, 102, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110]))
            } else {
                {
                    let owner_2 = (owner + 1f64);
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop34(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut identity: bool, mut source_ml_source1_temporary_values26: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index26: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result {
    loop {
        return if (source_ml_source1_temporary_index26 < ({ let ml_reference_array90 = source_ml_source1_temporary_values26.clone(); let ml_reference_projection90 = self::ml::Big::from_u128(ml_reference_array90.borrow().len() as u128); ml_reference_projection90 })) {
            {
                let value = {
                    let source_ml_source2_array_read_value80 = source_ml_source1_temporary_values26.clone();
                    {
                        let source_ml_source2_array_read_index81 = source_ml_source1_temporary_index26.clone();
                        { let ml_reference_array91 = source_ml_source2_array_read_value80.clone(); let ml_reference_position91 = source_ml_source2_array_read_index81.clone(); let ml_reference_projection91 = self::ml_array::at(&*ml_reference_array91.borrow(), ml_reference_position91.to_index()); ml_reference_projection91 }
                    }
                };
                {
                    let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                    if (type_ < (0f64)) {
                        {
                            let source_ml_source1_temporary_index26_2 = source_ml_source1_temporary_index26.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index26 = source_ml_source1_temporary_index26_2.clone();
                                continue;
                            }
                        }
                    } else {
                        {
                            let owner = 0f64;
                            {
                                let source_ml_source2_s82 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop35(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors.clone(), brands.clone(), representations.clone(), identity, type_, owner);
                                match source_ml_source2_s82.clone() {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done => {
                                        {
                                            let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![type_]));
                                            {
                                                let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                {
                                                    let source_ml_source2_s83 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop36(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), constructors.clone(), representations.clone(), identity, pending.clone(), seen.clone());
                                                    match source_ml_source2_s83.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done => {
                                                            {
                                                                let source_ml_source1_temporary_index26_3 = source_ml_source1_temporary_index26.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index26 = source_ml_source1_temporary_index26_3.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(source_ml_source1_temporary_result36) => {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result36.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(source_ml_source1_temporary_result37) => {
                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result37.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop32(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut node: f64, mut identity: bool, mut source_ml_source1_temporary_values25: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index25: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result {
    loop {
        return if (source_ml_source1_temporary_index25 < ({ let ml_reference_array92 = source_ml_source1_temporary_values25.clone(); let ml_reference_projection92 = self::ml::Big::from_u128(ml_reference_array92.borrow().len() as u128); ml_reference_projection92 })) {
            {
                let key = {
                    let source_ml_source2_array_read_value84 = source_ml_source1_temporary_values25.clone();
                    {
                        let source_ml_source2_array_read_index85 = source_ml_source1_temporary_index25.clone();
                        { let ml_reference_array93 = source_ml_source2_array_read_value84.clone(); let ml_reference_position93 = source_ml_source2_array_read_index85.clone(); let ml_reference_projection93 = self::ml_array::at(&*ml_reference_array93.borrow(), ml_reference_position93.to_index()); ml_reference_projection93 }
                    }
                };
                if !key.0.is_empty() {
                    {
                        let operand = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, key.clone());
                        if (operand < (0f64)) {
                            {
                                let source_ml_source1_temporary_index25_2 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index25 = source_ml_source1_temporary_index25_2.clone();
                                    continue;
                                }
                            }
                        } else {
                            {
                                let values = if ((self::dependency_function2(forms.clone(), operand)) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![operand]))
                                };
                                {
                                    let values_3 = if ((self::dependency_function2(forms.clone(), operand)) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                                        {
                                            let at = 0f64;
                                            {
                                                let values_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop33(starts.clone(), counts.clone(), targets.clone(), operand, values.clone(), at);
                                                values_2.clone()
                                            }
                                        }
                                    } else {
                                        values.clone()
                                    };
                                    {
                                        let source_ml_source1_temporary_values26 = values_3.clone();
                                        {
                                            let source_ml_source1_temporary_index26 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s86 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop34(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors.clone(), brands.clone(), representations.clone(), identity, source_ml_source1_temporary_values26.clone(), source_ml_source1_temporary_index26.clone());
                                                match source_ml_source2_s86.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done => {
                                                        {
                                                            let source_ml_source1_temporary_index25_3 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                source_ml_source1_temporary_index25 = source_ml_source1_temporary_index25_3.clone();
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result34) => {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(source_ml_source1_temporary_result34.clone())
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
                        let source_ml_source1_temporary_index25_4 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index25 = source_ml_source1_temporary_index25_4.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop31(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut node: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result {
    loop {
        return if (node < ({ let ml_reference_array94 = starts.clone(); let ml_reference_projection94 = ml_reference_array94.borrow().len() as f64; ml_reference_projection94 })) {
            {
                let kind = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107]));
                {
                    let proof = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![112]));
                    {
                        let operation = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 112]));
                        {
                            let reference = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]));
                            {
                                let identity = (((kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108]))) || ((kind == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))) && self::ml_array::includes(&{ let ml_reference_value95 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 113]), self::ml_string::Text::from_units(vec![110, 101])]))).borrow().clone(); ml_reference_value95 }, &operation.clone(), 0f64, false, |a, b| a == b))) || ((self::ml_array::includes(&{ let ml_reference_value96 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 113]), self::ml_string::Text::from_units(vec![110, 101])]))).borrow().clone(); ml_reference_value96 }, &proof.clone(), 0f64, false, |a, b| a == b) && (reference >= (0f64))) && ((self::dependency_function2(texts.clone(), reference)) == (self::ml_string::Text::from_units(vec![116, 114, 117, 101])))));
                                {
                                    let source_ml_source1_temporary_values25 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_string_unit_operand87 = self::read_java_script_closed_class_observation_operation_operands(kind.clone(), proof.clone(), operation.clone());
                                        {
                                            let source_ml_source2_string_unit_operand88 = self::ml_string::Text::from_units(vec![32]);
                                            {
                                                let source_ml_source2_string_split_limit89 = 4294967295f64;
                                                {
                                                    let source_ml_source2_string_split_separator90 = source_ml_source2_string_unit_operand88.clone();
                                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source2_string_unit_operand87), Some(&source_ml_source2_string_split_separator90), source_ml_source2_string_split_limit89)
                                                }
                                            }
                                        }
                                    }));
                                    {
                                        let source_ml_source1_temporary_index25 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s91 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop32(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, constructors.clone(), brands.clone(), representations.clone(), node, identity, source_ml_source1_temporary_values25.clone(), source_ml_source1_temporary_index25.clone());
                                            match source_ml_source2_s91.clone() {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done => {
                                                    {
                                                        let node_2 = (node + 1f64);
                                                        {
                                                            node = node_2;
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(source_ml_source1_temporary_result32) => {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(source_ml_source1_temporary_result32.clone())
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop40(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut kind: self::ml_string::Text, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ((self::dependency_function1(starts.clone(), node) + self::dependency_function1(counts.clone(), node)))) {
            if !(self::ml_array::includes(&{ let ml_reference_value97 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![116, 121, 112, 101]), self::ml_string::Text::from_units(vec![100, 111, 109, 97, 105, 110]), self::ml_string::Text::from_units(vec![115, 112, 97, 110]), self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 80, 97, 116, 104])]))).borrow().clone(); ml_reference_value97 }, &self::dependency_function2(labels.clone(), at), 0f64, false, |a, b| a == b)) {
                {
                    let pending_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value92 = pending.clone();
                        {
                            let source_ml_source2_concat_value93 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function1(targets.clone(), at), if ((kind == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) && ((self::dependency_function2(labels.clone(), at)) == (self::ml_string::Text::from_units(vec![102, 110])))) {
                                1f64
                            } else {
                                0f64
                            }]));
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value98 = source_ml_source2_concat_value92.borrow().clone(); ml_reference_value98 }), { let ml_reference_value99 = source_ml_source2_concat_value93.borrow().clone(); ml_reference_value99 })
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
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            pending.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39 {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop41(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner: self::ml_string::Text, mut direct: f64, mut node: f64, mut kind: self::ml_string::Text, mut object: f64, mut instance: f64, mut name: self::ml_string::Text, mut methods: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), methods))) {
            {
                let method = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), methods, at);
                if ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), method, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) != name) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                } else {
                    if ((kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) && (((self::dependency_function2(representations.clone(), instance)) != (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115]))) || (direct != (1f64)))) {
                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 115, 108, 111, 116, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 115]))
                    } else {
                        if (kind == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                            {
                                let args = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 115]));
                                {
                                    let value = if (args < (0f64)) {
                                        (-1f64)
                                    } else {
                                        self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), args, 0f64)
                                    };
                                    {
                                        let helper = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), method, self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114]));
                                        {
                                            let call_args = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, self::ml_string::Text::from_units(vec![97, 114, 103, 115]));
                                            {
                                                let receiver = if (call_args < (0f64)) {
                                                    (-1f64)
                                                } else {
                                                    self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), call_args, 0f64)
                                                };
                                                if (((((((owner != (self::dependency_function2(constructors.clone(), instance))) || ((self::dependency_function2(representations.clone(), instance)) != (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, self::ml_string::Text::from_units(vec![107]))) != (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), value, self::ml_string::Text::from_units(vec![102, 110]))) != helper)) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), object, self::ml_string::Text::from_units(vec![107]))) != (self::ml_string::Text::from_units(vec![118, 97, 114])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), receiver, self::ml_string::Text::from_units(vec![107]))) != (self::ml_string::Text::from_units(vec![118, 97, 114])))) || ((self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), object, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) != (self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), receiver, self::ml_string::Text::from_units(vec![110, 97, 109, 101]))))) {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 115, 108, 111, 116, 115, 32, 97, 114, 101, 32, 105, 110, 115, 116, 97, 108, 108, 101, 100, 32, 111, 110, 108, 121, 32, 98, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 102, 97, 99, 116, 111, 114, 121, 32, 105, 110, 115, 105, 100, 101, 32, 116, 104, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                } else {
                                                    {
                                                        let at_3 = (at + 1f64);
                                                        {
                                                            at = at_3;
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
                            {
                                let at_4 = (at + 1f64);
                                {
                                    at = at_4;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop38(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owner: self::ml_string::Text, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result {
    loop {
        return if { let ml_value = { let ml_reference_array101 = pending.clone(); let ml_reference_projection101 = ml_reference_array101.borrow().len() as f64; ml_reference_projection101 }; ml_value != 0f64 && !ml_value.is_nan() } {
            {
                let direct = self::dependency_function1(pending.clone(), ({ let ml_reference_array102 = pending.clone(); let ml_reference_projection102 = ml_reference_array102.borrow().len() as f64; ml_reference_projection102 } - 1f64));
                {
                    let node = self::dependency_function1(pending.clone(), ({ let ml_reference_array103 = pending.clone(); let ml_reference_projection103 = ml_reference_array103.borrow().len() as f64; ml_reference_projection103 } - 2f64));
                    {
                        let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value104 = pending.borrow().clone(); ml_reference_value104 }, 0f64, (-2f64))));
                        if ((node < (0f64)) || self::ml_array::includes(&{ let ml_reference_value105 = seen.borrow().clone(); ml_reference_value105 }, &((node * 2f64) + direct), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                            {
                                pending = pending_2.clone();
                                continue;
                            }
                        } else {
                            if (({ let ml_reference_array106 = seen.clone(); let ml_reference_projection106 = ml_reference_array106.borrow().len() as f64; ml_reference_projection106 }) > (100000f64)) {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 98, 111, 100, 121, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value94 = seen.clone();
                                        {
                                            let source_ml_source2_concat_value95 = std::rc::Rc::new(std::cell::RefCell::new(vec![((node * 2f64) + direct)]));
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value107 = source_ml_source2_concat_value94.borrow().clone(); ml_reference_value107 }), { let ml_reference_value108 = source_ml_source2_concat_value95.borrow().clone(); ml_reference_value108 })
                                        }
                                    }));
                                    {
                                        let kind = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![107]));
                                        {
                                            let type_ = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                                            {
                                                let class_owner = self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]));
                                                if (((kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) && (class_owner != (self::ml_string::Text::from_units(vec![])))) && (class_owner != owner)) {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 115, 32, 98, 101, 108, 111, 110, 103, 32, 111, 110, 108, 121, 32, 116, 111, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                } else {
                                                    {
                                                        let object = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]));
                                                        {
                                                            let object_type = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), object, self::ml_string::Text::from_units(vec![116, 121, 112, 101]));
                                                            {
                                                                let instance = self::ml_array::index_of(&{ let ml_reference_value109 = constructors.borrow().clone(); ml_reference_value109 }, &self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), object_type, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114])), 0f64, false, |a, b| a == b);
                                                                {
                                                                    let source_ml_source2_s97 = if (instance >= (0f64)) {
                                                                        {
                                                                            let definition = self::dependency_function1(owner_nodes.clone(), instance);
                                                                            {
                                                                                let brand = self::dependency_function2(brands.clone(), instance);
                                                                                {
                                                                                    let name = if (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) {
                                                                                        self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]))
                                                                                    } else {
                                                                                        if (kind == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                                                                                            self::read_record_field_write_name(self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 112])))
                                                                                        } else {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        }
                                                                                    };
                                                                                    if (name == brand) {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(self::ml_string::Text::from_units(vec![97, 32, 115, 121, 110, 116, 104, 101, 116, 105, 99, 32, 99, 108, 97, 115, 115, 32, 98, 114, 97, 110, 100, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 111, 98, 115, 101, 114, 118, 97, 98, 108, 101, 32, 112, 117, 98, 108, 105, 99, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100]))
                                                                                    } else {
                                                                                        {
                                                                                            let methods = self::dependency_function3(starts.clone(), counts.clone(), labels.clone(), targets.clone(), definition, self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115]));
                                                                                            {
                                                                                                let at = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s98 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop41(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), constructors.clone(), representations.clone(), owner.clone(), direct, node, kind.clone(), object, instance, name.clone(), methods, at);
                                                                                                    match source_ml_source2_s98.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(source_ml_source1_temporary_result41) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(source_ml_source1_temporary_result41.clone())
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
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next
                                                                    };
                                                                    match source_ml_source2_s97.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next => {
                                                                            {
                                                                                let at_2 = self::dependency_function1(starts.clone(), node);
                                                                                {
                                                                                    let pending_3 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop40(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending_2.clone(), node, kind.clone(), at_2);
                                                                                    {
                                                                                        (pending, seen) = (pending_3.clone(), seen_2.clone());
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(source_ml_source1_temporary_result40) => {
                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(source_ml_source1_temporary_result40.clone())
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop30(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut constructors: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut brands: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut representations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owner_nodes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut scopes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut scope: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result {
    loop {
        return if (scope < ({ let ml_reference_array110 = roots.clone(); let ml_reference_projection110 = ml_reference_array110.borrow().len() as f64; ml_reference_projection110 })) {
            {
                let owner = self::dependency_function2(scopes.clone(), scope);
                {
                    let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![self::dependency_function1(roots.clone(), scope), 0f64]));
                    {
                        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let source_ml_source2_s99 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop38(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), constructors.clone(), brands.clone(), representations.clone(), owner_nodes.clone(), owner.clone(), pending.clone(), seen.clone());
                            match source_ml_source2_s99.clone() {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done => {
                                    {
                                        let scope_2 = (scope + 1f64);
                                        {
                                            scope = scope_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(source_ml_source1_temporary_result38) => {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(source_ml_source1_temporary_result38.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop29(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: f64, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut scopes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut declaration: f64) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result {
    loop {
        return if ((declarations >= (0f64)) && (declaration < (self::dependency_function1(counts.clone(), declarations)))) {
            {
                let row = self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), declarations, declaration);
                {
                    let roots_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value100 = roots.clone();
                        {
                            let source_ml_source2_concat_value101 = std::rc::Rc::new(std::cell::RefCell::new(vec![row]));
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value111 = source_ml_source2_concat_value100.borrow().clone(); ml_reference_value111 }), { let ml_reference_value112 = source_ml_source2_concat_value101.borrow().clone(); ml_reference_value112 })
                        }
                    }));
                    {
                        let scopes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value102 = scopes.clone();
                            {
                                let source_ml_source2_concat_value103 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, self::ml_string::Text::from_units(vec![102, 117, 108, 108, 78, 97, 109, 101]))]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value113 = source_ml_source2_concat_value102.borrow().clone(); ml_reference_value113 }), { let ml_reference_value114 = source_ml_source2_concat_value103.borrow().clone(); ml_reference_value114 })
                            }
                        }));
                        {
                            let declaration_2 = (declaration + 1f64);
                            {
                                (roots, scopes, declaration) = (roots_2.clone(), scopes_2.clone(), declaration_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(roots.clone(), scopes.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop42(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut texts: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut methods: f64, mut complete: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (at < (self::dependency_function1(counts.clone(), methods))) {
            {
                let complete_2 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_concat_value104 = complete.clone();
                    {
                        let source_ml_source2_concat_value105 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::word(forms.clone(), texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function4(starts.clone(), counts.clone(), targets.clone(), methods, at), self::ml_string::Text::from_units(vec![110, 97, 109, 101]))]));
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value115 = source_ml_source2_concat_value104.borrow().clone(); ml_reference_value115 }), { let ml_reference_value116 = source_ml_source2_concat_value105.borrow().clone(); ml_reference_value116 })
                    }
                }));
                {
                    let at_2 = (at + 1f64);
                    {
                        (complete, at) = (complete_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            complete.clone()
        };
    }
}
