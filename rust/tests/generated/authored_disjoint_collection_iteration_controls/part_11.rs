pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop64(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut size: f64, mut width: f64, mut field_width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut work: f64, mut node: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result {
    loop {
        return if (node < size) {
            {
                let work_2 = (work + (({ let ml_reference_array186 = facts.clone(); let ml_reference_projection186 = ml_reference_array186.borrow().len() as f64; ml_reference_projection186 } + 1f64) * (self::read_required_number_fact(counts.clone(), node) + 4f64)));
                if ((((self::read_required_number_fact(facts.clone(), 0f64)) == ((-1f64))) || (work_2 > (268435456f64))) || (({ let ml_reference_array187 = facts.clone(); let ml_reference_projection187 = ml_reference_array187.borrow().len() as f64; ml_reference_projection187 }) > (65536f64))) {
                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
                } else {
                    {
                        let facts_3 = if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                            {
                                let facts_2 = self::add_roots(facts.clone(), 1f64, node, std::rc::Rc::new(std::cell::RefCell::new(vec![node])), width, span);
                                facts_2.clone()
                            }
                        } else {
                            facts.clone()
                        };
                        if !(self::ml_array::includes(&{ let ml_reference_value188 = facts_3.borrow().clone(); ml_reference_value188 }, &((4f64 * span) + self::read_required_number_fact(owners.clone(), node)), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                            {
                                let node_2 = (node + 1f64);
                                {
                                    (facts, work, node) = (facts_3.clone(), work_2, node_2);
                                    continue;
                                }
                            }
                        } else {
                            {
                                let role = self::text_fact(roles.clone(), node);
                                {
                                    let object = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                    {
                                        let value = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                        {
                                            let arguments_ = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116]));
                                            {
                                                let roots = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                {
                                                    let roots_3 = if (((((((role == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) || (role == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))) || (role == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) || (role == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (role == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))) || (role == (self::ml_string::Text::from_units(vec![115, 110, 97, 112, 115, 104, 111, 116])))) || (role == (self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116])))) {
                                                        {
                                                            let roots_2 = std::rc::Rc::new(std::cell::RefCell::new(vec![node]));
                                                            roots_2.clone()
                                                        }
                                                    } else {
                                                        roots.clone()
                                                    };
                                                    {
                                                        let roots_5 = if (((role == (self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 98, 108, 101]))) || (role == (self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103])))) || (role == (self::ml_string::Text::from_units(vec![114, 101, 97, 100])))) {
                                                            {
                                                                let roots_4 = if (value < (0f64)) {
                                                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                                } else {
                                                                    self::read_roots(facts_3.clone(), 1f64, value, width, span)
                                                                };
                                                                roots_4.clone()
                                                            }
                                                        } else {
                                                            roots_3.clone()
                                                        };
                                                        {
                                                            let source_ml_source2_s129 = if (role == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]))) {
                                                                {
                                                                    let actual = self::read_roots(facts_3.clone(), 1f64, object, width, span);
                                                                    {
                                                                        let work_3 = (work_2 + (({ let ml_reference_array189 = facts_3.clone(); let ml_reference_projection189 = ml_reference_array189.borrow().len() as f64; ml_reference_projection189 } + 1f64) * { let ml_reference_array190 = actual.clone(); let ml_reference_projection190 = ml_reference_array190.borrow().len() as f64; ml_reference_projection190 }));
                                                                        if (work_3 > (268435456f64)) {
                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
                                                                        } else {
                                                                            {
                                                                                let roots_6 = self::read_field(facts_3.clone(), actual.clone(), (self::ml_array::index_of(&{ let ml_reference_value191 = labels.borrow().clone(); ml_reference_value191 }, &self::text_fact(operations.clone(), node), 0f64, false, |a, b| a == b) + 1f64), field_width, width, span);
                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(work_3, roots_6.clone())
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(work_2, roots_5.clone())
                                                            };
                                                            match source_ml_source2_s129.clone() {
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(work_4, roots_7) => {
                                                                    {
                                                                        let roots_9 = if (role == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))) {
                                                                            {
                                                                                let roots_8 = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101]));
                                                                                roots_8.clone()
                                                                            }
                                                                        } else {
                                                                            roots_7.clone()
                                                                        };
                                                                        {
                                                                            let roots_11 = if (role == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                                {
                                                                                    let source_ml_source1_temporary_values46 = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116]));
                                                                                    {
                                                                                        let source_ml_source1_temporary_index46 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let roots_10 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop71(width, span, facts_3.clone(), roots_9.clone(), source_ml_source1_temporary_values46.clone(), source_ml_source1_temporary_index46.clone());
                                                                                            roots_10.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                roots_9.clone()
                                                                            };
                                                                            {
                                                                                let source_ml_source2_s130 = if ((role == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || (role == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])))) {
                                                                                    {
                                                                                        let callee = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101]));
                                                                                        {
                                                                                            let actual_2 = callee.clone();
                                                                                            {
                                                                                                let actual_4 = if (role == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) {
                                                                                                    {
                                                                                                        let actual_3 = if { let ml_value = { let ml_reference_array193 = callee.clone(); let ml_reference_projection193 = ml_reference_array193.borrow().len() as f64; ml_reference_projection193 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                                            self::read_roots(facts_3.clone(), 1f64, self::read_required_number_fact(callee.clone(), 0f64), width, span)
                                                                                                        } else {
                                                                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                                                                        };
                                                                                                        actual_3.clone()
                                                                                                    }
                                                                                                } else {
                                                                                                    actual_2.clone()
                                                                                                };
                                                                                                {
                                                                                                    let facts_4 = self::bind_calls(facts_3.clone(), self::read_required_number_fact(owners.clone(), node), actual_4.clone(), arguments_.clone(), self::ml_string::Text::from_units(vec![]), (-1f64), kinds.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, width, span);
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values48 = actual_4.clone();
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index48 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let roots_12 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop77(width, span, facts_4.clone(), roots_11.clone(), source_ml_source1_temporary_values48.clone(), source_ml_source1_temporary_index48.clone());
                                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin73::MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(facts_4.clone(), roots_12.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin73::MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(facts_3.clone(), roots_11.clone())
                                                                                };
                                                                                match source_ml_source2_s130.clone() {
                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin73::MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(facts_5, roots_13) => {
                                                                                        {
                                                                                            let source_ml_source2_s131 = if (role == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                                                                                                {
                                                                                                    let roots_16 = if (((self::read_reference_method_result(self::text_fact(kinds.clone(), object), self::text_fact(operations.clone(), node))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))) || (((self::text_fact(kinds.clone(), object)) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) && (((self::read_array_state_method_result(self::text_fact(operations.clone(), node))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))) || ((self::text_fact(operations.clone(), node)) == (self::ml_string::Text::from_units(vec![115, 111, 114, 116])))))) {
                                                                                                        {
                                                                                                            let roots_14 = self::read_roots(facts_5.clone(), 1f64, object, width, span);
                                                                                                            roots_14.clone()
                                                                                                        }
                                                                                                    } else {
                                                                                                        if self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), node)) {
                                                                                                            {
                                                                                                                let roots_15 = std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]));
                                                                                                                roots_15.clone()
                                                                                                            }
                                                                                                        } else {
                                                                                                            roots_13.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    if (self::read_disjoint_collection_iteration_callback(self::text_fact(operations.clone(), node)) && (({ let ml_reference_array194 = arguments_.clone(); let ml_reference_projection194 = ml_reference_array194.borrow().len() as f64; ml_reference_projection194 }) != (0f64))) {
                                                                                                        {
                                                                                                            let facts_6 = self::bind_calls(facts_5.clone(), self::read_required_number_fact(owners.clone(), node), self::read_roots(facts_5.clone(), 1f64, self::read_required_number_fact(arguments_.clone(), 0f64), width, span), arguments_.clone(), self::text_fact(operations.clone(), node), object, kinds.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, width, span);
                                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_6.clone(), roots_16.clone())
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_5.clone(), roots_16.clone())
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_5.clone(), roots_13.clone())
                                                                                            };
                                                                                            match source_ml_source2_s131.clone() {
                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_7, roots_17) => {
                                                                                                    {
                                                                                                        let roots_19 = if (((role == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))) || (role == (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 114, 101, 97, 100])))) && self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), node))) {
                                                                                                            {
                                                                                                                let roots_18 = std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]));
                                                                                                                roots_18.clone()
                                                                                                            }
                                                                                                        } else {
                                                                                                            roots_17.clone()
                                                                                                        };
                                                                                                        {
                                                                                                            let facts_9 = if ((role != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) && self::read_disjoint_collection_iteration_identity_kind(self::text_fact(kinds.clone(), node))) {
                                                                                                                {
                                                                                                                    let facts_8 = self::add_roots(facts_7.clone(), 1f64, node, roots_19.clone(), width, span);
                                                                                                                    facts_8.clone()
                                                                                                                }
                                                                                                            } else {
                                                                                                                facts_7.clone()
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_s132 = if (role == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) {
                                                                                                                    {
                                                                                                                        let actual_5 = self::read_roots(facts_9.clone(), 1f64, value, width, span);
                                                                                                                        {
                                                                                                                            let work_5 = (work_4 + (({ let ml_reference_array195 = facts_9.clone(); let ml_reference_projection195 = ml_reference_array195.borrow().len() as f64; ml_reference_projection195 } + 1f64) * { let ml_reference_array196 = actual_5.clone(); let ml_reference_projection196 = ml_reference_array196.borrow().len() as f64; ml_reference_projection196 }));
                                                                                                                            if (work_5 > (268435456f64)) {
                                                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let facts_10 = self::copy_fields(facts_9.clone(), actual_5.clone(), node, field_width, width, span);
                                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_10.clone(), work_5)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_9.clone(), work_4)
                                                                                                                };
                                                                                                                match source_ml_source2_s132.clone() {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_11, work_6) => {
                                                                                                                        if ((((role == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) || (role == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (role == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))) || (role == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))) {
                                                                                                                            {
                                                                                                                                let at = self::read_required_number_fact(starts.clone(), node);
                                                                                                                                {
                                                                                                                                    let facts_12 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop76(starts.clone(), counts.clone(), labels.clone(), targets.clone(), width, field_width, span, facts_11.clone(), node, role.clone(), object, at);
                                                                                                                                    {
                                                                                                                                        let node_3 = (node + 1f64);
                                                                                                                                        {
                                                                                                                                            (facts, work, node) = (facts_12.clone(), work_6, node_3);
                                                                                                                                            continue;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let node_4 = (node + 1f64);
                                                                                                                                {
                                                                                                                                    (facts, work, node) = (facts_11.clone(), work_6, node_4);
                                                                                                                                    continue;
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Return(source_ml_source1_temporary_result76) => {
                                                                                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(source_ml_source1_temporary_result76.clone())
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
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin70::MlSource1TemporaryProveDisjointCollectionIterationsJoin70Return(source_ml_source1_temporary_result78) => {
                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(source_ml_source1_temporary_result78.clone())
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
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Done(facts.clone(), work)
        };
    }
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56(mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut snapshots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut callbacks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut width: f64, mut field_width: f64, mut span: f64, mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut work: f64, mut settled: bool, mut seeded: bool, mut pass: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result {
    loop {
        return if ((pass < (128f64)) && !settled) {
            {
                let before = { let ml_reference_array197 = facts.clone(); let ml_reference_projection197 = ml_reference_array197.borrow().len() as f64; ml_reference_projection197 };
                {
                    let node = 0f64;
                    {
                        let source_ml_source2_s133 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop64(roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, size, width, field_width, span, facts.clone(), work, node);
                        match source_ml_source2_s133.clone() {
                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Done(facts_2, work_2) => {
                                {
                                    let at = 0f64;
                                    {
                                        let facts_3 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop65(functions.clone(), bodies.clone(), width, span, facts_2.clone(), at);
                                        if (({ let ml_reference_array198 = facts_3.clone(); let ml_reference_projection198 = ml_reference_array198.borrow().len() as f64; ml_reference_projection198 }) == before) {
                                            {
                                                let source_ml_source2_s134 = if !seeded {
                                                    {
                                                        let seeded_2 = true;
                                                        {
                                                            let source_ml_source1_temporary_values50 = std::rc::Rc::new(std::cell::RefCell::new(vec![snapshots.clone(), callbacks.clone()]));
                                                            {
                                                                let source_ml_source1_temporary_index50 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let facts_4 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop67(kinds.clone(), owners.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), width, span, facts_3.clone(), source_ml_source1_temporary_values50.clone(), source_ml_source1_temporary_index50.clone());
                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin66::MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(facts_4.clone(), seeded_2)
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin66::MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(facts_3.clone(), seeded)
                                                };
                                                match source_ml_source2_s134.clone() {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin66::MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(facts_5, seeded_3) => {
                                                        {
                                                            let settled_2 = (({ let ml_reference_array199 = facts_5.clone(); let ml_reference_projection199 = ml_reference_array199.borrow().len() as f64; ml_reference_projection199 }) == before);
                                                            {
                                                                let pass_2 = (pass + 1f64);
                                                                {
                                                                    (facts, work, settled, seeded, pass) = (facts_5.clone(), work_2, settled_2, seeded_3, pass_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let pass_3 = (pass + 1f64);
                                                {
                                                    (facts, work, pass) = (facts_3.clone(), work_2, pass_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(source_ml_source1_temporary_result69) => {
                                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(source_ml_source1_temporary_result69.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done(facts.clone(), settled)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done,
    MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55(mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut at: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result {
    loop {
        return if (at < ({ let ml_reference_array200 = functions.clone(); let ml_reference_projection200 = ml_reference_array200.borrow().len() as f64; ml_reference_projection200 })) {
            if ((((((((((!(self::ml_math::is_integer(self::read_required_number_fact(functions.clone(), at))) || !(self::ml_math::is_integer(self::read_required_number_fact(parameter_starts.clone(), at)))) || !(self::ml_math::is_integer(self::read_required_number_fact(parameter_counts.clone(), at)))) || !(self::ml_math::is_integer(self::read_required_number_fact(bodies.clone(), at)))) || ((self::read_required_number_fact(functions.clone(), at)) < (0f64))) || ((self::read_required_number_fact(functions.clone(), at)) >= size)) || ((self::read_required_number_fact(parameter_starts.clone(), at)) < (0f64))) || ((self::read_required_number_fact(parameter_counts.clone(), at)) < (0f64))) || (((self::read_required_number_fact(parameter_starts.clone(), at) + self::read_required_number_fact(parameter_counts.clone(), at))) > ({ let ml_reference_array201 = parameters.clone(); let ml_reference_projection201 = ml_reference_array201.borrow().len() as f64; ml_reference_projection201 }))) || ((self::read_required_number_fact(bodies.clone(), at)) < ((-1f64)))) || ((self::read_required_number_fact(bodies.clone(), at)) >= size)) {
                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop54Done,
    MlSource1TemporaryProveDisjointCollectionIterationsLoop54Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop54(mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut node: f64) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result {
    loop {
        return if (node < size) {
            if (((((((!(self::ml_math::is_integer(self::read_required_number_fact(starts.clone(), node))) || !(self::ml_math::is_integer(self::read_required_number_fact(counts.clone(), node)))) || !(self::ml_math::is_integer(self::read_required_number_fact(owners.clone(), node)))) || ((self::read_required_number_fact(starts.clone(), node)) < (0f64))) || ((self::read_required_number_fact(counts.clone(), node)) < (0f64))) || (((self::read_required_number_fact(starts.clone(), node) + self::read_required_number_fact(counts.clone(), node))) > ({ let ml_reference_array202 = targets.clone(); let ml_reference_projection202 = ml_reference_array202.borrow().len() as f64; ml_reference_projection202 }))) || ((self::read_required_number_fact(owners.clone(), node)) < ((-1f64)))) || ((self::read_required_number_fact(owners.clone(), node)) >= size)) {
                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Return(std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))
            } else {
                {
                    let node_2 = (node + 1f64);
                    {
                        node = node_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Done
        };
    }
}

fn ml_main() {
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![115, 101, 116]))) == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))), "assertion 1");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![97, 100, 100]))) == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))), "assertion 2");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))), "assertion 3");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100, 48]))) == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))), "assertion 4");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![103, 101, 116]))) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100]))), "assertion 5");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![109, 97, 112]))) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100]))), "assertion 6");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![99, 101, 108, 108]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 7");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![102, 117, 116, 117, 114, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]), self::ml_string::Text::from_units(vec![115, 101, 116]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 8");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![102, 117, 116, 117, 114, 101, 79, 112, 101, 114, 97, 116, 105, 111, 110]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 9");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![115, 101, 116]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 10");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![103, 101, 116]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 11");
    assert!(((self::read_disjoint_collection_iteration_method_effect(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100, 48, 48]))) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))), "assertion 12");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))) == (true)), "assertion 13");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![99, 101, 108, 108, 82, 101, 97, 100]))) == (true)), "assertion 14");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101]))) == (false)), "assertion 15");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 87, 114, 105, 116, 101]))) == (false)), "assertion 16");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) == (false)), "assertion 17");
    assert!(((self::accept_disjoint_collection_iteration_read_expression(self::ml_string::Text::from_units(vec![102, 117, 116, 117, 114, 101, 67, 104, 101, 99, 107, 101, 100, 79, 112, 101, 114, 97, 116, 105, 111, 110]))) == (false)), "assertion 18");
    assert!(((self::accept_disjoint_collection_iteration_callback_head(self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))) == (true)), "assertion 19");
    assert!(((self::accept_disjoint_collection_iteration_callback_head(self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) == (false)), "assertion 20");
    assert!(((self::accept_disjoint_collection_iteration_scope(true, 0f64, 0f64, 0f64, false, false)) == (true)), "assertion 21");
    assert!(((self::accept_disjoint_collection_iteration_scope(true, 0f64, 0f64, 0f64, false, true)) == (false)), "assertion 22");
    assert!(((self::accept_disjoint_collection_iteration_operation(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) == (true)), "assertion 23");
    assert!(((self::accept_disjoint_collection_iteration_operation(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) == (false)), "assertion 24");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))), "assertion 25");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))), "assertion 26");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))), "assertion 27");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))), "assertion 28");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))), "assertion 29");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))), "assertion 30");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 31");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 32");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![103, 101, 116]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 33");
    assert!(((self::read_disjoint_collection_iteration_item_form(self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![102, 117, 116, 117, 114, 101, 73, 116, 101, 114, 97, 116, 111, 114]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 34");
    assert!(((self::read_disjoint_collection_iteration_marker_role(self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 48, 32, 118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![108, 111, 111, 112]))), "assertion 35");
    assert!(((self::read_disjoint_collection_iteration_marker_role(self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 112, 114, 101, 97, 100, 32, 48, 32, 118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114]))), "assertion 36");
    assert!(((self::read_disjoint_collection_iteration_marker_role(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 37");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) == (true)), "assertion 38");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]))) == (true)), "assertion 39");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![97, 112, 112]))) == (false)), "assertion 40");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) == (false)), "assertion 41");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) == (false)), "assertion 42");
    assert!(((self::accept_disjoint_collection_spread_receiver(self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120]))) == (false)), "assertion 43");
    assert!(((self::accept_disjoint_collection_iteration_scope(true, 1f64, 0f64, 0f64, false, false)) == (false)), "assertion 44");
    assert!(((self::accept_disjoint_collection_iteration_scope(true, 0f64, 0f64, 1f64, false, false)) == (false)), "assertion 45");
    assert!(((self::accept_disjoint_collection_iteration_scope(false, 0f64, 0f64, 0f64, false, false)) == (false)), "assertion 46");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_disjoint_collection_iteration_controls() { main(); }
