// Generated from complete js/src/translation/rust-nominal-ownership-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result;
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result;
use super::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_nominal_ownership_node_role;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop8;
use super::source_ml_temporary_plan_rust_nominal_ownership_owners_loop12;

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
                                        let source_ml_source1_s59 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop12(ends.clone(), edges.clone(), roles.clone(), clone_fields.clone(), at, child_roles.clone(), child_clones.clone(), edge);
                                        match source_ml_source1_s59 {
                                            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop12Done => {
                                                {
                                                    let target = self::read_required_number_fact(targets.clone(), at);
                                                    {
                                                        let kind = self::read_required_text_fact(kinds.clone(), at);
                                                        {
                                                            let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored13 = { let ml_method_owner85 = roles.clone(); let ml_method_argument85_0 = if target >= 0f64 {
                                                                self::read_required_text_fact(known.clone(), target)
                                                            } else {
                                                                self::read_rust_nominal_ownership_node_role(&(kind), child_roles.clone())
                                                            }; let ml_inserted = vec![ml_method_argument85_0]; let mut ml_method_values85 = ml_method_owner85.borrow_mut(); let ml_length = ml_method_values85.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values85.extend(ml_inserted); ml_length as f64 };
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored14 = { let ml_method_owner86 = clone_fields.clone(); let ml_method_argument86_0 = if target >= 0f64 {
                                                                    self::read_required_boolean_fact(implemented.clone(), target)
                                                                } else {
                                                                    (((kind == "copy") || (kind == "string")) || (kind == "standard collection")) || ((self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("vector"), String::from("tuple"), String::from("constructor"), String::from("option"), String::from("result")]))).borrow().clone() }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values88 = { child_clones.borrow().clone() }; let ml_callback88 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values88.iter().cloned().enumerate().all(|(ml_index88, ml_value88)| ml_callback88(ml_value88)) }) && (!(self::read_rust_nominal_ownership_node_role(&(kind), child_roles.clone())).is_empty()))
                                                                }; let ml_inserted = vec![ml_method_argument86_0]; let mut ml_method_values86 = ml_method_owner86.borrow_mut(); let ml_length = ml_method_values86.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values86.extend(ml_inserted); ml_length as f64 };
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
                                let source_ml_source1_s60 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop7(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), count, known.clone(), implemented.clone(), work, roles.clone(), clone_fields.clone(), at);
                                match source_ml_source1_s60 {
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
                                let ml_reference_array91 = edges.clone();
                                ml_reference_array91.borrow().len() as f64
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
                                let source_ml_source1_s61 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop17(edges.clone(), at, end, edge);
                                match source_ml_source1_s61 {
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
                                    let source_ml_source1_s62 = self::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop19(identities.clone(), fields.clone(), owned.clone(), size, index, &(identity), peer);
                                    match source_ml_source1_s62 {
                                        self::MlTemporaryPlanRustNominalPatternTransfersLoop19Result::MlTemporaryPlanRustNominalPatternTransfersLoop19Done => {
                                            if (!(identity).is_empty()) && !(self::ml_array::includes(&{ seen.borrow().clone() }, &identity.clone(), 0f64, false, |a, b| a == b)) {
                                                {
                                                    let source_ml_temporary_plan_rust_nominal_pattern_transfers_ignored20 = { let ml_method_owner93 = seen.clone(); let ml_method_argument93_0 = identity.clone(); let ml_inserted = vec![ml_method_argument93_0]; let mut ml_method_values93 = ml_method_owner93.borrow_mut(); let ml_length = ml_method_values93.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values93.extend(ml_inserted); ml_length as f64 };
                                                    {
                                                        let source_ml_temporary_plan_rust_nominal_pattern_transfers_ignored21 = { let ml_method_owner94 = plan.clone(); let ml_method_argument94_0 = {
                                                            let source_ml_source1_string_argument63 = index;
                                                            self::ml_number::js_number(source_ml_source1_string_argument63)
                                                        }; let ml_inserted = vec![ml_method_argument94_0]; let mut ml_method_values94 = ml_method_owner94.borrow_mut(); let ml_length = ml_method_values94.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values94.extend(ml_inserted); ml_length as f64 };
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
                let ml_reference_array95 = owners.clone();
                ml_reference_array95.borrow().len() as f64
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
                        let ml_array_owner96 = known.clone();
                        let ml_array_argument96_0: f64 = owner;
                        let ml_array_argument96_1: f64 = 1f64;
                        let ml_array_argument96_2: bool = true;
                        let ml_array_length = ml_array_owner96.borrow().len();
                        let ml_array_start = {
                            let ml_relative_value: f64 = ml_array_argument96_0;
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
                            let ml_delete_value: f64 = ml_array_argument96_1;
                            ml_delete_value
                                .trunc()
                                .max(0f64)
                                .min((ml_array_length - ml_array_start) as f64)
                                as usize
                        };
                        let ml_array_inserted = vec![ml_array_argument96_2];
                        let _ = (ml_array_length - ml_array_count)
                            .checked_add(ml_array_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        let ml_removed: Vec<_> = ml_array_owner96
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
                let ml_reference_array97 = kinds.clone();
                ml_reference_array97.borrow().len() as f64
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
                                                            let ml_method_owner100 = safe.clone();
                                                            let ml_method_argument100_0 = actual_4;
                                                            let ml_inserted =
                                                                vec![ml_method_argument100_0];
                                                            let mut ml_method_values100 =
                                                                ml_method_owner100.borrow_mut();
                                                            let ml_length = ml_method_values100
                                                                .len()
                                                                .checked_add(ml_inserted.len())
                                                                .filter(|length| {
                                                                    *length <= 4_294_967_295usize
                                                                })
                                                                .expect("Invalid array length");
                                                            ml_method_values100.extend(ml_inserted);
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
                            let source_ml_source1_s64 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop23(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), known.clone(), work, safe.clone(), at);
                            match source_ml_source1_s64 {
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
