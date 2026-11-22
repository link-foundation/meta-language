pub fn source_ml_temporary_read_datatype_constructive_plan_loop17(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut mutual: bool, mut work: f64, mut recursive: bool, mut declaration: f64, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut domains: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut crossings: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> self::MlTemporaryReadDatatypeConstructivePlanLoop17Result {
    loop {
        return if (({ let ml_reference_array56 = pending.clone(); let ml_reference_projection56 = ml_reference_array56.borrow().len() as f64; ml_reference_projection56 }) > (0f64)) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryReadDatatypeConstructivePlanLoop17Result::MlTemporaryReadDatatypeConstructivePlanLoop17Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("callable data graph reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let at = self::read_required_number_fact(pending.clone(), ({ let ml_reference_array57 = pending.clone(); let ml_reference_projection57 = ml_reference_array57.borrow().len() as f64; ml_reference_projection57 } - 1f64));
                        {
                            let domain = self::read_required_boolean_fact(domains.clone(), ({ let ml_reference_array58 = domains.clone(); let ml_reference_projection58 = ml_reference_array58.borrow().len() as f64; ml_reference_projection58 } - 1f64));
                            {
                                let crossed = self::read_required_boolean_fact(crossings.clone(), ({ let ml_reference_array59 = crossings.clone(); let ml_reference_projection59 = ml_reference_array59.borrow().len() as f64; ml_reference_projection59 } - 1f64));
                                {
                                    let key = (((at * 4f64) + if domain {
                                        2f64
                                    } else {
                                        0f64
                                    }) + if crossed {
                                        1f64
                                    } else {
                                        0f64
                                    });
                                    {
                                        let source_ml_temporary_read_datatype_constructive_plan_ignored18 = { let ml_method_owner60 = pending.clone(); let ml_result = ml_method_owner60.borrow_mut().pop(); ml_result };
                                        {
                                            let source_ml_temporary_read_datatype_constructive_plan_ignored19 = { let ml_method_owner61 = domains.clone(); let ml_result = ml_method_owner61.borrow_mut().pop(); ml_result };
                                            {
                                                let source_ml_temporary_read_datatype_constructive_plan_ignored20 = { let ml_method_owner62 = crossings.clone(); let ml_result = ml_method_owner62.borrow_mut().pop(); ml_result };
                                                if self::read_required_boolean_fact(seen.clone(), key) {
                                                    {
                                                        work = work_2;
                                                        continue;
                                                    }
                                                } else {
                                                    {
                                                        let source_ml_temporary_read_datatype_constructive_plan_ignored21 = { let ml_array_owner63 = seen.clone(); let ml_array_argument63_0: f64 = key; let ml_array_argument63_1: f64 = 1f64; let ml_array_argument63_2: bool = true; let ml_array_length = ml_array_owner63.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument63_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument63_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument63_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner63.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                        {
                                                            let reference = self::read_required_number_fact(references.clone(), at);
                                                            if (reference >= (0f64)) {
                                                                if (reference == declaration) {
                                                                    if domain {
                                                                        self::MlTemporaryReadDatatypeConstructivePlanLoop17Result::MlTemporaryReadDatatypeConstructivePlanLoop17Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive callable data in a function domain lacks strict positivity")])))
                                                                    } else {
                                                                        if (crossed && !mutual) {
                                                                            self::MlTemporaryReadDatatypeConstructivePlanLoop17Result::MlTemporaryReadDatatypeConstructivePlanLoop17Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("mutually recursive callable data needs a mutual-inductive backend profile")])))
                                                                        } else {
                                                                            {
                                                                                let recursive_2 = true;
                                                                                {
                                                                                    (work, recursive) = (work_2, recursive_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let source_ml_temporary_read_datatype_constructive_plan_ignored22 = { let ml_method_owner64 = pending.clone(); let ml_method_argument64_0 = self::read_required_number_fact(roots.clone(), reference); let ml_inserted = vec![ml_method_argument64_0]; let mut ml_method_values64 = ml_method_owner64.borrow_mut(); let ml_length = ml_method_values64.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values64.extend(ml_inserted); ml_length as f64 };
                                                                        {
                                                                            let source_ml_temporary_read_datatype_constructive_plan_ignored23 = { let ml_method_owner65 = domains.clone(); let ml_method_argument65_0 = domain; let ml_inserted = vec![ml_method_argument65_0]; let mut ml_method_values65 = ml_method_owner65.borrow_mut(); let ml_length = ml_method_values65.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values65.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let source_ml_temporary_read_datatype_constructive_plan_ignored24 = { let ml_method_owner66 = crossings.clone(); let ml_method_argument66_0 = true; let ml_inserted = vec![ml_method_argument66_0]; let mut ml_method_values66 = ml_method_owner66.borrow_mut(); let ml_length = ml_method_values66.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values66.extend(ml_inserted); ml_length as f64 };
                                                                                {
                                                                                    work = work_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let start = if (at == (0f64)) {
                                                                        0f64
                                                                    } else {
                                                                        self::read_required_number_fact(ends.clone(), (at - 1f64))
                                                                    };
                                                                    {
                                                                        let end = self::read_required_number_fact(ends.clone(), at);
                                                                        {
                                                                            let child = start;
                                                                            {
                                                                                let source_ml_source1_s16 = self::source_ml_temporary_read_datatype_constructive_plan_loop25(kinds.clone(), edges.clone(), pending.clone(), domains.clone(), crossings.clone(), at, domain, crossed, end, child);
                                                                                match source_ml_source1_s16.clone() {
                                                                                    self::MlTemporaryReadDatatypeConstructivePlanLoop25Result::MlTemporaryReadDatatypeConstructivePlanLoop25Done => {
                                                                                        {
                                                                                            work = work_2;
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
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop17Result::MlTemporaryReadDatatypeConstructivePlanLoop17Done(work, recursive)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop16Result {
    MlTemporaryReadDatatypeConstructivePlanLoop16Done,
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop16(mut count: f64, mut seen: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop16Result {
    loop {
        return if (at < ((count * 4f64))) {
            {
                let source_ml_temporary_read_datatype_constructive_plan_ignored29 = { let ml_method_owner67 = seen.clone(); let ml_method_argument67_0 = false; let ml_inserted = vec![ml_method_argument67_0]; let mut ml_method_values67 = ml_method_owner67.borrow_mut(); let ml_length = ml_method_values67.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values67.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop16Result::MlTemporaryReadDatatypeConstructivePlanLoop16Done
        };
    }
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop2(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut mutual: bool, mut count: f64, mut work: f64, mut recursive: bool, mut declaration: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop2Result {
    loop {
        return if (declaration < ({ let ml_reference_array68 = roots.clone(); let ml_reference_projection68 = ml_reference_array68.borrow().len() as f64; ml_reference_projection68 })) {
            {
                let work_2 = (work + (count * 4f64));
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryReadDatatypeConstructivePlanLoop2Result::MlTemporaryReadDatatypeConstructivePlanLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("callable data graph reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_required_number_fact(roots.clone(), declaration)]));
                        {
                            let domains = std::rc::Rc::new(std::cell::RefCell::new(vec![false]));
                            {
                                let crossings = std::rc::Rc::new(std::cell::RefCell::new(vec![false]));
                                {
                                    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s17 = self::source_ml_temporary_read_datatype_constructive_plan_loop16(count, seen.clone(), at);
                                            match source_ml_source1_s17.clone() {
                                                self::MlTemporaryReadDatatypeConstructivePlanLoop16Result::MlTemporaryReadDatatypeConstructivePlanLoop16Done => {
                                                    {
                                                        let source_ml_source1_s18 = self::source_ml_temporary_read_datatype_constructive_plan_loop17(kinds.clone(), ends.clone(), edges.clone(), references.clone(), roots.clone(), mutual, work_2, recursive, declaration, pending.clone(), domains.clone(), crossings.clone(), seen.clone());
                                                        match source_ml_source1_s18.clone() {
                                                            self::MlTemporaryReadDatatypeConstructivePlanLoop17Result::MlTemporaryReadDatatypeConstructivePlanLoop17Done(work_3, recursive_2) => {
                                                                {
                                                                    let declaration_2 = (declaration + 1f64);
                                                                    {
                                                                        (work, recursive, declaration) = (work_3, recursive_2, declaration_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadDatatypeConstructivePlanLoop17Result::MlTemporaryReadDatatypeConstructivePlanLoop17Return(source_ml_temporary_result17) => {
                                                                self::MlTemporaryReadDatatypeConstructivePlanLoop2Result::MlTemporaryReadDatatypeConstructivePlanLoop2Return(source_ml_temporary_result17.clone())
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
            self::MlTemporaryReadDatatypeConstructivePlanLoop2Result::MlTemporaryReadDatatypeConstructivePlanLoop2Done(work, recursive)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop1Result {
    MlTemporaryReadDatatypeConstructivePlanLoop1Done(f64),
    MlTemporaryReadDatatypeConstructivePlanLoop1Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop30Result {
    MlTemporaryReadDatatypeConstructivePlanLoop30Done,
    MlTemporaryReadDatatypeConstructivePlanLoop30Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop30(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut end: f64, mut child: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop30Result {
    loop {
        return if (child < end) {
            {
                let index = self::read_required_number_fact(edges.clone(), child);
                if ((!(self::ml_math::is_integer(index)) || (index < (0f64))) || (index >= at)) {
                    self::MlTemporaryReadDatatypeConstructivePlanLoop30Result::MlTemporaryReadDatatypeConstructivePlanLoop30Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("callable data children need complete postorder identities")])))
                } else {
                    {
                        let child_2 = (child + 1f64);
                        {
                            child = child_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop30Result::MlTemporaryReadDatatypeConstructivePlanLoop30Done
        };
    }
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop1(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut previous: f64, mut at: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop1Result {
    loop {
        return if (at < count) {
            {
                let end = self::read_required_number_fact(ends.clone(), at);
                {
                    let reference = self::read_required_number_fact(references.clone(), at);
                    if (((((!(self::ml_math::is_integer(end)) || (end < previous)) || (end > ({ let ml_reference_array69 = edges.clone(); let ml_reference_projection69 = ml_reference_array69.borrow().len() as f64; ml_reference_projection69 }))) || !(self::ml_math::is_integer(reference))) || (reference < ((-1f64)))) || (reference >= ({ let ml_reference_array70 = roots.clone(); let ml_reference_projection70 = ml_reference_array70.borrow().len() as f64; ml_reference_projection70 }))) {
                        self::MlTemporaryReadDatatypeConstructivePlanLoop1Result::MlTemporaryReadDatatypeConstructivePlanLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete callable data graph")])))
                    } else {
                        {
                            let child = previous;
                            {
                                let source_ml_source1_s19 = self::source_ml_temporary_read_datatype_constructive_plan_loop30(edges.clone(), at, end, child);
                                match source_ml_source1_s19.clone() {
                                    self::MlTemporaryReadDatatypeConstructivePlanLoop30Result::MlTemporaryReadDatatypeConstructivePlanLoop30Done => {
                                        if (((reference >= (0f64)) && (((self::read_required_text_fact(kinds.clone(), at)) != "reference") || (end != previous))) || ((reference < (0f64)) && ((self::read_required_text_fact(kinds.clone(), at)) == "reference"))) {
                                            self::MlTemporaryReadDatatypeConstructivePlanLoop1Result::MlTemporaryReadDatatypeConstructivePlanLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("callable data reference needs its actual leaf identity")])))
                                        } else {
                                            {
                                                let previous_2 = end;
                                                {
                                                    let at_2 = (at + 1f64);
                                                    {
                                                        (previous, at) = (previous_2, at_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadDatatypeConstructivePlanLoop30Result::MlTemporaryReadDatatypeConstructivePlanLoop30Return(source_ml_temporary_result30) => {
                                        self::MlTemporaryReadDatatypeConstructivePlanLoop1Result::MlTemporaryReadDatatypeConstructivePlanLoop1Return(source_ml_temporary_result30.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop1Result::MlTemporaryReadDatatypeConstructivePlanLoop1Done(previous)
        };
    }
}

pub fn source_ml_temporary_render_callable_data_lean_default_loop31(mut fields: f64, mut value: String, mut at: f64) -> String {
    loop {
        return if (at < fields) {
            {
                let value_2 = format!("{}{}", value, (String::from(" default")));
                {
                    let at_2 = (at + 1f64);
                    {
                        (value, at) = (value_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

fn ml_main() {
    assert!(((self::read_formal_datatype_constructive_plan(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("constructor"), String::from("data"), String::from("constructor"), String::from("reference"), String::from("constructor"), String::from("data"), String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 2f64, 2f64, 3f64, 5f64, 5f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 4f64, 3f64, 5f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, (-1f64), (-1f64), (-1f64), 0f64, (-1f64), (-1f64), 0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, 6f64])), 7f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("witnesses"), String::from("0"), String::from("2"), String::from("0"), String::from("1")])))), "assertion 1");
    assert!(((self::read_formal_datatype_constructive_plan(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("constructor"), String::from("data"), String::from("reference"), String::from("constructor"), String::from("data"), String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 2f64, 3f64, 4f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 3f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, (-1f64), (-1f64), 0f64, (-1f64), (-1f64), 0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, 5f64])), 6f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("callable data has no finite constructive witness for every referenced datatype")])))), "assertion 2");
    assert!(((self::read_formal_datatype_constructive_plan(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("bool"), String::from("function"), String::from("constructor"), String::from("constructor"), String::from("data"), String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 2f64, 3f64, 3f64, 5f64, 5f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64), (-1f64), (-1f64), 0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![5f64])), 6f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive callable data in a function domain lacks strict positivity")])))), "assertion 3");
    assert!(((self::read_formal_datatype_constructive_plan(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete callable data declaration roots")])))), "assertion 4");
    assert!(((self::read_formal_datatype_lean_block_derivation(false, false, false)) == "Repr, DecidableEq"), "assertion 5");
    assert!(((self::read_formal_datatype_lean_block_derivation(false, false, true)) == "Repr"), "assertion 6");
    assert!(((self::read_formal_datatype_lean_block_derivation(true, false, false)) == ""), "assertion 7");
    assert!(((self::read_formal_datatype_lean_block_derivation(false, true, false)) == ""), "assertion 8");
    assert!(((self::render_formal_datatype_rocq_mutual_block(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Inductive First : Type :="), String::from("| End : First"), String::from("with Second : Type :="), String::from("| More : First -> Second.")])))) == "#[warnings=\"-register-all\"]\nInductive First : Type :=\n| End : First\nwith Second : Type :=\n| More : First -> Second."), "assertion 9");
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
fn all_literal_formal_mutual_datatype_controls() { main(); }
