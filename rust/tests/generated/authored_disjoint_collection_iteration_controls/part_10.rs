pub fn source_ml_source1_temporary_bind_calls_loop44(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut callback: self::ml_string::Text, mut receiver: f64, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut callee: f64, mut position: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < (self::read_required_number_fact(parameter_counts.clone(), position))) {
            {
                let parameter = self::read_required_number_fact(parameters.clone(), (self::read_required_number_fact(parameter_starts.clone(), position) + at));
                {
                    let roots = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let roots_7 = if (callback == (self::ml_string::Text::from_units(vec![]))) {
                            {
                                let roots_2 = if (at < ({ let ml_reference_array145 = arguments_.clone(); let ml_reference_projection145 = ml_reference_array145.borrow().len() as f64; ml_reference_projection145 })) {
                                    self::read_roots(result.clone(), 1f64, self::read_required_number_fact(arguments_.clone(), at), width, span)
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                };
                                roots_2.clone()
                            }
                        } else {
                            if self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), parameter)) {
                                if ((callback == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) && (at == (0f64))) {
                                    {
                                        let roots_3 = if (({ let ml_reference_array146 = arguments_.clone(); let ml_reference_projection146 = ml_reference_array146.borrow().len() as f64; ml_reference_projection146 }) > (1f64)) {
                                            self::read_roots(result.clone(), 1f64, self::read_required_number_fact(arguments_.clone(), 1f64), width, span)
                                        } else {
                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                        };
                                        {
                                            let source_ml_source1_temporary_values42 = self::read_roots(result.clone(), 5f64, callee, width, span);
                                            {
                                                let source_ml_source1_temporary_index42 = self::ml::Big::from_i128(0);
                                                {
                                                    let roots_4 = self::source_ml_source1_temporary_bind_calls_loop45(roots_3.clone(), source_ml_source1_temporary_values42.clone(), source_ml_source1_temporary_index42.clone());
                                                    roots_4.clone()
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if (at == (if (callback == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
                                        3f64
                                    } else {
                                        2f64
                                    })) {
                                        {
                                            let roots_5 = self::read_roots(result.clone(), 1f64, receiver, width, span);
                                            roots_5.clone()
                                        }
                                    } else {
                                        {
                                            let roots_6 = std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]));
                                            roots_6.clone()
                                        }
                                    }
                                }
                            } else {
                                roots.clone()
                            }
                        };
                        {
                            let result_2 = self::add_roots(result.clone(), 1f64, parameter, roots_7.clone(), width, span);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2.clone(), at_2);
                                    continue;
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

pub fn source_ml_source1_temporary_bind_calls_loop43(mut owner: f64, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut callback: self::ml_string::Text, mut receiver: f64, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values41: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index41: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index41 < ({ let ml_reference_array147 = source_ml_source1_temporary_values41.clone(); let ml_reference_projection147 = self::ml::Big::from_u128(ml_reference_array147.borrow().len() as u128); ml_reference_projection147 })) {
            {
                let callee = {
                    let source_ml_source2_array_read_value105 = source_ml_source1_temporary_values41.clone();
                    {
                        let source_ml_source2_array_read_index106 = source_ml_source1_temporary_index41.clone();
                        { let ml_reference_array148 = source_ml_source2_array_read_value105.clone(); let ml_reference_position148 = source_ml_source2_array_read_index106.clone(); let ml_reference_projection148 = self::ml_array::at(&*ml_reference_array148.borrow(), ml_reference_position148.to_index()); ml_reference_projection148 }
                    }
                };
                {
                    let position = self::ml_array::index_of(&{ let ml_reference_value149 = functions.borrow().clone(); ml_reference_value149 }, &callee, 0f64, false, |a, b| a == b);
                    if (position < (0f64)) {
                        {
                            let source_ml_source1_temporary_index41_2 = source_ml_source1_temporary_index41.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index41 = source_ml_source1_temporary_index41_2.clone();
                                continue;
                            }
                        }
                    } else {
                        {
                            let result_2 = self::add_fact(result.clone(), ((4f64 * span) + callee));
                            {
                                let result_3 = self::add_roots(result_2.clone(), 3f64, owner, std::rc::Rc::new(std::cell::RefCell::new(vec![callee])), width, span);
                                {
                                    let result_5 = if (((self::read_required_number_fact(bodies.clone(), position)) < (0f64)) && (callee != main_)) {
                                        {
                                            let result_4 = self::add_roots(result_3.clone(), 5f64, callee, std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])), width, span);
                                            result_4.clone()
                                        }
                                    } else {
                                        result_3.clone()
                                    };
                                    {
                                        let at = 0f64;
                                        {
                                            let result_6 = self::source_ml_source1_temporary_bind_calls_loop44(arguments_.clone(), callback.clone(), receiver, kinds.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), width, span, result_5.clone(), callee, position, at);
                                            {
                                                let source_ml_source1_temporary_index41_3 = source_ml_source1_temporary_index41.add(&self::ml::Big::from_i128(1));
                                                {
                                                    (result, source_ml_source1_temporary_index41) = (result_6.clone(), source_ml_source1_temporary_index41_3.clone());
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
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut callbacks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut safe: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array150 = callbacks.clone(); let ml_reference_projection150 = ml_reference_array150.borrow().len() as f64; ml_reference_projection150 })) {
            {
                let node = self::read_required_number_fact(callbacks.clone(), at);
                {
                    let source = self::read_required_number_fact(callbacks.clone(), (at + 1f64));
                    {
                        let callback = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116])));
                        if ((callback < (0f64)) || !(self::accept_disjoint_collection_iteration_callback_head(self::text_fact(roles.clone(), callback)))) {
                            {
                                let at_2 = (at + 2f64);
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        } else {
                            {
                                let scopes = self::read_roots(facts.clone(), 1f64, callback, width, span);
                                if ((({ let ml_reference_array151 = scopes.clone(); let ml_reference_projection151 = ml_reference_array151.borrow().len() as f64; ml_reference_projection151 }) != (0f64)) && self::accepts_effects(facts.clone(), roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), scopes.clone(), source, false, main_, width, span)) {
                                    {
                                        let safe_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value107 = safe.clone();
                                            {
                                                let source_ml_source2_concat_value108 = std::rc::Rc::new(std::cell::RefCell::new(vec![node]));
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value152 = source_ml_source2_concat_value107.borrow().clone(); ml_reference_value152 }), { let ml_reference_value153 = source_ml_source2_concat_value108.borrow().clone(); ml_reference_value153 })
                                            }
                                        }));
                                        {
                                            let at_3 = (at + 2f64);
                                            {
                                                (safe, at) = (safe_2.clone(), at_3);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at_4 = (at + 2f64);
                                        {
                                            at = at_4;
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
            safe.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut snapshot: f64, mut aliases: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (node < size) {
            if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) {
                {
                    let child = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                    if ((child >= (0f64)) && self::ml_array::includes(&{ let ml_reference_value154 = (self::read_roots(facts.clone(), 1f64, child, width, span)).borrow().clone(); ml_reference_value154 }, &snapshot, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                        {
                            let aliases_2 = self::add_value(aliases.clone(), node);
                            {
                                let node_2 = (node + 1f64);
                                {
                                    (aliases, node) = (aliases_2.clone(), node_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let node_3 = (node + 1f64);
                            {
                                node = node_3;
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let node_4 = (node + 1f64);
                    {
                        node = node_4;
                        continue;
                    }
                }
            }
        } else {
            aliases.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop62(mut aliases: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut passes_alias: bool, mut source_ml_source1_temporary_values52: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index52: self::ml::Big) -> bool {
    loop {
        return if (source_ml_source1_temporary_index52 < ({ let ml_reference_array155 = source_ml_source1_temporary_values52.clone(); let ml_reference_projection155 = self::ml::Big::from_u128(ml_reference_array155.borrow().len() as u128); ml_reference_projection155 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value109 = source_ml_source1_temporary_values52.clone();
                    {
                        let source_ml_source2_array_read_index110 = source_ml_source1_temporary_index52.clone();
                        { let ml_reference_array156 = source_ml_source2_array_read_value109.clone(); let ml_reference_position156 = source_ml_source2_array_read_index110.clone(); let ml_reference_projection156 = self::ml_array::at(&*ml_reference_array156.borrow(), ml_reference_position156.to_index()); ml_reference_projection156 }
                    }
                };
                if self::ml_array::includes(&{ let ml_reference_value157 = aliases.borrow().clone(); ml_reference_value157 }, &root, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                    {
                        let passes_alias_2 = true;
                        passes_alias_2
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index52_2 = source_ml_source1_temporary_index52.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index52 = source_ml_source1_temporary_index52_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            passes_alias
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop61(mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut aliases: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut passes_alias: bool, mut source_ml_source1_temporary_values51: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index51: self::ml::Big) -> bool {
    loop {
        return if (source_ml_source1_temporary_index51 < ({ let ml_reference_array158 = source_ml_source1_temporary_values51.clone(); let ml_reference_projection158 = self::ml::Big::from_u128(ml_reference_array158.borrow().len() as u128); ml_reference_projection158 })) {
            {
                let argument = {
                    let source_ml_source2_array_read_value111 = source_ml_source1_temporary_values51.clone();
                    {
                        let source_ml_source2_array_read_index112 = source_ml_source1_temporary_index51.clone();
                        { let ml_reference_array159 = source_ml_source2_array_read_value111.clone(); let ml_reference_position159 = source_ml_source2_array_read_index112.clone(); let ml_reference_projection159 = self::ml_array::at(&*ml_reference_array159.borrow(), ml_reference_position159.to_index()); ml_reference_projection159 }
                    }
                };
                {
                    let source_ml_source1_temporary_values52 = self::read_roots(facts.clone(), 1f64, argument, width, span);
                    {
                        let source_ml_source1_temporary_index52 = self::ml::Big::from_i128(0);
                        {
                            let passes_alias_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop62(aliases.clone(), passes_alias, source_ml_source1_temporary_values52.clone(), source_ml_source1_temporary_index52.clone());
                            if passes_alias_2 {
                                passes_alias_2
                            } else {
                                {
                                    let source_ml_source1_temporary_index51_2 = source_ml_source1_temporary_index51.add(&self::ml::Big::from_i128(1));
                                    {
                                        (passes_alias, source_ml_source1_temporary_index51) = (passes_alias_2, source_ml_source1_temporary_index51_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            passes_alias
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop63(mut scopes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values53: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index53: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index53 < ({ let ml_reference_array160 = source_ml_source1_temporary_values53.clone(); let ml_reference_projection160 = self::ml::Big::from_u128(ml_reference_array160.borrow().len() as u128); ml_reference_projection160 })) {
            {
                let callee = {
                    let source_ml_source2_array_read_value113 = source_ml_source1_temporary_values53.clone();
                    {
                        let source_ml_source2_array_read_index114 = source_ml_source1_temporary_index53.clone();
                        { let ml_reference_array161 = source_ml_source2_array_read_value113.clone(); let ml_reference_position161 = source_ml_source2_array_read_index114.clone(); let ml_reference_projection161 = self::ml_array::at(&*ml_reference_array161.borrow(), ml_reference_position161.to_index()); ml_reference_projection161 }
                    }
                };
                {
                    let scopes_2 = self::add_value(scopes.clone(), callee);
                    {
                        let source_ml_source1_temporary_index53_2 = source_ml_source1_temporary_index53.add(&self::ml::Big::from_i128(1));
                        {
                            (scopes, source_ml_source1_temporary_index53) = (scopes_2.clone(), source_ml_source1_temporary_index53_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            scopes.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop60(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut aliases: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut scopes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (node < size) {
            if (((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])))) {
                {
                    let actual = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116]));
                    {
                        let passes_alias = false;
                        {
                            let source_ml_source1_temporary_values51 = actual.clone();
                            {
                                let source_ml_source1_temporary_index51 = self::ml::Big::from_i128(0);
                                {
                                    let passes_alias_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop61(width, span, facts.clone(), aliases.clone(), passes_alias, source_ml_source1_temporary_values51.clone(), source_ml_source1_temporary_index51.clone());
                                    if passes_alias_2 {
                                        {
                                            let targets_ = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101]));
                                            {
                                                let callees = if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    targets_.clone()
                                                } else {
                                                    if { let ml_value = { let ml_reference_array163 = targets_.clone(); let ml_reference_projection163 = ml_reference_array163.borrow().len() as f64; ml_reference_projection163 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                        self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(targets_.clone(), 0f64), width, span)
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_temporary_values53 = callees.clone();
                                                    {
                                                        let source_ml_source1_temporary_index53 = self::ml::Big::from_i128(0);
                                                        {
                                                            let scopes_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop63(scopes.clone(), source_ml_source1_temporary_values53.clone(), source_ml_source1_temporary_index53.clone());
                                                            {
                                                                let node_2 = (node + 1f64);
                                                                {
                                                                    (scopes, node) = (scopes_2.clone(), node_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let node_3 = (node + 1f64);
                                            {
                                                node = node_3;
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
                {
                    let node_4 = (node + 1f64);
                    {
                        node = node_4;
                        continue;
                    }
                }
            }
        } else {
            scopes.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut snapshots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut accepted: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result {
    loop {
        return if (at < ({ let ml_reference_array164 = snapshots.clone(); let ml_reference_projection164 = ml_reference_array164.borrow().len() as f64; ml_reference_projection164 })) {
            {
                let snapshot = self::read_required_number_fact(snapshots.clone(), at);
                {
                    let source = self::read_required_number_fact(snapshots.clone(), (at + 1f64));
                    {
                        let aliases = std::rc::Rc::new(std::cell::RefCell::new(vec![snapshot]));
                        {
                            let scopes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            if ((self::text_fact(roles.clone(), snapshot)) == (self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116]))) {
                                if !(self::accepts_effects(facts.clone(), roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), source, true, main_, width, span)) {
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, snapshot])))
                                } else {
                                    {
                                        let accepted_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value115 = accepted.clone();
                                            {
                                                let source_ml_source2_concat_value116 = std::rc::Rc::new(std::cell::RefCell::new(vec![snapshot]));
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value165 = source_ml_source2_concat_value115.borrow().clone(); ml_reference_value165 }), { let ml_reference_value166 = source_ml_source2_concat_value116.borrow().clone(); ml_reference_value166 })
                                            }
                                        }));
                                        {
                                            let at_2 = (at + 2f64);
                                            {
                                                (accepted, at) = (accepted_2.clone(), at_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let node = 0f64;
                                    {
                                        let aliases_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59(roles.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), size, width, span, facts.clone(), snapshot, aliases.clone(), node);
                                        {
                                            let node_2 = 0f64;
                                            {
                                                let scopes_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop60(roles.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), size, width, span, facts.clone(), aliases_2.clone(), scopes.clone(), node_2);
                                                if ((({ let ml_reference_array167 = scopes_2.clone(); let ml_reference_projection167 = ml_reference_array167.borrow().len() as f64; ml_reference_projection167 }) == (0f64)) || !(self::accepts_effects(facts.clone(), roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), scopes_2.clone(), source, true, main_, width, span))) {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, snapshot])))
                                                } else {
                                                    {
                                                        let accepted_3 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value117 = accepted.clone();
                                                            {
                                                                let source_ml_source2_concat_value118 = std::rc::Rc::new(std::cell::RefCell::new(vec![snapshot]));
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value168 = source_ml_source2_concat_value117.borrow().clone(); ml_reference_value168 }), { let ml_reference_value169 = source_ml_source2_concat_value118.borrow().clone(); ml_reference_value169 })
                                                            }
                                                        }));
                                                        {
                                                            let at_3 = (at + 2f64);
                                                            {
                                                                (accepted, at) = (accepted_3.clone(), at_3);
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
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(accepted.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop65(mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array170 = functions.clone(); let ml_reference_projection170 = ml_reference_array170.borrow().len() as f64; ml_reference_projection170 })) {
            if (((self::read_required_number_fact(bodies.clone(), at)) >= (0f64)) && self::ml_array::includes(&{ let ml_reference_value171 = facts.borrow().clone(); ml_reference_value171 }, &((4f64 * span) + self::read_required_number_fact(functions.clone(), at)), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                {
                    let facts_2 = self::add_roots(facts.clone(), 5f64, self::read_required_number_fact(functions.clone(), at), self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(bodies.clone(), at), width, span), width, span);
                    {
                        let at_2 = (at + 1f64);
                        {
                            (facts, at) = (facts_2.clone(), at_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            facts.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin66 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop69(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut position: f64, mut parameter: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (parameter < ((self::read_required_number_fact(parameter_starts.clone(), position) + self::read_required_number_fact(parameter_counts.clone(), position)))) {
            if self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), self::read_required_number_fact(parameters.clone(), parameter))) {
                {
                    let facts_2 = self::add_roots(facts.clone(), 1f64, self::read_required_number_fact(parameters.clone(), parameter), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])), width, span);
                    {
                        let parameter_2 = (parameter + 1f64);
                        {
                            (facts, parameter) = (facts_2.clone(), parameter_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let parameter_3 = (parameter + 1f64);
                    {
                        parameter = parameter_3;
                        continue;
                    }
                }
            }
        } else {
            facts.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop68(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut list: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array172 = list.clone(); let ml_reference_projection172 = ml_reference_array172.borrow().len() as f64; ml_reference_projection172 })) {
            {
                let owner = self::read_required_number_fact(owners.clone(), self::read_required_number_fact(list.clone(), at));
                if ((owner >= (0f64)) && !(self::ml_array::includes(&{ let ml_reference_value173 = facts.borrow().clone(); ml_reference_value173 }, &((4f64 * span) + owner), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    {
                        let facts_2 = self::add_fact(facts.clone(), ((4f64 * span) + owner));
                        {
                            let position = self::ml_array::index_of(&{ let ml_reference_value174 = functions.borrow().clone(); ml_reference_value174 }, &owner, 0f64, false, |a, b| a == b);
                            if (position >= (0f64)) {
                                {
                                    let parameter = self::read_required_number_fact(parameter_starts.clone(), position);
                                    {
                                        let facts_3 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop69(kinds.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), width, span, facts_2.clone(), position, parameter);
                                        {
                                            let at_2 = (at + 2f64);
                                            {
                                                (facts, at) = (facts_3.clone(), at_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at_3 = (at + 2f64);
                                    {
                                        (facts, at) = (facts_2.clone(), at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_4 = (at + 2f64);
                        {
                            at = at_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            facts.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop67(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values50: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>, mut source_ml_source1_temporary_index50: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index50 < ({ let ml_reference_array175 = source_ml_source1_temporary_values50.clone(); let ml_reference_projection175 = self::ml::Big::from_u128(ml_reference_array175.borrow().len() as u128); ml_reference_projection175 })) {
            {
                let list = {
                    let source_ml_source2_array_read_value119 = source_ml_source1_temporary_values50.clone();
                    {
                        let source_ml_source2_array_read_index120 = source_ml_source1_temporary_index50.clone();
                        { let ml_reference_array176 = source_ml_source2_array_read_value119.clone(); let ml_reference_position176 = source_ml_source2_array_read_index120.clone(); let ml_reference_projection176 = self::ml_array::at(&*ml_reference_array176.borrow(), ml_reference_position176.to_index()); ml_reference_projection176 }
                    }
                };
                {
                    let at = 0f64;
                    {
                        let facts_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop68(kinds.clone(), owners.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), width, span, facts.clone(), list.clone(), at);
                        {
                            let source_ml_source1_temporary_index50_2 = source_ml_source1_temporary_index50.add(&self::ml::Big::from_i128(1));
                            {
                                (facts, source_ml_source1_temporary_index50) = (facts_2.clone(), source_ml_source1_temporary_index50_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            facts.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop64Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values47: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index47: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index47 < ({ let ml_reference_array177 = source_ml_source1_temporary_values47.clone(); let ml_reference_projection177 = self::ml::Big::from_u128(ml_reference_array177.borrow().len() as u128); ml_reference_projection177 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value121 = source_ml_source1_temporary_values47.clone();
                    {
                        let source_ml_source2_array_read_index122 = source_ml_source1_temporary_index47.clone();
                        { let ml_reference_array178 = source_ml_source2_array_read_value121.clone(); let ml_reference_position178 = source_ml_source2_array_read_index122.clone(); let ml_reference_projection178 = self::ml_array::at(&*ml_reference_array178.borrow(), ml_reference_position178.to_index()); ml_reference_projection178 }
                    }
                };
                {
                    let roots_2 = self::add_value(roots.clone(), root);
                    {
                        let source_ml_source1_temporary_index47_2 = source_ml_source1_temporary_index47.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index47) = (roots_2.clone(), source_ml_source1_temporary_index47_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop71(mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values46: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index46: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index46 < ({ let ml_reference_array179 = source_ml_source1_temporary_values46.clone(); let ml_reference_projection179 = self::ml::Big::from_u128(ml_reference_array179.borrow().len() as u128); ml_reference_projection179 })) {
            {
                let child = {
                    let source_ml_source2_array_read_value123 = source_ml_source1_temporary_values46.clone();
                    {
                        let source_ml_source2_array_read_index124 = source_ml_source1_temporary_index46.clone();
                        { let ml_reference_array180 = source_ml_source2_array_read_value123.clone(); let ml_reference_position180 = source_ml_source2_array_read_index124.clone(); let ml_reference_projection180 = self::ml_array::at(&*ml_reference_array180.borrow(), ml_reference_position180.to_index()); ml_reference_projection180 }
                    }
                };
                {
                    let source_ml_source1_temporary_values47 = self::read_roots(facts.clone(), 1f64, child, width, span);
                    {
                        let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                        {
                            let roots_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72(roots.clone(), source_ml_source1_temporary_values47.clone(), source_ml_source1_temporary_index47.clone());
                            {
                                let source_ml_source1_temporary_index46_2 = source_ml_source1_temporary_index46.add(&self::ml::Big::from_i128(1));
                                {
                                    (roots, source_ml_source1_temporary_index46) = (roots_2.clone(), source_ml_source1_temporary_index46_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop76(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut field_width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut role: self::ml_string::Text, mut object: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ((self::read_required_number_fact(starts.clone(), node) + self::read_required_number_fact(counts.clone(), node)))) {
            if ((self::text_fact(labels.clone(), at)).starts_with(&self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 58])) || (self::text_fact(labels.clone(), at)).starts_with(&self::ml_string::Text::from_units(vec![105, 116, 101, 109, 58]))) {
                {
                    let actual = if (role == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                        self::read_roots(facts.clone(), 1f64, object, width, span)
                    } else {
                        std::rc::Rc::new(std::cell::RefCell::new(vec![node]))
                    };
                    {
                        let facts_2 = self::add_field(facts.clone(), actual.clone(), (self::ml_array::index_of(&{ let ml_reference_value181 = labels.borrow().clone(); ml_reference_value181 }, &self::text_fact(labels.clone(), at), 0f64, false, |a, b| a == b) + 1f64), self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(targets.clone(), at), width, span), field_width, width, span);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (facts, at) = (facts_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            facts.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin75 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
    MlSource1TemporaryProveDisjointCollectionIterationsJoin75Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin74 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin73 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop78(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values49: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index49: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index49 < ({ let ml_reference_array182 = source_ml_source1_temporary_values49.clone(); let ml_reference_projection182 = self::ml::Big::from_u128(ml_reference_array182.borrow().len() as u128); ml_reference_projection182 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value125 = source_ml_source1_temporary_values49.clone();
                    {
                        let source_ml_source2_array_read_index126 = source_ml_source1_temporary_index49.clone();
                        { let ml_reference_array183 = source_ml_source2_array_read_value125.clone(); let ml_reference_position183 = source_ml_source2_array_read_index126.clone(); let ml_reference_projection183 = self::ml_array::at(&*ml_reference_array183.borrow(), ml_reference_position183.to_index()); ml_reference_projection183 }
                    }
                };
                {
                    let roots_2 = self::add_value(roots.clone(), root);
                    {
                        let source_ml_source1_temporary_index49_2 = source_ml_source1_temporary_index49.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index49) = (roots_2.clone(), source_ml_source1_temporary_index49_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop77(mut width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values48: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index48: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index48 < ({ let ml_reference_array184 = source_ml_source1_temporary_values48.clone(); let ml_reference_projection184 = self::ml::Big::from_u128(ml_reference_array184.borrow().len() as u128); ml_reference_projection184 })) {
            {
                let target = {
                    let source_ml_source2_array_read_value127 = source_ml_source1_temporary_values48.clone();
                    {
                        let source_ml_source2_array_read_index128 = source_ml_source1_temporary_index48.clone();
                        { let ml_reference_array185 = source_ml_source2_array_read_value127.clone(); let ml_reference_position185 = source_ml_source2_array_read_index128.clone(); let ml_reference_projection185 = self::ml_array::at(&*ml_reference_array185.borrow(), ml_reference_position185.to_index()); ml_reference_projection185 }
                    }
                };
                {
                    let source_ml_source1_temporary_values49 = self::read_roots(facts.clone(), 5f64, target, width, span);
                    {
                        let source_ml_source1_temporary_index49 = self::ml::Big::from_i128(0);
                        {
                            let roots_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop78(roots.clone(), source_ml_source1_temporary_values49.clone(), source_ml_source1_temporary_index49.clone());
                            {
                                let source_ml_source1_temporary_index48_2 = source_ml_source1_temporary_index48.add(&self::ml::Big::from_i128(1));
                                {
                                    (roots, source_ml_source1_temporary_index48) = (roots_2.clone(), source_ml_source1_temporary_index48_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            roots.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin70 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryProveDisjointCollectionIterationsJoin70Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}
