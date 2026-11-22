// Generated from complete js/src/translation/rust-nominal-ownership-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result;
use super::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Result;
use super::ml_math;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop29;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop28(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut elements: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut previous: f64,
    mut at: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result {
    loop {
        return if at
            < ({
                let ml_reference_array101 = kinds.clone();
                ml_reference_array101.borrow().len() as f64
            }) {
            {
                let kind = self::read_required_text_fact(kinds.clone(), at);
                {
                    let end = self::read_required_number_fact(ends.clone(), at);
                    if (!(self::ml_math::is_integer(end)) || (end < previous))
                        || (end
                            > ({
                                let ml_reference_array102 = edges.clone();
                                ml_reference_array102.borrow().len() as f64
                            }))
                    {
                        self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s65 = self::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop29(edges.clone(), at, end, edge);
                                match source_ml_source1_s65 {
                                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Done => {
                                        {
                                            let selected = kind.clone();
                                            {
                                                let selected_4 = if kind == "pending vector" {
                                                    {
                                                        let selected_2 = String::from("unknown");
                                                        if end == (previous + 1f64) {
                                                            {
                                                                let child = self::read_required_number_fact(edges.clone(), previous);
                                                                {
                                                                    let target = self::read_required_number_fact(targets.clone(), child);
                                                                    if (((((self::read_required_text_fact(kinds.clone(), child)) == "reference") && self::ml_math::is_integer(target)) && (target >= 0f64)) && (target < ({ let ml_reference_array103 = elements.clone(); ml_reference_array103.borrow().len() as f64 }))) && self::read_required_boolean_fact(elements.clone(), target) {
                                                                        {
                                                                            String::from("vector")
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
                                                    selected
                                                };
                                                {
                                                    let source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_ignored30 = { let ml_method_owner104 = result.clone(); let ml_method_argument104_0 = selected_4; let ml_inserted = vec![ml_method_argument104_0]; let mut ml_method_values104 = ml_method_owner104.borrow_mut(); let ml_length = ml_method_values104.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values104.extend(ml_inserted); ml_length as f64 };
                                                    {
                                                        let previous_2 = end;
                                                        {
                                                            let at_2 = at + 1f64;
                                                            {
                                                                (previous, at) = (previous_2, at_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Return(source_ml_temporary_result30) => {
                                        self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Return(source_ml_temporary_result30.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Done(previous)
        };
    }
}
