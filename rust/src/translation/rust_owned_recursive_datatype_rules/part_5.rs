// Generated from complete js/src/translation/rust-owned-recursive-datatype-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_nominal_ownership_node_role;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_nominal_field_lifetime_error(
    field: bool,
    wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
) -> String {
    if field {
        {
            let references = {
                let ml_reference_array142 = std::rc::Rc::new(std::cell::RefCell::new({
                    let ml_values140 = { wrappers.borrow().clone() };
                    let ml_callback140 =
                        std::rc::Rc::new(move |layer: String| -> bool { layer == "reference" });
                    ml_values140
                        .iter()
                        .cloned()
                        .enumerate()
                        .filter_map(|(ml_index140, ml_value140)| {
                            if ml_callback140(ml_value140.clone()) {
                                Some(ml_value140)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                }));
                ml_reference_array142.borrow().len() as f64
            };
            if (references
                != ({
                    let ml_reference_array143 = lifetimes.clone();
                    ml_reference_array143.borrow().len() as f64
                }))
                || {
                    let ml_values144 = { lifetimes.borrow().clone() };
                    let ml_callback144 =
                        std::rc::Rc::new(move |lifetime: String| -> bool { lifetime != "static" });
                    ml_values144
                        .iter()
                        .cloned()
                        .enumerate()
                        .any(|(ml_index144, ml_value144)| ml_callback144(ml_value144))
                }
            {
                String::from(
                    "a nongeneric datatype reference field requires its explicit static lifetime",
                )
            } else {
                String::new()
            }
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_vector_element_role(
    kind: &str,
    copy: bool,
    direct: bool,
    fields: bool,
    actual: &str,
) -> String {
    if (actual == "copy") || (actual == "owned") {
        actual.to_owned()
    } else if ((((kind).is_empty()) && copy) && direct) && !fields {
        String::from("copy")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_vector_element_type_match(
    actual: &str,
    expected: &str,
    same: bool,
    copy: bool,
) -> bool {
    same || ((((actual).is_empty()) && (expected == "fixed")) && copy)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_vector_nominal_type_identity(
    left: &str,
    right: &str,
    references: bool,
) -> bool {
    (references && (!(left).is_empty())) && (left == right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_box_pointer_owner_child(
    kind: &str,
    path: &str,
    arguments_count: f64,
) -> bool {
    ((kind == "library") && (path == "alloc::boxed::Box")) && (arguments_count == 1f64)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop4Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop4Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop4(
    mut size: f64,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut owner: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Result {
    loop {
        if owner < size {
            {
                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored5 = {
                    let ml_method_owner146 = result.clone();
                    let ml_method_argument146_0 =
                        self::read_required_text_fact(known.clone(), owner);
                    let ml_inserted = vec![ml_method_argument146_0];
                    let mut ml_method_values146 = ml_method_owner146.borrow_mut();
                    let ml_length = ml_method_values146
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values146.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored6 = {
                        let ml_method_owner147 = result.clone();
                        let ml_method_argument147_0 =
                            if self::read_required_boolean_fact(implemented.clone(), owner) {
                                String::from("Clone")
                            } else {
                                String::new()
                            };
                        let ml_inserted = vec![ml_method_argument147_0];
                        let mut ml_method_values147 = ml_method_owner147.borrow_mut();
                        let ml_length = ml_method_values147
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values147.extend(ml_inserted);
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
        return {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop9Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop9Done(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop9(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut end: f64,
    mut ready: bool,
    mut clone: bool,
    mut edge: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Result {
    loop {
        if edge < end {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                {
                    let ready_3 = if (self::read_required_text_fact(roles.clone(), child)) == "" {
                        { false }
                    } else {
                        ready
                    };
                    if !(self::read_required_boolean_fact(clone_fields.clone(), child)) {
                        {
                            let clone_2 = false;
                            {
                                let edge_2 = edge + 1f64;
                                {
                                    (ready, clone, edge) = (ready_3, clone_2, edge_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let edge_3 = edge + 1f64;
                            {
                                (ready, edge) = (ready_3, edge_3);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Done(ready, clone)
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
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop8(
    mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut size: f64,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut changed: bool,
    mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owner: f64,
) -> bool {
    loop {
        return if owner < size {
            if self::read_required_boolean_fact(copies.clone(), owner) {
                {
                    let owner_2 = owner + 1f64;
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            }
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
                            let end = self::read_required_number_fact(ends.clone(), root);
                            {
                                let ready = end > start;
                                {
                                    let clone = true;
                                    {
                                        let edge = start;
                                        {
                                            let source_ml_source1_s65 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop9(edges.clone(), roles.clone(), clone_fields.clone(), end, ready, clone, edge);
                                            match source_ml_source1_s65 {
                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop9Done(ready_2, clone_2) => {
                                                    {
                                                        let changed_3 = if ready_2 && ((self::read_required_text_fact(known.clone(), owner)).is_empty()) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored10 = { let ml_array_owner148 = known.clone(); let ml_array_argument148_0: f64 = owner; let ml_array_argument148_1: f64 = 1f64; let ml_array_argument148_2: String = String::from("owned"); let ml_array_length = ml_array_owner148.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument148_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument148_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument148_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner148.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                {
                                                                    true
                                                                }
                                                            }
                                                        } else {
                                                            changed
                                                        };
                                                        if ((ready_2 && clone_2) && self::read_required_boolean_fact(clones.clone(), owner)) && !(self::read_required_boolean_fact(implemented.clone(), owner)) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored11 = { let ml_array_owner149 = implemented.clone(); let ml_array_argument149_0: f64 = owner; let ml_array_argument149_1: f64 = 1f64; let ml_array_argument149_2: bool = true; let ml_array_length = ml_array_owner149.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument149_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument149_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument149_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner149.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                {
                                                                    let changed_4 = true;
                                                                    {
                                                                        let owner_3 = owner + 1f64;
                                                                        {
                                                                            (changed, owner) = (changed_4, owner_3);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
{
                                                            {
                                                                let owner_4 = owner + 1f64;
                                                                {
                                                                    (changed, owner) = (changed_3, owner_4);
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
            changed
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(f64),
    MlTemporaryPlanRustNominalOwnershipOwnersLoop7Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop12Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop12(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
    mut child_roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut child_clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut edge: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result {
    loop {
        if edge < (self::read_required_number_fact(ends.clone(), at)) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                {
                    let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored15 = {
                        let ml_method_owner150 = child_roles.clone();
                        let ml_method_argument150_0 =
                            self::read_required_text_fact(roles.clone(), child);
                        let ml_inserted = vec![ml_method_argument150_0];
                        let mut ml_method_values150 = ml_method_owner150.borrow_mut();
                        let ml_length = ml_method_values150
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values150.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored16 = {
                            let ml_method_owner151 = child_clones.clone();
                            let ml_method_argument151_0 =
                                self::read_required_boolean_fact(clone_fields.clone(), child);
                            let ml_inserted = vec![ml_method_argument151_0];
                            let mut ml_method_values151 = ml_method_owner151.borrow_mut();
                            let ml_length = ml_method_values151
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            ml_method_values151.extend(ml_inserted);
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
            }
        }
        return {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Done
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
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop7(
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut work: f64,
    mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut at: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result {
    loop {
        return if at < count {
            {
                let work_2 = work + 1f64;
                if work_2 > 1_048_576f64 {
                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("actual nominal ownership graph reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let child_roles =
                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                        {
                            let child_clones =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let start = if at == 0f64 {
                                    0f64
                                } else {
                                    self::read_required_number_fact(ends.clone(), (at - 1f64))
                                };
                                {
                                    let edge = start;
                                    {
                                        let source_ml_source1_s66 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop12(ends.clone(), edges.clone(), roles.clone(), clone_fields.clone(), at, child_roles.clone(), child_clones.clone(), edge);
                                        match source_ml_source1_s66 {
                                            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Done => {
                                                {
                                                    let target = self::read_required_number_fact(targets.clone(), at);
                                                    {
                                                        let kind = self::read_required_text_fact(kinds.clone(), at);
                                                        {
                                                            let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored13 = { let ml_method_owner152 = roles.clone(); let ml_method_argument152_0 = if target >= 0f64 {
                                                                self::read_required_text_fact(known.clone(), target)
                                                            } else {
                                                                self::read_rust_nominal_ownership_node_role(&(kind), child_roles.clone())
                                                            }; let ml_inserted = vec![ml_method_argument152_0]; let mut ml_method_values152 = ml_method_owner152.borrow_mut(); let ml_length = ml_method_values152.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values152.extend(ml_inserted); ml_length as f64 };
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored14 = { let ml_method_owner153 = clone_fields.clone(); let ml_method_argument153_0 = if target >= 0f64 {
                                                                    self::read_required_boolean_fact(implemented.clone(), target)
                                                                } else {
                                                                    (((kind == "copy") || (kind == "string")) || (kind == "standard collection")) || ((self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("vector"), String::from("tuple"), String::from("constructor"), String::from("option"), String::from("result")]))).borrow().clone() }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values155 = { child_clones.borrow().clone() }; let ml_callback155 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values155.iter().cloned().enumerate().all(|(ml_index155, ml_value155)| ml_callback155(ml_value155)) }) && (!(self::read_rust_nominal_ownership_node_role(&(kind), child_roles.clone())).is_empty()))
                                                                }; let ml_inserted = vec![ml_method_argument153_0]; let mut ml_method_values153 = ml_method_owner153.borrow_mut(); let ml_length = ml_method_values153.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values153.extend(ml_inserted); ml_length as f64 };
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
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(work)
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
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop3(
    mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut size: f64,
    mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut changed: bool,
    mut work: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let roles = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                    {
                        let clone_fields =
                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s67 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop7(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), count, known.clone(), implemented.clone(), work, roles.clone(), clone_fields.clone(), at);
                                match source_ml_source1_s67 {
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(work_2) => {
                                        {
                                            let owner = 0f64;
                                            {
                                                let changed_3 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop8(copies.clone(), clones.clone(), ends.clone(), edges.clone(), roots.clone(), size, known.clone(), implemented.clone(), changed_2, roles.clone(), clone_fields.clone(), owner);
                                                {
                                                    (changed, work) = (changed_3, work_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Return(source_ml_temporary_result11) => {
                                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(source_ml_temporary_result11.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop2Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(
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
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop2(
    mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut size: f64,
    mut owner: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result {
    loop {
        return if owner < size {
            {
                let name = self::read_required_text_fact(owners.clone(), owner);
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    if ((((((name).is_empty())
                        || ((self::ml_array::index_of(
                            &{ owners.borrow().clone() },
                            &name.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) != owner))
                        || !(self::ml_math::is_integer(root)))
                        || (root < 0f64))
                        || (root >= count))
                        || ((self::read_required_text_fact(kinds.clone(), root)) != "nominal")
                    {
                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal declaration roots")])))
                    } else if self::read_required_boolean_fact(copies.clone(), owner)
                        && !(self::read_required_boolean_fact(clones.clone(), owner))
                    {
                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("actual nominal Copy must retain its complete Clone derivation")])))
                    } else {
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
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop1Done(f64),
    MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop17Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop17Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop17Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop17(
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
    mut end: f64,
    mut edge: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Result {
    loop {
        return if edge < end {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                if (!(self::ml_math::is_integer(child)) || (child < 0f64)) || (child >= at) {
                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("nominal field edges need actual postorder identities")])))
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
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop1(
    mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut count: f64,
    mut size: f64,
    mut previous: f64,
    mut at: f64,
) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result {
    loop {
        return if at < count {
            {
                let end = self::read_required_number_fact(ends.clone(), at);
                {
                    let target = self::read_required_number_fact(targets.clone(), at);
                    if ((((!(self::ml_math::is_integer(end)) || (end < previous))
                        || (end
                            > ({
                                let ml_reference_array158 = edges.clone();
                                ml_reference_array158.borrow().len() as f64
                            })))
                        || !(self::ml_math::is_integer(target)))
                        || (target < -1f64))
                        || (target >= size)
                    {
                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership graph")])))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s68 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop17(edges.clone(), at, end, edge);
                                match source_ml_source1_s68 {
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Done => {
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
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop17Return(source_ml_temporary_result17) => {
                                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(source_ml_temporary_result17.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Done(previous)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalPatternTransfersLoop18Result {
    MlTemporaryPlanRustNominalPatternTransfersLoop18Done,
    MlTemporaryPlanRustNominalPatternTransfersLoop18Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalPatternTransfersLoop19Result {
    MlTemporaryPlanRustNominalPatternTransfersLoop19Done,
    MlTemporaryPlanRustNominalPatternTransfersLoop19Return(
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_pattern_transfers_loop19(
    mut identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut size: f64,
    mut index: f64,
    identity: &str,
    mut peer: f64,
) -> self::MlTemporaryPlanRustNominalPatternTransfersLoop19Result {
    loop {
        return if peer < size {
            if (((peer != index)
                && ((self::read_required_text_fact(identities.clone(), peer)) == identity))
                && self::read_required_boolean_fact(owned.clone(), peer))
                && !(self::read_required_boolean_fact(fields.clone(), peer))
            {
                self::MlTemporaryPlanRustNominalPatternTransfersLoop19Result::MlTemporaryPlanRustNominalPatternTransfersLoop19Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a whole nominal alias cannot also move its owned fields")])))
            } else {
                {
                    let peer_2 = peer + 1f64;
                    {
                        peer = peer_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalPatternTransfersLoop19Result::MlTemporaryPlanRustNominalPatternTransfersLoop19Done
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
pub fn source_ml_temporary_plan_rust_nominal_pattern_transfers_loop18(
    mut identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut produced: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut deferred: bool,
    mut size: f64,
    mut plan: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    mut index: f64,
) -> self::MlTemporaryPlanRustNominalPatternTransfersLoop18Result {
    loop {
        return if index < size {
            if !(self::read_required_boolean_fact(fields.clone(), index))
                || !(self::read_required_boolean_fact(owned.clone(), index))
            {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                if deferred {
                    self::MlTemporaryPlanRustNominalPatternTransfersLoop18Result::MlTemporaryPlanRustNominalPatternTransfersLoop18Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("owned nominal guarded fields need their deferred partial-move interpretation")])))
                } else {
                    {
                        let identity = self::read_required_text_fact(identities.clone(), index);
                        if ((identity).is_empty())
                            && !(self::read_required_boolean_fact(produced.clone(), index))
                        {
                            self::MlTemporaryPlanRustNominalPatternTransfersLoop18Result::MlTemporaryPlanRustNominalPatternTransfersLoop18Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a nominal field move needs its actual available parent or actual producer")])))
                        } else {
                            {
                                let peer = 0f64;
                                {
                                    let source_ml_source1_s69 = self::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop19(identities.clone(), fields.clone(), owned.clone(), size, index, &(identity), peer);
                                    match source_ml_source1_s69 {
                                        self::MlTemporaryPlanRustNominalPatternTransfersLoop19Result::MlTemporaryPlanRustNominalPatternTransfersLoop19Done => {
                                            if (!(identity).is_empty()) && !(self::ml_array::includes(&{ seen.borrow().clone() }, &identity.clone(), 0f64, false, |a, b| a == b)) {
                                                {
                                                    let source_ml_temporary_plan_rust_nominal_pattern_transfers_ignored20 = { let ml_method_owner160 = seen.clone(); let ml_method_argument160_0 = identity.clone(); let ml_inserted = vec![ml_method_argument160_0]; let mut ml_method_values160 = ml_method_owner160.borrow_mut(); let ml_length = ml_method_values160.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values160.extend(ml_inserted); ml_length as f64 };
                                                    {
                                                        let source_ml_temporary_plan_rust_nominal_pattern_transfers_ignored21 = { let ml_method_owner161 = plan.clone(); let ml_method_argument161_0 = {
                                                            let source_ml_source1_string_argument70 = index;
                                                            self::ml_number::js_number(source_ml_source1_string_argument70)
                                                        }; let ml_inserted = vec![ml_method_argument161_0]; let mut ml_method_values161 = ml_method_owner161.borrow_mut(); let ml_length = ml_method_values161.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values161.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let index_3 = index + 1f64;
                                                            {
                                                                index = index_3;
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
{
                                                {
                                                    let index_4 = index + 1f64;
                                                    {
                                                        index = index_4;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanRustNominalPatternTransfersLoop19Result::MlTemporaryPlanRustNominalPatternTransfersLoop19Return(source_ml_temporary_result21) => {
                                            self::MlTemporaryPlanRustNominalPatternTransfersLoop18Result::MlTemporaryPlanRustNominalPatternTransfersLoop18Return(source_ml_temporary_result21.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalPatternTransfersLoop18Result::MlTemporaryPlanRustNominalPatternTransfersLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalVectorElementOwnersLoop22Result {
    MlTemporaryPlanRustNominalVectorElementOwnersLoop22Done,
    MlTemporaryPlanRustNominalVectorElementOwnersLoop22Return(
        std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    ),
}
