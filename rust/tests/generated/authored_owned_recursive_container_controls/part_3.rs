pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop6(mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < size) {
            if self::read_required_boolean_fact(copies.clone(), owner) {
                {
                    let owner_2 = (owner + 1f64);
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            } else {
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    {
                        let start = if (root == (0f64)) {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (root - 1f64))
                        };
                        {
                            let end = self::read_required_number_fact(ends.clone(), root);
                            {
                                let ready = (end > start);
                                {
                                    let clone = true;
                                    {
                                        let edge = start;
                                        {
                                            let source_ml_source1_s62 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop7(edges.clone(), roles.clone(), clone_fields.clone(), end, ready, clone, edge);
                                            match source_ml_source1_s62.clone() {
                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(ready_2, clone_2) => {
                                                    {
                                                        let changed_3 = if (ready_2 && ((self::read_required_text_fact(known.clone(), owner)) == "")) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored8 = { let ml_array_owner148 = known.clone(); let ml_array_argument148_0: f64 = owner; let ml_array_argument148_1: f64 = 1f64; let ml_array_argument148_2: String = String::from("owned"); let ml_array_length = ml_array_owner148.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument148_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument148_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument148_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner148.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                {
                                                                    let changed_2 = true;
                                                                    changed_2
                                                                }
                                                            }
                                                        } else {
                                                            changed
                                                        };
                                                        if (((ready_2 && clone_2) && self::read_required_boolean_fact(clones.clone(), owner)) && !(self::read_required_boolean_fact(implemented.clone(), owner))) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored9 = { let ml_array_owner149 = implemented.clone(); let ml_array_argument149_0: f64 = owner; let ml_array_argument149_1: f64 = 1f64; let ml_array_argument149_2: bool = true; let ml_array_length = ml_array_owner149.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument149_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument149_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument149_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner149.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                {
                                                                    let changed_4 = true;
                                                                    {
                                                                        let owner_3 = (owner + 1f64);
                                                                        {
                                                                            (changed, owner) = (changed_4, owner_3);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let owner_4 = (owner + 1f64);
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
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop5Done(f64, std::rc::Rc<std::cell::RefCell<Vec<String>>>, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustNominalOwnershipOwnersLoop5Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop10Done(std::rc::Rc<std::cell::RefCell<Vec<String>>>, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop10(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64, mut child_roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut child_clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut edge: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), at))) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                {
                    let child_roles_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value150 = child_roles.borrow().clone(); ml_reference_value150 }, vec![self::read_required_text_fact(roles.clone(), child)])));
                    {
                        let child_clones_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value151 = child_clones.borrow().clone(); ml_reference_value151 }, vec![self::read_required_boolean_fact(clone_fields.clone(), child)])));
                        {
                            let edge_2 = (edge + 1f64);
                            {
                                (child_roles, child_clones, edge) = (child_roles_2.clone(), child_clones_2.clone(), edge_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Done(child_roles.clone(), child_clones.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop5(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("actual nominal ownership graph reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let child_roles = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                        {
                            let child_clones = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let start = if (at == (0f64)) {
                                    0f64
                                } else {
                                    self::read_required_number_fact(ends.clone(), (at - 1f64))
                                };
                                {
                                    let edge = start;
                                    {
                                        let source_ml_source1_s63 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop10(ends.clone(), edges.clone(), roles.clone(), clone_fields.clone(), at, child_roles.clone(), child_clones.clone(), edge);
                                        match source_ml_source1_s63.clone() {
                                            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Done(child_roles_2, child_clones_2) => {
                                                {
                                                    let target = self::read_required_number_fact(targets.clone(), at);
                                                    {
                                                        let kind = self::read_required_text_fact(kinds.clone(), at);
                                                        {
                                                            let roles_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value152 = roles.borrow().clone(); ml_reference_value152 }, vec![if (target >= (0f64)) {
                                                                self::read_required_text_fact(known.clone(), target)
                                                            } else {
                                                                self::read_rust_nominal_ownership_node_role(kind.clone(), child_roles_2.clone())
                                                            }])));
                                                            {
                                                                let clone_fields_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value153 = clone_fields.borrow().clone(); ml_reference_value153 }, vec![if (target >= (0f64)) {
                                                                    self::read_required_boolean_fact(implemented.clone(), target)
                                                                } else {
                                                                    ((((kind == "copy") || (kind == "string")) || (kind == "standard collection")) || ((self::ml_array::includes(&{ let ml_reference_value154 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("vector"), String::from("tuple"), String::from("constructor"), String::from("option"), String::from("result")]))).borrow().clone(); ml_reference_value154 }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values155 = { let ml_reference_value156 = child_clones_2.borrow().clone(); ml_reference_value156 }; let ml_callback155 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values155.iter().cloned().enumerate().all(|(ml_index155, ml_value155)| ml_callback155(ml_value155)) }) && ((self::read_rust_nominal_ownership_node_role(kind.clone(), child_roles_2.clone())) != "")))
                                                                }])));
                                                                {
                                                                    let at_2 = (at + 1f64);
                                                                    {
                                                                        (work, roles, clone_fields, at) = (work_2, roles_2.clone(), clone_fields_2.clone(), at_2);
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
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Done(work, roles.clone(), clone_fields.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop3(mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut size: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut work: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let roles = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                    {
                        let clone_fields = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s64 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop5(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), count, known.clone(), implemented.clone(), work, roles.clone(), clone_fields.clone(), at);
                                match source_ml_source1_s64.clone() {
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Done(work_2, roles_2, clone_fields_2) => {
                                        {
                                            let owner = 0f64;
                                            {
                                                let changed_3 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop6(copies.clone(), clones.clone(), ends.clone(), edges.clone(), roots.clone(), size, known.clone(), implemented.clone(), changed_2, roles_2.clone(), clone_fields_2.clone(), owner);
                                                {
                                                    (changed, work) = (changed_3, work_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Return(source_ml_temporary_result9) => {
                                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(source_ml_temporary_result9.clone())
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
    MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop2(mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut size: f64, mut owner: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result {
    loop {
        return if (owner < size) {
            {
                let name = self::read_required_text_fact(owners.clone(), owner);
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    if ((((((name == "") || ((self::ml_array::index_of(&{ let ml_reference_value157 = owners.borrow().clone(); ml_reference_value157 }, &name.clone(), 0f64, false, |a, b| a == b)) != owner)) || !(self::ml_math::is_integer(root))) || (root < (0f64))) || (root >= count)) || ((self::read_required_text_fact(kinds.clone(), root)) != "nominal")) {
                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal declaration roots")])))
                    } else {
                        if (self::read_required_boolean_fact(copies.clone(), owner) && !(self::read_required_boolean_fact(clones.clone(), owner))) {
                            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("actual nominal Copy must retain its complete Clone derivation")])))
                        } else {
                            {
                                let owner_2 = (owner + 1f64);
                                {
                                    owner = owner_2;
                                    continue;
                                }
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
    MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop11Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop11Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop11(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut end: f64, mut edge: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result {
    loop {
        return if (edge < end) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                if ((!(self::ml_math::is_integer(child)) || (child < (0f64))) || (child >= at)) {
                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("nominal field edges need actual postorder identities")])))
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
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Done
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop1(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut size: f64, mut previous: f64, mut at: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result {
    loop {
        return if (at < count) {
            {
                let end = self::read_required_number_fact(ends.clone(), at);
                {
                    let target = self::read_required_number_fact(targets.clone(), at);
                    if (((((!(self::ml_math::is_integer(end)) || (end < previous)) || (end > ({ let ml_reference_array158 = edges.clone(); let ml_reference_projection158 = ml_reference_array158.borrow().len() as f64; ml_reference_projection158 }))) || !(self::ml_math::is_integer(target))) || (target < ((-1f64)))) || (target >= size)) {
                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership graph")])))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s65 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop11(edges.clone(), at, end, edge);
                                match source_ml_source1_s65.clone() {
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Done => {
                                        {
                                            let previous_2 = end;
                                            {
                                                let at_2 = (at + 1f64);
                                                {
                                                    (previous, at) = (previous_2, at_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Return(source_ml_temporary_result11) => {
                                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(source_ml_temporary_result11.clone())
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
pub enum MlTemporaryPlanRustNominalPatternTransfersLoop12Result {
    MlTemporaryPlanRustNominalPatternTransfersLoop12Done(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
    MlTemporaryPlanRustNominalPatternTransfersLoop12Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalPatternTransfersLoop13Result {
    MlTemporaryPlanRustNominalPatternTransfersLoop13Done,
    MlTemporaryPlanRustNominalPatternTransfersLoop13Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_pattern_transfers_loop13(mut identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut size: f64, mut index: f64, mut identity: String, mut peer: f64) -> self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result {
    loop {
        return if (peer < size) {
            if ((((peer != index) && ((self::read_required_text_fact(identities.clone(), peer)) == identity)) && self::read_required_boolean_fact(owned.clone(), peer)) && !(self::read_required_boolean_fact(fields.clone(), peer))) {
                self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a whole nominal alias cannot also move its owned fields")])))
            } else {
                {
                    let peer_2 = (peer + 1f64);
                    {
                        peer = peer_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Done
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_pattern_transfers_loop12(mut identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut produced: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut deferred: bool, mut size: f64, mut plan: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut index: f64) -> self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result {
    loop {
        return if (index < size) {
            if (!(self::read_required_boolean_fact(fields.clone(), index)) || !(self::read_required_boolean_fact(owned.clone(), index))) {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            } else {
                if deferred {
                    self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("owned nominal guarded fields need their deferred partial-move interpretation")])))
                } else {
                    {
                        let identity = self::read_required_text_fact(identities.clone(), index);
                        if ((identity == "") && !(self::read_required_boolean_fact(produced.clone(), index))) {
                            self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a nominal field move needs its actual available parent or actual producer")])))
                        } else {
                            {
                                let peer = 0f64;
                                {
                                    let source_ml_source1_s66 = self::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop13(identities.clone(), fields.clone(), owned.clone(), size, index, identity.clone(), peer);
                                    match source_ml_source1_s66.clone() {
                                        self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Done => {
                                            if ((identity != "") && !(self::ml_array::includes(&{ let ml_reference_value159 = seen.borrow().clone(); ml_reference_value159 }, &identity.clone(), 0f64, false, |a, b| a == b))) {
                                                {
                                                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value160 = seen.borrow().clone(); ml_reference_value160 }, vec![identity.clone()])));
                                                    {
                                                        let plan_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value161 = plan.borrow().clone(); ml_reference_value161 }, vec![{
                                                            let source_ml_source1_string_argument67 = index;
                                                            self::ml_number::js_number(source_ml_source1_string_argument67)
                                                        }])));
                                                        {
                                                            let index_3 = (index + 1f64);
                                                            {
                                                                (plan, seen, index) = (plan_2.clone(), seen_2.clone(), index_3);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let index_4 = (index + 1f64);
                                                    {
                                                        index = index_4;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Return(source_ml_temporary_result13) => {
                                            self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Return(source_ml_temporary_result13.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Done(plan.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result {
    MlTemporaryPlanRustNominalVectorElementOwnersLoop14Done,
    MlTemporaryPlanRustNominalVectorElementOwnersLoop14Return(std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop16(mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < ({ let ml_reference_array162 = owners.clone(); let ml_reference_projection162 = ml_reference_array162.borrow().len() as f64; ml_reference_projection162 })) {
            if (((!(self::read_required_boolean_fact(known.clone(), owner)) && self::read_required_boolean_fact(copies.clone(), owner)) && self::read_required_boolean_fact(clones.clone(), owner)) && self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(roots.clone(), owner))) {
                {
                    let source_ml_temporary_plan_rust_nominal_vector_element_owners_ignored17 = { let ml_array_owner163 = known.clone(); let ml_array_argument163_0: f64 = owner; let ml_array_argument163_1: f64 = 1f64; let ml_array_argument163_2: bool = true; let ml_array_length = ml_array_owner163.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument163_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument163_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument163_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner163.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                    {
                        let changed_2 = true;
                        {
                            let owner_2 = (owner + 1f64);
                            {
                                (changed, owner) = (changed_2, owner_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let owner_3 = (owner + 1f64);
                    {
                        owner = owner_3;
                        continue;
                    }
                }
            }
        } else {
            changed
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result {
    MlTemporaryPlanRustNominalVectorElementOwnersLoop15Done(f64, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustNominalVectorElementOwnersLoop15Return(std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop18(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut actual: bool, mut end: f64, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if !(self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let actual_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (actual, edge) = (actual_2, edge_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let edge_3 = (edge + 1f64);
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        } else {
            actual
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop15(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result {
    loop {
        return if (at < ({ let ml_reference_array164 = kinds.clone(); let ml_reference_projection164 = ml_reference_array164.borrow().len() as f64; ml_reference_projection164 })) {
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
                                    if self::ml_array::includes(&{ let ml_reference_value165 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("string")]))).borrow().clone(); ml_reference_value165 }, &kind.clone(), 0f64, false, |a, b| a == b) {
                                        self::read_required_boolean_fact(leaves.clone(), at)
                                    } else {
                                        self::ml_array::includes(&{ let ml_reference_value166 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("constructor"), String::from("tuple"), String::from("option"), String::from("result"), String::from("nominal")]))).borrow().clone(); ml_reference_value166 }, &kind.clone(), 0f64, false, |a, b| a == b)
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
                                                        let safe_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value167 = safe.borrow().clone(); ml_reference_value167 }, vec![actual_4])));
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
                            let source_ml_source1_s68 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop15(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), known.clone(), work, safe.clone(), at);
                            match source_ml_source1_s68.clone() {
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
        return if (at < ({ let ml_reference_array168 = kinds.clone(); let ml_reference_projection168 = ml_reference_array168.borrow().len() as f64; ml_reference_projection168 })) {
            {
                let kind = self::read_required_text_fact(kinds.clone(), at);
                {
                    let end = self::read_required_number_fact(ends.clone(), at);
                    if ((!(self::ml_math::is_integer(end)) || (end < previous)) || (end > ({ let ml_reference_array169 = edges.clone(); let ml_reference_projection169 = ml_reference_array169.borrow().len() as f64; ml_reference_projection169 }))) {
                        self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s69 = self::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop20(edges.clone(), at, end, edge);
                                match source_ml_source1_s69.clone() {
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
                                                                    if ((((((self::read_required_text_fact(kinds.clone(), child)) == "reference") && self::ml_math::is_integer(target)) && (target >= (0f64))) && (target < ({ let ml_reference_array170 = elements.clone(); let ml_reference_projection170 = ml_reference_array170.borrow().len() as f64; ml_reference_projection170 }))) && self::read_required_boolean_fact(elements.clone(), target)) {
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
                                                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value171 = result.borrow().clone(); ml_reference_value171 }, vec![selected_4.clone()])));
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

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop22(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut copy: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if !(self::read_required_boolean_fact(copy_nodes.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let copy_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (copy, edge) = (copy_2, edge_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let edge_3 = (edge + 1f64);
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        } else {
            copy
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop21(mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<bool>>> {
    loop {
        return if (at < count) {
            {
                let kind = self::read_required_text_fact(kinds.clone(), at);
                {
                    let target = self::read_required_number_fact(targets.clone(), at);
                    {
                        let start = if (at == (0f64)) {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (at - 1f64))
                        };
                        {
                            let end = self::read_required_number_fact(ends.clone(), at);
                            {
                                let copy = if (target >= (0f64)) {
                                    self::read_required_boolean_fact(copies.clone(), target)
                                } else {
                                    (kind == "copy")
                                };
                                {
                                    let copy_4 = if (kind == "tuple") {
                                        {
                                            let copy_2 = true;
                                            {
                                                let edge = start;
                                                {
                                                    let copy_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop22(edges.clone(), copy_nodes.clone(), end, copy_2, edge);
                                                    copy_3
                                                }
                                            }
                                        }
                                    } else {
                                        copy
                                    };
                                    {
                                        let copy_nodes_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value172 = copy_nodes.borrow().clone(); ml_reference_value172 }, vec![copy_4])));
                                        {
                                            let at_2 = (at + 1f64);
                                            {
                                                (copy_nodes, at) = (copy_nodes_2.clone(), at_2);
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
            copy_nodes.clone()
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop27(mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut owner: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    loop {
        return if (owner < size) {
            {
                let admitted = (self::read_required_boolean_fact(eligible.clone(), owner) && self::read_required_boolean_fact(inhabited.clone(), owner));
                {
                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value173 = result.borrow().clone(); ml_reference_value173 }, vec![if admitted {
                        if self::read_required_boolean_fact(copies.clone(), owner) {
                            String::from("copy")
                        } else {
                            String::from("owned")
                        }
                    } else {
                        String::from("")
                    }])));
                    {
                        let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value174 = result_2.borrow().clone(); ml_reference_value174 }, vec![if self::read_required_boolean_fact(implemented.clone(), owner) {
                            String::from("Clone")
                        } else {
                            String::from("")
                        }])));
                        {
                            let result_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value175 = result_3.borrow().clone(); ml_reference_value175 }, vec![if admitted {
                                String::from("closed")
                            } else {
                                String::from("")
                            }])));
                            {
                                let result_5 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value176 = result_4.borrow().clone(); ml_reference_value176 }, vec![{
                                    let source_ml_source1_string_argument70 = self::read_required_number_fact(ranks.clone(), owner);
                                    self::ml_number::js_number(source_ml_source1_string_argument70)
                                }])));
                                {
                                    let result_6 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value177 = result_5.borrow().clone(); ml_reference_value177 }, vec![{
                                        let source_ml_source1_string_argument71 = self::read_required_number_fact(witnesses.clone(), owner);
                                        self::ml_number::js_number(source_ml_source1_string_argument71)
                                    }])));
                                    {
                                        let owner_2 = (owner + 1f64);
                                        {
                                            (result, owner) = (result_6.clone(), owner_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Done,
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}
