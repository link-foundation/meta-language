pub fn read_rust_owned_record_place_token_count(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> f64 {
    {
        let depth = 0f64;
        {
            let atom = true;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s22 = self::source_ml_temporary_read_rust_owned_record_place_token_count_loop5(forms.clone(), depth, atom, at);
                    match source_ml_source1_s22.clone() {
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Done => {
                            (-1f64)
                        }
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(source_ml_temporary_result5) => {
                            source_ml_temporary_result5
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_record_state_assignment_token_count(forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> f64 {
    {
        let count = self::read_rust_owned_record_place_token_count(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values5 = { let ml_reference_value6 = forms.borrow().clone(); ml_reference_value6 }; let ml_callback5 = std::rc::Rc::new(move |form: self::ml_string::Text| -> self::ml_string::Text { if (form == (self::ml_string::Text::from_units(vec![61]))) {
            self::ml_string::Text::from_units(vec![125])
        } else {
            form.clone()
        } }); ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5)).collect::<Vec<_>>() })));
        if (((count >= (0f64)) && {
            let source_ml_source1_comparison_operand25 = {
                let source_ml_source1_array_read_value23 = forms.clone();
                {
                    let source_ml_source1_array_read_index24 = count;
                    { let ml_reference_array7 = source_ml_source1_array_read_value23.clone(); let ml_reference_position7 = source_ml_source1_array_read_index24; let ml_reference_projection7 = self::ml_array::optional_at(&*ml_reference_array7.borrow(), self::ml_array::number_index(ml_reference_position7)); ml_reference_projection7 }
                }
            };
            {
                let source_ml_source1_comparison_operand26 = self::ml_string::Text::from_units(vec![61]);
                match (source_ml_source1_comparison_operand25.clone(), source_ml_source1_comparison_operand26.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) && self::ml_array::includes(&{ let ml_reference_value8 = (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value9 = forms.borrow().clone(); ml_reference_value9 }, 0f64, count)))).borrow().clone(); ml_reference_value8 }, &self::ml_string::Text::from_units(vec![46]), 0f64, false, |a, b| a == b)) {
            count
        } else {
            (-1f64)
        }
    }
}

pub fn read_rust_owned_record_state_storage_disposition(kind: self::ml_string::Text, layers: f64, role: self::ml_string::Text) -> self::ml_string::Text {
    if (((layers != (0f64)) || ((role != (self::ml_string::Text::from_units(vec![99, 111, 112, 121]))) && (role != (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))))) || (kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) {
        self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
    } else {
        if (((kind == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100]))) || (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) || (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) {
            self::ml_string::Text::from_units(vec![99, 104, 105, 108, 100, 114, 101, 110])
        } else {
            if self::ml_array::includes(&{ let ml_reference_value10 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![117, 110, 105, 116]), self::ml_string::Text::from_units(vec![98, 111, 111, 108]), self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]), self::ml_string::Text::from_units(vec![105, 110, 116]), self::ml_string::Text::from_units(vec![110, 97, 116]), self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])]))).borrow().clone(); ml_reference_value10 }, &kind.clone(), 0f64, false, |a, b| a == b) {
                self::ml_string::Text::from_units(vec![108, 101, 97, 102])
            } else {
                self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
            }
        }
    }
}

pub fn accept_rust_owned_record_state_storage_owner(kind: self::ml_string::Text, path: self::ml_string::Text) -> bool {
    ((kind != (self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]))) || ((path != (self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 98, 111, 120, 101, 100, 58, 58, 66, 111, 120]))) && (path != (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 58, 58, 98, 111, 120, 101, 100, 58, 58, 66, 111, 120])))))
}

pub fn read_rust_owned_record_state_storage_error(nodes: f64, depth: f64) -> self::ml_string::Text {
    if (((((self::ml_math::is_integer(nodes) && (nodes >= (0f64))) && (nodes <= (65536f64))) && self::ml_math::is_integer(depth)) && (depth >= (0f64))) && (depth <= (256f64))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 99, 111, 114, 100, 32, 115, 116, 97, 116, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 102, 105, 110, 105, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
    }
}

pub fn read_rust_owned_record_mutable_binding_error(ordinary: bool, written: bool, profile: self::ml_string::Text, binding: bool, lexical: bool, layers: f64) -> self::ml_string::Text {
    if (((((ordinary && written) && (profile == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100])))) && binding) && lexical) && (layers == (0f64))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![97, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 111, 119, 110, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 111, 111, 116])
    }
}

pub fn read_rust_owned_record_state_write_error(mutable: bool, callable: bool, scope: bool, root: self::ml_string::Text, field: self::ml_string::Text, type_: bool, loan: bool) -> self::ml_string::Text {
    if !mutable {
        self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 98, 105, 110, 100, 105, 110, 103])
    } else {
        if !callable {
            self::ml_string::Text::from_units(vec![97, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 32, 115, 116, 97, 116, 101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])
        } else {
            if !scope {
                self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 99, 111, 114, 100, 32, 115, 116, 97, 116, 101, 32, 114, 101, 115, 117, 108, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 98, 101, 102, 111, 114, 101, 32, 97, 110, 32, 111, 117, 116, 101, 114, 32, 112, 108, 97, 99, 101, 32, 99, 97, 110, 32, 98, 101, 32, 99, 104, 97, 110, 103, 101, 100])
            } else {
                if ((root != (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) || (((field != (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) && (field != (self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100])))) && (field != (self::ml_string::Text::from_units(vec![112, 111, 115, 115, 105, 98, 108, 121, 32, 109, 111, 118, 101, 100]))))) {
                    self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 32, 114, 101, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 116, 105, 108, 108, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 111, 114, 32, 109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100])
                } else {
                    if !type_ {
                        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 109, 117, 115, 116, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 110, 100, 32, 111, 119, 110, 101, 114])
                    } else {
                        if loan {
                            self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 111, 118, 101, 114, 108, 97, 112, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 114, 118, 105, 118, 105, 110, 103, 32, 119, 104, 111, 108, 101, 32, 111, 114, 32, 102, 105, 101, 108, 100, 32, 108, 111, 97, 110])
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn test_rust_owned_record_state_loan_overlap(root: self::ml_string::Text, target: self::ml_string::Text, loans: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, lexical: bool) -> bool {
    {
        let precise = false;
        {
            let source_ml_temporary_values6 = loans.clone();
            {
                let source_ml_temporary_index6 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s27 = self::source_ml_temporary_test_rust_owned_record_state_loan_overlap_loop7(root.clone(), target.clone(), precise, source_ml_temporary_values6.clone(), source_ml_temporary_index6.clone());
                    match source_ml_source1_s27.clone() {
                        self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Done(precise_2) => {
                            if precise_2 {
                                false
                            } else {
                                (lexical || self::ml_array::includes(&{ let ml_reference_value11 = loans.borrow().clone(); ml_reference_value11 }, &root.clone(), 0f64, false, |a, b| a == b))
                            }
                        }
                        self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Return(source_ml_temporary_result7) => {
                            source_ml_temporary_result7
                        }
                    }
                }
            }
        }
    }
}

pub fn plan_rust_owned_record_state_packet(roots: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if ((!(self::ml_math::is_integer(roots)) || (roots < (0f64))) || (roots > (256f64))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]));
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_temporary_plan_rust_owned_record_state_packet_loop8(roots, result.clone(), index);
                    result_2.clone()
                }
            }
        }
    }
}

pub fn plan_rust_owned_record_state_fields(fields: f64, selected: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((((!(self::ml_math::is_integer(fields)) || (fields < (1f64))) || (fields > (1024f64))) || !(self::ml_math::is_integer(selected))) || (selected < (0f64))) || (selected >= fields)) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_temporary_plan_rust_owned_record_state_fields_loop9(fields, selected, result.clone(), index);
                    result_2.clone()
                }
            }
        }
    }
}

pub fn read_rust_owned_record_state_field_identity(root: self::ml_string::Text, fields: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::ml_string::Text {
    {
        let result = root.clone();
        {
            let source_ml_temporary_values10 = fields.clone();
            {
                let source_ml_temporary_index10 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s28 = self::source_ml_temporary_read_rust_owned_record_state_field_identity_loop11(result.clone(), source_ml_temporary_values10.clone(), source_ml_temporary_index10.clone());
                    match source_ml_source1_s28.clone() {
                        self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Done(result_2) => {
                            result_2.clone()
                        }
                        self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Return(source_ml_temporary_result11) => {
                            source_ml_temporary_result11.clone()
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result {
    MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done,
    MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_rust_owned_record_ultimate_root_loop1(mut root: self::ml_string::Text, mut depth: f64) -> self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result {
    loop {
        return if (depth < (1024f64)) {
            {
                let parent = self::read_rust_owned_record_field_root(root.clone());
                if (parent == (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(root.clone())
                } else {
                    {
                        let root_2 = parent.clone();
                        {
                            let depth_2 = (depth + 1f64);
                            {
                                (root, depth) = (root_2.clone(), depth_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result {
    MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done,
    MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(bool),
}

pub fn source_ml_temporary_test_rust_owned_record_field_descendant_loop2(mut root: self::ml_string::Text, mut parent: self::ml_string::Text, mut depth: f64) -> self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result {
    loop {
        return if ((depth < (1024f64)) && (parent != (self::ml_string::Text::from_units(vec![])))) {
            if (parent == root) {
                self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(true)
            } else {
                {
                    let parent_2 = self::read_rust_owned_record_field_root(parent.clone());
                    {
                        let depth_2 = (depth + 1f64);
                        {
                            (parent, depth) = (parent_2.clone(), depth_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done,
    MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_rust_owned_record_whole_error_loop4(mut root: self::ml_string::Text, mut states: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut index: f64, mut source_ml_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_temporary_index3: self::ml::Big) -> self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    loop {
        return if (source_ml_temporary_index3 < ({ let ml_reference_array12 = source_ml_temporary_values3.clone(); let ml_reference_projection12 = self::ml::Big::from_u128(ml_reference_array12.borrow().len() as u128); ml_reference_projection12 })) {
            {
                let identity = {
                    let source_ml_source1_array_read_value29 = source_ml_temporary_values3.clone();
                    {
                        let source_ml_source1_array_read_index30 = source_ml_temporary_index3.clone();
                        { let ml_reference_array13 = source_ml_source1_array_read_value29.clone(); let ml_reference_position13 = source_ml_source1_array_read_index30.clone(); let ml_reference_projection13 = self::ml_array::at(&*ml_reference_array13.borrow(), ml_reference_position13.to_index()); ml_reference_projection13 }
                    }
                };
                if (self::test_rust_owned_record_field_descendant(root.clone(), identity.clone()) && {
                    let source_ml_source1_comparison_operand33 = {
                        let source_ml_source1_array_read_value31 = states.clone();
                        {
                            let source_ml_source1_array_read_index32 = index;
                            { let ml_reference_array14 = source_ml_source1_array_read_value31.clone(); let ml_reference_position14 = source_ml_source1_array_read_index32; let ml_reference_projection14 = self::ml_array::optional_at(&*ml_reference_array14.borrow(), self::ml_array::number_index(ml_reference_position14)); ml_reference_projection14 }
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand34 = self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]);
                        !(match (source_ml_source1_comparison_operand33.clone(), source_ml_source1_comparison_operand34.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                    }
                }) {
                    self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_temporary_index3) = (index_2, source_ml_temporary_index3_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result {
    MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Done,
    MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(f64),
}

pub fn source_ml_temporary_read_rust_owned_record_place_token_count_loop5(mut forms: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut depth: f64, mut atom: bool, mut at: f64) -> self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result {
    loop {
        return if (at < ({ let ml_reference_array15 = forms.clone(); let ml_reference_projection15 = ml_reference_array15.borrow().len() as f64; ml_reference_projection15 })) {
            {
                let form = {
                    let source_ml_source1_array_read_value35 = forms.clone();
                    {
                        let source_ml_source1_array_read_index36 = at;
                        { let ml_reference_array16 = source_ml_source1_array_read_value35.clone(); let ml_reference_position16 = source_ml_source1_array_read_index36; let ml_reference_projection16 = self::ml_array::optional_at(&*ml_reference_array16.borrow(), self::ml_array::number_index(ml_reference_position16)); ml_reference_projection16 }
                    }
                };
                if atom {
                    if {
                        let source_ml_source1_comparison_operand37 = form.clone();
                        {
                            let source_ml_source1_comparison_operand38 = self::ml_string::Text::from_units(vec![40]);
                            match (source_ml_source1_comparison_operand37.clone(), source_ml_source1_comparison_operand38.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    } {
                        {
                            let depth_2 = (depth + 1f64);
                            if (depth_2 >= (self::read_rust_owned_record_place_depth_limit())) {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return((-1f64))
                            } else {
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (depth, at) = (depth_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if {
                            let source_ml_source1_comparison_operand39 = form.clone();
                            {
                                let source_ml_source1_comparison_operand40 = self::ml_string::Text::from_units(vec![110, 97, 109, 101]);
                                match (source_ml_source1_comparison_operand39.clone(), source_ml_source1_comparison_operand40.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            }
                        } {
                            {
                                let atom_2 = false;
                                {
                                    let at_3 = (at + 1f64);
                                    {
                                        (atom, at) = (atom_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return((-1f64))
                        }
                    }
                } else {
                    if {
                        let source_ml_source1_comparison_operand41 = form.clone();
                        {
                            let source_ml_source1_comparison_operand42 = self::ml_string::Text::from_units(vec![46]);
                            match (source_ml_source1_comparison_operand41.clone(), source_ml_source1_comparison_operand42.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    } {
                        {
                            let at_4 = (at + 1f64);
                            if ({
                                let source_ml_source1_comparison_operand45 = {
                                    let source_ml_source1_array_read_value43 = forms.clone();
                                    {
                                        let source_ml_source1_array_read_index44 = at_4;
                                        { let ml_reference_array17 = source_ml_source1_array_read_value43.clone(); let ml_reference_position17 = source_ml_source1_array_read_index44; let ml_reference_projection17 = self::ml_array::optional_at(&*ml_reference_array17.borrow(), self::ml_array::number_index(ml_reference_position17)); ml_reference_projection17 }
                                    }
                                };
                                {
                                    let source_ml_source1_comparison_operand46 = self::ml_string::Text::from_units(vec![110, 97, 109, 101]);
                                    !(match (source_ml_source1_comparison_operand45.clone(), source_ml_source1_comparison_operand46.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }
                            } && {
                                let source_ml_source1_comparison_operand49 = {
                                    let source_ml_source1_array_read_value47 = forms.clone();
                                    {
                                        let source_ml_source1_array_read_index48 = at_4;
                                        { let ml_reference_array18 = source_ml_source1_array_read_value47.clone(); let ml_reference_position18 = source_ml_source1_array_read_index48; let ml_reference_projection18 = self::ml_array::optional_at(&*ml_reference_array18.borrow(), self::ml_array::number_index(ml_reference_position18)); ml_reference_projection18 }
                                    }
                                };
                                {
                                    let source_ml_source1_comparison_operand50 = self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]);
                                    !(match (source_ml_source1_comparison_operand49.clone(), source_ml_source1_comparison_operand50.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }
                            }) {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return((-1f64))
                            } else {
                                {
                                    let at_5 = (at_4 + 1f64);
                                    {
                                        at = at_5;
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if {
                            let source_ml_source1_comparison_operand51 = form.clone();
                            {
                                let source_ml_source1_comparison_operand52 = self::ml_string::Text::from_units(vec![41]);
                                match (source_ml_source1_comparison_operand51.clone(), source_ml_source1_comparison_operand52.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            }
                        } {
                            if (depth == (0f64)) {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return((-1f64))
                            } else {
                                {
                                    let depth_3 = (depth - 1f64);
                                    {
                                        let at_6 = (at + 1f64);
                                        {
                                            (depth, at) = (depth_3, at_6);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            if {
                                let source_ml_source1_comparison_operand53 = form.clone();
                                {
                                    let source_ml_source1_comparison_operand54 = self::ml_string::Text::from_units(vec![125]);
                                    match (source_ml_source1_comparison_operand53.clone(), source_ml_source1_comparison_operand54.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return(if (depth == (0f64)) {
                                    at
                                } else {
                                    (-1f64)
                                })
                            } else {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Return((-1f64))
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result {
    MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Done(bool),
    MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Return(bool),
}

pub fn source_ml_temporary_test_rust_owned_record_state_loan_overlap_loop7(mut root: self::ml_string::Text, mut target: self::ml_string::Text, mut precise: bool, mut source_ml_temporary_values6: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_temporary_index6: self::ml::Big) -> self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result {
    loop {
        return if (source_ml_temporary_index6 < ({ let ml_reference_array19 = source_ml_temporary_values6.clone(); let ml_reference_projection19 = self::ml::Big::from_u128(ml_reference_array19.borrow().len() as u128); ml_reference_projection19 })) {
            {
                let loan = {
                    let source_ml_source1_array_read_value55 = source_ml_temporary_values6.clone();
                    {
                        let source_ml_source1_array_read_index56 = source_ml_temporary_index6.clone();
                        { let ml_reference_array20 = source_ml_source1_array_read_value55.clone(); let ml_reference_position20 = source_ml_source1_array_read_index56.clone(); let ml_reference_projection20 = self::ml_array::at(&*ml_reference_array20.borrow(), ml_reference_position20.to_index()); ml_reference_projection20 }
                    }
                };
                if self::test_rust_owned_record_field_descendant(root.clone(), loan.clone()) {
                    {
                        let precise_2 = true;
                        if (((loan == target) || self::test_rust_owned_record_field_descendant(target.clone(), loan.clone())) || self::test_rust_owned_record_field_descendant(loan.clone(), target.clone())) {
                            self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Return(true)
                        } else {
                            {
                                let source_ml_temporary_index6_2 = source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                                {
                                    (precise, source_ml_temporary_index6) = (precise_2, source_ml_temporary_index6_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index6_3 = source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index6 = source_ml_temporary_index6_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Result::MlTemporaryTestRustOwnedRecordStateLoanOverlapLoop7Done(precise)
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_record_state_packet_loop8(mut roots: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < roots) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value21 = result.borrow().clone(); ml_reference_value21 }, vec![(index + 1f64)])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_record_state_fields_loop9(mut fields: f64, mut selected: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < fields) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value22 = result.borrow().clone(); ml_reference_value22 }, vec![if (index == selected) {
                    (-1f64)
                } else {
                    index
                }])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2.clone(), index_2);
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
pub enum MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result {
    MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Done(self::ml_string::Text),
    MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_rust_owned_record_state_field_identity_loop11(mut result: self::ml_string::Text, mut source_ml_temporary_values10: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index10: self::ml::Big) -> self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result {
    loop {
        return if (source_ml_temporary_index10 < ({ let ml_reference_array23 = source_ml_temporary_values10.clone(); let ml_reference_projection23 = self::ml::Big::from_u128(ml_reference_array23.borrow().len() as u128); ml_reference_projection23 })) {
            {
                let index = {
                    let source_ml_source1_array_read_value57 = source_ml_temporary_values10.clone();
                    {
                        let source_ml_source1_array_read_index58 = source_ml_temporary_index10.clone();
                        { let ml_reference_array24 = source_ml_source1_array_read_value57.clone(); let ml_reference_position24 = source_ml_source1_array_read_index58.clone(); let ml_reference_projection24 = self::ml_array::at(&*ml_reference_array24.borrow(), ml_reference_position24.to_index()); ml_reference_projection24 }
                    }
                };
                {
                    let result_2 = self::read_rust_owned_record_field_identity(result.clone(), index);
                    if (result_2 == (self::ml_string::Text::from_units(vec![]))) {
                        self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Return(self::ml_string::Text::from_units(vec![]))
                    } else {
                        {
                            let source_ml_temporary_index10_2 = source_ml_temporary_index10.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_temporary_index10) = (result_2.clone(), source_ml_temporary_index10_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Result::MlTemporaryReadRustOwnedRecordStateFieldIdentityLoop11Done(result.clone())
        };
    }
}

fn ml_main() {
    assert!(((self::read_rust_owned_record_mutable_binding_error(true, true, self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), true, true, 0f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 1");
    assert!(((self::read_rust_owned_record_mutable_binding_error(true, false, self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), true, true, 0f64)) == (self::ml_string::Text::from_units(vec![97, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 111, 119, 110, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 111, 111, 116]))), "assertion 2");
    assert!(((self::read_rust_owned_record_mutable_binding_error(true, true, self::ml_string::Text::from_units(vec![99, 111, 112, 121]), true, true, 0f64)) == (self::ml_string::Text::from_units(vec![97, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 111, 119, 110, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 111, 111, 116]))), "assertion 3");
    assert!(((self::read_rust_owned_record_mutable_binding_error(true, true, self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), false, true, 0f64)) == (self::ml_string::Text::from_units(vec![97, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 111, 119, 110, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 111, 111, 116]))), "assertion 4");
    assert!(((self::read_rust_owned_record_state_write_error(true, true, true, self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), true, false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 5");
    assert!(((self::read_rust_owned_record_state_write_error(true, true, true, self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]), true, false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 6");
    assert!(((self::read_rust_owned_record_state_write_error(true, true, true, self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), true, true)) == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 111, 118, 101, 114, 108, 97, 112, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 114, 118, 105, 118, 105, 110, 103, 32, 119, 104, 111, 108, 101, 32, 111, 114, 32, 102, 105, 101, 108, 100, 32, 108, 111, 97, 110]))), "assertion 7");
    assert!(((self::read_rust_owned_record_state_write_error(true, true, true, self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), true, false)) == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 32, 114, 101, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 116, 105, 108, 108, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 111, 114, 32, 109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100]))), "assertion 8");
    assert!(((self::read_rust_owned_record_state_write_error(true, false, true, self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), true, false)) == (self::ml_string::Text::from_units(vec![97, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 32, 115, 116, 97, 116, 101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110]))), "assertion 9");
    assert!(((self::read_rust_owned_record_state_write_error(true, true, false, self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), true, false)) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 99, 111, 114, 100, 32, 115, 116, 97, 116, 101, 32, 114, 101, 115, 117, 108, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 98, 101, 102, 111, 114, 101, 32, 97, 110, 32, 111, 117, 116, 101, 114, 32, 112, 108, 97, 99, 101, 32, 99, 97, 110, 32, 98, 101, 32, 99, 104, 97, 110, 103, 101, 100]))), "assertion 10");
    assert!(((self::read_rust_owned_record_state_write_error(false, true, true, self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), true, false)) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 98, 105, 110, 100, 105, 110, 103]))), "assertion 11");
    assert!(((self::read_rust_owned_record_state_write_error(true, true, true, self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), false, false)) == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 109, 117, 115, 116, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 110, 100, 32, 111, 119, 110, 101, 114]))), "assertion 12");
    assert!(((self::test_rust_owned_record_state_loan_overlap(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 49])])), true)) == (false)), "assertion 13");
    assert!(((self::test_rust_owned_record_state_loan_overlap(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48])])), false)) == (true)), "assertion 14");
    assert!(((self::test_rust_owned_record_state_loan_overlap(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48])])), false)) == (true)), "assertion 15");
    assert!(((self::test_rust_owned_record_state_loan_overlap(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![55])])), false)) == (true)), "assertion 16");
    assert!(((self::test_rust_owned_record_state_loan_overlap(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())), true)) == (true)), "assertion 17");
    assert!(((self::test_rust_owned_record_state_loan_overlap(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![55, 48])])), false)) == (false)), "assertion 18");
    assert!(((self::plan_rust_owned_record_state_packet(0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))), "assertion 19");
    assert!(((self::plan_rust_owned_record_state_packet(2f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])))), "assertion 20");
    assert!(((self::plan_rust_owned_record_state_packet(257f64)) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 21");
    assert!(((self::plan_rust_owned_record_state_fields(3f64, 1f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), 2f64])))), "assertion 22");
    assert!(((self::plan_rust_owned_record_state_fields(0f64, 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 23");
    assert!(((self::read_rust_owned_record_state_field_identity(self::ml_string::Text::from_units(vec![55]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49]))), "assertion 24");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_state_assignment_token_count(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 97, 109, 101]), self::ml_string::Text::from_units(vec![46]), self::ml_string::Text::from_units(vec![110, 97, 109, 101]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![111, 116, 104, 101, 114])]))), 3f64), "assertion 25");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_state_assignment_token_count(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![110, 97, 109, 101]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![46]), self::ml_string::Text::from_units(vec![110, 97, 109, 101]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![111, 116, 104, 101, 114])]))), 5f64), "assertion 26");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_state_assignment_token_count(std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 97, 109, 101]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![111, 116, 104, 101, 114])]))), (-1f64)), "assertion 27");
    assert!(((self::read_rust_owned_record_state_storage_disposition(self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]), 0f64, self::ml_string::Text::from_units(vec![99, 111, 112, 121]))) == (self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101]))), "assertion 28");
    assert!(((self::read_rust_owned_record_state_storage_disposition(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), 0f64, self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))) == (self::ml_string::Text::from_units(vec![108, 101, 97, 102]))), "assertion 29");
    assert!(((self::read_rust_owned_record_state_storage_disposition(self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100]), 0f64, self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))) == (self::ml_string::Text::from_units(vec![99, 104, 105, 108, 100, 114, 101, 110]))), "assertion 30");
    assert!(((self::read_rust_owned_record_state_storage_disposition(self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]), 1f64, self::ml_string::Text::from_units(vec![99, 111, 112, 121]))) == (self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101]))), "assertion 31");
    assert!(((self::accept_rust_owned_record_state_storage_owner(self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]), self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 98, 111, 120, 101, 100, 58, 58, 66, 111, 120]))) == (false)), "assertion 32");
    assert!(((self::accept_rust_owned_record_state_storage_owner(self::ml_string::Text::from_units(vec![108, 105, 98, 114, 97, 114, 121]), self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 115, 116, 114, 105, 110, 103, 58, 58, 83, 116, 114, 105, 110, 103]))) == (true)), "assertion 33");
    assert!(((self::read_rust_owned_record_state_storage_error(65536f64, 256f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 34");
    assert!(((self::read_rust_owned_record_state_storage_error(65537f64, 256f64)) == (self::ml_string::Text::from_units(vec![116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 99, 111, 114, 100, 32, 115, 116, 97, 116, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 102, 105, 110, 105, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))), "assertion 35");
    assert!(((self::read_rust_owned_record_state_storage_disposition(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), 0f64, self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))) == (self::ml_string::Text::from_units(vec![99, 104, 105, 108, 100, 114, 101, 110]))), "assertion 36");
    assert!(((self::read_rust_owned_record_state_write_error(true, true, true, self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![112, 111, 115, 115, 105, 98, 108, 121, 32, 109, 111, 118, 101, 100]), true, false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 37");
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
fn all_literal_owned_record_state_controls() { main(); }
