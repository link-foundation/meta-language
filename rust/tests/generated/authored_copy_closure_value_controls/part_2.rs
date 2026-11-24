pub fn source_ml_temporary_read_rust_copy_closure_value_borrow_error_loop7(mut source_ml_temporary_values6: Vec<String>, mut source_ml_temporary_index6: self::ml::Big) -> self::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Result {
    loop {
        return if (source_ml_temporary_index6 < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))) {
            {
                let state = self::ml_array::at(&source_ml_temporary_values6, source_ml_temporary_index6.to_index());
                if (state != "available") {
                    self::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Result::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Return(String::from("a borrowed Copy closure environment does not outlive this value"))
                } else {
                    {
                        let source_ml_temporary_index6_2 = source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index6 = source_ml_temporary_index6_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Result::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Result {
    MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Done,
    MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Result {
    MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Done,
    MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Return(String),
}

pub fn source_ml_temporary_read_rust_copy_closure_parameter_header_error_loop9(mut names: Vec<String>, mut index: f64, mut before: f64) -> self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Result {
    loop {
        return if (before < index) {
            if ((self::read_required_text_fact(names.clone(), before)) == (self::read_required_text_fact(names.clone(), index))) {
                self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Return(String::from("closure parameters require distinct actual bindings"))
            } else {
                {
                    let before_2 = (before + 1f64);
                    {
                        before = before_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Done
        };
    }
}

pub fn source_ml_temporary_read_rust_copy_closure_parameter_header_error_loop8(mut names: Vec<String>, mut copies: Vec<bool>, mut index: f64) -> self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Result {
    loop {
        return if (index < ((names.len() as f64))) {
            if !(self::read_required_boolean_fact(copies.clone(), index)) {
                self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Return(String::from("closure parameters require direct immutable Copy source types"))
            } else {
                {
                    let before = 0f64;
                    {
                        let source_ml_source1_s13 = self::source_ml_temporary_read_rust_copy_closure_parameter_header_error_loop9(names.clone(), index, before);
                        match source_ml_source1_s13.clone() {
                            self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Done => {
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        index = index_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop9Return(source_ml_temporary_result9) => {
                                self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Return(source_ml_temporary_result9.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Result {
    MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Done,
    MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Result {
    MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Done,
    MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Return(String),
}

pub fn source_ml_temporary_read_rust_copy_closure_pattern_binding_error_loop11(mut names: Vec<String>, mut index: f64, mut before: f64) -> self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Result {
    loop {
        return if (before < index) {
            if ((self::read_required_text_fact(names.clone(), before)) == (self::read_required_text_fact(names.clone(), index))) {
                self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Return(String::from("closure parameters require distinct actual bindings"))
            } else {
                {
                    let before_2 = (before + 1f64);
                    {
                        before = before_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Done
        };
    }
}

pub fn source_ml_temporary_read_rust_copy_closure_pattern_binding_error_loop10(mut names: Vec<String>, mut index: f64) -> self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Result {
    loop {
        return if (index < ((names.len() as f64))) {
            {
                let before = 0f64;
                {
                    let source_ml_source1_s14 = self::source_ml_temporary_read_rust_copy_closure_pattern_binding_error_loop11(names.clone(), index, before);
                    match source_ml_source1_s14.clone() {
                        self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Done => {
                            {
                                let index_2 = (index + 1f64);
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop11Return(source_ml_temporary_result11) => {
                            self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Return(source_ml_temporary_result11.clone())
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Done
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_copy_closure_value(true, vec![String::from("x")], vec![true], true, true, Vec::<bool>::new(), Vec::<bool>::new(), 0f64)) == (true)), "assertion 1");
    assert!(((self::accept_rust_copy_closure_value(true, Vec::<String>::new(), Vec::<bool>::new(), true, true, vec![true], vec![false], 0f64)) == (true)), "assertion 2");
    assert!(((self::accept_rust_copy_closure_value(true, Vec::<String>::new(), Vec::<bool>::new(), false, true, Vec::<bool>::new(), Vec::<bool>::new(), 0f64)) == (false)), "assertion 3");
    assert!(((self::accept_rust_copy_closure_value(true, Vec::<String>::new(), Vec::<bool>::new(), true, false, Vec::<bool>::new(), Vec::<bool>::new(), 0f64)) == (false)), "assertion 4");
    assert!(((self::accept_rust_copy_closure_value(true, Vec::<String>::new(), Vec::<bool>::new(), true, true, vec![false], vec![false], 0f64)) == (false)), "assertion 5");
    assert!(((self::accept_rust_copy_closure_value(true, Vec::<String>::new(), Vec::<bool>::new(), true, true, vec![true], vec![true], 0f64)) == (false)), "assertion 6");
    assert!(((self::accept_rust_copy_closure_value(true, Vec::<String>::new(), Vec::<bool>::new(), true, true, vec![true], Vec::<bool>::new(), 0f64)) == (false)), "assertion 7");
    assert!(((self::accept_rust_copy_closure_value(true, Vec::<String>::new(), Vec::<bool>::new(), true, true, Vec::<bool>::new(), Vec::<bool>::new(), 1f64)) == (false)), "assertion 8");
    assert!(((self::accept_rust_copy_closure_value(false, Vec::<String>::new(), Vec::<bool>::new(), true, true, Vec::<bool>::new(), Vec::<bool>::new(), 0f64)) == (false)), "assertion 9");
    assert!(((self::read_rust_copy_closure_value_borrow_error(Vec::<String>::new())) == ""), "assertion 10");
    assert!(((self::read_rust_copy_closure_value_borrow_error(vec![String::from("available"), String::from("available")])) == ""), "assertion 11");
    assert!(((self::read_rust_copy_closure_value_borrow_error(vec![String::from("available"), String::from("dropped")])) == "a borrowed Copy closure environment does not outlive this value"), "assertion 12");
    assert!(((self::read_rust_copy_closure_value_borrow_error(vec![String::from("")])) == "a borrowed Copy closure environment does not outlive this value"), "assertion 13");
    assert!(((self::read_rust_copy_closure_returned_borrow_error(0f64)) == ""), "assertion 14");
    assert!(((self::read_rust_copy_closure_returned_borrow_error(1f64)) == "returning a borrowed callable environment needs its explicit callable result lifetime contract"), "assertion 15");
    assert!(((self::read_rust_copy_closure_loan_capture_error(1f64, vec![false])) == ""), "assertion 16");
    assert!(((self::read_rust_copy_closure_loan_capture_error(1f64, Vec::<bool>::new())) == "a Copy closure environment needs every actual capture loan fact"), "assertion 17");
    assert!(((self::read_rust_copy_closure_loan_capture_error(1f64, vec![true])) == "a Copy closure environment cannot discard borrowed-place or local loan provenance"), "assertion 18");
    assert!(((self::read_rust_copy_closure_join_error(true, true, true)) == ""), "assertion 19");
    assert!(((self::read_rust_copy_closure_join_error(true, true, false)) == "distinct anonymous closure types require an actual source coercion"), "assertion 20");
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
fn all_literal_copy_closure_value_controls() { main(); }
