pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop15(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result {
    loop {
        return if (at < ({ let ml_reference_array97 = kinds.clone(); let ml_reference_projection97 = ml_reference_array97.borrow().len() as f64; ml_reference_projection97 })) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new())))
                } else {
                    {
                        let target = self::read_required_number_fact(targets.clone(), at);
                        {
                            let kind = self::read_required_text_fact(kinds.clone(), at);
                            {
                                let actual = if (target >= (0f64)) {
                                    self::read_required_boolean_fact(known.clone(), target)
                                } else {
                                    if self::ml_array::includes(&{ let ml_reference_value98 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("string")]))).borrow().clone(); ml_reference_value98 }, &kind.clone(), 0f64, false, |a, b| a == b) {
                                        self::read_required_boolean_fact(leaves.clone(), at)
                                    } else {
                                        self::ml_array::includes(&{ let ml_reference_value99 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("constructor"), String::from("tuple"), String::from("option"), String::from("result"), String::from("nominal")]))).borrow().clone(); ml_reference_value99 }, &kind.clone(), 0f64, false, |a, b| a == b)
                                    }
                                };
                                {
                                    let start = if (at == (0f64)) {
                                        0f64
                                    } else {
                                        self::read_required_number_fact(ends.clone(), (at - 1f64))
                                    };
                                    {
                                        let end = self::read_required_number_fact(ends.clone(), at);
                                        {
                                            let actual_3 = if ((kind == "nominal") && (start == end)) {
                                                {
                                                    let actual_2 = false;
                                                    actual_2
                                                }
                                            } else {
                                                actual
                                            };
                                            {
                                                let edge = start;
                                                {
                                                    let actual_4 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop18(edges.clone(), safe.clone(), actual_3, end, edge);
                                                    {
                                                        let safe_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value100 = safe.borrow().clone(); ml_reference_value100 }, vec![actual_4])));
                                                        {
                                                            let at_2 = (at + 1f64);
                                                            {
                                                                (work, safe, at) = (work_2, safe_2.clone(), at_2);
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
        } else {
            self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Done(work, safe.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop14(mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut work: f64) -> self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let safe = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s63 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop15(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), known.clone(), work, safe.clone(), at);
                            match source_ml_source1_s63.clone() {
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Done(work_2, safe_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop16(owners.clone(), copies.clone(), clones.clone(), roots.clone(), known.clone(), changed_2, safe_2.clone(), owner);
                                            {
                                                (changed, work) = (changed_3, work_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Return(source_ml_temporary_result17) => {
                                    self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Return(source_ml_temporary_result17.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result {
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Done(std::rc::Rc<std::cell::RefCell<Vec<String>>>, f64),
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result {
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Done,
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop20(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut end: f64, mut edge: f64) -> self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result {
    loop {
        return if (edge < end) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                if ((!(self::ml_math::is_integer(child)) || (child < (0f64))) || (child >= at)) {
                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))
                } else {
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            edge = edge_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Done
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop19(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut elements: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut previous: f64, mut at: f64) -> self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result {
    loop {
        return if (at < ({ let ml_reference_array101 = kinds.clone(); let ml_reference_projection101 = ml_reference_array101.borrow().len() as f64; ml_reference_projection101 })) {
            {
                let kind = self::read_required_text_fact(kinds.clone(), at);
                {
                    let end = self::read_required_number_fact(ends.clone(), at);
                    if ((!(self::ml_math::is_integer(end)) || (end < previous)) || (end > ({ let ml_reference_array102 = edges.clone(); let ml_reference_projection102 = ml_reference_array102.borrow().len() as f64; ml_reference_projection102 }))) {
                        self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s64 = self::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop20(edges.clone(), at, end, edge);
                                match source_ml_source1_s64.clone() {
                                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Done => {
                                        {
                                            let selected = kind.clone();
                                            {
                                                let selected_4 = if (kind == "pending vector") {
                                                    {
                                                        let selected_2 = String::from("unknown");
                                                        if (end == ((previous + 1f64))) {
                                                            {
                                                                let child = self::read_required_number_fact(edges.clone(), previous);
                                                                {
                                                                    let target = self::read_required_number_fact(targets.clone(), child);
                                                                    if ((((((self::read_required_text_fact(kinds.clone(), child)) == "reference") && self::ml_math::is_integer(target)) && (target >= (0f64))) && (target < ({ let ml_reference_array103 = elements.clone(); let ml_reference_projection103 = ml_reference_array103.borrow().len() as f64; ml_reference_projection103 }))) && self::read_required_boolean_fact(elements.clone(), target)) {
                                                                        {
                                                                            let selected_3 = String::from("vector");
                                                                            selected_3.clone()
                                                                        }
                                                                    } else {
                                                                        selected_2.clone()
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            selected_2.clone()
                                                        }
                                                    }
                                                } else {
                                                    selected.clone()
                                                };
                                                {
                                                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value104 = result.borrow().clone(); ml_reference_value104 }, vec![selected_4.clone()])));
                                                    {
                                                        let previous_2 = end;
                                                        {
                                                            let at_2 = (at + 1f64);
                                                            {
                                                                (result, previous, at) = (result_2.clone(), previous_2, at_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Return(source_ml_temporary_result20) => {
                                        self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(source_ml_temporary_result20.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Done(result.clone(), previous)
        };
    }
}

fn ml_main() {
    assert!(((self::plan_rust_nominal_ownership_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Failure")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("string"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("owned"), String::from("")])))), "assertion 1");
    assert!(((self::plan_rust_nominal_ownership_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Failure")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("string"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("owned"), String::from("Clone")])))), "assertion 2");
    assert!(((self::plan_rust_nominal_ownership_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Code")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("owned"), String::from("")])))), "assertion 3");
    assert!(((self::plan_rust_nominal_ownership_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Code")])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("copy"), String::from("Clone")])))), "assertion 4");
    assert!(((self::plan_rust_nominal_ownership_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Loop")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from(""), String::from("")])))), "assertion 5");
    assert!(((self::plan_rust_nominal_ownership_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Failure"), String::from("Outer")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("string"), String::from("constructor"), String::from("nominal"), String::from("reference"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 2f64, 3f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 3f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64), (-1f64), (-1f64), 0f64, (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, 5f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("owned"), String::from(""), String::from("owned"), String::from("")])))), "assertion 6");
    assert!(((self::read_rust_nominal_ownership_sum_node(String::from("std::option::Option"), 1f64, true, false, true)) == "option"), "assertion 7");
    assert!(((self::read_rust_nominal_ownership_sum_node(String::from("local::Option"), 1f64, true, false, true)) == "unknown"), "assertion 8");
    assert!(((self::read_rust_nominal_ownership_sum_node(String::from("std::result::Result"), 1f64, true, false, true)) == "unknown"), "assertion 9");
    assert!(((self::read_rust_nominal_ownership_sum_node(String::from("std::option::Option"), 1f64, false, false, true)) == "unknown"), "assertion 10");
    assert!(((self::read_rust_nominal_clone_implementation_error(false)) == "Clone needs each actual nominal Clone implementation independently of ownership"), "assertion 11");
    assert!(((self::read_rust_nominal_clone_implementation_error(true)) == ""), "assertion 12");
    assert!(((self::plan_rust_nominal_pattern_transfers(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("parent"), String::from("parent")])), std::rc::Rc::new(std::cell::RefCell::new(vec![true, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false])), false)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("transfers"), String::from("0")])))), "assertion 13");
    assert!(((self::plan_rust_nominal_pattern_transfers(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("parent"), String::from("parent")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false])), false)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a whole nominal alias cannot also move its owned fields")])))), "assertion 14");
    assert!(((self::plan_rust_nominal_pattern_transfers(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), false)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("transfers")])))), "assertion 15");
    assert!(((self::plan_rust_nominal_pattern_transfers(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), false)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a nominal field move needs its actual available parent or actual producer")])))), "assertion 16");
    assert!(((self::plan_rust_nominal_pattern_transfers(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("parent")])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), true)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("owned nominal guarded fields need their deferred partial-move interpretation")])))), "assertion 17");
    assert!(((self::read_rust_nominal_record_update_error(String::from("owned"))) == "owned nominal record updates need their actual partial-field move interpretation"), "assertion 18");
    assert!(((self::read_rust_nominal_record_update_error(String::from("copy"))) == ""), "assertion 19");
    assert!(((self::read_rust_nominal_ownership_graph_error(65536f64, 131072f64, 256f64)) == ""), "assertion 20");
    assert!(((self::read_rust_nominal_ownership_graph_error(65537f64, 0f64, 0f64)) == "the actual nominal ownership field graph exceeds its finite source profile"), "assertion 21");
    assert!(((self::read_rust_nominal_ownership_graph_error(0f64, 131073f64, 0f64)) == "the actual nominal ownership field graph exceeds its finite source profile"), "assertion 22");
    assert!(((self::read_rust_nominal_ownership_graph_error(0f64, 0f64, 257f64)) == "the actual nominal ownership field graph exceeds its finite source profile"), "assertion 23");
    assert!(((self::read_rust_nominal_ownership_graph_error(f64::NAN, 0f64, 0f64)) == "the actual nominal ownership field graph exceeds its finite source profile"), "assertion 24");
    assert!(((self::read_rust_nominal_ownership_graph_error(0f64, f64::INFINITY, 0f64)) == "the actual nominal ownership field graph exceeds its finite source profile"), "assertion 25");
    assert!(((self::read_rust_nominal_ownership_graph_error(0f64, 0f64, (-1f64))) == "the actual nominal ownership field graph exceeds its finite source profile"), "assertion 26");
    assert!(((self::read_rust_nominal_ownership_graph_error(1.5f64, 0f64, 0f64)) == "the actual nominal ownership field graph exceeds its finite source profile"), "assertion 27");
    assert!(((self::plan_rust_nominal_vector_element_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Code")])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![true, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![true])))), "assertion 28");
    assert!(((self::plan_rust_nominal_vector_element_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![false])))), "assertion 29");
    assert!(((self::accept_rust_nominal_vector_element_leaf(String::from("function"), std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))) == (false)), "assertion 30");
    assert!(((self::accept_rust_nominal_vector_element_leaf(String::from("fixed"), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])))) == (false)), "assertion 31");
    assert!(((self::accept_rust_nominal_vector_element(String::from("copy"), true, true)) == (true)), "assertion 32");
    assert!(((self::accept_rust_nominal_vector_element(String::from("owned"), true, true)) == (false)), "assertion 33");
    assert!(((self::accept_rust_nominal_vector_element(String::from("copy"), false, true)) == (false)), "assertion 34");
    assert!(((self::read_rust_nominal_vector_element_admission(false, false, true, true, false, String::from("named"), 0f64)) == "defer"), "assertion 35");
    assert!(((self::read_rust_nominal_vector_element_admission(false, false, true, true, true, String::from("named"), 0f64)) == "refuse"), "assertion 36");
    assert!(((self::read_rust_nominal_vector_element_admission(false, false, false, true, false, String::from("named"), 0f64)) == "refuse"), "assertion 37");
    assert!(((self::read_rust_nominal_vector_element_admission(false, false, true, true, false, String::from("named"), 1f64)) == "refuse"), "assertion 38");
    assert!(((self::read_rust_nominal_vector_element_admission(false, true, false, false, true, String::from("named"), 0f64)) == "admit"), "assertion 39");
    assert!(((self::read_rust_nominal_vector_element_admission(true, false, false, false, true, String::from("fixed"), 0f64)) == "admit"), "assertion 40");
    assert!(((self::read_rust_nominal_constructor_error(true, 0f64, 0f64)) == ""), "assertion 41");
    assert!(((self::read_rust_nominal_constructor_error(true, 2f64, 2f64)) == ""), "assertion 42");
    assert!(((self::read_rust_nominal_constructor_error(true, 3f64, 3f64)) == ""), "assertion 43");
    assert!(((self::read_rust_nominal_constructor_error(true, 2f64, 1f64)) == "nominal construction needs its actual constructor owner and exact finite field arity"), "assertion 44");
    assert!(((self::read_rust_nominal_constructor_error(false, 2f64, 2f64)) == "nominal construction needs its actual constructor owner and exact finite field arity"), "assertion 45");
    assert!(((self::read_rust_nominal_constructor_error(true, 65537f64, 65537f64)) == "nominal construction needs its actual constructor owner and exact finite field arity"), "assertion 46");
    assert!(((self::read_rust_nominal_ownership_vector_template_node(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), true, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("owned vector"), String::from("named"), true)) == "pending vector"), "assertion 47");
    assert!(((self::read_rust_nominal_ownership_vector_template_node(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), true, String::from("declaration"), String::from("local::Vec"), 1f64, String::from("owned vector"), String::from("named"), true)) == "unknown"), "assertion 48");
    assert!(((self::read_rust_nominal_ownership_vector_template_node(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), true, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("owned vector"), String::from("named"), true)) == "unknown"), "assertion 49");
    assert!(((self::read_rust_nominal_ownership_vector_template_node(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), true, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("owned vector"), String::from("named"), false)) == "unknown"), "assertion 50");
    assert!(((self::read_rust_nominal_ownership_vector_template_node(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), false, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("owned vector"), String::from("named"), true)) == "unknown"), "assertion 51");
    assert!(((self::plan_rust_nominal_ownership_vector_nodes(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("pending vector"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("vector"), String::from("constructor"), String::from("nominal")])))), "assertion 52");
    assert!(((self::plan_rust_nominal_ownership_vector_nodes(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("pending vector"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("unknown"), String::from("constructor"), String::from("nominal")])))), "assertion 53");
    assert!(((self::plan_rust_nominal_ownership_vector_nodes(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("pending vector")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("unknown")])))), "assertion 54");
    assert!(((self::plan_rust_nominal_ownership_vector_nodes(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("pending vector")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("unknown")])))), "assertion 55");
    assert!(((self::plan_rust_nominal_ownership_vector_nodes(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("pending vector")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))), "assertion 56");
    assert!(((self::plan_rust_nominal_vector_element_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Cycle")])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![false])))), "assertion 57");
    assert!(((self::accept_rust_nominal_verified_vector_clone(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), true, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("owned vector"), true)) == (true)), "assertion 58");
    assert!(((self::accept_rust_nominal_verified_vector_clone(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), true, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("owned vector"), false)) == (false)), "assertion 59");
    assert!(((self::accept_rust_nominal_verified_vector_clone(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), true, String::from("declaration"), String::from("local::Vec"), 1f64, String::from("owned vector"), true)) == (false)), "assertion 60");
    assert!(((self::accept_rust_nominal_verified_vector_clone(String::from("array"), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), true, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("owned vector"), true)) == (false)), "assertion 61");
    assert!(((self::accept_rust_nominal_clone_inherent_override(String::from("S"), String::from("S"), 1f64)) == (true)), "assertion 62");
    assert!(((self::accept_rust_nominal_clone_inherent_override(String::from("S"), String::from("Other"), 1f64)) == (false)), "assertion 63");
    assert!(((self::accept_rust_nominal_clone_inherent_override(String::from("S"), String::from("S"), 0f64)) == (false)), "assertion 64");
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
fn all_literal_nominal_ownership_controls() { main(); }
