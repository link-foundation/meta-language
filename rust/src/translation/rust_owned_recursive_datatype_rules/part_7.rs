// Generated from complete js/src/translation/rust-owned-recursive-datatype-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Result;
use super::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Result;
use super::ml_array;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop42;
use super::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop43;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop35(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut count: f64,
    mut size: f64,
    mut work: f64,
    mut changed: bool,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s75 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop42(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, work, implemented.clone(), values.clone(), at);
                            match source_ml_source1_s75 {
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Done(work_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop43(roots.clone(), size, changed_2, implemented.clone(), values.clone(), owner);
                                            {
                                                (work, changed) = (work_2, changed_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop42Return(source_ml_temporary_result44) => {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Return(source_ml_temporary_result44.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop35Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Done(f64, bool),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop49(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut changed: bool,
    mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut rank: f64,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owner: f64,
    mut root: f64,
    mut start: f64,
    mut edge: f64,
) -> bool {
    loop {
        return if edge < (self::read_required_number_fact(ends.clone(), root)) {
            if self::read_required_boolean_fact(
                values.clone(),
                self::read_required_number_fact(edges.clone(), edge),
            ) {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored50 = {
                        let ml_array_owner180 = inhabited.clone();
                        let ml_array_argument180_0: f64 = owner;
                        let ml_array_argument180_1: f64 = 1f64;
                        let ml_array_argument180_2: bool = true;
                        let ml_array_length = ml_array_owner180.borrow().len();
                        let ml_array_start = {
                            let ml_relative_value: f64 = ml_array_argument180_0;
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
                            let ml_delete_value: f64 = ml_array_argument180_1;
                            ml_delete_value
                                .trunc()
                                .max(0f64)
                                .min((ml_array_length - ml_array_start) as f64)
                                as usize
                        };
                        let ml_array_inserted = vec![ml_array_argument180_2];
                        let _ = (ml_array_length - ml_array_count)
                            .checked_add(ml_array_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        let ml_removed: Vec<_> = ml_array_owner180
                            .borrow_mut()
                            .splice(
                                ml_array_start..ml_array_start + ml_array_count,
                                ml_array_inserted,
                            )
                            .collect();
                        std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                    };
                    {
                        let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored51 = {
                            let ml_array_owner181 = ranks.clone();
                            let ml_array_argument181_0: f64 = owner;
                            let ml_array_argument181_1: f64 = 1f64;
                            let ml_array_argument181_2: f64 = rank;
                            let ml_array_length = ml_array_owner181.borrow().len();
                            let ml_array_start = {
                                let ml_relative_value: f64 = ml_array_argument181_0;
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
                                let ml_delete_value: f64 = ml_array_argument181_1;
                                ml_delete_value
                                    .trunc()
                                    .max(0f64)
                                    .min((ml_array_length - ml_array_start) as f64)
                                    as usize
                            };
                            let ml_array_inserted = vec![ml_array_argument181_2];
                            let _ = (ml_array_length - ml_array_count)
                                .checked_add(ml_array_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            let ml_removed: Vec<_> = ml_array_owner181
                                .borrow_mut()
                                .splice(
                                    ml_array_start..ml_array_start + ml_array_count,
                                    ml_array_inserted,
                                )
                                .collect();
                            std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                        };
                        {
                            let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored52 = {
                                let ml_array_owner182 = witnesses.clone();
                                let ml_array_argument182_0: f64 = owner;
                                let ml_array_argument182_1: f64 = 1f64;
                                let ml_array_argument182_2: f64 = edge - start;
                                let ml_array_length = ml_array_owner182.borrow().len();
                                let ml_array_start = {
                                    let ml_relative_value: f64 = ml_array_argument182_0;
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
                                    let ml_delete_value: f64 = ml_array_argument182_1;
                                    ml_delete_value
                                        .trunc()
                                        .max(0f64)
                                        .min((ml_array_length - ml_array_start) as f64)
                                        as usize
                                };
                                let ml_array_inserted = vec![ml_array_argument182_2];
                                let _ = (ml_array_length - ml_array_count)
                                    .checked_add(ml_array_inserted.len())
                                    .filter(|length| *length <= 4_294_967_295usize)
                                    .expect("Invalid array length");
                                let ml_removed: Vec<_> = ml_array_owner182
                                    .borrow_mut()
                                    .splice(
                                        ml_array_start..ml_array_start + ml_array_count,
                                        ml_array_inserted,
                                    )
                                    .collect();
                                std::rc::Rc::new(std::cell::RefCell::new(ml_removed))
                            };
                            { true }
                        }
                    }
                }
            } else {
                {
                    let edge_2 = edge + 1f64;
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
        } else {
            changed
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
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop48(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut size: f64,
    mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut changed: bool,
    mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut rank: f64,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owner: f64,
) -> bool {
    loop {
        if owner < size {
            if self::read_required_boolean_fact(eligible.clone(), owner)
                && !(self::read_required_boolean_fact(inhabited.clone(), owner))
            {
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    {
                        let start = if root == 0f64 {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (root - 1f64))
                        };
                        {
                            let edge = start;
                            {
                                let changed_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop49(ends.clone(), edges.clone(), changed, inhabited.clone(), ranks.clone(), witnesses.clone(), rank, values.clone(), owner, root, start, edge);
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
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop53(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut end: f64,
    mut value: bool,
    mut edge: f64,
) -> bool {
    loop {
        if edge < end {
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
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop54(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut end: f64,
    mut value: bool,
    mut edge: f64,
) -> bool {
    loop {
        if edge < end {
            if self::read_required_boolean_fact(
                values.clone(),
                self::read_required_number_fact(edges.clone(), edge),
            ) {
                {
                    let value_2 = true;
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
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop47(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut count: f64,
    mut work: f64,
    mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Result {
    loop {
        return if at < count {
            {
                let work_2 = work + 1f64;
                if work_2 > 1_048_576f64 {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
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
                                    let end = self::read_required_number_fact(ends.clone(), at);
                                    {
                                        let value = if target >= 0f64 {
                                            self::read_required_boolean_fact(
                                                inhabited.clone(),
                                                target,
                                            )
                                        } else if kind == "standard collection" {
                                            true
                                        } else if (kind == "copy") || (kind == "string") {
                                            self::read_required_boolean_fact(leaves.clone(), at)
                                        } else {
                                            self::ml_array::includes(
                                                &{
                                                    (std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![
                                                            String::from("vector"),
                                                            String::from("pending vector"),
                                                            String::from("option"),
                                                        ],
                                                    )))
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
                                            let value_4 = if ((kind == "constructor")
                                                || (kind == "tuple"))
                                                || (kind == "box")
                                            {
                                                {
                                                    let value_2 = true;
                                                    {
                                                        let edge = start;
                                                        {
                                                            self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop53(edges.clone(), values.clone(), end, value_2, edge)
                                                        }
                                                    }
                                                }
                                            } else {
                                                value
                                            };
                                            {
                                                let value_7 = if (kind == "nominal")
                                                    || (kind == "result")
                                                {
                                                    {
                                                        let value_5 = false;
                                                        {
                                                            let edge_2 = start;
                                                            {
                                                                self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop54(edges.clone(), values.clone(), end, value_5, edge_2)
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    value_4
                                                };
                                                {
                                                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored55 = {
                                                        let ml_method_owner184 = values.clone();
                                                        let ml_method_argument184_0 = value_7;
                                                        let ml_inserted =
                                                            vec![ml_method_argument184_0];
                                                        let mut ml_method_values184 =
                                                            ml_method_owner184.borrow_mut();
                                                        let ml_length = ml_method_values184
                                                            .len()
                                                            .checked_add(ml_inserted.len())
                                                            .filter(|length| {
                                                                *length <= 4_294_967_295usize
                                                            })
                                                            .expect("Invalid array length");
                                                        ml_method_values184.extend(ml_inserted);
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
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Done(work)
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
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop34(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut count: f64,
    mut size: f64,
    mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut work: f64,
    mut changed: bool,
    mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut rank: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let rank_2 = rank + 1f64;
                    {
                        let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s76 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop47(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, work, inhabited.clone(), values.clone(), at);
                                match source_ml_source1_s76 {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Done(work_2) => {
                                        {
                                            let owner = 0f64;
                                            {
                                                let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop48(ends.clone(), edges.clone(), roots.clone(), size, eligible.clone(), changed_2, inhabited.clone(), ranks.clone(), witnesses.clone(), rank_2, values.clone(), owner);
                                                {
                                                    (work, changed, rank) = (work_2, changed_3, rank_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop47Return(source_ml_temporary_result52) => {
                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Return(source_ml_temporary_result52.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop34Done(work, changed)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Done(f64, bool),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop33Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop57(
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut size: f64,
    mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut changed: bool,
    mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owner: f64,
) -> bool {
    loop {
        if owner < size {
            if self::read_required_boolean_fact(eligible.clone(), owner)
                && !(self::read_required_boolean_fact(
                    safe.clone(),
                    self::read_required_number_fact(roots.clone(), owner),
                ))
            {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored58 = {
                        let ml_array_owner185 = eligible.clone();
                        let ml_array_argument185_0: f64 = owner;
                        let ml_array_argument185_1: f64 = 1f64;
                        let ml_array_argument185_2: bool = false;
                        let ml_array_length = ml_array_owner185.borrow().len();
                        let ml_array_start = {
                            let ml_relative_value: f64 = ml_array_argument185_0;
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
                            let ml_delete_value: f64 = ml_array_argument185_1;
                            ml_delete_value
                                .trunc()
                                .max(0f64)
                                .min((ml_array_length - ml_array_start) as f64)
                                as usize
                        };
                        let ml_array_inserted = vec![ml_array_argument185_2];
                        let _ = (ml_array_length - ml_array_count)
                            .checked_add(ml_array_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        let ml_removed: Vec<_> = ml_array_owner185
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
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop59(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut end: f64,
    mut value: bool,
    mut edge: f64,
) -> bool {
    loop {
        if edge < end {
            if !(self::read_required_boolean_fact(
                safe.clone(),
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
pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop56(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut count: f64,
    mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut work: f64,
    mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Result {
    loop {
        return if at < count {
            {
                let work_2 = work + 1f64;
                if work_2 > 1_048_576f64 {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
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
                                    let end = self::read_required_number_fact(ends.clone(), at);
                                    {
                                        let value = if target >= 0f64 {
                                            self::read_required_boolean_fact(
                                                eligible.clone(),
                                                target,
                                            )
                                        } else if kind == "standard collection" {
                                            true
                                        } else if (kind == "copy") || (kind == "string") {
                                            self::read_required_boolean_fact(leaves.clone(), at)
                                        } else {
                                            self::ml_array::includes(
                                                &{
                                                    (std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![
                                                            String::from("box"),
                                                            String::from("vector"),
                                                            String::from("pending vector"),
                                                            String::from("option"),
                                                            String::from("result"),
                                                            String::from("constructor"),
                                                            String::from("nominal"),
                                                        ],
                                                    )))
                                                    .borrow()
                                                    .clone()
                                                },
                                                &kind.clone(),
                                                0f64,
                                                false,
                                                |a, b| a == b,
                                            ) || ((kind == "tuple")
                                                && self::read_required_boolean_fact(
                                                    copy_nodes.clone(),
                                                    at,
                                                ))
                                        };
                                        {
                                            let value_3 = if ((((((kind == "box")
                                                || (kind == "vector"))
                                                || (kind == "pending vector"))
                                                || (kind == "option"))
                                                && ((end - start) != 1f64))
                                                || ((kind == "result") && ((end - start) != 2f64)))
                                                || ((kind == "nominal") && (start == end))
                                            {
                                                { false }
                                            } else {
                                                value
                                            };
                                            {
                                                let edge = start;
                                                {
                                                    let value_4 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop59(edges.clone(), safe.clone(), end, value_3, edge);
                                                    {
                                                        let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored60 = {
                                                            let ml_method_owner187 = safe.clone();
                                                            let ml_method_argument187_0 = value_4;
                                                            let ml_inserted =
                                                                vec![ml_method_argument187_0];
                                                            let mut ml_method_values187 =
                                                                ml_method_owner187.borrow_mut();
                                                            let ml_length = ml_method_values187
                                                                .len()
                                                                .checked_add(ml_inserted.len())
                                                                .filter(|length| {
                                                                    *length <= 4_294_967_295usize
                                                                })
                                                                .expect("Invalid array length");
                                                            ml_method_values187.extend(ml_inserted);
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop56Done(work)
        };
    }
}
