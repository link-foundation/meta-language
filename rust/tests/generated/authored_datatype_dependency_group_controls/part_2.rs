pub fn source_ml_temporary_plan_datatype_dependency_groups_loop18(mut count: f64, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut last: f64, mut edge: f64) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result {
    loop {
        return if (edge < last) {
            {
                let target = self::read_required_number_fact(edges.clone(), edge);
                if (((!(self::ml_math::is_integer(target)) || {
                    let source_ml_source1_same_value_operand16 = target;
                    {
                        let source_ml_source1_same_value_operand17 = (-0f64);
                        (((source_ml_source1_same_value_operand16 == source_ml_source1_same_value_operand17) && ((source_ml_source1_same_value_operand16 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand16)) == ((1f64 / source_ml_source1_same_value_operand17))))) || ((source_ml_source1_same_value_operand16 != source_ml_source1_same_value_operand16) && (source_ml_source1_same_value_operand17 != source_ml_source1_same_value_operand17)))
                    }
                }) || (target < (0f64))) || (target >= count)) {
                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result::MlTemporaryPlanDatatypeDependencyGroupsLoop18Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let next_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value33 = next.borrow().clone(); ml_reference_value33 }, vec![self::read_required_number_fact(heads.clone(), target)])));
                        {
                            let sources_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value34 = sources.borrow().clone(); ml_reference_value34 }, vec![node])));
                            {
                                let source_ml_temporary_plan_datatype_dependency_groups_ignored19 = { let ml_array_owner35 = heads.clone(); let ml_array_argument35_0: f64 = target; let ml_array_argument35_1: f64 = 1f64; let ml_array_argument35_2: f64 = edge; let ml_array_length = ml_array_owner35.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument35_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument35_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument35_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner35.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                {
                                    let edge_2 = (edge + 1f64);
                                    {
                                        (next, sources, edge) = (next_2.clone(), sources_2.clone(), edge_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result::MlTemporaryPlanDatatypeDependencyGroupsLoop18Done(next.clone(), sources.clone())
        };
    }
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop3(mut count: f64, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut first: f64, mut node: f64) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result {
    loop {
        return if (node < count) {
            {
                let last = self::read_required_number_fact(ends.clone(), node);
                if (((!(self::ml_math::is_integer(last)) || {
                    let source_ml_source1_same_value_operand18 = last;
                    {
                        let source_ml_source1_same_value_operand19 = (-0f64);
                        (((source_ml_source1_same_value_operand18 == source_ml_source1_same_value_operand19) && ((source_ml_source1_same_value_operand18 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand18)) == ((1f64 / source_ml_source1_same_value_operand19))))) || ((source_ml_source1_same_value_operand18 != source_ml_source1_same_value_operand18) && (source_ml_source1_same_value_operand19 != source_ml_source1_same_value_operand19)))
                    }
                }) || (last < first)) || (last > ({ let ml_reference_array36 = edges.clone(); let ml_reference_projection36 = ml_reference_array36.borrow().len() as f64; ml_reference_projection36 }))) {
                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let starts_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value37 = starts.borrow().clone(); ml_reference_value37 }, vec![first])));
                        {
                            let edge = first;
                            {
                                let source_ml_source1_s20 = self::source_ml_temporary_plan_datatype_dependency_groups_loop18(count, edges.clone(), heads.clone(), next.clone(), sources.clone(), node, last, edge);
                                match source_ml_source1_s20.clone() {
                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result::MlTemporaryPlanDatatypeDependencyGroupsLoop18Done(next_2, sources_2) => {
                                        {
                                            let first_2 = last;
                                            {
                                                let node_2 = (node + 1f64);
                                                {
                                                    (starts, next, sources, first, node) = (starts_2.clone(), next_2.clone(), sources_2.clone(), first_2, node_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result::MlTemporaryPlanDatatypeDependencyGroupsLoop18Return(source_ml_temporary_result18) => {
                                        self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(source_ml_temporary_result18.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Done(starts.clone(), next.clone(), sources.clone(), first)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop2Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop2Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop2(mut count: f64, mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop2Result {
    loop {
        return if (node < count) {
            {
                let heads_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value38 = heads.borrow().clone(); ml_reference_value38 }, vec![(-1f64)])));
                {
                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value39 = seen.borrow().clone(); ml_reference_value39 }, vec![0f64])));
                    {
                        let labels_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value40 = labels.borrow().clone(); ml_reference_value40 }, vec![(-1f64)])));
                        {
                            let node_2 = (node + 1f64);
                            {
                                (heads, seen, labels, node) = (heads_2.clone(), seen_2.clone(), labels_2.clone(), node_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop2Result::MlTemporaryPlanDatatypeDependencyGroupsLoop2Done(heads.clone(), seen.clone(), labels.clone())
        };
    }
}

fn ml_main() {
    assert!(((self::plan_datatype_dependency_groups(0f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 1");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))), "assertion 2");
    assert!(((self::plan_datatype_dependency_groups(2f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])))), "assertion 3");
    assert!(((self::plan_datatype_dependency_groups(3f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 2f64])))), "assertion 4");
    assert!(((self::plan_datatype_dependency_groups(3f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 0f64])))), "assertion 5");
    assert!(((self::plan_datatype_dependency_groups(4f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64, 3f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 2f64, 2f64])))), "assertion 6");
    assert!(((self::plan_datatype_dependency_groups(4f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])))), "assertion 7");
    assert!(((self::plan_datatype_dependency_groups(5f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64, 4f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64, 3f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 2f64, 2f64, 4f64])))), "assertion 8");
    assert!(((self::plan_datatype_dependency_groups(4f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, 3f64, 0f64, 1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 0f64, 1f64])))), "assertion 9");
    assert!(((self::plan_datatype_dependency_groups(2f64, std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])))), "assertion 10");
    assert!(((self::plan_datatype_dependency_groups(3f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])))), "assertion 11");
    assert!(((self::plan_datatype_dependency_groups(2f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 12");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 13");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 14");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0.5f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 15");
    assert!(((self::plan_datatype_dependency_groups((-1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 16");
    assert!(((self::plan_datatype_dependency_groups(8193f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 17");
    assert!(((self::plan_datatype_dependency_groups(2f64, std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 18");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 19");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![(-0f64)])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 20");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-0f64)])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 21");
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
fn all_literal_datatype_dependency_group_controls() { main(); }
