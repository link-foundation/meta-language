// Generated from complete js/src/translation/rust-owned-recursive-datatype-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop24(
    mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut changed: bool,
    mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owner: f64,
) -> bool {
    loop {
        if owner
            < ({
                let ml_reference_array162 = owners.clone();
                ml_reference_array162.borrow().len() as f64
            })
        {
            if ((!(self::read_required_boolean_fact(known.clone(), owner))
                && self::read_required_boolean_fact(copies.clone(), owner))
                && self::read_required_boolean_fact(clones.clone(), owner))
                && self::read_required_boolean_fact(
                    safe.clone(),
                    self::read_required_number_fact(roots.clone(), owner),
                )
            {
                {
                    let source_ml_temporary_plan_rust_nominal_vector_element_owners_ignored25 = {
                        let ml_array_owner163 = known.clone();
                        let ml_array_argument163_0: f64 = owner;
                        let ml_array_argument163_1: f64 = 1f64;
                        let ml_array_argument163_2: bool = true;
                        let ml_array_length = ml_array_owner163.borrow().len();
                        let ml_array_start = {
                            let ml_relative_value: f64 = ml_array_argument163_0;
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
                            let ml_delete_value: f64 = ml_array_argument163_1;
                            ml_delete_value
                                .trunc()
                                .max(0f64)
                                .min((ml_array_length - ml_array_start) as f64)
                                as usize
                        };
                        let ml_array_inserted = vec![ml_array_argument163_2];
                        let _ = (ml_array_length - ml_array_count)
                            .checked_add(ml_array_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        let ml_removed: Vec<_> = ml_array_owner163
                            .borrow_mut()
                            .splice(
                                ml_array_start..ml_array_start + ml_array_count,
                                ml_array_inserted,
                            )
                            .collect();
                        std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                    };
                    {
                        let changed_2 = true;
                        {
                            let owner_2 = owner + 1f64;
                            {
                                (changed, owner) = (changed_2, owner_2);
                                continue;
                            }
                        }
                    }
                }
            }
            {
                {
                    let owner_3 = owner + 1f64;
                    {
                        owner = owner_3;
                        continue;
                    }
                }
            }
        }
        return { changed };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalVectorElementOwnersLoop23Result {
    MlTemporaryPlanRustNominalVectorElementOwnersLoop23Done(f64),
    MlTemporaryPlanRustNominalVectorElementOwnersLoop23Return(
        std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop26(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut actual: bool,
    mut end: f64,
    mut edge: f64,
) -> bool {
    loop {
        if edge < end {
            if !(self::read_required_boolean_fact(
                safe.clone(),
                self::read_required_number_fact(edges.clone(), edge),
            )) {
                {
                    let actual_2 = false;
                    {
                        let edge_2 = edge + 1f64;
                        {
                            (actual, edge) = (actual_2, edge_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let edge_3 = edge + 1f64;
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        }
        return { actual };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop23(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut work: f64,
    mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Result {
    loop {
        return if at
            < ({
                let ml_reference_array164 = kinds.clone();
                ml_reference_array164.borrow().len() as f64
            }) {
            {
                let work_2 = work + 1f64;
                if work_2 > 1_048_576f64 {
                    self::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new())))
                } else {
                    {
                        let target = self::read_required_number_fact(targets.clone(), at);
                        {
                            let kind = self::read_required_text_fact(kinds.clone(), at);
                            {
                                let actual = if target >= 0f64 {
                                    self::read_required_boolean_fact(known.clone(), target)
                                } else if self::ml_array::includes(
                                    &{
                                        (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            String::from("copy"),
                                            String::from("string"),
                                        ])))
                                        .borrow()
                                        .clone()
                                    },
                                    &kind.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                ) {
                                    self::read_required_boolean_fact(leaves.clone(), at)
                                } else {
                                    self::ml_array::includes(
                                        &{
                                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                String::from("constructor"),
                                                String::from("tuple"),
                                                String::from("option"),
                                                String::from("result"),
                                                String::from("nominal"),
                                            ])))
                                            .borrow()
                                            .clone()
                                        },
                                        &kind.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    )
                                };
                                {
                                    let start = if at == 0f64 {
                                        0f64
                                    } else {
                                        self::read_required_number_fact(ends.clone(), (at - 1f64))
                                    };
                                    {
                                        let end = self::read_required_number_fact(ends.clone(), at);
                                        {
                                            let actual_3 = if (kind == "nominal") && (start == end)
                                            {
                                                { false }
                                            } else {
                                                actual
                                            };
                                            {
                                                let edge = start;
                                                {
                                                    let actual_4 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop26(edges.clone(), safe.clone(), actual_3, end, edge);
                                                    {
                                                        let source_ml_temporary_plan_rust_nominal_vector_element_owners_ignored27 = {
                                                            let ml_method_owner167 = safe.clone();
                                                            let ml_method_argument167_0 = actual_4;
                                                            let ml_inserted =
                                                                vec![ml_method_argument167_0];
                                                            let mut ml_method_values167 =
                                                                ml_method_owner167.borrow_mut();
                                                            let ml_length = ml_method_values167
                                                                .len()
                                                                .checked_add(ml_inserted.len())
                                                                .filter(|length| {
                                                                    *length <= 4_294_967_295usize
                                                                })
                                                                .expect("Invalid array length");
                                                            ml_method_values167.extend(ml_inserted);
                                                            ml_length as f64
                                                        };
                                                        {
                                                            let at_2 = at + 1f64;
                                                            {
                                                                (work, at) = (work_2, at_2);
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
            self::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Done(work)
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
pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop22(
    mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut changed: bool,
    mut work: f64,
) -> self::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let safe = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s71 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop23(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), known.clone(), work, safe.clone(), at);
                            match source_ml_source1_s71 {
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Done(work_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop24(owners.clone(), copies.clone(), clones.clone(), roots.clone(), known.clone(), changed_2, safe.clone(), owner);
                                            {
                                                (changed, work) = (changed_3, work_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop23Return(source_ml_temporary_result25) => {
                                    self::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Return(source_ml_temporary_result25.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop22Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result {
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Done(f64),
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Result {
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Done,
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop29(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
    mut end: f64,
    mut edge: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Result {
    loop {
        return if edge < end {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                if (!(self::ml_math::is_integer(child)) || (child < 0f64)) || (child >= at) {
                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))
                } else {
                    {
                        let edge_2 = edge + 1f64;
                        {
                            edge = edge_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop29Done
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
                let ml_reference_array168 = kinds.clone();
                ml_reference_array168.borrow().len() as f64
            }) {
            {
                let kind = self::read_required_text_fact(kinds.clone(), at);
                {
                    let end = self::read_required_number_fact(ends.clone(), at);
                    if (!(self::ml_math::is_integer(end)) || (end < previous))
                        || (end
                            > ({
                                let ml_reference_array169 = edges.clone();
                                ml_reference_array169.borrow().len() as f64
                            }))
                    {
                        self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop28Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s72 = self::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop29(edges.clone(), at, end, edge);
                                match source_ml_source1_s72 {
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
                                                                    if (((((self::read_required_text_fact(kinds.clone(), child)) == "reference") && self::ml_math::is_integer(target)) && (target >= 0f64)) && (target < ({ let ml_reference_array170 = elements.clone(); ml_reference_array170.borrow().len() as f64 }))) && self::read_required_boolean_fact(elements.clone(), target) {
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
                                                    let source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_ignored30 = { let ml_method_owner171 = result.clone(); let ml_method_argument171_0 = selected_4; let ml_inserted = vec![ml_method_argument171_0]; let mut ml_method_values171 = ml_method_owner171.borrow_mut(); let ml_length = ml_method_values171.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values171.extend(ml_inserted); ml_length as f64 };
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop36Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop36Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop36(
    mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut size: f64,
    mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut owner: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop36Result {
    loop {
        if owner < size {
            {
                let admitted = self::read_required_boolean_fact(eligible.clone(), owner)
                    && self::read_required_boolean_fact(inhabited.clone(), owner);
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored37 = {
                        let ml_method_owner172 = result.clone();
                        let ml_method_argument172_0 = if admitted {
                            if self::read_required_boolean_fact(copies.clone(), owner) {
                                String::from("copy")
                            } else {
                                String::from("owned")
                            }
                        } else {
                            String::new()
                        };
                        let ml_inserted = vec![ml_method_argument172_0];
                        let mut ml_method_values172 = ml_method_owner172.borrow_mut();
                        let ml_length = ml_method_values172
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values172.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored38 = {
                            let ml_method_owner173 = result.clone();
                            let ml_method_argument173_0 =
                                if self::read_required_boolean_fact(implemented.clone(), owner) {
                                    String::from("Clone")
                                } else {
                                    String::new()
                                };
                            let ml_inserted = vec![ml_method_argument173_0];
                            let mut ml_method_values173 = ml_method_owner173.borrow_mut();
                            let ml_length = ml_method_values173
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            ml_method_values173.extend(ml_inserted);
                            ml_length as f64
                        };
                        {
                            let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored39 = {
                                let ml_method_owner174 = result.clone();
                                let ml_method_argument174_0 = if admitted {
                                    String::from("closed")
                                } else {
                                    String::new()
                                };
                                let ml_inserted = vec![ml_method_argument174_0];
                                let mut ml_method_values174 = ml_method_owner174.borrow_mut();
                                let ml_length = ml_method_values174
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                ml_method_values174.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored40 = {
                                    let ml_method_owner175 = result.clone();
                                    let ml_method_argument175_0 = {
                                        let source_ml_source1_string_argument73 =
                                            self::read_required_number_fact(ranks.clone(), owner);
                                        self::ml_number::js_number(
                                            source_ml_source1_string_argument73,
                                        )
                                    };
                                    let ml_inserted = vec![ml_method_argument175_0];
                                    let mut ml_method_values175 = ml_method_owner175.borrow_mut();
                                    let ml_length = ml_method_values175
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4_294_967_295usize)
                                        .expect("Invalid array length");
                                    ml_method_values175.extend(ml_inserted);
                                    ml_length as f64
                                };
                                {
                                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored41 = {
                                        let ml_method_owner176 = result.clone();
                                        let ml_method_argument176_0 = {
                                            let source_ml_source1_string_argument74 =
                                                self::read_required_number_fact(
                                                    witnesses.clone(),
                                                    owner,
                                                );
                                            self::ml_number::js_number(
                                                source_ml_source1_string_argument74,
                                            )
                                        };
                                        let ml_inserted = vec![ml_method_argument176_0];
                                        let mut ml_method_values176 =
                                            ml_method_owner176.borrow_mut();
                                        let ml_length = ml_method_values176
                                            .len()
                                            .checked_add(ml_inserted.len())
                                            .filter(|length| *length <= 4_294_967_295usize)
                                            .expect("Invalid array length");
                                        ml_method_values176.extend(ml_inserted);
                                        ml_length as f64
                                    };
                                    {
                                        let owner_2 = owner + 1f64;
                                        {
                                            owner = owner_2;
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
        return {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop36Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop36Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Done,
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop43(
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut size: f64,
    mut changed: bool,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owner: f64,
) -> bool {
    loop {
        if owner < size {
            if self::read_required_boolean_fact(implemented.clone(), owner)
                && !(self::read_required_boolean_fact(
                    values.clone(),
                    self::read_required_number_fact(roots.clone(), owner),
                ))
            {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored44 = {
                        let ml_array_owner177 = implemented.clone();
                        let ml_array_argument177_0: f64 = owner;
                        let ml_array_argument177_1: f64 = 1f64;
                        let ml_array_argument177_2: bool = false;
                        let ml_array_length = ml_array_owner177.borrow().len();
                        let ml_array_start = {
                            let ml_relative_value: f64 = ml_array_argument177_0;
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
                            let ml_delete_value: f64 = ml_array_argument177_1;
                            ml_delete_value
                                .trunc()
                                .max(0f64)
                                .min((ml_array_length - ml_array_start) as f64)
                                as usize
                        };
                        let ml_array_inserted = vec![ml_array_argument177_2];
                        let _ = (ml_array_length - ml_array_count)
                            .checked_add(ml_array_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        let ml_removed: Vec<_> = ml_array_owner177
                            .borrow_mut()
                            .splice(
                                ml_array_start..ml_array_start + ml_array_count,
                                ml_array_inserted,
                            )
                            .collect();
                        std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                    };
                    {
                        let changed_2 = true;
                        {
                            let owner_2 = owner + 1f64;
                            {
                                (changed, owner) = (changed_2, owner_2);
                                continue;
                            }
                        }
                    }
                }
            }
            {
                {
                    let owner_3 = owner + 1f64;
                    {
                        owner = owner_3;
                        continue;
                    }
                }
            }
        }
        return { changed };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop45(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
    mut value: bool,
    mut edge: f64,
) -> bool {
    loop {
        if edge < (self::read_required_number_fact(ends.clone(), at)) {
            if !(self::read_required_boolean_fact(
                values.clone(),
                self::read_required_number_fact(edges.clone(), edge),
            )) {
                {
                    let value_2 = false;
                    {
                        let edge_2 = edge + 1f64;
                        {
                            (value, edge) = (value_2, edge_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let edge_3 = edge + 1f64;
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        }
        return { value };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop42(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut count: f64,
    mut work: f64,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Result {
    loop {
        return if at < count {
            {
                let work_2 = work + 1f64;
                if work_2 > 1_048_576f64 {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let kind = self::read_required_text_fact(kinds.clone(), at);
                        {
                            let target = self::read_required_number_fact(targets.clone(), at);
                            {
                                let start = if at == 0f64 {
                                    0f64
                                } else {
                                    self::read_required_number_fact(ends.clone(), (at - 1f64))
                                };
                                {
                                    let value = if target >= 0f64 {
                                        self::read_required_boolean_fact(
                                            implemented.clone(),
                                            target,
                                        )
                                    } else if kind == "standard collection" {
                                        true
                                    } else if (kind == "copy") || (kind == "string") {
                                        self::read_required_boolean_fact(leaves.clone(), at)
                                    } else {
                                        self::ml_array::includes(
                                            &{
                                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    String::from("box"),
                                                    String::from("vector"),
                                                    String::from("pending vector"),
                                                    String::from("option"),
                                                    String::from("result"),
                                                    String::from("constructor"),
                                                    String::from("nominal"),
                                                    String::from("tuple"),
                                                ])))
                                                .borrow()
                                                .clone()
                                            },
                                            &kind.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        )
                                    };
                                    {
                                        let edge = start;
                                        {
                                            let value_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop45(ends.clone(), edges.clone(), values.clone(), at, value, edge);
                                            {
                                                let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored46 = {
                                                    let ml_method_owner179 = values.clone();
                                                    let ml_method_argument179_0 = value_2;
                                                    let ml_inserted = vec![ml_method_argument179_0];
                                                    let mut ml_method_values179 =
                                                        ml_method_owner179.borrow_mut();
                                                    let ml_length = ml_method_values179
                                                        .len()
                                                        .checked_add(ml_inserted.len())
                                                        .filter(|length| {
                                                            *length <= 4_294_967_295usize
                                                        })
                                                        .expect("Invalid array length");
                                                    ml_method_values179.extend(ml_inserted);
                                                    ml_length as f64
                                                };
                                                {
                                                    let at_2 = at + 1f64;
                                                    {
                                                        (work, at) = (work_2, at_2);
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
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Done(work)
        };
    }
}
