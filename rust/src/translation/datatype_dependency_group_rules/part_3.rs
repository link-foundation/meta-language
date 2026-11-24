// Generated from complete js/src/translation/datatype-dependency-group-rules.js. Do not edit by hand.
use super::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result;
use super::MlTemporaryPlanDatatypeDependencyGroupsLoop26Result;
use super::ml_math;
use super::read_required_number_fact;
use super::source_ml_temporary_plan_datatype_dependency_groups_loop26;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop3(
    mut count: f64,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut first: f64,
    mut node: f64,
) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result {
    loop {
        return if node < count {
            {
                let last = self::read_required_number_fact(ends.clone(), node);
                if ((!(self::ml_math::is_integer(last)) || {
                    let source_ml_source1_same_value_operand21 = last;
                    {
                        let source_ml_source1_same_value_operand22 = -0f64;
                        (((source_ml_source1_same_value_operand21
                            == source_ml_source1_same_value_operand22)
                            && ((source_ml_source1_same_value_operand21 != 0f64)
                                || ((1f64 / source_ml_source1_same_value_operand21)
                                    == (1f64 / source_ml_source1_same_value_operand22))))
                            || ((source_ml_source1_same_value_operand21
                                != source_ml_source1_same_value_operand21)
                                && (source_ml_source1_same_value_operand22
                                    != source_ml_source1_same_value_operand22)))
                    }
                }) || (last < first))
                    || (last
                        > ({
                            let ml_reference_array36 = edges.clone();
                            ml_reference_array36.borrow().len() as f64
                        }))
                {
                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let source_ml_temporary_plan_datatype_dependency_groups_ignored25 = {
                            let ml_method_owner37 = starts.clone();
                            let ml_method_argument37_0 = first;
                            let ml_inserted = vec![ml_method_argument37_0];
                            let mut ml_method_values37 = ml_method_owner37.borrow_mut();
                            let ml_length = ml_method_values37
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            ml_method_values37.extend(ml_inserted);
                            ml_length as f64
                        };
                        {
                            let edge = first;
                            {
                                let source_ml_source1_s23 = self::source_ml_temporary_plan_datatype_dependency_groups_loop26(count, edges.clone(), heads.clone(), next.clone(), sources.clone(), node, last, edge);
                                match source_ml_source1_s23 {
                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop26Result::MlTemporaryPlanDatatypeDependencyGroupsLoop26Done => {
                                        {
                                            let first_2 = last;
                                            {
                                                let node_2 = node + 1f64;
                                                {
                                                    (first, node) = (first_2, node_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop26Result::MlTemporaryPlanDatatypeDependencyGroupsLoop26Return(source_ml_temporary_result26) => {
                                        self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(source_ml_temporary_result26.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Done(first)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop2Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop2Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop2(
    mut count: f64,
    mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut node: f64,
) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop2Result {
    loop {
        if node < count {
            {
                let source_ml_temporary_plan_datatype_dependency_groups_ignored30 = {
                    let ml_method_owner38 = heads.clone();
                    let ml_method_argument38_0 = -1f64;
                    let ml_inserted = vec![ml_method_argument38_0];
                    let mut ml_method_values38 = ml_method_owner38.borrow_mut();
                    let ml_length = ml_method_values38
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values38.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let source_ml_temporary_plan_datatype_dependency_groups_ignored31 = {
                        let ml_method_owner39 = seen.clone();
                        let ml_method_argument39_0 = 0f64;
                        let ml_inserted = vec![ml_method_argument39_0];
                        let mut ml_method_values39 = ml_method_owner39.borrow_mut();
                        let ml_length = ml_method_values39
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values39.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_temporary_plan_datatype_dependency_groups_ignored32 = {
                            let ml_method_owner40 = labels.clone();
                            let ml_method_argument40_0 = -1f64;
                            let ml_inserted = vec![ml_method_argument40_0];
                            let mut ml_method_values40 = ml_method_owner40.borrow_mut();
                            let ml_length = ml_method_values40
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            ml_method_values40.extend(ml_inserted);
                            ml_length as f64
                        };
                        {
                            let node_2 = node + 1f64;
                            {
                                node = node_2;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop2Result::MlTemporaryPlanDatatypeDependencyGroupsLoop2Done
        };
    }
}
