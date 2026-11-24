pub fn read_rust_owned_record_nested_completion_error(admitted: bool, overlap: bool) -> self::ml_string::Text {
    if !admitted {
        self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 108, 111, 111, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 102, 105, 110, 105, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 97, 100, 109, 105, 115, 115, 105, 111, 110])
    } else {
        if overlap {
            self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 108, 111, 111, 112, 32, 99, 97, 110, 110, 111, 116, 32, 119, 114, 105, 116, 101, 32, 105, 116, 115, 32, 101, 110, 99, 108, 111, 115, 105, 110, 103, 32, 112, 114, 111, 103, 114, 101, 115, 115, 32, 99, 111, 117, 110, 116, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_rust_owned_record_loop_footprint_error(nodes: f64, text: f64) -> self::ml_string::Text {
    if (((!(self::accept_rust_owned_record_loop_count(nodes)) || !(self::accept_rust_owned_record_loop_count(text))) || (nodes > (100000f64))) || (text > (16777216f64))) {
        self::ml_string::Text::from_units(vec![116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 108, 111, 111, 112, 32, 119, 114, 105, 116, 101, 32, 102, 111, 111, 116, 112, 114, 105, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_rust_owned_record_loop_residual_count_error(count: f64) -> self::ml_string::Text {
    if ((self::accept_rust_owned_record_loop_count(count) && (count >= (1f64))) && (count <= (256f64))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 97, 103, 97, 116, 101, 100, 32, 108, 111, 111, 112, 32, 101, 120, 105, 116, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 100, 117, 99, 101, 100, 32, 114, 101, 115, 105, 100, 117, 97, 108, 32, 98, 114, 97, 110, 99, 104, 32, 119, 105, 116, 104, 105, 110, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
    }
}

pub fn accept_rust_source_region_span(start: f64, end: f64) -> bool {
    (((((start >= (0f64)) && (((start % 1f64)) == (0f64))) && (end > start)) && (end <= (9007199254740991f64))) && (((end % 1f64)) == (0f64)))
}

pub fn accept_rust_source_state_binder(name_length: f64, identity_length: f64, start: f64, end: f64) -> bool {
    (((((name_length > (0f64)) && (name_length <= (4096f64))) && (identity_length > (0f64))) && (identity_length <= (4096f64))) && self::accept_rust_source_region_span(start, end))
}

pub fn accept_rust_source_region_composition(signature: self::ml_string::Text, exclusive: self::ml_string::Text, callback: self::ml_string::Text) -> bool {
    (((signature == (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 51]))) && (exclusive == (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 101, 120, 99, 108, 117, 115, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 108, 111, 97, 110, 115, 32, 49])))) && (callback == (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 99, 108, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 70, 110, 77, 117, 116, 32, 115, 112, 101, 99, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 115, 32, 49]))))
}

pub fn accept_rust_source_state_transition_model(language: self::ml_string::Text, model: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    (((language == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) && (model == (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 115, 111, 117, 114, 99, 101, 32, 115, 116, 97, 116, 101, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 49])))) && ((operation == (self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101]))) || (operation == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])))))
}

pub fn accept_rust_owned_vector_source_fold_model(language: self::ml_string::Text, model: self::ml_string::Text, slots: f64, start: f64, end: f64) -> bool {
    (((((language == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) && (model == (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 111, 119, 110, 101, 100, 32, 118, 101, 99, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 102, 111, 108, 100, 32, 49])))) && (slots >= (0f64))) && (slots <= (512f64))) && self::accept_rust_source_region_span(start, end))
}

pub fn accept_rust_finite_loop_state_transform(language: self::ml_string::Text, model: self::ml_string::Text, name: self::ml_string::Text, generated: bool, slots: f64, start: f64, end: f64) -> bool {
    (((((((language == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) && (model == (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 102, 105, 110, 105, 116, 101, 32, 108, 111, 111, 112, 32, 115, 116, 97, 116, 101, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 49])))) && generated) && self::test_rust_owned_record_loop_chunk_name(name.clone())) && (slots >= (1f64))) && (slots <= (512f64))) && self::accept_rust_source_region_span(start, end))
}

pub fn read_rust_owned_vector_source_fold_control_variants() -> Vec<self::ml_string::Text> {
    vec![self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 82, 101, 115, 117, 108, 116]), self::ml_string::Text::from_units(vec![83, 116, 97, 116, 101, 109, 101, 110, 116, 67, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![76, 111, 111, 112, 66, 114, 101, 97, 107]), self::ml_string::Text::from_units(vec![76, 111, 111, 112, 67, 111, 110, 116, 105, 110, 117, 101])]
}

pub fn read_rust_owned_record_update_version_error(minor: f64) -> self::ml_string::Text {
    if ((self::ml_math::is_integer(minor) && (minor >= (90f64))) && (minor < (100f64))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 111, 117, 116, 115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61, 49, 46, 57, 48, 46, 48, 32, 60, 49, 46, 49, 48, 48, 46, 48])
    }
}

pub fn test_rust_owned_record_field_copy(proven: bool, copy: bool, wrappers: Vec<self::ml_string::Text>) -> bool {
    (proven && ({
        let source_ml_source2_comparison_operand5 = self::ml_array::optional_at(&wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand6 = self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]);
            match (source_ml_source2_comparison_operand5.clone(), source_ml_source2_comparison_operand6.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    } || ((((wrappers.len() as f64)) == (0f64)) && copy)))
}

pub fn read_rust_owned_record_field_mode(copy: bool, borrowed: bool) -> self::ml_string::Text {
    if copy {
        self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100, 32, 118, 97, 108, 117, 101])
    } else {
        if borrowed {
            self::ml_string::Text::from_units(vec![98, 111, 114, 114, 111, 119, 101, 100, 32, 112, 108, 97, 99, 101])
        } else {
            self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 112, 108, 97, 99, 101])
        }
    }
}

pub fn read_rust_owned_record_field_identity(root: self::ml_string::Text, index: f64) -> self::ml_string::Text {
    if ((((root != (self::ml_string::Text::from_units(vec![]))) && self::ml_math::is_integer(index)) && (index >= (0f64))) && (index < (1024f64))) {
        (((self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32])).concat(&root)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&{
            let source_ml_source2_string_argument7 = index;
            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument7))
        })
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_rust_owned_record_field_root(identity: self::ml_string::Text) -> self::ml_string::Text {
    if !(identity.starts_with(&self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let boundary = {
                let source_ml_source2_string_search_operand8 = identity.clone();
                {
                    let source_ml_source2_string_search_operand9 = self::ml_string::Text::from_units(vec![32]);
                    {
                        let source_ml_source2_string_search_text10 = source_ml_source2_string_search_operand9.clone();
                        {
                            let source_ml_source2_string_search_position11 = f64::NAN;
                            source_ml_source2_string_search_operand8.last_index_of(&source_ml_source2_string_search_text10, source_ml_source2_string_search_position11)
                        }
                    }
                }
            };
            {
                let root = {
                    let source_ml_source2_string_range_operand12 = identity.clone();
                    {
                        let source_ml_source2_string_range_operand13 = 13f64;
                        {
                            let source_ml_source2_string_range_operand14 = boundary;
                            {
                                let source_ml_source2_string_range_position15 = source_ml_source2_string_range_operand13;
                                {
                                    let source_ml_source2_string_range_position16 = source_ml_source2_string_range_operand14;
                                    source_ml_source2_string_range_operand12.slice(source_ml_source2_string_range_position15, source_ml_source2_string_range_position16)
                                }
                            }
                        }
                    }
                };
                {
                    let index = self::ml_string_number::convert_java_script_string_number_units(&({
                        let source_ml_source2_string_range_operand17 = identity.clone();
                        {
                            let source_ml_source2_string_range_operand18 = (boundary + 1f64);
                            {
                                let source_ml_source2_string_range_position19 = source_ml_source2_string_range_operand18;
                                {
                                    let source_ml_source2_string_range_position20 = f64::INFINITY;
                                    source_ml_source2_string_range_operand17.slice(source_ml_source2_string_range_position19, source_ml_source2_string_range_position20)
                                }
                            }
                        }
                    }).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                    if ((self::read_rust_owned_record_field_identity(root.clone(), index)) == identity) {
                        root.clone()
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_record_absent_field_state(root: self::ml_string::Text) -> self::ml_string::Text {
    if (root == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) {
        self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])
    } else {
        self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
    }
}

pub fn read_rust_owned_record_ultimate_root(identity: self::ml_string::Text) -> self::ml_string::Text {
    {
        let root = self::read_rust_owned_record_field_root(identity.clone());
        if (root == (self::ml_string::Text::from_units(vec![]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let depth = 0f64;
                {
                    let source_ml_source2_s21 = self::source_ml_source1_temporary_read_rust_owned_record_ultimate_root_loop3(root.clone(), depth);
                    match source_ml_source2_s21.clone() {
                        self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Return(source_ml_source1_temporary_result3) => {
                            source_ml_source1_temporary_result3.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn test_rust_owned_record_field_descendant(root: self::ml_string::Text, identity: self::ml_string::Text) -> bool {
    {
        let parent = self::read_rust_owned_record_field_root(identity.clone());
        {
            let depth = 0f64;
            {
                let source_ml_source2_s22 = self::source_ml_source1_temporary_test_rust_owned_record_field_descendant_loop4(root.clone(), parent.clone(), depth);
                match source_ml_source2_s22.clone() {
                    self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Done => {
                        false
                    }
                    self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Return(source_ml_source1_temporary_result4) => {
                        source_ml_source1_temporary_result4
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_record_whole_error(root: self::ml_string::Text, identities: Vec<self::ml_string::Text>, states: Vec<self::ml_string::Text>) -> self::ml_string::Text {
    if (((identities.len() as f64)) != ((states.len() as f64))) {
        self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 97, 118, 97, 105, 108, 97, 98, 105, 108, 105, 116, 121, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 116, 97, 116, 101])
    } else {
        if (root == (self::ml_string::Text::from_units(vec![]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_temporary_values5 = identities;
                    {
                        let source_ml_source1_temporary_index5 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s23 = self::source_ml_source1_temporary_read_rust_owned_record_whole_error_loop6(root.clone(), states, index, source_ml_source1_temporary_values5, source_ml_source1_temporary_index5.clone());
                            match source_ml_source2_s23.clone() {
                                self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Done => {
                                    self::ml_string::Text::from_units(vec![])
                                }
                                self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Return(source_ml_source1_temporary_result6) => {
                                    source_ml_source1_temporary_result6.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_record_field_error(root: self::ml_string::Text, field: self::ml_string::Text) -> self::ml_string::Text {
    if ((root == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) && (field == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![97, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 117, 110, 109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100])
    }
}

pub fn read_rust_owned_record_update_error(profile: self::ml_string::Text, owner: bool, layers: f64, fields: f64, positions: f64) -> self::ml_string::Text {
    if ((((((((profile == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))) || (profile == (self::ml_string::Text::from_units(vec![99, 111, 112, 121])))) && owner) && (layers == (0f64))) && self::ml_math::is_integer(fields)) && (fields >= (0f64))) && (fields <= (1024f64))) && (positions == fields)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 115, 116, 114, 117, 99, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 102, 105, 101, 108, 100, 32, 108, 97, 121, 111, 117, 116])
    }
}

pub fn accept_rust_mutable_record_value_loans(reference: bool, place: bool, copy: bool, contained: f64) -> bool {
    ((reference || (place && !copy)) || (contained > (0f64)))
}

pub fn select_rust_mutable_record_loan_root(root: self::ml_string::Text, lexical: self::ml_string::Text, loans: Vec<self::ml_string::Text>) -> bool {
    if (root == (self::ml_string::Text::from_units(vec![]))) {
        false
    } else {
        {
            let source_ml_source1_temporary_values7 = loans;
            {
                let source_ml_source1_temporary_index7 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s24 = self::source_ml_source1_temporary_select_rust_mutable_record_loan_root_loop8(root.clone(), lexical.clone(), source_ml_source1_temporary_values7, source_ml_source1_temporary_index7.clone());
                    match source_ml_source2_s24.clone() {
                        self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Done => {
                            false
                        }
                        self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Return(source_ml_source1_temporary_result8) => {
                            source_ml_source1_temporary_result8
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_mutable_record_loan_identity(ordinal: f64) -> self::ml_string::Text {
    if self::accept_rust_owned_record_loop_count(ordinal) {
        (self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 97, 110, 32])).concat(&{
            let source_ml_source2_string_argument25 = ordinal;
            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument25))
        })
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_rust_mutable_record_loan_budget_error(nodes: f64, text: f64, added_nodes: f64, added_text: f64) -> self::ml_string::Text {
    if (((((!(self::accept_rust_owned_record_loop_count(nodes)) || !(self::accept_rust_owned_record_loop_count(text))) || !(self::accept_rust_owned_record_loop_count(added_nodes))) || !(self::accept_rust_owned_record_loop_count(added_text))) || (nodes > ((100000f64 - added_nodes)))) || (text > ((16777216f64 - added_text)))) {
        self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 32, 108, 111, 97, 110, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn accept_rust_mutable_record_loan_escape(carried: bool, root_survives: bool, live: bool) -> bool {
    ((carried && root_survives) && live)
}

pub fn read_rust_mutable_record_loan_move_error(owned: bool, overlapping: bool) -> self::ml_string::Text {
    if (owned && overlapping) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 109, 111, 118, 101, 32, 111, 118, 101, 114, 108, 97, 112, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 114, 118, 105, 118, 105, 110, 103, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 32, 108, 111, 97, 110])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn source_ml_source1_temporary_plan_rust_owned_record_loop_iterations_loop1(mut result: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (255f64)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![index], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result {
    MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Done,
    MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_rust_owned_record_loop_progress_error_loop2(mut exits: Vec<self::ml_string::Text>, mut counts: Vec<f64>, mut budget_error: self::ml_string::Text, mut index: f64) -> self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result {
    loop {
        return if (index < ((exits.len() as f64))) {
            {
                let exit = self::ml_array::optional_at(&exits, self::ml_array::number_index(index));
                {
                    let count = self::ml_array::optional_at(&counts, self::ml_array::number_index(index));
                    if {
                        let source_ml_source2_comparison_operand26 = exit.clone();
                        {
                            let source_ml_source2_comparison_operand27 = None::<()>;
                            match (source_ml_source2_comparison_operand26.clone(), source_ml_source2_comparison_operand27.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                    } else {
                        if {
                            let source_ml_source2_comparison_operand28 = count.clone();
                            {
                                let source_ml_source2_comparison_operand29 = None::<()>;
                                match (source_ml_source2_comparison_operand28.clone(), source_ml_source2_comparison_operand29.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                            }
                        } {
                            self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                        } else {
                            if ((!(self::accept_rust_owned_record_loop_exit(exit.clone().expect("a proven present value"))) || !(self::accept_rust_owned_record_loop_count(count.clone().expect("a proven present value")))) || ((count.clone().expect("a proven present value")) > (256f64))) {
                                self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                            } else {
                                if (self::test_rust_owned_record_loop_continuing_exit(exit.clone().expect("a proven present value")) && ((count.clone().expect("a proven present value")) != (1f64))) {
                                    self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 111, 110, 116, 105, 110, 117, 105, 110, 103, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 111, 112, 32, 112, 97, 116, 104, 32, 109, 117, 115, 116, 32, 100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 117, 110, 116, 101, 114, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 99, 101]))
                                } else {
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            index = index_2;
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
            self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result {
    MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Done,
    MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_rust_owned_record_ultimate_root_loop3(mut root: self::ml_string::Text, mut depth: f64) -> self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result {
    loop {
        return if (depth < (1024f64)) {
            {
                let parent = self::read_rust_owned_record_field_root(root.clone());
                if (parent == (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Return(root.clone())
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
            self::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Result::MlSource1TemporaryReadRustOwnedRecordUltimateRootLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result {
    MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Done,
    MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Return(bool),
}

pub fn source_ml_source1_temporary_test_rust_owned_record_field_descendant_loop4(mut root: self::ml_string::Text, mut parent: self::ml_string::Text, mut depth: f64) -> self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result {
    loop {
        return if ((depth < (1024f64)) && (parent != (self::ml_string::Text::from_units(vec![])))) {
            if (parent == root) {
                self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Return(true)
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
            self::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Result::MlSource1TemporaryTestRustOwnedRecordFieldDescendantLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result {
    MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Done,
    MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_rust_owned_record_whole_error_loop6(mut root: self::ml_string::Text, mut states: Vec<self::ml_string::Text>, mut index: f64, mut source_ml_source1_temporary_values5: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_index5: self::ml::Big) -> self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result {
    loop {
        return if (source_ml_source1_temporary_index5 < (self::ml::Big::from_u128(source_ml_source1_temporary_values5.len() as u128))) {
            {
                let identity = self::ml_array::at(&source_ml_source1_temporary_values5, source_ml_source1_temporary_index5.to_index());
                if (self::test_rust_owned_record_field_descendant(root.clone(), identity.clone()) && {
                    let source_ml_source2_comparison_operand30 = self::ml_array::optional_at(&states, self::ml_array::number_index(index));
                    {
                        let source_ml_source2_comparison_operand31 = self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]);
                        !(match (source_ml_source2_comparison_operand30.clone(), source_ml_source2_comparison_operand31.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                    }
                }) {
                    self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Return(self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            let source_ml_source1_temporary_index5_2 = source_ml_source1_temporary_index5.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_source1_temporary_index5) = (index_2, source_ml_source1_temporary_index5_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Result::MlSource1TemporaryReadRustOwnedRecordWholeErrorLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result {
    MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Done,
    MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Return(bool),
}

pub fn source_ml_source1_temporary_select_rust_mutable_record_loan_root_loop8(mut root: self::ml_string::Text, mut lexical: self::ml_string::Text, mut source_ml_source1_temporary_values7: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_index7: self::ml::Big) -> self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result {
    loop {
        return if (source_ml_source1_temporary_index7 < (self::ml::Big::from_u128(source_ml_source1_temporary_values7.len() as u128))) {
            {
                let loan = self::ml_array::at(&source_ml_source1_temporary_values7, source_ml_source1_temporary_index7.to_index());
                if (((loan == root) || ((lexical != (self::ml_string::Text::from_units(vec![]))) && (loan == lexical))) || self::test_rust_owned_record_field_descendant(root.clone(), loan.clone())) {
                    self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Return(true)
                } else {
                    {
                        let source_ml_source1_temporary_index7_2 = source_ml_source1_temporary_index7.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index7 = source_ml_source1_temporary_index7_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Result::MlSource1TemporarySelectRustMutableRecordLoanRootLoop8Done
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_mutable_record_value_loans(true, false, true, 0f64)) == (true)), "assertion 1");
    assert!(((self::accept_rust_mutable_record_value_loans(false, true, false, 0f64)) == (true)), "assertion 2");
    assert!(((self::accept_rust_mutable_record_value_loans(false, true, true, 0f64)) == (false)), "assertion 3");
    assert!(((self::accept_rust_mutable_record_value_loans(false, false, true, 1f64)) == (true)), "assertion 4");
    assert!(((self::select_rust_mutable_record_loan_root(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 56]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 49])])) == (true)), "assertion 5");
    assert!(((self::select_rust_mutable_record_loan_root(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 56]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 48, 32, 49])])) == (false)), "assertion 6");
    assert!(((self::select_rust_mutable_record_loan_root(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 56]), vec![self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 56])])) == (true)), "assertion 7");
    assert!(((self::read_rust_mutable_record_loan_identity(9f64)) == (self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 97, 110, 32, 57]))), "assertion 8");
    assert!(((self::read_rust_mutable_record_loan_identity((-1f64))) == (self::ml_string::Text::from_units(vec![]))), "assertion 9");
    assert!(((self::read_rust_mutable_record_loan_identity(1.5f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 10");
    assert!(((self::read_rust_mutable_record_loan_budget_error(0f64, 0f64, 100000f64, 16777216f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 11");
    assert!(((self::read_rust_mutable_record_loan_budget_error(1f64, 0f64, 100000f64, 0f64)) == (self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 32, 108, 111, 97, 110, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))), "assertion 12");
    assert!(((self::read_rust_mutable_record_loan_budget_error(0f64, 1f64, 0f64, 16777216f64)) == (self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 32, 108, 111, 97, 110, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))), "assertion 13");
    assert!(((self::accept_rust_mutable_record_loan_escape(true, true, true)) == (true)), "assertion 14");
    assert!(((self::accept_rust_mutable_record_loan_escape(false, true, true)) == (false)), "assertion 15");
    assert!(((self::accept_rust_mutable_record_loan_escape(true, false, true)) == (false)), "assertion 16");
    assert!(((self::accept_rust_mutable_record_loan_escape(true, true, false)) == (false)), "assertion 17");
    assert!(((self::read_rust_mutable_record_loan_move_error(true, true)) == (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 109, 111, 118, 101, 32, 111, 118, 101, 114, 108, 97, 112, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 114, 118, 105, 118, 105, 110, 103, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 32, 108, 111, 97, 110]))), "assertion 18");
    assert!(((self::read_rust_mutable_record_loan_move_error(true, false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 19");
    assert!(((self::read_rust_mutable_record_loan_move_error(false, true)) == (self::ml_string::Text::from_units(vec![]))), "assertion 20");
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
fn all_literal_mutable_record_loan_controls() { main(); }
