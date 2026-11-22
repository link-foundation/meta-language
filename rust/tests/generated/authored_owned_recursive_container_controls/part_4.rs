pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop29(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut changed: bool, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < size) {
            if (self::read_required_boolean_fact(implemented.clone(), owner) && !(self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(roots.clone(), owner)))) {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored30 = { let ml_array_owner178 = implemented.clone(); let ml_array_argument178_0: f64 = owner; let ml_array_argument178_1: f64 = 1f64; let ml_array_argument178_2: bool = false; let ml_array_length = ml_array_owner178.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument178_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument178_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument178_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner178.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
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
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Done(f64, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop31(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64, mut value: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), at))) {
            if !(self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let value_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (value, edge) = (value_2, edge_2);
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
            value
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop28(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut work: f64, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
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
                                    let value = if (target >= (0f64)) {
                                        self::read_required_boolean_fact(implemented.clone(), target)
                                    } else {
                                        if (kind == "standard collection") {
                                            true
                                        } else {
                                            if ((kind == "copy") || (kind == "string")) {
                                                self::read_required_boolean_fact(leaves.clone(), at)
                                            } else {
                                                self::ml_array::includes(&{ let ml_reference_value179 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("vector"), String::from("pending vector"), String::from("option"), String::from("result"), String::from("constructor"), String::from("nominal"), String::from("tuple")]))).borrow().clone(); ml_reference_value179 }, &kind.clone(), 0f64, false, |a, b| a == b)
                                            }
                                        }
                                    };
                                    {
                                        let edge = start;
                                        {
                                            let value_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop31(ends.clone(), edges.clone(), values.clone(), at, value, edge);
                                            {
                                                let values_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value180 = values.borrow().clone(); ml_reference_value180 }, vec![value_2])));
                                                {
                                                    let at_2 = (at + 1f64);
                                                    {
                                                        (work, values, at) = (work_2, values_2.clone(), at_2);
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Done(work, values.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop26(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut size: f64, mut work: f64, mut changed: bool, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s72 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop28(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, work, implemented.clone(), values.clone(), at);
                            match source_ml_source1_s72.clone() {
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Done(work_2, values_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop29(roots.clone(), size, changed_2, implemented.clone(), values_2.clone(), owner);
                                            {
                                                (work, changed) = (work_2, changed_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Return(source_ml_temporary_result30) => {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Return(source_ml_temporary_result30.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Done(f64, bool),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop34(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut changed: bool, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut rank: f64, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64, mut root: f64, mut start: f64, mut edge: f64) -> bool {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), root))) {
            if self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge)) {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored35 = { let ml_array_owner181 = inhabited.clone(); let ml_array_argument181_0: f64 = owner; let ml_array_argument181_1: f64 = 1f64; let ml_array_argument181_2: bool = true; let ml_array_length = ml_array_owner181.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument181_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument181_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument181_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner181.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                    {
                        let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored36 = { let ml_array_owner182 = ranks.clone(); let ml_array_argument182_0: f64 = owner; let ml_array_argument182_1: f64 = 1f64; let ml_array_argument182_2: f64 = rank; let ml_array_length = ml_array_owner182.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument182_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument182_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument182_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner182.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                        {
                            let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored37 = { let ml_array_owner183 = witnesses.clone(); let ml_array_argument183_0: f64 = owner; let ml_array_argument183_1: f64 = 1f64; let ml_array_argument183_2: f64 = (edge - start); let ml_array_length = ml_array_owner183.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument183_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument183_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument183_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner183.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                            {
                                let changed_2 = true;
                                changed_2
                            }
                        }
                    }
                }
            } else {
                {
                    let edge_2 = (edge + 1f64);
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

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop33(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut rank: f64, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < size) {
            if (self::read_required_boolean_fact(eligible.clone(), owner) && !(self::read_required_boolean_fact(inhabited.clone(), owner))) {
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    {
                        let start = if (root == (0f64)) {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (root - 1f64))
                        };
                        {
                            let edge = start;
                            {
                                let changed_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop34(ends.clone(), edges.clone(), changed, inhabited.clone(), ranks.clone(), witnesses.clone(), rank, values.clone(), owner, root, start, edge);
                                {
                                    let owner_2 = (owner + 1f64);
                                    {
                                        (changed, owner) = (changed_2, owner_2);
                                        continue;
                                    }
                                }
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
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(f64, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop38(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut value: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if !(self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let value_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (value, edge) = (value_2, edge_2);
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
            value
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop39(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut value: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge)) {
                {
                    let value_2 = true;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (value, edge) = (value_2, edge_2);
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
            value
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop32(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut work: f64, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
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
                                        let value = if (target >= (0f64)) {
                                            self::read_required_boolean_fact(inhabited.clone(), target)
                                        } else {
                                            if (kind == "standard collection") {
                                                true
                                            } else {
                                                if ((kind == "copy") || (kind == "string")) {
                                                    self::read_required_boolean_fact(leaves.clone(), at)
                                                } else {
                                                    self::ml_array::includes(&{ let ml_reference_value184 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("vector"), String::from("pending vector"), String::from("option")]))).borrow().clone(); ml_reference_value184 }, &kind.clone(), 0f64, false, |a, b| a == b)
                                                }
                                            }
                                        };
                                        {
                                            let value_4 = if (((kind == "constructor") || (kind == "tuple")) || (kind == "box")) {
                                                {
                                                    let value_2 = true;
                                                    {
                                                        let edge = start;
                                                        {
                                                            let value_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop38(edges.clone(), values.clone(), end, value_2, edge);
                                                            value_3
                                                        }
                                                    }
                                                }
                                            } else {
                                                value
                                            };
                                            {
                                                let value_7 = if ((kind == "nominal") || (kind == "result")) {
                                                    {
                                                        let value_5 = false;
                                                        {
                                                            let edge_2 = start;
                                                            {
                                                                let value_6 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop39(edges.clone(), values.clone(), end, value_5, edge_2);
                                                                value_6
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    value_4
                                                };
                                                {
                                                    let values_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value185 = values.borrow().clone(); ml_reference_value185 }, vec![value_7])));
                                                    {
                                                        let at_2 = (at + 1f64);
                                                        {
                                                            (work, values, at) = (work_2, values_2.clone(), at_2);
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(work, values.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop25(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut changed: bool, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut rank: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let rank_2 = (rank + 1f64);
                    {
                        let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s73 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop32(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, work, inhabited.clone(), values.clone(), at);
                                match source_ml_source1_s73.clone() {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(work_2, values_2) => {
                                        {
                                            let owner = 0f64;
                                            {
                                                let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop33(ends.clone(), edges.clone(), roots.clone(), size, eligible.clone(), changed_2, inhabited.clone(), ranks.clone(), witnesses.clone(), rank_2, values_2.clone(), owner);
                                                {
                                                    (work, changed, rank) = (work_2, changed_3, rank_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(source_ml_temporary_result37) => {
                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Return(source_ml_temporary_result37.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Done(work, changed)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Done(f64, bool),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop41(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < size) {
            if (self::read_required_boolean_fact(eligible.clone(), owner) && !(self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(roots.clone(), owner)))) {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored42 = { let ml_array_owner186 = eligible.clone(); let ml_array_argument186_0: f64 = owner; let ml_array_argument186_1: f64 = 1f64; let ml_array_argument186_2: bool = false; let ml_array_length = ml_array_owner186.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument186_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument186_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument186_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner186.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
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
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Done(f64, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop43(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut value: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if !(self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let value_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (value, edge) = (value_2, edge_2);
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
            value
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop40(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
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
                                        let value = if (target >= (0f64)) {
                                            self::read_required_boolean_fact(eligible.clone(), target)
                                        } else {
                                            if (kind == "standard collection") {
                                                true
                                            } else {
                                                if ((kind == "copy") || (kind == "string")) {
                                                    self::read_required_boolean_fact(leaves.clone(), at)
                                                } else {
                                                    (self::ml_array::includes(&{ let ml_reference_value187 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("vector"), String::from("pending vector"), String::from("option"), String::from("result"), String::from("constructor"), String::from("nominal")]))).borrow().clone(); ml_reference_value187 }, &kind.clone(), 0f64, false, |a, b| a == b) || ((kind == "tuple") && self::read_required_boolean_fact(copy_nodes.clone(), at)))
                                                }
                                            }
                                        };
                                        {
                                            let value_3 = if (((((((kind == "box") || (kind == "vector")) || (kind == "pending vector")) || (kind == "option")) && (((end - start)) != (1f64))) || ((kind == "result") && (((end - start)) != (2f64)))) || ((kind == "nominal") && (start == end))) {
                                                {
                                                    let value_2 = false;
                                                    value_2
                                                }
                                            } else {
                                                value
                                            };
                                            {
                                                let edge = start;
                                                {
                                                    let value_4 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop43(edges.clone(), safe.clone(), end, value_3, edge);
                                                    {
                                                        let safe_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value188 = safe.borrow().clone(); ml_reference_value188 }, vec![value_4])));
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Done(work, safe.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop24(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut changed: bool) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let safe = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s74 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop40(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, eligible.clone(), copy_nodes.clone(), work, safe.clone(), at);
                            match source_ml_source1_s74.clone() {
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Done(work_2, safe_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop41(roots.clone(), size, eligible.clone(), changed_2, safe_2.clone(), owner);
                                            {
                                                (work, changed) = (work_2, changed_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Return(source_ml_temporary_result42) => {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Return(source_ml_temporary_result42.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Done(work, changed)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop44(mut count: f64, mut seen: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<bool>>> {
    loop {
        return if (at < count) {
            {
                let seen_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value189 = seen.borrow().clone(); ml_reference_value189 }, vec![false])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (seen, at) = (seen_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            seen.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop47(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut edge: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), at))) {
            {
                let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value190 = pending.borrow().clone(); ml_reference_value190 }, vec![self::read_required_number_fact(edges.clone(), edge)])));
                {
                    let edge_2 = (edge + 1f64);
                    {
                        (pending, edge) = (pending_2.clone(), edge_2);
                        continue;
                    }
                }
            }
        } else {
            pending.clone()
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop45(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut work: f64, mut owner: f64, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result {
    loop {
        return if { let ml_value = { let ml_reference_array192 = pending.clone(); let ml_reference_projection192 = ml_reference_array192.borrow().len() as f64; ml_reference_projection192 }; ml_value != 0f64 && !ml_value.is_nan() } {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let at = self::read_required_number_fact(pending.clone(), ({ let ml_reference_array193 = pending.clone(); let ml_reference_projection193 = ml_reference_array193.borrow().len() as f64; ml_reference_projection193 } - 1f64));
                        {
                            let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value194 = pending.borrow().clone(); ml_reference_value194 }, 0f64, (-1f64))));
                            if self::read_required_boolean_fact(seen.clone(), at) {
                                {
                                    (work, pending) = (work_2, pending_2.clone());
                                    continue;
                                }
                            } else {
                                {
                                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored46 = { let ml_array_owner195 = seen.clone(); let ml_array_argument195_0: f64 = at; let ml_array_argument195_1: f64 = 1f64; let ml_array_argument195_2: bool = true; let ml_array_length = ml_array_owner195.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument195_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument195_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument195_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner195.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                    {
                                        let kind = self::read_required_text_fact(kinds.clone(), at);
                                        {
                                            let target = self::read_required_number_fact(targets.clone(), at);
                                            if (target >= (0f64)) {
                                                if (target == owner) {
                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a recursive source datatype size cycle needs an actual Box or Vec indirection")])))
                                                } else {
                                                    {
                                                        let pending_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value196 = pending_2.borrow().clone(); ml_reference_value196 }, vec![self::read_required_number_fact(roots.clone(), target)])));
                                                        {
                                                            (work, pending) = (work_2, pending_3.clone());
                                                            continue;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (((kind == "box") || (kind == "vector")) || (kind == "pending vector")) {
                                                    {
                                                        (work, pending) = (work_2, pending_2.clone());
                                                        continue;
                                                    }
                                                } else {
                                                    {
                                                        let start = if (at == (0f64)) {
                                                            0f64
                                                        } else {
                                                            self::read_required_number_fact(ends.clone(), (at - 1f64))
                                                        };
                                                        {
                                                            let edge = start;
                                                            {
                                                                let pending_4 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop47(ends.clone(), edges.clone(), pending_2.clone(), at, edge);
                                                                {
                                                                    (work, pending) = (work_2, pending_4.clone());
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Done(work)
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop23(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut size: f64, mut work: f64, mut owner: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result {
    loop {
        return if (owner < size) {
            {
                let work_2 = (work + count);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_required_number_fact(roots.clone(), owner)]));
                        {
                            let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let at = 0f64;
                                {
                                    let seen_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop44(count, seen.clone(), at);
                                    {
                                        let source_ml_source1_s75 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop45(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), work_2, owner, pending.clone(), seen_2.clone());
                                        match source_ml_source1_s75.clone() {
                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Done(work_3) => {
                                                {
                                                    let owner_2 = (owner + 1f64);
                                                    {
                                                        (work, owner) = (work_3, owner_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Return(source_ml_temporary_result45) => {
                                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Return(source_ml_temporary_result45.clone())
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Done(work)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result {
    MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Done,
    MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Return(bool),
}

pub fn source_ml_temporary_accept_rust_owned_box_layer_clone_loop48(mut wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut copied: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result {
    loop {
        return if (at < ({ let ml_reference_array197 = owners.clone(); let ml_reference_projection197 = ml_reference_array197.borrow().len() as f64; ml_reference_projection197 })) {
            if ((self::read_required_text_fact(wrappers.clone(), at)) != "box") {
                self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Return(false)
            } else {
                if self::read_required_boolean_fact(copied.clone(), at) {
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Return(true)
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Done
        };
    }
}
