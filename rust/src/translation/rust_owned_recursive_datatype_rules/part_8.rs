// Generated from complete js/src/translation/rust-owned-recursive-datatype-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Result;
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Result;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop56;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop57;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop33(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut count: f64,
    mut size: f64,
    mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut work: f64,
    mut changed: bool,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let safe = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s77 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop56(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, eligible.clone(), copy_nodes.clone(), work, safe.clone(), at);
                            match source_ml_source1_s77 {
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Done(work_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop57(roots.clone(), size, eligible.clone(), changed_2, safe.clone(), owner);
                                            {
                                                (work, changed) = (work_2, changed_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Return(source_ml_temporary_result58) => {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Return(source_ml_temporary_result58.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Done(work, changed)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop66Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop66Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop66(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
    mut edge: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop66Result {
    loop {
        if edge < (self::read_required_number_fact(ends.clone(), at)) {
            {
                let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored67 = {
                    let ml_method_owner188 = pending.clone();
                    let ml_method_argument188_0 =
                        self::read_required_number_fact(edges.clone(), edge);
                    let ml_inserted = vec![ml_method_argument188_0];
                    let mut ml_method_values188 = ml_method_owner188.borrow_mut();
                    let ml_length = ml_method_values188
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values188.extend(ml_inserted);
                    ml_length as f64
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
        return {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop66Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop66Done
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
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop62(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut work: f64,
    mut owner: f64,
    mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Result {
    loop {
        return {
            let ml_library_condition_1 = {
                let ml_value = {
                    let ml_reference_array190 = pending.clone();
                    ml_reference_array190.borrow().len() as f64
                };
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_1 {
                {
                    let work_2 = work + 1f64;
                    if work_2 > 1_048_576f64 {
                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                    } else {
                        {
                            let at = self::read_required_number_fact(
                                pending.clone(),
                                ({
                                    let ml_reference_array191 = pending.clone();
                                    ml_reference_array191.borrow().len() as f64
                                } - 1f64),
                            );
                            {
                                let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored63 = {
                                    let ml_method_owner192 = pending.clone();
                                    ml_method_owner192.borrow_mut().pop()
                                };
                                if self::read_required_boolean_fact(seen.clone(), at) {
                                    {
                                        work = work_2;
                                        continue;
                                    }
                                }
                                {
                                    {
                                        let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored64 = {
                                            let ml_array_owner193 = seen.clone();
                                            let ml_array_argument193_0: f64 = at;
                                            let ml_array_argument193_1: f64 = 1f64;
                                            let ml_array_argument193_2: bool = true;
                                            let ml_array_length = ml_array_owner193.borrow().len();
                                            let ml_array_start = {
                                                let ml_relative_value: f64 = ml_array_argument193_0;
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
                                                })
                                                    as usize
                                            };
                                            let ml_array_count = {
                                                let ml_delete_value: f64 = ml_array_argument193_1;
                                                ml_delete_value
                                                    .trunc()
                                                    .max(0f64)
                                                    .min((ml_array_length - ml_array_start) as f64)
                                                    as usize
                                            };
                                            let ml_array_inserted = vec![ml_array_argument193_2];
                                            let _ = (ml_array_length - ml_array_count)
                                                .checked_add(ml_array_inserted.len())
                                                .filter(|length| *length <= 4_294_967_295usize)
                                                .expect("Invalid array length");
                                            let ml_removed: Vec<_> = ml_array_owner193
                                                .borrow_mut()
                                                .splice(
                                                    ml_array_start..ml_array_start + ml_array_count,
                                                    ml_array_inserted,
                                                )
                                                .collect();
                                            std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                                        };
                                        {
                                            let kind =
                                                self::read_required_text_fact(kinds.clone(), at);
                                            {
                                                let target = self::read_required_number_fact(
                                                    targets.clone(),
                                                    at,
                                                );
                                                if target >= 0f64 {
                                                    if target == owner {
                                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a recursive source datatype size cycle needs an actual Box or Vec indirection")])))
                                                    } else {
                                                        {
                                                            let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored65 = {
                                                                let ml_method_owner194 =
                                                                    pending.clone();
                                                                let ml_method_argument194_0 =
                                                                    self::read_required_number_fact(
                                                                        roots.clone(),
                                                                        target,
                                                                    );
                                                                let ml_inserted =
                                                                    vec![ml_method_argument194_0];
                                                                let mut ml_method_values194 =
                                                                    ml_method_owner194.borrow_mut();
                                                                let ml_length = ml_method_values194
                                                                    .len()
                                                                    .checked_add(ml_inserted.len())
                                                                    .filter(|length| {
                                                                        *length
                                                                            <= 4_294_967_295usize
                                                                    })
                                                                    .expect("Invalid array length");
                                                                ml_method_values194
                                                                    .extend(ml_inserted);
                                                                ml_length as f64
                                                            };
                                                            {
                                                                work = work_2;
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if ((kind == "box") || (kind == "vector"))
                                                        || (kind == "pending vector")
                                                    {
                                                        {
                                                            work = work_2;
                                                            continue;
                                                        }
                                                    }
                                                    {
                                                        {
                                                            let start = if at == 0f64 {
                                                                0f64
                                                            } else {
                                                                self::read_required_number_fact(
                                                                    ends.clone(),
                                                                    (at - 1f64),
                                                                )
                                                            };
                                                            {
                                                                let edge = start;
                                                                {
                                                                    let source_ml_source1_s78 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop66(ends.clone(), edges.clone(), pending.clone(), at, edge);
                                                                    match source_ml_source1_s78 {
                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop66Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop66Done => {
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
            } else {
                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Done(work)
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop61Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop61Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop61(
    mut count: f64,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop61Result {
    loop {
        if at < count {
            {
                let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored68 = {
                    let ml_method_owner195 = seen.clone();
                    let ml_method_argument195_0 = false;
                    let ml_inserted = vec![ml_method_argument195_0];
                    let mut ml_method_values195 = ml_method_owner195.borrow_mut();
                    let ml_length = ml_method_values195
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values195.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop61Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop61Done
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
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop32(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut size: f64,
    mut work: f64,
    mut owner: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result {
    loop {
        return if owner < size {
            {
                let work_2 = work + count;
                if work_2 > 1_048_576f64 {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![
                            self::read_required_number_fact(roots.clone(), owner),
                        ]));
                        {
                            let seen =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let at = 0f64;
                                {
                                    let source_ml_source1_s79 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop61(count, seen.clone(), at);
                                    match source_ml_source1_s79 {
                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop61Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop61Done => {
                                            {
                                                let source_ml_source1_s80 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop62(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), work_2, owner, pending, seen.clone());
                                                match source_ml_source1_s80 {
                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Done(work_3) => {
                                                        {
                                                            let owner_2 = owner + 1f64;
                                                            {
                                                                (work, owner) = (work_3, owner_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop62Return(source_ml_temporary_result62) => {
                                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(source_ml_temporary_result62.clone())
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(work)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop31Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop31Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop69(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut end: f64,
    mut copy: bool,
    mut edge: f64,
) -> bool {
    loop {
        if edge < end {
            if !(self::read_required_boolean_fact(
                copy_nodes.clone(),
                self::read_required_number_fact(edges.clone(), edge),
            )) {
                {
                    let copy_2 = false;
                    {
                        let edge_2 = edge + 1f64;
                        {
                            (copy, edge) = (copy_2, edge_2);
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
        return { copy };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop31(
    mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop31Result {
    loop {
        if at < count {
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
                            let end = self::read_required_number_fact(ends.clone(), at);
                            {
                                let copy = if target >= 0f64 {
                                    self::read_required_boolean_fact(copies.clone(), target)
                                } else {
                                    kind == "copy"
                                };
                                {
                                    let copy_4 = if kind == "tuple" {
                                        {
                                            let copy_2 = true;
                                            {
                                                let edge = start;
                                                {
                                                    self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop69(edges.clone(), copy_nodes.clone(), end, copy_2, edge)
                                                }
                                            }
                                        }
                                    } else {
                                        copy
                                    };
                                    {
                                        let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored70 = {
                                            let ml_method_owner196 = copy_nodes.clone();
                                            let ml_method_argument196_0 = copy_4;
                                            let ml_inserted = vec![ml_method_argument196_0];
                                            let mut ml_method_values196 =
                                                ml_method_owner196.borrow_mut();
                                            let ml_length = ml_method_values196
                                                .len()
                                                .checked_add(ml_inserted.len())
                                                .filter(|length| *length <= 4_294_967_295usize)
                                                .expect("Invalid array length");
                                            ml_method_values196.extend(ml_inserted);
                                            ml_length as f64
                                        };
                                        {
                                            let at_2 = at + 1f64;
                                            {
                                                at = at_2;
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
        return {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop31Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Result {
    MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Done,
    MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_owned_box_layer_clone_loop71(
    mut wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut copied: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Result {
    loop {
        return if at
            < ({
                let ml_reference_array197 = owners.clone();
                ml_reference_array197.borrow().len() as f64
            }) {
            if (self::read_required_text_fact(wrappers.clone(), at)) == "box" {
                if self::read_required_boolean_fact(copied.clone(), at) {
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Return(true)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            } else {
                self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Return(false)
            }
        } else {
            self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop71Done
        };
    }
}
