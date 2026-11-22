pub fn plan_rust_nominal_ownership_owners(owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let count = { let ml_reference_array46 = kinds.clone(); let ml_reference_projection46 = ml_reference_array46.borrow().len() as f64; ml_reference_projection46 };
        {
            let size = { let ml_reference_array47 = owners.clone(); let ml_reference_projection47 = ml_reference_array47.borrow().len() as f64; ml_reference_projection47 };
            if (((((((size > (4096f64)) || (({ let ml_reference_array48 = copies.clone(); let ml_reference_projection48 = ml_reference_array48.borrow().len() as f64; ml_reference_projection48 }) != size)) || (({ let ml_reference_array49 = clones.clone(); let ml_reference_projection49 = ml_reference_array49.borrow().len() as f64; ml_reference_projection49 }) != size)) || (({ let ml_reference_array50 = roots.clone(); let ml_reference_projection50 = ml_reference_array50.borrow().len() as f64; ml_reference_projection50 }) != size)) || (({ let ml_reference_array51 = ends.clone(); let ml_reference_projection51 = ml_reference_array51.borrow().len() as f64; ml_reference_projection51 }) != count)) || (({ let ml_reference_array52 = targets.clone(); let ml_reference_projection52 = ml_reference_array52.borrow().len() as f64; ml_reference_projection52 }) != count)) || ((self::read_rust_nominal_ownership_graph_error(count, { let ml_reference_array53 = edges.clone(); let ml_reference_projection53 = ml_reference_array53.borrow().len() as f64; ml_reference_projection53 }, 0f64)) != "")) {
                std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership graph")]))
            } else {
                {
                    let previous = 0f64;
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s51 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop1(ends.clone(), edges.clone(), targets.clone(), count, size, previous, at);
                            match source_ml_source1_s51.clone() {
                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Done(previous_2) => {
                                    if (previous_2 != ({ let ml_reference_array54 = edges.clone(); let ml_reference_projection54 = ml_reference_array54.borrow().len() as f64; ml_reference_projection54 })) {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership edges")]))
                                    } else {
                                        {
                                            let owner = 0f64;
                                            {
                                                let source_ml_source1_s52 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop2(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), roots.clone(), count, size, owner);
                                                match source_ml_source1_s52.clone() {
                                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Done => {
                                                        {
                                                            let known = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values55 = { let ml_reference_value56 = owners.borrow().clone(); ml_reference_value56 }; let ml_callback55 = { let copies = copies.clone(); std::rc::Rc::new(move |x_: String, index: f64| -> String { if self::read_required_boolean_fact(copies.clone(), index) {
                                                                String::from("copy")
                                                            } else {
                                                                String::from("")
                                                            } }) }; ml_values55.iter().cloned().enumerate().map(|(ml_index55, ml_value55)| ml_callback55(ml_value55, (ml_index55 as f64))).collect::<Vec<_>>() }));
                                                            {
                                                                let implemented = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values57 = { let ml_reference_value58 = owners.borrow().clone(); ml_reference_value58 }; let ml_callback57 = { let copies = copies.clone(); std::rc::Rc::new(move |x_: String, index: f64| -> bool { self::read_required_boolean_fact(copies.clone(), index) }) }; ml_values57.iter().cloned().enumerate().map(|(ml_index57, ml_value57)| ml_callback57(ml_value57, (ml_index57 as f64))).collect::<Vec<_>>() }));
                                                                {
                                                                    let changed = true;
                                                                    {
                                                                        let work = 0f64;
                                                                        {
                                                                            let source_ml_source1_s53 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop3(copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), count, size, known.clone(), implemented.clone(), changed, work);
                                                                            match source_ml_source1_s53.clone() {
                                                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done => {
                                                                                    {
                                                                                        let result = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles")]));
                                                                                        {
                                                                                            let owner_2 = 0f64;
                                                                                            {
                                                                                                let result_2 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop4(size, known.clone(), implemented.clone(), result.clone(), owner_2);
                                                                                                result_2.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(source_ml_temporary_result4) => {
                                                                                    source_ml_temporary_result4.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(source_ml_temporary_result10) => {
                                                        source_ml_temporary_result10.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(source_ml_temporary_result10_2) => {
                                    source_ml_temporary_result10_2.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_nominal_clone_error(ordinary: bool, role: String, actual_clone: bool, wrappers: f64, borrowed: bool, tracking: bool, available: bool, produced: bool) -> String {
    if (!ordinary || !actual_clone) {
        String::from("nominal Clone needs its actual complete derived implementation")
    } else {
        if ((wrappers != (0f64)) || borrowed) {
            String::from("nominal Clone needs an actual direct source value")
        } else {
            if ((role == "copy") || (((role == "owned") && tracking) && (available || produced))) {
                String::from("")
            } else {
                String::from("nominal Clone needs an available value or actual producer independently of its type")
            }
        }
    }
}

pub fn read_rust_nominal_record_update_error(role: String) -> String {
    if (role == "owned") {
        String::from("owned nominal record updates need their actual partial-field move interpretation")
    } else {
        String::from("")
    }
}

pub fn read_rust_nominal_clone_implementation_error(implemented: bool) -> String {
    if implemented {
        String::from("")
    } else {
        String::from("Clone needs each actual nominal Clone implementation independently of ownership")
    }
}

pub fn plan_rust_nominal_pattern_transfers(identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, produced: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, deferred: bool) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let size = { let ml_reference_array59 = identities.clone(); let ml_reference_projection59 = ml_reference_array59.borrow().len() as f64; ml_reference_projection59 };
        if ((((size > (65536f64)) || (({ let ml_reference_array60 = fields.clone(); let ml_reference_projection60 = ml_reference_array60.borrow().len() as f64; ml_reference_projection60 }) != size)) || (({ let ml_reference_array61 = owned.clone(); let ml_reference_projection61 = ml_reference_array61.borrow().len() as f64; ml_reference_projection61 }) != size)) || (({ let ml_reference_array62 = produced.clone(); let ml_reference_projection62 = ml_reference_array62.borrow().len() as f64; ml_reference_projection62 }) != size)) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete nominal pattern transfer facts")]))
        } else {
            {
                let plan = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("transfers")]));
                {
                    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_s54 = self::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop12(identities.clone(), fields.clone(), owned.clone(), produced.clone(), deferred, size, plan.clone(), seen.clone(), index);
                            match source_ml_source1_s54.clone() {
                                self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Done(plan_2) => {
                                    plan_2.clone()
                                }
                                self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Return(source_ml_temporary_result12) => {
                                    source_ml_temporary_result12.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_nominal_pattern_alternative_error(same: bool) -> String {
    if same {
        String::from("")
    } else {
        String::from("every alternative needs the same actual nominal field moves, owners and parent lineage")
    }
}

pub fn accept_rust_nominal_vector_element_leaf(kind: String, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> bool {
    ((({ let ml_reference_array63 = wrappers.clone(); let ml_reference_projection63 = ml_reference_array63.borrow().len() as f64; ml_reference_projection63 }) == (0f64)) && self::accept_rust_source_array_element(kind.clone(), false))
}

pub fn plan_rust_nominal_vector_element_owners(owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> std::rc::Rc<std::cell::RefCell<Vec<bool>>> {
    {
        let checked = self::plan_rust_nominal_ownership_owners(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone());
        if (((self::read_required_text_fact(checked.clone(), 0f64)) != "profiles") || (({ let ml_reference_array64 = leaves.clone(); let ml_reference_projection64 = ml_reference_array64.borrow().len() as f64; ml_reference_projection64 }) != ({ let ml_reference_array65 = kinds.clone(); let ml_reference_projection65 = ml_reference_array65.borrow().len() as f64; ml_reference_projection65 }))) {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()))
        } else {
            {
                let known = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values66 = { let ml_reference_value67 = owners.borrow().clone(); ml_reference_value67 }; let ml_callback66 = std::rc::Rc::new(move || -> bool { false }); ml_values66.iter().cloned().enumerate().map(|(ml_index66, ml_value66)| ml_callback66()).collect::<Vec<_>>() }));
                {
                    let changed = true;
                    {
                        let work = 0f64;
                        {
                            let source_ml_source1_s55 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop14(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), known.clone(), changed, work);
                            match source_ml_source1_s55.clone() {
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Done => {
                                    known.clone()
                                }
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Return(source_ml_temporary_result14) => {
                                    source_ml_temporary_result14.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_nominal_vector_element(profile: String, clone: bool, fields: bool) -> bool {
    (((profile == "copy") && clone) && fields)
}

pub fn read_rust_nominal_vector_element_admission(legacy: bool, nominal: bool, header: bool, collecting: bool, complete: bool, kind: String, wrappers: f64) -> String {
    if (legacy || nominal) {
        String::from("admit")
    } else {
        if ((((header && collecting) && !complete) && (kind == "named")) && (wrappers == (0f64))) {
            String::from("defer")
        } else {
            String::from("refuse")
        }
    }
}

pub fn read_rust_nominal_constructor_error(actual: bool, fields: f64, arguments_: f64) -> String {
    if (((((actual && self::ml_math::is_integer(fields)) && self::ml_math::is_integer(arguments_)) && (fields >= (0f64))) && (fields <= (65536f64))) && (fields == arguments_)) {
        String::from("")
    } else {
        String::from("nominal construction needs its actual constructor owner and exact finite field arity")
    }
}

pub fn read_rust_nominal_ownership_vector_template_node(kind: String, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, proven: bool, owner: String, path: String, arguments_: f64, model: String, element: String, scope: bool) -> String {
    if ((((((((scope && (kind == "array")) && (({ let ml_reference_array68 = wrappers.clone(); let ml_reference_projection68 = ml_reference_array68.borrow().len() as f64; ml_reference_projection68 }) == (0f64))) && proven) && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value69 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value69 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) && (model == "owned vector")) && (element == "named")) {
        String::from("pending vector")
    } else {
        String::from("unknown")
    }
}

pub fn plan_rust_nominal_ownership_vector_nodes(kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, elements: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if (((({ let ml_reference_array70 = kinds.clone(); let ml_reference_projection70 = ml_reference_array70.borrow().len() as f64; ml_reference_projection70 }) != ({ let ml_reference_array71 = ends.clone(); let ml_reference_projection71 = ml_reference_array71.borrow().len() as f64; ml_reference_projection71 })) || (({ let ml_reference_array72 = kinds.clone(); let ml_reference_projection72 = ml_reference_array72.borrow().len() as f64; ml_reference_projection72 }) != ({ let ml_reference_array73 = targets.clone(); let ml_reference_projection73 = ml_reference_array73.borrow().len() as f64; ml_reference_projection73 }))) || ((self::read_rust_nominal_ownership_graph_error({ let ml_reference_array74 = kinds.clone(); let ml_reference_projection74 = ml_reference_array74.borrow().len() as f64; ml_reference_projection74 }, { let ml_reference_array75 = edges.clone(); let ml_reference_projection75 = ml_reference_array75.borrow().len() as f64; ml_reference_projection75 }, 0f64)) != "")) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
            {
                let previous = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s56 = self::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop19(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), elements.clone(), result.clone(), previous, at);
                        match source_ml_source1_s56.clone() {
                            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Done(result_2, previous_2) => {
                                if (previous_2 == ({ let ml_reference_array76 = edges.clone(); let ml_reference_projection76 = ml_reference_array76.borrow().len() as f64; ml_reference_projection76 })) {
                                    result_2.clone()
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
                                }
                            }
                            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(source_ml_temporary_result19) => {
                                source_ml_temporary_result19.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_nominal_verified_vector_clone(kind: String, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, proven: bool, owner: String, path: String, arguments_: f64, model: String, element: bool) -> bool {
    ((((((((kind == "array") && (({ let ml_reference_array77 = wrappers.clone(); let ml_reference_projection77 = ml_reference_array77.borrow().len() as f64; ml_reference_projection77 }) == (0f64))) && proven) && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value78 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value78 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) && (model == "owned vector")) && element)
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop4(mut size: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut owner: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    loop {
        return if (owner < size) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value79 = result.borrow().clone(); ml_reference_value79 }, vec![self::read_required_text_fact(known.clone(), owner)])));
                {
                    let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value80 = result_2.borrow().clone(); ml_reference_value80 }, vec![if self::read_required_boolean_fact(implemented.clone(), owner) {
                        String::from("Clone")
                    } else {
                        String::from("")
                    }])));
                    {
                        let owner_2 = (owner + 1f64);
                        {
                            (result, owner) = (result_3.clone(), owner_2);
                            continue;
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
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(bool, bool),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop7(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut ready: bool, mut clone: bool, mut edge: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result {
    loop {
        return if (edge < end) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                {
                    let ready_3 = if ((self::read_required_text_fact(roles.clone(), child)) == "") {
                        {
                            let ready_2 = false;
                            ready_2
                        }
                    } else {
                        ready
                    };
                    if !(self::read_required_boolean_fact(clone_fields.clone(), child)) {
                        {
                            let clone_2 = false;
                            {
                                let edge_2 = (edge + 1f64);
                                {
                                    (ready, clone, edge) = (ready_3, clone_2, edge_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let edge_3 = (edge + 1f64);
                            {
                                (ready, edge) = (ready_3, edge_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(ready, clone)
        };
    }
}

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
                                            let source_ml_source1_s57 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop7(edges.clone(), roles.clone(), clone_fields.clone(), end, ready, clone, edge);
                                            match source_ml_source1_s57.clone() {
                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(ready_2, clone_2) => {
                                                    {
                                                        let changed_3 = if (ready_2 && ((self::read_required_text_fact(known.clone(), owner)) == "")) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored8 = { let ml_array_owner81 = known.clone(); let ml_array_argument81_0: f64 = owner; let ml_array_argument81_1: f64 = 1f64; let ml_array_argument81_2: String = String::from("owned"); let ml_array_length = ml_array_owner81.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument81_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument81_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument81_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner81.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
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
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored9 = { let ml_array_owner82 = implemented.clone(); let ml_array_argument82_0: f64 = owner; let ml_array_argument82_1: f64 = 1f64; let ml_array_argument82_2: bool = true; let ml_array_length = ml_array_owner82.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument82_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument82_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument82_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner82.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
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
                    let child_roles_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value83 = child_roles.borrow().clone(); ml_reference_value83 }, vec![self::read_required_text_fact(roles.clone(), child)])));
                    {
                        let child_clones_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value84 = child_clones.borrow().clone(); ml_reference_value84 }, vec![self::read_required_boolean_fact(clone_fields.clone(), child)])));
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
                                        let source_ml_source1_s58 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop10(ends.clone(), edges.clone(), roles.clone(), clone_fields.clone(), at, child_roles.clone(), child_clones.clone(), edge);
                                        match source_ml_source1_s58.clone() {
                                            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Done(child_roles_2, child_clones_2) => {
                                                {
                                                    let target = self::read_required_number_fact(targets.clone(), at);
                                                    {
                                                        let kind = self::read_required_text_fact(kinds.clone(), at);
                                                        {
                                                            let roles_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value85 = roles.borrow().clone(); ml_reference_value85 }, vec![if (target >= (0f64)) {
                                                                self::read_required_text_fact(known.clone(), target)
                                                            } else {
                                                                self::read_rust_nominal_ownership_node_role(kind.clone(), child_roles_2.clone())
                                                            }])));
                                                            {
                                                                let clone_fields_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value86 = clone_fields.borrow().clone(); ml_reference_value86 }, vec![if (target >= (0f64)) {
                                                                    self::read_required_boolean_fact(implemented.clone(), target)
                                                                } else {
                                                                    ((((kind == "copy") || (kind == "string")) || (kind == "standard collection")) || ((self::ml_array::includes(&{ let ml_reference_value87 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("vector"), String::from("tuple"), String::from("constructor"), String::from("option"), String::from("result")]))).borrow().clone(); ml_reference_value87 }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values88 = { let ml_reference_value89 = child_clones_2.borrow().clone(); ml_reference_value89 }; let ml_callback88 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values88.iter().cloned().enumerate().all(|(ml_index88, ml_value88)| ml_callback88(ml_value88)) }) && ((self::read_rust_nominal_ownership_node_role(kind.clone(), child_roles_2.clone())) != "")))
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
                                let source_ml_source1_s59 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop5(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), count, known.clone(), implemented.clone(), work, roles.clone(), clone_fields.clone(), at);
                                match source_ml_source1_s59.clone() {
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
                    if ((((((name == "") || ((self::ml_array::index_of(&{ let ml_reference_value90 = owners.borrow().clone(); ml_reference_value90 }, &name.clone(), 0f64, false, |a, b| a == b)) != owner)) || !(self::ml_math::is_integer(root))) || (root < (0f64))) || (root >= count)) || ((self::read_required_text_fact(kinds.clone(), root)) != "nominal")) {
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
                    if (((((!(self::ml_math::is_integer(end)) || (end < previous)) || (end > ({ let ml_reference_array91 = edges.clone(); let ml_reference_projection91 = ml_reference_array91.borrow().len() as f64; ml_reference_projection91 }))) || !(self::ml_math::is_integer(target))) || (target < ((-1f64)))) || (target >= size)) {
                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership graph")])))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s60 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop11(edges.clone(), at, end, edge);
                                match source_ml_source1_s60.clone() {
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
                                    let source_ml_source1_s61 = self::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop13(identities.clone(), fields.clone(), owned.clone(), size, index, identity.clone(), peer);
                                    match source_ml_source1_s61.clone() {
                                        self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Done => {
                                            if ((identity != "") && !(self::ml_array::includes(&{ let ml_reference_value92 = seen.borrow().clone(); ml_reference_value92 }, &identity.clone(), 0f64, false, |a, b| a == b))) {
                                                {
                                                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value93 = seen.borrow().clone(); ml_reference_value93 }, vec![identity.clone()])));
                                                    {
                                                        let plan_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value94 = plan.borrow().clone(); ml_reference_value94 }, vec![{
                                                            let source_ml_source1_string_argument62 = index;
                                                            self::ml_number::js_number(source_ml_source1_string_argument62)
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
        return if (owner < ({ let ml_reference_array95 = owners.clone(); let ml_reference_projection95 = ml_reference_array95.borrow().len() as f64; ml_reference_projection95 })) {
            if (((!(self::read_required_boolean_fact(known.clone(), owner)) && self::read_required_boolean_fact(copies.clone(), owner)) && self::read_required_boolean_fact(clones.clone(), owner)) && self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(roots.clone(), owner))) {
                {
                    let source_ml_temporary_plan_rust_nominal_vector_element_owners_ignored17 = { let ml_array_owner96 = known.clone(); let ml_array_argument96_0: f64 = owner; let ml_array_argument96_1: f64 = 1f64; let ml_array_argument96_2: bool = true; let ml_array_length = ml_array_owner96.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument96_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument96_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument96_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner96.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
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
