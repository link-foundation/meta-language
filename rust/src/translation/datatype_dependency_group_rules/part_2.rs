// Generated from complete js/src/translation/datatype-dependency-group-rules.js. Do not edit by hand.
use super::MlTemporaryPlanDatatypeDependencyGroupsJoin11;
use super::MlTemporaryPlanDatatypeDependencyGroupsLoop5Result;
use super::MlTemporaryPlanDatatypeDependencyGroupsLoop10Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_required_number_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop10(
    mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut root: f64,
    mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edge: f64,
) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop10Result {
    loop {
        return if edge >= 0f64 {
            {
                let source = self::read_required_number_fact(sources.clone(), edge);
                {
                    let source_ml_source1_s13 = if (self::read_required_number_fact(
                        labels.clone(),
                        source,
                    )) < 0f64
                    {
                        {
                            let source_ml_temporary_plan_datatype_dependency_groups_ignored12 = {
                                let ml_array_owner11 = labels.clone();
                                let ml_array_argument11_0: f64 = source;
                                let ml_array_argument11_1: f64 = 1f64;
                                let ml_array_argument11_2: f64 = root;
                                let ml_array_length = ml_array_owner11.borrow().len();
                                let ml_array_start = {
                                    let ml_relative_value: f64 = ml_array_argument11_0;
                                    let ml_relative = if ml_relative_value.is_nan() {
                                        0f64
                                    } else {
                                        ml_relative_value.trunc()
                                    };
                                    let ml_bound = ml_array_length as f64;
                                    (if ml_relative < 0f64 {
                                        (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                    } else {
                                        ml_relative.min(ml_bound)
                                    }) as usize
                                };
                                let ml_array_count = {
                                    let ml_delete_value: f64 = ml_array_argument11_1;
                                    ml_delete_value
                                        .trunc()
                                        .max(0f64)
                                        .min((ml_array_length - ml_array_start) as f64)
                                        as usize
                                };
                                let ml_array_inserted = vec![ml_array_argument11_2];
                                let _ = (ml_array_length - ml_array_count)
                                    .checked_add(ml_array_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                let ml_removed: Vec<_> = ml_array_owner11
                                    .borrow_mut()
                                    .splice(
                                        ml_array_start..ml_array_start + ml_array_count,
                                        ml_array_inserted,
                                    )
                                    .collect();
                                std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                            };
                            {
                                let source_ml_temporary_plan_datatype_dependency_groups_ignored13 = {
                                    let ml_method_owner12 = stack.clone();
                                    let ml_method_argument12_0 = source;
                                    let ml_inserted = vec![ml_method_argument12_0];
                                    let mut ml_method_values12 = ml_method_owner12.borrow_mut();
                                    let ml_length = ml_method_values12
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4_294_967_295usize)
                                        .expect("Invalid array length");
                                    ml_method_values12.extend(ml_inserted);
                                    ml_length as f64
                                };
                                self::MlTemporaryPlanDatatypeDependencyGroupsJoin11::MlTemporaryPlanDatatypeDependencyGroupsJoin11Next
                            }
                        }
                    } else {
                        self::MlTemporaryPlanDatatypeDependencyGroupsJoin11::MlTemporaryPlanDatatypeDependencyGroupsJoin11Next
                    };
                    match source_ml_source1_s13 {
                        self::MlTemporaryPlanDatatypeDependencyGroupsJoin11::MlTemporaryPlanDatatypeDependencyGroupsJoin11Next => {
                            {
                                let edge_2 = self::read_required_number_fact(next.clone(), edge);
                                {
                                    edge = edge_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop10Result::MlTemporaryPlanDatatypeDependencyGroupsLoop10Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop7(
    mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut root: f64,
    mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut members: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut representative: f64,
) -> f64 {
    loop {
        return {
            let ml_library_condition_2 = {
                let ml_value = {
                    let ml_reference_array14 = stack.clone();
                    ml_reference_array14.borrow().len() as f64
                };
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_2 {
                {
                    let node = self::read_required_number_fact(
                        stack.clone(),
                        ({
                            let ml_reference_array15 = stack.clone();
                            ml_reference_array15.borrow().len() as f64
                        } - 1f64),
                    );
                    {
                        let source_ml_temporary_plan_datatype_dependency_groups_ignored8 = {
                            let ml_method_owner16 = stack.clone();
                            ml_method_owner16.borrow_mut().pop()
                        };
                        {
                            let source_ml_temporary_plan_datatype_dependency_groups_ignored9 = {
                                let ml_method_owner17 = members.clone();
                                let ml_method_argument17_0 = node;
                                let ml_inserted = vec![ml_method_argument17_0];
                                let mut ml_method_values17 = ml_method_owner17.borrow_mut();
                                let ml_length = ml_method_values17
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values17.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let representative_2 = self::ml_math::min(representative, node);
                                {
                                    let edge = self::read_required_number_fact(heads.clone(), node);
                                    {
                                        let source_ml_source1_s14 = self::source_ml_temporary_plan_datatype_dependency_groups_loop10(next.clone(), sources.clone(), labels.clone(), root, stack.clone(), edge);
                                        match source_ml_source1_s14 {
                                        self::MlTemporaryPlanDatatypeDependencyGroupsLoop10Result::MlTemporaryPlanDatatypeDependencyGroupsLoop10Done => {
                                            {
                                                representative = representative_2;
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
            } else {
                representative
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop14Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop14Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop14(
    mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut representative: f64,
    mut source_ml_temporary_values1: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_temporary_index1: self::ml::Big,
) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop14Result {
    loop {
        if source_ml_temporary_index1
            < ({
                let ml_reference_array18 = source_ml_temporary_values1.clone();
                self::ml::Big::from_u128(ml_reference_array18.borrow().len() as u128)
            })
        {
            {
                let member = {
                    let source_ml_source1_array_read_value15 = source_ml_temporary_values1.clone();
                    {
                        let source_ml_source1_array_read_index16 =
                            source_ml_temporary_index1.clone();
                        {
                            let ml_reference_array19 = source_ml_source1_array_read_value15;
                            let ml_reference_position19 = source_ml_source1_array_read_index16;
                            self::ml_array::at(
                                &*ml_reference_array19.borrow(),
                                ml_reference_position19.to_index(),
                            )
                        }
                    }
                };
                {
                    let source_ml_temporary_plan_datatype_dependency_groups_ignored15 = {
                        let ml_array_owner20 = labels.clone();
                        let ml_array_argument20_0: f64 = member;
                        let ml_array_argument20_1: f64 = 1f64;
                        let ml_array_argument20_2: f64 = representative;
                        let ml_array_length = ml_array_owner20.borrow().len();
                        let ml_array_start = {
                            let ml_relative_value: f64 = ml_array_argument20_0;
                            let ml_relative = if ml_relative_value.is_nan() {
                                0f64
                            } else {
                                ml_relative_value.trunc()
                            };
                            let ml_bound = ml_array_length as f64;
                            (if ml_relative < 0f64 {
                                (ml_bound + ml_relative).clamp(0f64, ml_bound)
                            } else {
                                ml_relative.min(ml_bound)
                            }) as usize
                        };
                        let ml_array_count = {
                            let ml_delete_value: f64 = ml_array_argument20_1;
                            ml_delete_value
                                .trunc()
                                .max(0f64)
                                .min((ml_array_length - ml_array_start) as f64)
                                as usize
                        };
                        let ml_array_inserted = vec![ml_array_argument20_2];
                        let _ = (ml_array_length - ml_array_count)
                            .checked_add(ml_array_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        let ml_removed: Vec<_> = ml_array_owner20
                            .borrow_mut()
                            .splice(
                                ml_array_start..ml_array_start + ml_array_count,
                                ml_array_inserted,
                            )
                            .collect();
                        std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                    };
                    {
                        let source_ml_temporary_index1_2 =
                            source_ml_temporary_index1.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index1 = source_ml_temporary_index1_2;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop14Result::MlTemporaryPlanDatatypeDependencyGroupsLoop14Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop5(
    mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut order: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop5Result {
    loop {
        return if at >= 0f64 {
            {
                let root = self::read_required_number_fact(order.clone(), at);
                if (self::read_required_number_fact(labels.clone(), root)) >= 0f64 {
                    {
                        let at_2 = at - 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let stack = std::rc::Rc::new(std::cell::RefCell::new(vec![root]));
                        {
                            let members =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let source_ml_temporary_plan_datatype_dependency_groups_ignored6 = {
                                    let ml_array_owner21 = labels.clone();
                                    let ml_array_argument21_0: f64 = root;
                                    let ml_array_argument21_1: f64 = 1f64;
                                    let ml_array_argument21_2: f64 = root;
                                    let ml_array_length = ml_array_owner21.borrow().len();
                                    let ml_array_start = {
                                        let ml_relative_value: f64 = ml_array_argument21_0;
                                        let ml_relative = if ml_relative_value.is_nan() {
                                            0f64
                                        } else {
                                            ml_relative_value.trunc()
                                        };
                                        let ml_bound = ml_array_length as f64;
                                        (if ml_relative < 0f64 {
                                            (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                        } else {
                                            ml_relative.min(ml_bound)
                                        }) as usize
                                    };
                                    let ml_array_count = {
                                        let ml_delete_value: f64 = ml_array_argument21_1;
                                        ml_delete_value
                                            .trunc()
                                            .max(0f64)
                                            .min((ml_array_length - ml_array_start) as f64)
                                            as usize
                                    };
                                    let ml_array_inserted = vec![ml_array_argument21_2];
                                    let _ = (ml_array_length - ml_array_count)
                                        .checked_add(ml_array_inserted.len())
                                        .filter(|length| *length <= 4_294_967_295usize)
                                        .expect("Invalid array length");
                                    let ml_removed: Vec<_> = ml_array_owner21
                                        .borrow_mut()
                                        .splice(
                                            ml_array_start..ml_array_start + ml_array_count,
                                            ml_array_inserted,
                                        )
                                        .collect();
                                    std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                                };
                                {
                                    let representative = root;
                                    {
                                        let representative_2 = self::source_ml_temporary_plan_datatype_dependency_groups_loop7(heads.clone(), next.clone(), sources.clone(), labels.clone(), root, stack, members.clone(), representative);
                                        {
                                            let source_ml_temporary_values1 = members.clone();
                                            {
                                                let source_ml_temporary_index1 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s17 = self::source_ml_temporary_plan_datatype_dependency_groups_loop14(labels.clone(), representative_2, source_ml_temporary_values1, source_ml_temporary_index1);
                                                    match source_ml_source1_s17 {
                                                        self::MlTemporaryPlanDatatypeDependencyGroupsLoop14Result::MlTemporaryPlanDatatypeDependencyGroupsLoop14Done => {
                                                            {
                                                                let at_3 = at - 1f64;
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
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop5Result::MlTemporaryPlanDatatypeDependencyGroupsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop4Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop4Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop17Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop17Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop17(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut order: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop17Result {
    loop {
        return {
            let ml_library_condition_1 = {
                let ml_value = {
                    let ml_reference_array23 = stack.clone();
                    ml_reference_array23.borrow().len() as f64
                };
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_1 {
                {
                    let top = {
                        let ml_reference_array24 = stack.clone();
                        ml_reference_array24.borrow().len() as f64
                    } - 1f64;
                    {
                        let node = self::read_required_number_fact(stack.clone(), top);
                        {
                            let position = self::read_required_number_fact(positions.clone(), top);
                            if position < (self::read_required_number_fact(ends.clone(), node)) {
                                {
                                    let source_ml_temporary_plan_datatype_dependency_groups_ignored18 = {
                                        let ml_array_owner25 = positions.clone();
                                        let ml_array_argument25_0: f64 = top;
                                        let ml_array_argument25_1: f64 = 1f64;
                                        let ml_array_argument25_2: f64 = position + 1f64;
                                        let ml_array_length = ml_array_owner25.borrow().len();
                                        let ml_array_start = {
                                            let ml_relative_value: f64 = ml_array_argument25_0;
                                            let ml_relative = if ml_relative_value.is_nan() {
                                                0f64
                                            } else {
                                                ml_relative_value.trunc()
                                            };
                                            let ml_bound = ml_array_length as f64;
                                            (if ml_relative < 0f64 {
                                                (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                            } else {
                                                ml_relative.min(ml_bound)
                                            }) as usize
                                        };
                                        let ml_array_count = {
                                            let ml_delete_value: f64 = ml_array_argument25_1;
                                            ml_delete_value
                                                .trunc()
                                                .max(0f64)
                                                .min((ml_array_length - ml_array_start) as f64)
                                                as usize
                                        };
                                        let ml_array_inserted = vec![ml_array_argument25_2];
                                        let _ = (ml_array_length - ml_array_count)
                                            .checked_add(ml_array_inserted.len())
                                            .filter(|length| *length <= 4_294_967_295usize)
                                            .expect("Invalid array length");
                                        let ml_removed: Vec<_> = ml_array_owner25
                                            .borrow_mut()
                                            .splice(
                                                ml_array_start..ml_array_start + ml_array_count,
                                                ml_array_inserted,
                                            )
                                            .collect();
                                        std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                                    };
                                    {
                                        let target = self::read_required_number_fact(
                                            edges.clone(),
                                            position,
                                        );
                                        if (self::read_required_number_fact(seen.clone(), target))
                                            == 0f64
                                        {
                                            {
                                                let source_ml_temporary_plan_datatype_dependency_groups_ignored19 = {
                                                    let ml_array_owner26 = seen.clone();
                                                    let ml_array_argument26_0: f64 = target;
                                                    let ml_array_argument26_1: f64 = 1f64;
                                                    let ml_array_argument26_2: f64 = 1f64;
                                                    let ml_array_length =
                                                        ml_array_owner26.borrow().len();
                                                    let ml_array_start = {
                                                        let ml_relative_value: f64 =
                                                            ml_array_argument26_0;
                                                        let ml_relative =
                                                            if ml_relative_value.is_nan() {
                                                                0f64
                                                            } else {
                                                                ml_relative_value.trunc()
                                                            };
                                                        let ml_bound = ml_array_length as f64;
                                                        (if ml_relative < 0f64 {
                                                            (ml_bound + ml_relative)
                                                                .clamp(0f64, ml_bound)
                                                        } else {
                                                            ml_relative.min(ml_bound)
                                                        })
                                                            as usize
                                                    };
                                                    let ml_array_count = {
                                                        let ml_delete_value: f64 =
                                                            ml_array_argument26_1;
                                                        ml_delete_value.trunc().max(0f64).min(
                                                            (ml_array_length - ml_array_start)
                                                                as f64,
                                                        )
                                                            as usize
                                                    };
                                                    let ml_array_inserted =
                                                        vec![ml_array_argument26_2];
                                                    let _ = (ml_array_length - ml_array_count)
                                                        .checked_add(ml_array_inserted.len())
                                                        .filter(|length| {
                                                            *length <= 4_294_967_295usize
                                                        })
                                                        .expect("Invalid array length");
                                                    let ml_removed: Vec<_> = ml_array_owner26
                                                        .borrow_mut()
                                                        .splice(
                                                            ml_array_start
                                                                ..ml_array_start + ml_array_count,
                                                            ml_array_inserted,
                                                        )
                                                        .collect();
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        ml_removed,
                                                    ))
                                                };
                                                {
                                                    let source_ml_temporary_plan_datatype_dependency_groups_ignored20 = {
                                                        let ml_method_owner27 = stack.clone();
                                                        let ml_method_argument27_0 = target;
                                                        let ml_inserted =
                                                            vec![ml_method_argument27_0];
                                                        let mut ml_method_values27 =
                                                            ml_method_owner27.borrow_mut();
                                                        let ml_length = ml_method_values27
                                                            .len()
                                                            .checked_add(ml_inserted.len())
                                                            .filter(|length| {
                                                                *length <= 4_294_967_295usize
                                                            })
                                                            .expect("Invalid array length");
                                                        ml_method_values27.extend(ml_inserted);
                                                        ml_length as f64
                                                    };
                                                    {
                                                        let source_ml_temporary_plan_datatype_dependency_groups_ignored21 = {
                                                            let ml_method_owner28 =
                                                                positions.clone();
                                                            let ml_method_argument28_0 =
                                                                self::read_required_number_fact(
                                                                    starts.clone(),
                                                                    target,
                                                                );
                                                            let ml_inserted =
                                                                vec![ml_method_argument28_0];
                                                            let mut ml_method_values28 =
                                                                ml_method_owner28.borrow_mut();
                                                            let ml_length = ml_method_values28
                                                                .len()
                                                                .checked_add(ml_inserted.len())
                                                                .filter(|length| {
                                                                    *length <= 4_294_967_295usize
                                                                })
                                                                .expect("Invalid array length");
                                                            ml_method_values28.extend(ml_inserted);
                                                            ml_length as f64
                                                        };
                                                        {
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            {
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let source_ml_temporary_plan_datatype_dependency_groups_ignored22 = {
                                        let ml_method_owner29 = stack.clone();
                                        ml_method_owner29.borrow_mut().pop()
                                    };
                                    {
                                        let source_ml_temporary_plan_datatype_dependency_groups_ignored23 = {
                                            let ml_method_owner30 = positions.clone();
                                            ml_method_owner30.borrow_mut().pop()
                                        };
                                        {
                                            let source_ml_temporary_plan_datatype_dependency_groups_ignored24 = {
                                                let ml_method_owner31 = order.clone();
                                                let ml_method_argument31_0 = node;
                                                let ml_inserted = vec![ml_method_argument31_0];
                                                let mut ml_method_values31 =
                                                    ml_method_owner31.borrow_mut();
                                                let ml_length = ml_method_values31
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values31.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            {
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
            {
                self::MlTemporaryPlanDatatypeDependencyGroupsLoop17Result::MlTemporaryPlanDatatypeDependencyGroupsLoop17Done
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop4(
    mut count: f64,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut order: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut root: f64,
) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop4Result {
    loop {
        return if root < count {
            if (self::read_required_number_fact(seen.clone(), root)) != 0f64 {
                {
                    let root_2 = root + 1f64;
                    {
                        root = root_2;
                        continue;
                    }
                }
            }
            {
                {
                    let stack = std::rc::Rc::new(std::cell::RefCell::new(vec![root]));
                    {
                        let positions = std::rc::Rc::new(std::cell::RefCell::new(vec![
                            self::read_required_number_fact(starts.clone(), root),
                        ]));
                        {
                            let source_ml_temporary_plan_datatype_dependency_groups_ignored16 = {
                                let ml_array_owner32 = seen.clone();
                                let ml_array_argument32_0: f64 = root;
                                let ml_array_argument32_1: f64 = 1f64;
                                let ml_array_argument32_2: f64 = 1f64;
                                let ml_array_length = ml_array_owner32.borrow().len();
                                let ml_array_start = {
                                    let ml_relative_value: f64 = ml_array_argument32_0;
                                    let ml_relative = if ml_relative_value.is_nan() {
                                        0f64
                                    } else {
                                        ml_relative_value.trunc()
                                    };
                                    let ml_bound = ml_array_length as f64;
                                    (if ml_relative < 0f64 {
                                        (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                    } else {
                                        ml_relative.min(ml_bound)
                                    }) as usize
                                };
                                let ml_array_count = {
                                    let ml_delete_value: f64 = ml_array_argument32_1;
                                    ml_delete_value
                                        .trunc()
                                        .max(0f64)
                                        .min((ml_array_length - ml_array_start) as f64)
                                        as usize
                                };
                                let ml_array_inserted = vec![ml_array_argument32_2];
                                let _ = (ml_array_length - ml_array_count)
                                    .checked_add(ml_array_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                let ml_removed: Vec<_> = ml_array_owner32
                                    .borrow_mut()
                                    .splice(
                                        ml_array_start..ml_array_start + ml_array_count,
                                        ml_array_inserted,
                                    )
                                    .collect();
                                std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                            };
                            {
                                let source_ml_source1_s18 = self::source_ml_temporary_plan_datatype_dependency_groups_loop17(ends.clone(), edges.clone(), starts.clone(), seen.clone(), order.clone(), stack, positions);
                                match source_ml_source1_s18 {
                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop17Result::MlTemporaryPlanDatatypeDependencyGroupsLoop17Done => {
                                        {
                                            let root_3 = root + 1f64;
                                            {
                                                root = root_3;
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
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop4Result::MlTemporaryPlanDatatypeDependencyGroupsLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop3Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop3Done(f64),
    MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop26Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop26Done,
    MlTemporaryPlanDatatypeDependencyGroupsLoop26Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_datatype_dependency_groups_loop26(
    mut count: f64,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut node: f64,
    mut last: f64,
    mut edge: f64,
) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop26Result {
    loop {
        return if edge < last {
            {
                let target = self::read_required_number_fact(edges.clone(), edge);
                if ((!(self::ml_math::is_integer(target)) || {
                    let source_ml_source1_same_value_operand19 = target;
                    {
                        let source_ml_source1_same_value_operand20 = -0f64;
                        (((source_ml_source1_same_value_operand19
                            == source_ml_source1_same_value_operand20)
                            && ((source_ml_source1_same_value_operand19 != 0f64)
                                || ((1f64 / source_ml_source1_same_value_operand19)
                                    == (1f64 / source_ml_source1_same_value_operand20))))
                            || ((source_ml_source1_same_value_operand19
                                != source_ml_source1_same_value_operand19)
                                && (source_ml_source1_same_value_operand20
                                    != source_ml_source1_same_value_operand20)))
                    }
                }) || (target < 0f64))
                    || (target >= count)
                {
                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop26Result::MlTemporaryPlanDatatypeDependencyGroupsLoop26Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let source_ml_temporary_plan_datatype_dependency_groups_ignored27 = {
                            let ml_method_owner33 = next.clone();
                            let ml_method_argument33_0 =
                                self::read_required_number_fact(heads.clone(), target);
                            let ml_inserted = vec![ml_method_argument33_0];
                            let mut ml_method_values33 = ml_method_owner33.borrow_mut();
                            let ml_length = ml_method_values33
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            ml_method_values33.extend(ml_inserted);
                            ml_length as f64
                        };
                        {
                            let source_ml_temporary_plan_datatype_dependency_groups_ignored28 = {
                                let ml_method_owner34 = sources.clone();
                                let ml_method_argument34_0 = node;
                                let ml_inserted = vec![ml_method_argument34_0];
                                let mut ml_method_values34 = ml_method_owner34.borrow_mut();
                                let ml_length = ml_method_values34
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values34.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_temporary_plan_datatype_dependency_groups_ignored29 = {
                                    let ml_array_owner35 = heads.clone();
                                    let ml_array_argument35_0: f64 = target;
                                    let ml_array_argument35_1: f64 = 1f64;
                                    let ml_array_argument35_2: f64 = edge;
                                    let ml_array_length = ml_array_owner35.borrow().len();
                                    let ml_array_start = {
                                        let ml_relative_value: f64 = ml_array_argument35_0;
                                        let ml_relative = if ml_relative_value.is_nan() {
                                            0f64
                                        } else {
                                            ml_relative_value.trunc()
                                        };
                                        let ml_bound = ml_array_length as f64;
                                        (if ml_relative < 0f64 {
                                            (ml_bound + ml_relative).clamp(0f64, ml_bound)
                                        } else {
                                            ml_relative.min(ml_bound)
                                        }) as usize
                                    };
                                    let ml_array_count = {
                                        let ml_delete_value: f64 = ml_array_argument35_1;
                                        ml_delete_value
                                            .trunc()
                                            .max(0f64)
                                            .min((ml_array_length - ml_array_start) as f64)
                                            as usize
                                    };
                                    let ml_array_inserted = vec![ml_array_argument35_2];
                                    let _ = (ml_array_length - ml_array_count)
                                        .checked_add(ml_array_inserted.len())
                                        .filter(|length| *length <= 4_294_967_295usize)
                                        .expect("Invalid array length");
                                    let ml_removed: Vec<_> = ml_array_owner35
                                        .borrow_mut()
                                        .splice(
                                            ml_array_start..ml_array_start + ml_array_count,
                                            ml_array_inserted,
                                        )
                                        .collect();
                                    std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                                };
                                {
                                    let edge_2 = edge + 1f64;
                                    {
                                        edge = edge_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop26Result::MlTemporaryPlanDatatypeDependencyGroupsLoop26Done
        };
    }
}
