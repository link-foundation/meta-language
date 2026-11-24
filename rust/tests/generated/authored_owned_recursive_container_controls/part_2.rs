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

pub fn plan_rust_owned_recursive_nominal_owners(owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let checked = self::plan_rust_nominal_ownership_owners(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone());
        if ((((self::read_required_text_fact(checked.clone(), 0f64)) != "profiles") || (({ let ml_reference_array79 = leaves.clone(); let ml_reference_projection79 = ml_reference_array79.borrow().len() as f64; ml_reference_projection79 }) != ({ let ml_reference_array80 = kinds.clone(); let ml_reference_projection80 = ml_reference_array80.borrow().len() as f64; ml_reference_projection80 }))) || ((self::read_rust_nominal_ownership_graph_error({ let ml_reference_array81 = kinds.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }, { let ml_reference_array82 = edges.clone(); let ml_reference_projection82 = ml_reference_array82.borrow().len() as f64; ml_reference_projection82 }, 0f64)) != "")) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual recursive datatype graph")]))
        } else {
            {
                let count = { let ml_reference_array83 = kinds.clone(); let ml_reference_projection83 = ml_reference_array83.borrow().len() as f64; ml_reference_projection83 };
                {
                    let size = { let ml_reference_array84 = owners.clone(); let ml_reference_projection84 = ml_reference_array84.borrow().len() as f64; ml_reference_projection84 };
                    {
                        let eligible = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values85 = { let ml_reference_value86 = owners.borrow().clone(); ml_reference_value86 }; let ml_callback85 = std::rc::Rc::new(move || -> bool { true }); ml_values85.iter().cloned().enumerate().map(|(ml_index85, ml_value85)| ml_callback85()).collect::<Vec<_>>() }));
                        {
                            let copy_nodes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let work = 0f64;
                                {
                                    let at = 0f64;
                                    {
                                        let copy_nodes_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop21(copies.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), count, copy_nodes.clone(), at);
                                        {
                                            let owner = 0f64;
                                            {
                                                let source_ml_source1_s57 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop23(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), count, size, work, owner);
                                                match source_ml_source1_s57.clone() {
                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Done(work_2) => {
                                                        {
                                                            let changed = true;
                                                            {
                                                                let source_ml_source1_s58 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop24(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, eligible.clone(), copy_nodes_2.clone(), work_2, changed);
                                                                match source_ml_source1_s58.clone() {
                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Done(work_3, changed_2) => {
                                                                        {
                                                                            let inhabited = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values87 = { let ml_reference_value88 = owners.borrow().clone(); ml_reference_value88 }; let ml_callback87 = std::rc::Rc::new(move || -> bool { false }); ml_values87.iter().cloned().enumerate().map(|(ml_index87, ml_value87)| ml_callback87()).collect::<Vec<_>>() }));
                                                                            {
                                                                                let ranks = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values89 = { let ml_reference_value90 = owners.borrow().clone(); ml_reference_value90 }; let ml_callback89 = std::rc::Rc::new(move || -> f64 { (-1f64) }); ml_values89.iter().cloned().enumerate().map(|(ml_index89, ml_value89)| ml_callback89()).collect::<Vec<_>>() }));
                                                                                {
                                                                                    let witnesses = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values91 = { let ml_reference_value92 = owners.borrow().clone(); ml_reference_value92 }; let ml_callback91 = std::rc::Rc::new(move || -> f64 { (-1f64) }); ml_values91.iter().cloned().enumerate().map(|(ml_index91, ml_value91)| ml_callback91()).collect::<Vec<_>>() }));
                                                                                    {
                                                                                        let rank = 0f64;
                                                                                        {
                                                                                            let changed_3 = true;
                                                                                            {
                                                                                                let source_ml_source1_s59 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop25(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, eligible.clone(), work_3, changed_3, inhabited.clone(), ranks.clone(), witnesses.clone(), rank);
                                                                                                match source_ml_source1_s59.clone() {
                                                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Done(work_4, changed_4) => {
                                                                                                        {
                                                                                                            let implemented = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values93 = { let ml_reference_value94 = owners.borrow().clone(); ml_reference_value94 }; let ml_callback93 = { let eligible = eligible.clone(); let inhabited = inhabited.clone(); let copies = copies.clone(); let clones = clones.clone(); std::rc::Rc::new(move |x_: String, owner_2: f64| -> bool { ((self::read_required_boolean_fact(eligible.clone(), owner_2) && self::read_required_boolean_fact(inhabited.clone(), owner_2)) && (self::read_required_boolean_fact(copies.clone(), owner_2) || self::read_required_boolean_fact(clones.clone(), owner_2))) }) }; ml_values93.iter().cloned().enumerate().map(|(ml_index93, ml_value93)| ml_callback93(ml_value93, (ml_index93 as f64))).collect::<Vec<_>>() }));
                                                                                                            {
                                                                                                                let changed_5 = true;
                                                                                                                {
                                                                                                                    let source_ml_source1_s60 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop26(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, work_4, changed_5, implemented.clone());
                                                                                                                    match source_ml_source1_s60.clone() {
                                                                                                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Done => {
                                                                                                                            {
                                                                                                                                let result = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles")]));
                                                                                                                                {
                                                                                                                                    let owner_2 = 0f64;
                                                                                                                                    {
                                                                                                                                        let result_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop27(copies.clone(), size, eligible.clone(), inhabited.clone(), ranks.clone(), witnesses.clone(), implemented.clone(), result.clone(), owner_2);
                                                                                                                                        result_2.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Return(source_ml_temporary_result27) => {
                                                                                                                            source_ml_temporary_result27.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Return(source_ml_temporary_result31) => {
                                                                                                        source_ml_temporary_result31.clone()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Return(source_ml_temporary_result39) => {
                                                                        source_ml_temporary_result39.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Return(source_ml_temporary_result43) => {
                                                        source_ml_temporary_result43.clone()
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

pub fn accept_rust_owned_box_type_node(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, owner: String, path: String, arguments_: f64) -> bool {
    (((((({ let ml_reference_array95 = wrappers.clone(); let ml_reference_projection95 = ml_reference_array95.borrow().len() as f64; ml_reference_projection95 }) > (0f64)) && ((self::read_required_text_fact(wrappers.clone(), 0f64)) == "box")) && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value96 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::boxed::Box"), String::from("alloc::boxed::Box")]))).borrow().clone(); ml_reference_value96 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64)))
}

pub fn accept_rust_owned_nominal_header_type(data: bool, complete: bool, role: String) -> bool {
    ((data && complete) && ((role == "copy") || (role == "owned")))
}

pub fn read_rust_owned_container_type_node(kind: String, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, proven: bool, owner: String, path: String, arguments_: f64, model: String, leaf: String, copy: bool, nominal: String, scope: bool) -> String {
    if (!scope || !proven) {
        String::from("unknown")
    } else {
        if self::accept_rust_owned_box_type_node(wrappers.clone(), owner.clone(), path.clone(), arguments_) {
            String::from("box")
        } else {
            if { let ml_value = { let ml_reference_array98 = wrappers.clone(); let ml_reference_projection98 = ml_reference_array98.borrow().len() as f64; ml_reference_projection98 }; ml_value != 0f64 && !ml_value.is_nan() } {
                if { let ml_values99 = { let ml_reference_value100 = wrappers.borrow().clone(); ml_reference_value100 }; let ml_callback99 = std::rc::Rc::new(move |value: String| -> bool { (value == "reference") }); ml_values99.iter().cloned().enumerate().all(|(ml_index99, ml_value99)| ml_callback99(ml_value99)) } {
                    String::from("reference")
                } else {
                    String::from("unknown")
                }
            } else {
                if ((kind == "named") && ((nominal == "copy") || (nominal == "owned"))) {
                    String::from("named")
                } else {
                    if (((((kind == "array") && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value101 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value101 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) && (model == "owned vector")) {
                        String::from("vector")
                    } else {
                        if (((leaf == "copy") && copy) && self::ml_array::includes(&{ let ml_reference_value102 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("bool"), String::from("fixed"), String::from("float"), String::from("unit")]))).borrow().clone(); ml_reference_value102 }, &kind.clone(), 0f64, false, |a, b| a == b)) {
                            String::from("copy")
                        } else {
                            if (leaf == "string") {
                                String::from("string")
                            } else {
                                if (((kind == "tuple") && (leaf == "tuple")) && copy) {
                                    String::from("tuple")
                                } else {
                                    String::from("unknown")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_container_sum_node(owner: String, arguments_: f64, proven: bool, provisional: bool, wrappers: f64) -> String {
    if ((proven && !provisional) && (wrappers == (0f64))) {
        if ((owner == "std::option::Option") && (arguments_ == (1f64))) {
            String::from("option")
        } else {
            if ((owner == "std::result::Result") && (arguments_ == (2f64))) {
                String::from("result")
            } else {
                String::from("unknown")
            }
        }
    } else {
        String::from("unknown")
    }
}

pub fn read_rust_owned_container_node_role(kind: String, children: std::rc::Rc<std::cell::RefCell<Vec<String>>>, nominal: String) -> String {
    if ((kind == "copy") || (kind == "reference")) {
        String::from("copy")
    } else {
        if (kind == "string") {
            String::from("owned")
        } else {
            if (kind == "named") {
                if ((nominal == "copy") || (nominal == "owned")) {
                    nominal.clone()
                } else {
                    String::from("")
                }
            } else {
                if { let ml_values103 = { let ml_reference_value104 = children.borrow().clone(); ml_reference_value104 }; let ml_callback103 = std::rc::Rc::new(move |value: String| -> bool { ((value != "copy") && (value != "owned")) }); ml_values103.iter().cloned().enumerate().any(|(ml_index103, ml_value103)| ml_callback103(ml_value103)) } {
                    String::from("")
                } else {
                    if ((kind == "box") || (kind == "vector")) {
                        if (({ let ml_reference_array105 = children.clone(); let ml_reference_projection105 = ml_reference_array105.borrow().len() as f64; ml_reference_projection105 }) == (1f64)) {
                            String::from("owned")
                        } else {
                            String::from("")
                        }
                    } else {
                        if (kind == "tuple") {
                            if { let ml_values106 = { let ml_reference_value107 = children.borrow().clone(); ml_reference_value107 }; let ml_callback106 = std::rc::Rc::new(move |value: String| -> bool { (value == "copy") }); ml_values106.iter().cloned().enumerate().all(|(ml_index106, ml_value106)| ml_callback106(ml_value106)) } {
                                String::from("copy")
                            } else {
                                String::from("")
                            }
                        } else {
                            if (((kind == "option") && (({ let ml_reference_array108 = children.clone(); let ml_reference_projection108 = ml_reference_array108.borrow().len() as f64; ml_reference_projection108 }) == (1f64))) || ((kind == "result") && (({ let ml_reference_array109 = children.clone(); let ml_reference_projection109 = ml_reference_array109.borrow().len() as f64; ml_reference_projection109 }) == (2f64)))) {
                                if self::ml_array::includes(&{ let ml_reference_value110 = children.borrow().clone(); ml_reference_value110 }, &String::from("owned"), 0f64, false, |a, b| a == b) {
                                    String::from("owned")
                                } else {
                                    String::from("copy")
                                }
                            } else {
                                String::from("")
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_owned_container_clone(kind: String, children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, nominal: bool) -> bool {
    (((((kind == "copy") || (kind == "reference")) || (kind == "string")) || ((kind == "named") && nominal)) || (self::ml_array::includes(&{ let ml_reference_value111 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("vector"), String::from("tuple"), String::from("option"), String::from("result")]))).borrow().clone(); ml_reference_value111 }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values112 = { let ml_reference_value113 = children.borrow().clone(); ml_reference_value113 }; let ml_callback112 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values112.iter().cloned().enumerate().all(|(ml_index112, ml_value112)| ml_callback112(ml_value112)) }))
}

pub fn read_rust_owned_box_canonical_owner(owner: String) -> String {
    if ((owner == "Box") || (owner == "std::boxed::Box")) {
        String::from("std::boxed::Box")
    } else {
        if (owner == "alloc::boxed::Box") {
            String::from("alloc::boxed::Box")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_owned_box_constructor_error(owner: String, payload: String, arguments_: f64) -> String {
    if ((self::ml_array::includes(&{ let ml_reference_value114 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::boxed::Box"), String::from("alloc::boxed::Box")]))).borrow().clone(); ml_reference_value114 }, &owner.clone(), 0f64, false, |a, b| a == b) && ((payload == "copy") || (payload == "owned"))) && (arguments_ == (1f64))) {
        String::from("")
    } else {
        String::from("Box construction needs its authentic standard owner and one actual admitted payload")
    }
}

pub fn read_rust_owned_box_move_error(tracking: bool, available: bool, produced: bool) -> String {
    if (tracking && (available || produced)) {
        String::from("")
    } else {
        String::from("moving a Box needs its actual available binding or completed allocation producer")
    }
}

pub fn accept_rust_owned_nominal_vector_element(role: String, closed: bool) -> bool {
    ((role == "owned") && closed)
}

pub fn accept_rust_owned_container_source(ordinary: bool, minor: f64) -> bool {
    (((ordinary && self::ml_math::is_integer(minor)) && (minor >= (90f64))) && (minor < (100f64)))
}

pub fn read_rust_owned_container_leaf_role(kind: String, bits: String, signed: bool, proven: bool, owner: String, path: String, arguments_: f64, model: String) -> String {
    if !proven {
        String::from("")
    } else {
        if (self::accept_rust_primitive_from_source(kind.clone(), bits.clone(), signed, owner.clone(), path.clone(), arguments_, proven, 0f64) || ((((kind == "unit") && (owner == "unit")) && (path == "")) && (arguments_ == (0f64)))) {
            String::from("copy")
        } else {
            if (((((kind == "string") && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value115 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::string::String"), String::from("alloc::string::String")]))).borrow().clone(); ml_reference_value115 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (0f64))) && (model == "owned string")) {
                String::from("owned")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_rust_owned_box_intrinsic_copy_value(kind: String, actual_copy: bool) -> bool {
    (self::ml_array::includes(&{ let ml_reference_value116 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("num"), String::from("bool"), String::from("unit"), String::from("str")]))).borrow().clone(); ml_reference_value116 }, &kind.clone(), 0f64, false, |a, b| a == b) || ((kind == "tuple") && actual_copy))
}

pub fn accept_rust_owned_box_value_layers(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> bool {
    ((((((({ let ml_reference_array117 = wrappers.clone(); let ml_reference_projection117 = ml_reference_array117.borrow().len() as f64; ml_reference_projection117 }) > (0f64)) && ((self::read_required_text_fact(wrappers.clone(), 0f64)) == "box")) && (({ let ml_reference_array118 = owners.clone(); let ml_reference_projection118 = ml_reference_array118.borrow().len() as f64; ml_reference_projection118 }) > (0f64))) && (({ let ml_reference_array119 = owners.clone(); let ml_reference_projection119 = ml_reference_array119.borrow().len() as f64; ml_reference_projection119 }) <= (256f64))) && (({ let ml_reference_array120 = owners.clone(); let ml_reference_projection120 = ml_reference_array120.borrow().len() as f64; ml_reference_projection120 }) <= ({ let ml_reference_array121 = wrappers.clone(); let ml_reference_projection121 = ml_reference_array121.borrow().len() as f64; ml_reference_projection121 }))) && { let ml_values122 = { let ml_reference_value123 = owners.borrow().clone(); ml_reference_value123 }; let ml_callback122 = std::rc::Rc::new(move |owner: String| -> bool { self::ml_array::includes(&{ let ml_reference_value124 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::boxed::Box"), String::from("alloc::boxed::Box")]))).borrow().clone(); ml_reference_value124 }, &owner.clone(), 0f64, false, |a, b| a == b) }); ml_values122.iter().cloned().enumerate().all(|(ml_index122, ml_value122)| ml_callback122(ml_value122)) })
}

pub fn read_rust_owned_container_child_model(kind: String, owner: String, path: String, arguments_: f64) -> String {
    if ((((owner == "library") && (kind == "string")) && self::ml_array::includes(&{ let ml_reference_value125 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::string::String"), String::from("alloc::string::String")]))).borrow().clone(); ml_reference_value125 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (0f64))) {
        String::from("owned string")
    } else {
        if ((((owner == "library") && (kind == "array")) && self::ml_array::includes(&{ let ml_reference_value126 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value126 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) {
            String::from("owned vector")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_owned_container_runtime_metadata_cost(role: String, units: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((!(self::ml_math::is_integer(units)) || (units < (0f64))) || (units > (8388608f64))) || !(self::ml_array::includes(&{ let ml_reference_value127 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("origin"), String::from("list"), String::from("binding"), String::from("layer"), String::from("owner"), String::from("intrinsic copy"), String::from("payload loans"), String::from("payload field loans"), String::from("value loans"), String::from("value field loans"), String::from("place loans"), String::from("loans"), String::from("temporary loans")]))).borrow().clone(); ml_reference_value127 }, &role.clone(), 0f64, false, |a, b| a == b))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        if (role == "origin") {
            std::rc::Rc::new(std::cell::RefCell::new(vec![16f64, (256f64 + units)]))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, ((role.encode_utf16().count() as f64) + units)]))
        }
    }
}

pub fn read_rust_owned_box_clone_error(actual: bool, clone: bool, readonly: bool, closed: bool, available: bool, produced: bool) -> String {
    if (!actual || !clone) {
        String::from("Box Clone needs its actual admitted payload Clone implementation")
    } else {
        if (readonly && !closed) {
            String::from("a borrowed Box Clone needs its closed payload lifetime projection")
        } else {
            if ((readonly || available) || produced) {
                String::from("")
            } else {
                String::from("Box Clone needs its actual available source value or actual producer")
            }
        }
    }
}

pub fn accept_rust_owned_box_layer_clone(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, copied: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> bool {
    if (!(self::accept_rust_owned_box_value_layers(wrappers.clone(), owners.clone())) || (({ let ml_reference_array128 = owners.clone(); let ml_reference_projection128 = ml_reference_array128.borrow().len() as f64; ml_reference_projection128 }) != ({ let ml_reference_array129 = copied.clone(); let ml_reference_projection129 = ml_reference_array129.borrow().len() as f64; ml_reference_projection129 }))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s61 = self::source_ml_temporary_accept_rust_owned_box_layer_clone_loop48(wrappers.clone(), owners.clone(), copied.clone(), at);
                match source_ml_source1_s61.clone() {
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Done => {
                        false
                    }
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Return(source_ml_temporary_result48) => {
                        source_ml_temporary_result48
                    }
                }
            }
        }
    }
}

pub fn accept_rust_owned_standard_sum_box_payload(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: f64, owner: String, path: String, arguments_: f64, boxes: f64) -> bool {
    ((((self::accept_rust_owned_box_type_node(wrappers.clone(), owner.clone(), path.clone(), arguments_) && { let ml_values130 = { let ml_reference_value131 = wrappers.borrow().clone(); ml_reference_value131 }; let ml_callback130 = std::rc::Rc::new(move |layer: String| -> bool { (layer == "box") }); ml_values130.iter().cloned().enumerate().all(|(ml_index130, ml_value130)| ml_callback130(ml_value130)) }) && (lifetimes == (0f64))) && self::ml_math::is_integer(boxes)) && (boxes == ({ let ml_reference_array132 = wrappers.clone(); let ml_reference_projection132 = ml_reference_array132.borrow().len() as f64; ml_reference_projection132 })))
}

pub fn accept_rust_owned_container_value_loan_role(role: String) -> bool {
    self::ml_array::includes(&{ let ml_reference_value133 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("value loans"), String::from("value field loans"), String::from("payload loans"), String::from("payload field loans")]))).borrow().clone(); ml_reference_value133 }, &role.clone(), 0f64, false, |a, b| a == b)
}

pub fn read_rust_owned_box_payload_error(closed: bool, intrinsic: bool, borrowed: bool, produced: bool) -> String {
    if (closed || ((intrinsic || produced) && !borrowed)) {
        String::from("")
    } else {
        String::from("stored Box payload references or callable captures need their complete lifetime/drop profile")
    }
}

pub fn read_rust_owned_container_depth_error(wrappers: f64, layers: f64) -> String {
    if (((((!(self::ml_math::is_integer(wrappers)) || !(self::ml_math::is_integer(layers))) || (wrappers < (0f64))) || (layers < (0f64))) || (wrappers > (256f64))) || (layers > (256f64))) {
        String::from("owned container source syntax and value lineage exceed their finite depth profile")
    } else {
        String::from("")
    }
}

pub fn accept_rust_owned_container_closed(kind: String, children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, nominal: bool) -> bool {
    ((((kind == "copy") || (kind == "string")) || ((kind == "named") && nominal)) || (self::ml_array::includes(&{ let ml_reference_value134 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("vector"), String::from("tuple"), String::from("option"), String::from("result")]))).borrow().clone(); ml_reference_value134 }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values135 = { let ml_reference_value136 = children.borrow().clone(); ml_reference_value136 }; let ml_callback135 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values135.iter().cloned().enumerate().all(|(ml_index135, ml_value135)| ml_callback135(ml_value135)) }))
}

pub fn read_rust_owned_box_type_error(selected: bool, header: bool, complete: bool, role: String, closed: bool) -> String {
    if (((!selected || header) || !complete) || ((role == "owned") && closed)) {
        String::from("")
    } else {
        String::from("an owned Box source type needs its complete closed payload ownership graph")
    }
}

pub fn accept_rust_owned_box_call_result(actual: bool, function_: bool, owner: bool, type_: bool, recorded: bool) -> bool {
    ((((actual && function_) && owner) && type_) && recorded)
}

pub fn plan_rust_owned_nominal_runtime_facts(selected: bool, old_role: String, old_clone: bool, role: String, clone: bool, closed: bool, rank: f64, constructor: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if ((old_role == "copy") || (old_role == "owned")) {
        std::rc::Rc::new(std::cell::RefCell::new(vec![old_role.clone(), if old_clone {
            String::from("Clone")
        } else {
            String::from("")
        }]))
    } else {
        if ((((((((!selected || (role != "owned")) || !closed) || !(self::ml_math::is_integer(rank))) || (rank < (1f64))) || (rank > (65536f64))) || !(self::ml_math::is_integer(constructor))) || (constructor < (0f64))) || (constructor > (65536f64))) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")]))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("owned"), if clone {
                String::from("Clone")
            } else {
                String::from("")
            }]))
        }
    }
}

pub fn read_rust_owned_vector_signature_phase(selected: bool, header: bool, complete: bool, kind: String, wrappers: f64, owner: String, path: String, arguments_: f64, element: String, profile: String, closed: bool) -> String {
    if ((((((!selected || (kind != "array")) || (wrappers != (0f64))) || (owner != "library")) || !(self::ml_array::includes(&{ let ml_reference_value137 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value137 }, &path.clone(), 0f64, false, |a, b| a == b))) || (arguments_ != (1f64))) || (element != "named")) {
        String::from("")
    } else {
        if (header && !complete) {
            String::from("defer")
        } else {
            if ((complete && (profile == "owned")) && closed) {
                String::from("admit")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_owned_vector_element_production_error(role: String, same: bool, transferred: bool, available: bool, produced: bool, collecting: bool) -> String {
    if (!(self::ml_array::includes(&{ let ml_reference_value138 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("owned")]))).borrow().clone(); ml_reference_value138 }, &role.clone(), 0f64, false, |a, b| a == b)) || !same) {
        String::from("owned vector construction needs each actual complete matching element type")
    } else {
        if ((((collecting || (role == "copy")) || transferred) || available) || produced) {
            String::from("")
        } else {
            String::from("an owned vector element needs its actual available binding or completed producer")
        }
    }
}

pub fn accept_rust_owned_vector_call_result(actual: bool, function_: bool, owner: bool, type_: bool, recorded: bool) -> bool {
    ((((actual && function_) && owner) && type_) && recorded)
}

pub fn accept_rust_owned_vector_named_element(role: String, closed: bool, wrappers: f64) -> bool {
    (((wrappers == (0f64)) && closed) && ((role == "copy") || (role == "owned")))
}

pub fn read_rust_owned_vector_element_count_error(count: f64) -> String {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (65536f64))) {
        String::from("owned vector construction exceeds its finite actual element profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_owned_vector_named_admission(previous: String, actual: bool) -> String {
    if ((previous == "refuse") && actual) {
        String::from("admit")
    } else {
        previous.clone()
    }
}

pub fn accept_rust_owned_vector_produced_value(node: String, role: String, closed: bool, produced: bool) -> bool {
    ((((node == "vector") && (role == "owned")) && closed) && produced)
}

pub fn accept_rust_owned_vector_function_result(phase: String, actual: bool, owner: bool, type_: bool, value: bool, borrowed: bool, collecting: bool) -> bool {
    ((((((phase == "admit") && actual) && owner) && type_) && (collecting || value)) && !borrowed)
}

pub fn read_rust_owned_vector_empty_constructor_disposition(phase: String, owner: String, method: String, arguments_: f64) -> String {
    if (((phase != "admit") || !(self::ml_array::includes(&{ let ml_reference_value139 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Vec"), String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value139 }, &owner.clone(), 0f64, false, |a, b| a == b))) || (method != "new")) {
        String::from("")
    } else {
        if (arguments_ != (0f64)) {
            String::from("arity")
        } else {
            String::from("construct")
        }
    }
}

pub fn read_rust_owned_box_clone_source_error(collecting: bool, actual: bool, clone: bool, readonly: bool, closed: bool, available: bool, produced: bool) -> String {
    {
        let error = self::read_rust_owned_box_clone_error(actual, clone, readonly, closed, available, produced);
        if (((collecting && actual) && clone) && (!readonly || closed)) {
            String::from("")
        } else {
            error.clone()
        }
    }
}

pub fn read_rust_owned_vector_clone_error(collecting: bool, actual: bool, clone: bool, closed: bool, readonly: bool, available: bool, produced: bool) -> String {
    if ((!actual || !clone) || !closed) {
        String::from("vector Clone needs its actual closed element Clone implementation")
    } else {
        if (((collecting || readonly) || available) || produced) {
            String::from("")
        } else {
            String::from("vector Clone needs its actual available source value or actual producer")
        }
    }
}

pub fn accept_rust_owned_argument_read_check(wrappers: f64, transferred: bool) -> bool {
    ((wrappers > (0f64)) && !transferred)
}

pub fn read_rust_owned_nominal_field_lifetime_error(field: bool, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if !field {
        String::from("")
    } else {
        {
            let references = { let ml_reference_array142 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values140 = { let ml_reference_value141 = wrappers.borrow().clone(); ml_reference_value141 }; let ml_callback140 = std::rc::Rc::new(move |layer: String| -> bool { (layer == "reference") }); ml_values140.iter().cloned().enumerate().filter_map(|(ml_index140, ml_value140)| if ml_callback140(ml_value140.clone()) { Some(ml_value140) } else { None }).collect::<Vec<_>>() })); let ml_reference_projection142 = ml_reference_array142.borrow().len() as f64; ml_reference_projection142 };
            if ((references != ({ let ml_reference_array143 = lifetimes.clone(); let ml_reference_projection143 = ml_reference_array143.borrow().len() as f64; ml_reference_projection143 })) || { let ml_values144 = { let ml_reference_value145 = lifetimes.borrow().clone(); ml_reference_value145 }; let ml_callback144 = std::rc::Rc::new(move |lifetime: String| -> bool { (lifetime != "static") }); ml_values144.iter().cloned().enumerate().any(|(ml_index144, ml_value144)| ml_callback144(ml_value144)) }) {
                String::from("a nongeneric datatype reference field requires its explicit static lifetime")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_owned_vector_element_role(kind: String, copy: bool, direct: bool, fields: bool, actual: String) -> String {
    if ((actual == "copy") || (actual == "owned")) {
        actual.clone()
    } else {
        if ((((kind == "") && copy) && direct) && !fields) {
            String::from("copy")
        } else {
            String::from("")
        }
    }
}

pub fn accept_rust_owned_vector_element_type_match(actual: String, expected: String, same: bool, copy: bool) -> bool {
    (same || (((actual == "") && (expected == "fixed")) && copy))
}

pub fn accept_rust_owned_vector_nominal_type_identity(left: String, right: String, references: bool) -> bool {
    ((references && (left != "")) && (left == right))
}

pub fn accept_rust_owned_box_pointer_owner_child(kind: String, path: String, arguments_count: f64) -> bool {
    (((kind == "library") && (path == "alloc::boxed::Box")) && (arguments_count == (1f64)))
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop4(mut size: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut owner: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    loop {
        return if (owner < size) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value146 = result.borrow().clone(); ml_reference_value146 }, vec![self::read_required_text_fact(known.clone(), owner)])));
                {
                    let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value147 = result_2.borrow().clone(); ml_reference_value147 }, vec![if self::read_required_boolean_fact(implemented.clone(), owner) {
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
