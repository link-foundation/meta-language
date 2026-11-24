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

pub fn source_ml_temporary_read_rust_owned_record_whole_error_loop4(mut root: self::ml_string::Text, mut states: Vec<self::ml_string::Text>, mut index: f64, mut source_ml_temporary_values3: Vec<self::ml_string::Text>, mut source_ml_temporary_index3: self::ml::Big) -> self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let identity = self::ml_array::at(&source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                if (self::test_rust_owned_record_field_descendant(root.clone(), identity.clone()) && {
                    let source_ml_source1_comparison_operand20 = self::ml_array::optional_at(&states, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand21 = self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]);
                        !(match (source_ml_source1_comparison_operand20.clone(), source_ml_source1_comparison_operand21.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
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

fn ml_main() {
    assert!(((self::read_rust_owned_record_update_version_error(90f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 1");
    assert!(((self::read_rust_owned_record_update_version_error(99f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 2");
    assert!(((self::read_rust_owned_record_update_version_error(89f64)) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 111, 117, 116, 115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61, 49, 46, 57, 48, 46, 48, 32, 60, 49, 46, 49, 48, 48, 46, 48]))), "assertion 3");
    assert!(((self::read_rust_owned_record_update_version_error(100f64)) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 111, 117, 116, 115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61, 49, 46, 57, 48, 46, 48, 32, 60, 49, 46, 49, 48, 48, 46, 48]))), "assertion 4");
    assert!(((self::read_rust_owned_record_field_identity(self::ml_string::Text::from_units(vec![55]), 0f64)) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]))), "assertion 5");
    assert!(((self::read_rust_owned_record_field_identity(self::ml_string::Text::from_units(vec![55]), 1023f64)) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 49, 48, 50, 51]))), "assertion 6");
    assert!(((self::read_rust_owned_record_field_identity(self::ml_string::Text::from_units(vec![55]), 1024f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 7");
    assert!(((self::read_rust_owned_record_field_identity(self::ml_string::Text::from_units(vec![]), 0f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 8");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]))) == (self::ml_string::Text::from_units(vec![55]))), "assertion 9");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 48]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 10");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 49, 48, 50, 52]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 11");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 55]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 12");
    assert!(((self::read_rust_owned_record_field_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49]))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48]))), "assertion 13");
    assert!(((self::read_rust_owned_record_ultimate_root(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49]))) == (self::ml_string::Text::from_units(vec![55]))), "assertion 14");
    assert!(((self::test_rust_owned_record_field_descendant(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49]))) == (true)), "assertion 15");
    assert!(((self::test_rust_owned_record_field_descendant(self::ml_string::Text::from_units(vec![55]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 48, 32, 48]))) == (false)), "assertion 16");
    assert!(((self::read_rust_owned_record_whole_error(self::ml_string::Text::from_units(vec![55]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48])], vec![self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])])) == (self::ml_string::Text::from_units(vec![]))), "assertion 17");
    assert!(((self::read_rust_owned_record_whole_error(self::ml_string::Text::from_units(vec![55]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48])], vec![self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100])])) == (self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))), "assertion 18");
    assert!(((self::read_rust_owned_record_whole_error(self::ml_string::Text::from_units(vec![55]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 32, 48, 32, 49])], vec![self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100])])) == (self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))), "assertion 19");
    assert!(((self::read_rust_owned_record_whole_error(self::ml_string::Text::from_units(vec![55]), vec![self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 55, 48, 32, 48])], vec![self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100])])) == (self::ml_string::Text::from_units(vec![]))), "assertion 20");
    assert!(((self::read_rust_owned_record_field_error(self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 21");
    assert!(((self::read_rust_owned_record_field_error(self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]), self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]))) == (self::ml_string::Text::from_units(vec![97, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 117, 110, 109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100]))), "assertion 22");
    assert!(((self::read_rust_owned_record_field_error(self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]), self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) == (self::ml_string::Text::from_units(vec![97, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 117, 110, 109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100]))), "assertion 23");
    assert!(((self::read_rust_owned_record_absent_field_state(self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))) == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))), "assertion 24");
    assert!(((self::read_rust_owned_record_absent_field_state(self::ml_string::Text::from_units(vec![109, 111, 118, 101, 100]))) == (self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116]))), "assertion 25");
    assert!(((self::read_rust_owned_record_update_error(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), true, 0f64, 2f64, 2f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 26");
    assert!(((self::read_rust_owned_record_update_error(self::ml_string::Text::from_units(vec![99, 111, 112, 121]), true, 0f64, 2f64, 2f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 27");
    assert!(((self::read_rust_owned_record_update_error(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), false, 0f64, 2f64, 2f64)) == (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 115, 116, 114, 117, 99, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 102, 105, 101, 108, 100, 32, 108, 97, 121, 111, 117, 116]))), "assertion 28");
    assert!(((self::read_rust_owned_record_update_error(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), true, 1f64, 2f64, 2f64)) == (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101, 101, 32, 115, 116, 114, 117, 99, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 102, 105, 101, 108, 100, 32, 108, 97, 121, 111, 117, 116]))), "assertion 29");
    assert!(((self::test_rust_owned_record_field_copy(true, true, Vec::<self::ml_string::Text>::new())) == (true)), "assertion 30");
    assert!(((self::test_rust_owned_record_field_copy(true, false, vec![self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101])])) == (true)), "assertion 31");
    assert!(((self::test_rust_owned_record_field_copy(true, false, vec![self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101]), self::ml_string::Text::from_units(vec![98, 111, 120])])) == (true)), "assertion 32");
    assert!(((self::test_rust_owned_record_field_copy(true, true, vec![self::ml_string::Text::from_units(vec![98, 111, 120])])) == (false)), "assertion 33");
    assert!(((self::test_rust_owned_record_field_copy(false, true, Vec::<self::ml_string::Text>::new())) == (false)), "assertion 34");
    assert!(((self::read_rust_owned_record_field_mode(true, true)) == (self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100, 32, 118, 97, 108, 117, 101]))), "assertion 35");
    assert!(((self::read_rust_owned_record_field_mode(false, true)) == (self::ml_string::Text::from_units(vec![98, 111, 114, 114, 111, 119, 101, 100, 32, 112, 108, 97, 99, 101]))), "assertion 36");
    assert!(((self::read_rust_owned_record_field_mode(false, false)) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 112, 108, 97, 99, 101]))), "assertion 37");
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
fn all_literal_owned_record_update_controls() { main(); }
