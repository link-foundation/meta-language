pub fn source_ml_source1_temporary_render_array_callback_call_loop8(mut operation: self::ml_string::Text, mut parameter_count: f64, mut suffix: self::ml_string::Text, mut clone_element: bool, mut arguments_text: self::ml_string::Text, mut position: f64, mut maximum: f64) -> self::ml_string::Text {
    loop {
        return if ((position < parameter_count) && (position < maximum)) {
            {
                let arguments_text_3 = if (position > (0f64)) {
                    {
                        let arguments_text_2 = arguments_text.concat(&self::ml_string::Text::from_units(vec![44, 32]));
                        arguments_text_2.clone()
                    }
                } else {
                    arguments_text.clone()
                };
                {
                    let arguments_text_4 = arguments_text_3.concat(&self::render_array_callback_argument(operation.clone(), position, suffix.clone(), clone_element));
                    {
                        let position_2 = (position + 1f64);
                        {
                            (arguments_text, position) = (arguments_text_4.clone(), position_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            arguments_text.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_array_state_method_suffix_loop9(mut text: self::ml_string::Text, mut suffix: f64) -> f64 {
    loop {
        return if (text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))) || text.contains(&((self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))).concat(&self::ml_string::Text::from_units(vec![95])))) {
            {
                let suffix_2 = (suffix + 1f64);
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        } else {
            suffix
        };
    }
}

pub fn source_ml_source1_temporary_render_array_state_method_expression_loop10(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut types: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut suffix: self::ml_string::Text, mut output: self::ml_string::Text, mut index: f64) -> self::ml_string::Text {
    loop {
        return if (index < ({ let ml_reference_array92 = arguments_.clone(); let ml_reference_projection92 = ml_reference_array92.borrow().len() as f64; ml_reference_projection92 })) {
            {
                let output_2 = output.concat(&((((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))).concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&({
                    let source_ml_source2_array_read_value66 = types.clone();
                    {
                        let source_ml_source2_array_read_index67 = index;
                        { let ml_reference_array93 = source_ml_source2_array_read_value66.clone(); let ml_reference_position93 = source_ml_source2_array_read_index67; let ml_reference_projection93 = self::ml_array::optional_at(&*ml_reference_array93.borrow(), self::ml_array::number_index(ml_reference_position93)); ml_reference_projection93 }
                    }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&({
                    let source_ml_source2_array_read_value68 = arguments_.clone();
                    {
                        let source_ml_source2_array_read_index69 = index;
                        { let ml_reference_array94 = source_ml_source2_array_read_value68.clone(); let ml_reference_position94 = source_ml_source2_array_read_index69; let ml_reference_projection94 = self::ml_array::optional_at(&*ml_reference_array94.borrow(), self::ml_array::number_index(ml_reference_position94)); ml_reference_projection94 }
                    }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (output, index) = (output_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            output.clone()
        };
    }
}

pub fn source_ml_source1_temporary_render_array_state_method_expression_loop11(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut suffix: self::ml_string::Text, mut names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (index < ({ let ml_reference_array95 = arguments_.clone(); let ml_reference_projection95 = ml_reference_array95.borrow().len() as f64; ml_reference_projection95 })) {
            {
                let names_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value96 = names.borrow().clone(); ml_reference_value96 }, vec![(((self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (names, index) = (names_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            names.clone()
        };
    }
}

pub fn source_ml_source1_temporary_render_array_state_method_expression_loop12(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut inserted: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (index < ({ let ml_reference_array97 = arguments_.clone(); let ml_reference_projection97 = ml_reference_array97.borrow().len() as f64; ml_reference_projection97 })) {
            {
                let inserted_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value98 = inserted.borrow().clone(); ml_reference_value98 }, vec![({
                    let source_ml_source2_array_read_value70 = names.clone();
                    {
                        let source_ml_source2_array_read_index71 = index;
                        { let ml_reference_array99 = source_ml_source2_array_read_value70.clone(); let ml_reference_position99 = source_ml_source2_array_read_index71; let ml_reference_projection99 = self::ml_array::optional_at(&*ml_reference_array99.borrow(), self::ml_array::number_index(ml_reference_position99)); ml_reference_projection99 }
                    }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (inserted, index) = (inserted_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            inserted.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_roots_loop14(mut tag: f64, mut node: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values13: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index13: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index13 < ({ let ml_reference_array100 = source_ml_source1_temporary_values13.clone(); let ml_reference_projection100 = self::ml::Big::from_u128(ml_reference_array100.borrow().len() as u128); ml_reference_projection100 })) {
            {
                let fact = {
                    let source_ml_source2_array_read_value72 = source_ml_source1_temporary_values13.clone();
                    {
                        let source_ml_source2_array_read_index73 = source_ml_source1_temporary_index13.clone();
                        { let ml_reference_array101 = source_ml_source2_array_read_value72.clone(); let ml_reference_position101 = source_ml_source2_array_read_index73.clone(); let ml_reference_projection101 = self::ml_array::at(&*ml_reference_array101.borrow(), ml_reference_position101.to_index()); ml_reference_projection101 }
                    }
                };
                if ((((fact / span)).floor()) == tag) {
                    {
                        let value = (fact % span);
                        if ((((value / width)).floor()) == node) {
                            {
                                let result_2 = self::add_value(result.clone(), ((value % width) - 1f64));
                                {
                                    let source_ml_source1_temporary_index13_2 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index13) = (result_2.clone(), source_ml_source1_temporary_index13_2.clone());
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_temporary_index13_3 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index13 = source_ml_source1_temporary_index13_3.clone();
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index13_4 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index13 = source_ml_source1_temporary_index13_4.clone();
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

pub fn source_ml_source1_temporary_add_roots_loop17(mut present: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values15: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index15: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index15 < ({ let ml_reference_array102 = source_ml_source1_temporary_values15.clone(); let ml_reference_projection102 = self::ml::Big::from_u128(ml_reference_array102.borrow().len() as u128); ml_reference_projection102 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value74 = source_ml_source1_temporary_values15.clone();
                    {
                        let source_ml_source2_array_read_index75 = source_ml_source1_temporary_index15.clone();
                        { let ml_reference_array103 = source_ml_source2_array_read_value74.clone(); let ml_reference_position103 = source_ml_source2_array_read_index75.clone(); let ml_reference_projection103 = self::ml_array::at(&*ml_reference_array103.borrow(), ml_reference_position103.to_index()); ml_reference_projection103 }
                    }
                };
                {
                    let present_2 = self::add_value(present.clone(), root);
                    {
                        let source_ml_source1_temporary_index15_2 = source_ml_source1_temporary_index15.add(&self::ml::Big::from_i128(1));
                        {
                            (present, source_ml_source1_temporary_index15) = (present_2.clone(), source_ml_source1_temporary_index15_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            present.clone()
        };
    }
}

pub fn source_ml_source1_temporary_add_roots_loop18(mut tag: f64, mut node: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values16: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index16: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index16 < ({ let ml_reference_array104 = source_ml_source1_temporary_values16.clone(); let ml_reference_projection104 = self::ml::Big::from_u128(ml_reference_array104.borrow().len() as u128); ml_reference_projection104 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value76 = source_ml_source1_temporary_values16.clone();
                    {
                        let source_ml_source2_array_read_index77 = source_ml_source1_temporary_index16.clone();
                        { let ml_reference_array105 = source_ml_source2_array_read_value76.clone(); let ml_reference_position105 = source_ml_source2_array_read_index77.clone(); let ml_reference_projection105 = self::ml_array::at(&*ml_reference_array105.borrow(), ml_reference_position105.to_index()); ml_reference_projection105 }
                    }
                };
                {
                    let result_2 = self::add_fact(result.clone(), ((((tag * span) + (node * width)) + root) + 1f64));
                    {
                        let source_ml_source1_temporary_index16_2 = source_ml_source1_temporary_index16.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index16) = (result_2.clone(), source_ml_source1_temporary_index16_2.clone());
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

pub fn source_ml_source1_temporary_edges_loop19(mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut label: self::ml_string::Text, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ((self::read_required_number_fact(starts.clone(), node) + self::read_required_number_fact(counts.clone(), node)))) {
            if ((self::text_fact(labels.clone(), at)) == label) {
                {
                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value78 = result.clone();
                        {
                            let source_ml_source2_concat_value79 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_required_number_fact(targets.clone(), at)]));
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value106 = source_ml_source2_concat_value78.borrow().clone(); ml_reference_value106 }), { let ml_reference_value107 = source_ml_source2_concat_value79.borrow().clone(); ml_reference_value107 })
                        }
                    }));
                    {
                        let at_2 = (at + 1f64);
                        {
                            (result, at) = (result_2.clone(), at_2);
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
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_field_loop23(mut field: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut root: f64, mut source_ml_source1_temporary_values21: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index21: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index21 < ({ let ml_reference_array108 = source_ml_source1_temporary_values21.clone(); let ml_reference_projection108 = self::ml::Big::from_u128(ml_reference_array108.borrow().len() as u128); ml_reference_projection108 })) {
            {
                let fact = {
                    let source_ml_source2_array_read_value80 = source_ml_source1_temporary_values21.clone();
                    {
                        let source_ml_source2_array_read_index81 = source_ml_source1_temporary_index21.clone();
                        { let ml_reference_array109 = source_ml_source2_array_read_value80.clone(); let ml_reference_position109 = source_ml_source2_array_read_index81.clone(); let ml_reference_projection109 = self::ml_array::at(&*ml_reference_array109.borrow(), ml_reference_position109.to_index()); ml_reference_projection109 }
                    }
                };
                if ((((fact / span)).floor()) == (2f64)) {
                    {
                        let value = (fact % span);
                        {
                            let owner = ((value / (field_width * width))).floor();
                            {
                                let actual = (((value / width)).floor() % field_width);
                                if ((owner == root) && ((actual == (0f64)) || (actual == field))) {
                                    {
                                        let result_2 = self::add_value(result.clone(), ((value % width) - 1f64));
                                        {
                                            let source_ml_source1_temporary_index21_2 = source_ml_source1_temporary_index21.add(&self::ml::Big::from_i128(1));
                                            {
                                                (result, source_ml_source1_temporary_index21) = (result_2.clone(), source_ml_source1_temporary_index21_2.clone());
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let source_ml_source1_temporary_index21_3 = source_ml_source1_temporary_index21.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index21 = source_ml_source1_temporary_index21_3.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index21_4 = source_ml_source1_temporary_index21.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index21 = source_ml_source1_temporary_index21_4.clone();
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

pub fn source_ml_source1_temporary_read_field_loop22(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut field: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values20: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index20: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index20 < ({ let ml_reference_array110 = source_ml_source1_temporary_values20.clone(); let ml_reference_projection110 = self::ml::Big::from_u128(ml_reference_array110.borrow().len() as u128); ml_reference_projection110 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value82 = source_ml_source1_temporary_values20.clone();
                    {
                        let source_ml_source2_array_read_index83 = source_ml_source1_temporary_index20.clone();
                        { let ml_reference_array111 = source_ml_source2_array_read_value82.clone(); let ml_reference_position111 = source_ml_source2_array_read_index83.clone(); let ml_reference_projection111 = self::ml_array::at(&*ml_reference_array111.borrow(), ml_reference_position111.to_index()); ml_reference_projection111 }
                    }
                };
                if (root < (0f64)) {
                    {
                        let result_2 = self::add_value(result.clone(), (-1f64));
                        {
                            let source_ml_source1_temporary_index20_2 = source_ml_source1_temporary_index20.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index20) = (result_2.clone(), source_ml_source1_temporary_index20_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_values21 = facts.clone();
                        {
                            let source_ml_source1_temporary_index21 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_source1_temporary_read_field_loop23(field, field_width, width, span, result.clone(), root, source_ml_source1_temporary_values21.clone(), source_ml_source1_temporary_index21.clone());
                                {
                                    let source_ml_source1_temporary_index20_3 = source_ml_source1_temporary_index20.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index20) = (result_3.clone(), source_ml_source1_temporary_index20_3.clone());
                                        continue;
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

pub fn source_ml_source1_temporary_add_field_loop27(mut field: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut owner: f64, mut source_ml_source1_temporary_values25: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index25: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index25 < ({ let ml_reference_array112 = source_ml_source1_temporary_values25.clone(); let ml_reference_projection112 = self::ml::Big::from_u128(ml_reference_array112.borrow().len() as u128); ml_reference_projection112 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value84 = source_ml_source1_temporary_values25.clone();
                    {
                        let source_ml_source2_array_read_index85 = source_ml_source1_temporary_index25.clone();
                        { let ml_reference_array113 = source_ml_source2_array_read_value84.clone(); let ml_reference_position113 = source_ml_source2_array_read_index85.clone(); let ml_reference_projection113 = self::ml_array::at(&*ml_reference_array113.borrow(), ml_reference_position113.to_index()); ml_reference_projection113 }
                    }
                };
                {
                    let result_2 = self::add_fact(result.clone(), ((((2f64 * span) + (((owner * field_width) + field) * width)) + root) + 1f64));
                    {
                        let source_ml_source1_temporary_index25_2 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index25) = (result_2.clone(), source_ml_source1_temporary_index25_2.clone());
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

pub fn source_ml_source1_temporary_add_field_loop26(mut field: f64, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values24: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index24: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index24 < ({ let ml_reference_array114 = source_ml_source1_temporary_values24.clone(); let ml_reference_projection114 = self::ml::Big::from_u128(ml_reference_array114.borrow().len() as u128); ml_reference_projection114 })) {
            {
                let owner = {
                    let source_ml_source2_array_read_value86 = source_ml_source1_temporary_values24.clone();
                    {
                        let source_ml_source2_array_read_index87 = source_ml_source1_temporary_index24.clone();
                        { let ml_reference_array115 = source_ml_source2_array_read_value86.clone(); let ml_reference_position115 = source_ml_source2_array_read_index87.clone(); let ml_reference_projection115 = self::ml_array::at(&*ml_reference_array115.borrow(), ml_reference_position115.to_index()); ml_reference_projection115 }
                    }
                };
                if (owner >= (0f64)) {
                    {
                        let source_ml_source1_temporary_values25 = roots.clone();
                        {
                            let source_ml_source1_temporary_index25 = self::ml::Big::from_i128(0);
                            {
                                let result_2 = self::source_ml_source1_temporary_add_field_loop27(field, field_width, width, span, result.clone(), owner, source_ml_source1_temporary_values25.clone(), source_ml_source1_temporary_index25.clone());
                                {
                                    let source_ml_source1_temporary_index24_2 = source_ml_source1_temporary_index24.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index24) = (result_2.clone(), source_ml_source1_temporary_index24_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index24_3 = source_ml_source1_temporary_index24.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index24 = source_ml_source1_temporary_index24_3.clone();
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

pub fn source_ml_source1_temporary_copy_fields_loop31(mut destination: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut root: f64, mut source_ml_source1_temporary_values29: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index29: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index29 < ({ let ml_reference_array116 = source_ml_source1_temporary_values29.clone(); let ml_reference_projection116 = self::ml::Big::from_u128(ml_reference_array116.borrow().len() as u128); ml_reference_projection116 })) {
            {
                let fact = {
                    let source_ml_source2_array_read_value88 = source_ml_source1_temporary_values29.clone();
                    {
                        let source_ml_source2_array_read_index89 = source_ml_source1_temporary_index29.clone();
                        { let ml_reference_array117 = source_ml_source2_array_read_value88.clone(); let ml_reference_position117 = source_ml_source2_array_read_index89.clone(); let ml_reference_projection117 = self::ml_array::at(&*ml_reference_array117.borrow(), ml_reference_position117.to_index()); ml_reference_projection117 }
                    }
                };
                if ((((fact / span)).floor()) == (2f64)) {
                    {
                        let value = (fact % span);
                        if ((((value / (field_width * width))).floor()) == root) {
                            {
                                let result_2 = self::add_fact(result.clone(), (((2f64 * span) + ((destination * field_width) * width)) + (value % (field_width * width))));
                                {
                                    let source_ml_source1_temporary_index29_2 = source_ml_source1_temporary_index29.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index29) = (result_2.clone(), source_ml_source1_temporary_index29_2.clone());
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_temporary_index29_3 = source_ml_source1_temporary_index29.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index29 = source_ml_source1_temporary_index29_3.clone();
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index29_4 = source_ml_source1_temporary_index29.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index29 = source_ml_source1_temporary_index29_4.clone();
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

pub fn source_ml_source1_temporary_copy_fields_loop30(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut destination: f64, mut field_width: f64, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values28: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index28: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index28 < ({ let ml_reference_array118 = source_ml_source1_temporary_values28.clone(); let ml_reference_projection118 = self::ml::Big::from_u128(ml_reference_array118.borrow().len() as u128); ml_reference_projection118 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value90 = source_ml_source1_temporary_values28.clone();
                    {
                        let source_ml_source2_array_read_index91 = source_ml_source1_temporary_index28.clone();
                        { let ml_reference_array119 = source_ml_source2_array_read_value90.clone(); let ml_reference_position119 = source_ml_source2_array_read_index91.clone(); let ml_reference_projection119 = self::ml_array::at(&*ml_reference_array119.borrow(), ml_reference_position119.to_index()); ml_reference_projection119 }
                    }
                };
                if (root < (0f64)) {
                    {
                        let result_2 = self::add_field(result.clone(), std::rc::Rc::new(std::cell::RefCell::new(vec![destination])), 0f64, std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])), field_width, width, span);
                        {
                            let source_ml_source1_temporary_index28_2 = source_ml_source1_temporary_index28.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index28) = (result_2.clone(), source_ml_source1_temporary_index28_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_values29 = facts.clone();
                        {
                            let source_ml_source1_temporary_index29 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_source1_temporary_copy_fields_loop31(destination, field_width, width, span, result.clone(), root, source_ml_source1_temporary_values29.clone(), source_ml_source1_temporary_index29.clone());
                                {
                                    let source_ml_source1_temporary_index28_3 = source_ml_source1_temporary_index28.add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index28) = (result_3.clone(), source_ml_source1_temporary_index28_3.clone());
                                        continue;
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReachableLoop35Result {
    MlSource1TemporaryReachableLoop35Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReachableLoop36Result {
    MlSource1TemporaryReachableLoop36Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
}

pub fn source_ml_source1_temporary_reachable_loop36(mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut progress: bool, mut source_ml_source1_temporary_values33: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index33: self::ml::Big) -> self::MlSource1TemporaryReachableLoop36Result {
    loop {
        return if (source_ml_source1_temporary_index33 < ({ let ml_reference_array120 = source_ml_source1_temporary_values33.clone(); let ml_reference_projection120 = self::ml::Big::from_u128(ml_reference_array120.borrow().len() as u128); ml_reference_projection120 })) {
            {
                let target = {
                    let source_ml_source2_array_read_value92 = source_ml_source1_temporary_values33.clone();
                    {
                        let source_ml_source2_array_read_index93 = source_ml_source1_temporary_index33.clone();
                        { let ml_reference_array121 = source_ml_source2_array_read_value92.clone(); let ml_reference_position121 = source_ml_source2_array_read_index93.clone(); let ml_reference_projection121 = self::ml_array::at(&*ml_reference_array121.borrow(), ml_reference_position121.to_index()); ml_reference_projection121 }
                    }
                };
                if !(self::ml_array::includes(&{ let ml_reference_value122 = result.borrow().clone(); ml_reference_value122 }, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                    {
                        let result_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value94 = result.clone();
                            {
                                let source_ml_source2_concat_value95 = std::rc::Rc::new(std::cell::RefCell::new(vec![target]));
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value123 = source_ml_source2_concat_value94.borrow().clone(); ml_reference_value123 }), { let ml_reference_value124 = source_ml_source2_concat_value95.borrow().clone(); ml_reference_value124 })
                            }
                        }));
                        {
                            let progress_2 = true;
                            {
                                let source_ml_source1_temporary_index33_2 = source_ml_source1_temporary_index33.add(&self::ml::Big::from_i128(1));
                                {
                                    (result, progress, source_ml_source1_temporary_index33) = (result_2.clone(), progress_2, source_ml_source1_temporary_index33_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index33_3 = source_ml_source1_temporary_index33.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index33 = source_ml_source1_temporary_index33_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReachableLoop36Result::MlSource1TemporaryReachableLoop36Done(result.clone(), progress)
        };
    }
}

pub fn source_ml_source1_temporary_reachable_loop35(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut progress: bool, mut source_ml_source1_temporary_values32: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index32: self::ml::Big) -> self::MlSource1TemporaryReachableLoop35Result {
    loop {
        return if (source_ml_source1_temporary_index32 < ({ let ml_reference_array125 = source_ml_source1_temporary_values32.clone(); let ml_reference_projection125 = self::ml::Big::from_u128(ml_reference_array125.borrow().len() as u128); ml_reference_projection125 })) {
            {
                let owner = {
                    let source_ml_source2_array_read_value96 = source_ml_source1_temporary_values32.clone();
                    {
                        let source_ml_source2_array_read_index97 = source_ml_source1_temporary_index32.clone();
                        { let ml_reference_array126 = source_ml_source2_array_read_value96.clone(); let ml_reference_position126 = source_ml_source2_array_read_index97.clone(); let ml_reference_projection126 = self::ml_array::at(&*ml_reference_array126.borrow(), ml_reference_position126.to_index()); ml_reference_projection126 }
                    }
                };
                {
                    let source_ml_source1_temporary_values33 = self::called(facts.clone(), owner, width, span);
                    {
                        let source_ml_source1_temporary_index33 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s98 = self::source_ml_source1_temporary_reachable_loop36(result.clone(), progress, source_ml_source1_temporary_values33.clone(), source_ml_source1_temporary_index33.clone());
                            match source_ml_source2_s98.clone() {
                                self::MlSource1TemporaryReachableLoop36Result::MlSource1TemporaryReachableLoop36Done(result_2, progress_2) => {
                                    {
                                        let source_ml_source1_temporary_index32_2 = source_ml_source1_temporary_index32.add(&self::ml::Big::from_i128(1));
                                        {
                                            (result, progress, source_ml_source1_temporary_index32) = (result_2.clone(), progress_2, source_ml_source1_temporary_index32_2.clone());
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
            self::MlSource1TemporaryReachableLoop35Result::MlSource1TemporaryReachableLoop35Done(result.clone(), progress)
        };
    }
}

pub fn source_ml_source1_temporary_reachable_loop34(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut span: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut progress: bool) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if progress {
            {
                let progress_2 = false;
                {
                    let source_ml_source1_temporary_values32 = result.clone();
                    {
                        let source_ml_source1_temporary_index32 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s99 = self::source_ml_source1_temporary_reachable_loop35(facts.clone(), width, span, result.clone(), progress_2, source_ml_source1_temporary_values32.clone(), source_ml_source1_temporary_index32.clone());
                            match source_ml_source2_s99.clone() {
                                self::MlSource1TemporaryReachableLoop35Result::MlSource1TemporaryReachableLoop35Done(result_2, progress_3) => {
                                    {
                                        (result, progress) = (result_2.clone(), progress_3);
                                        continue;
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
pub enum MlSource1TemporaryAcceptsEffectsLoop39Result {
    MlSource1TemporaryAcceptsEffectsLoop39Done,
    MlSource1TemporaryAcceptsEffectsLoop39Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsJoin40 {
    MlSource1TemporaryAcceptsEffectsJoin40Next,
    MlSource1TemporaryAcceptsEffectsJoin40Return(bool),
}

pub fn source_ml_source1_temporary_accepts_effects_loop39(mut facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source: f64, mut width: f64, mut span: f64, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut reached: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> self::MlSource1TemporaryAcceptsEffectsLoop39Result {
    loop {
        return if (node < ({ let ml_reference_array127 = roles.clone(); let ml_reference_projection127 = ml_reference_array127.borrow().len() as f64; ml_reference_projection127 })) {
            if self::ml_array::includes(&{ let ml_reference_value128 = reached.borrow().clone(); ml_reference_value128 }, &self::read_required_number_fact(owners.clone(), node), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                if (((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))) || ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 32, 119, 114, 105, 116, 101])))) {
                    self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                } else {
                    {
                        let source_ml_source2_s100 = if (((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])))) {
                            {
                                let actual = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101]));
                                {
                                    let callees = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let callees_4 = if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                            {
                                                let callees_2 = actual.clone();
                                                callees_2.clone()
                                            }
                                        } else {
                                            if (({ let ml_reference_array129 = actual.clone(); let ml_reference_projection129 = ml_reference_array129.borrow().len() as f64; ml_reference_projection129 }) != (0f64)) {
                                                {
                                                    let callees_3 = self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(actual.clone(), 0f64), width, span);
                                                    callees_3.clone()
                                                }
                                            } else {
                                                callees.clone()
                                            }
                                        };
                                        if ((({ let ml_reference_array130 = callees_4.clone(); let ml_reference_projection130 = ml_reference_array130.borrow().len() as f64; ml_reference_projection130 }) == (0f64)) || { let ml_values131 = { let ml_reference_value132 = callees_4.borrow().clone(); ml_reference_value132 }; let ml_callback131 = { let functions = functions.clone(); std::rc::Rc::new(move |target: f64| -> bool { !(self::ml_array::includes(&{ let ml_reference_value133 = functions.borrow().clone(); ml_reference_value133 }, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values131.iter().cloned().enumerate().any(|(ml_index131, ml_value131)| ml_callback131(ml_value131)) }) {
                                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Return(false)
                                        } else {
                                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next
                        };
                        match source_ml_source2_s100.clone() {
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next => {
                                if ((self::text_fact(roles.clone(), node)) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                                    {
                                        let object = self::first_number(self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                        {
                                            let effect = self::read_disjoint_collection_iteration_method_effect(self::text_fact(kinds.clone(), object), self::text_fact(operations.clone(), node));
                                            if (effect == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))) {
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                            } else {
                                                if ((effect == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && !(self::ml_array::includes(&{ let ml_reference_value134 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![99, 101, 108, 108])]))).borrow().clone(); ml_reference_value134 }, &self::text_fact(kinds.clone(), object), 0f64, false, |a, b| a == b))) {
                                                    self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                                } else {
                                                    if ((effect == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && (((self::text_fact(kinds.clone(), object)) == (self::text_fact(kinds.clone(), source))) && !(self::separate_roots(roots.clone(), self::read_roots(facts.clone(), 1f64, object, width, span))))) {
                                                        self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                                    } else {
                                                        {
                                                            let arguments_ = self::edges(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116]));
                                                            if (self::read_disjoint_collection_iteration_callback(self::text_fact(operations.clone(), node)) && (({ let ml_reference_array135 = arguments_.clone(); let ml_reference_projection135 = ml_reference_array135.borrow().len() as f64; ml_reference_projection135 }) != (0f64))) {
                                                                {
                                                                    let callees_5 = self::read_roots(facts.clone(), 1f64, self::read_required_number_fact(arguments_.clone(), 0f64), width, span);
                                                                    if ((({ let ml_reference_array136 = callees_5.clone(); let ml_reference_projection136 = ml_reference_array136.borrow().len() as f64; ml_reference_projection136 }) == (0f64)) || { let ml_values137 = { let ml_reference_value138 = callees_5.borrow().clone(); ml_reference_value138 }; let ml_callback137 = { let functions = functions.clone(); std::rc::Rc::new(move |target: f64| -> bool { !(self::ml_array::includes(&{ let ml_reference_value139 = functions.borrow().clone(); ml_reference_value139 }, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values137.iter().cloned().enumerate().any(|(ml_index137, ml_value137)| ml_callback137(ml_value137)) }) {
                                                                        self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                                                    } else {
                                                                        {
                                                                            let node_2 = (node + 1f64);
                                                                            {
                                                                                node = node_2;
                                                                                continue;
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
                            }
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Return(source_ml_source1_temporary_result40) => {
                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(source_ml_source1_temporary_result40)
                            }
                        }
                    }
                }
            } else {
                {
                    let node_5 = (node + 1f64);
                    {
                        node = node_5;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsLoop38Result {
    MlSource1TemporaryAcceptsEffectsLoop38Done,
    MlSource1TemporaryAcceptsEffectsLoop38Return(bool),
}

pub fn source_ml_source1_temporary_accepts_effects_loop38(mut functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut main_: f64, mut source_ml_source1_temporary_values37: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index37: self::ml::Big) -> self::MlSource1TemporaryAcceptsEffectsLoop38Result {
    loop {
        return if (source_ml_source1_temporary_index37 < ({ let ml_reference_array140 = source_ml_source1_temporary_values37.clone(); let ml_reference_projection140 = self::ml::Big::from_u128(ml_reference_array140.borrow().len() as u128); ml_reference_projection140 })) {
            {
                let scope = {
                    let source_ml_source2_array_read_value101 = source_ml_source1_temporary_values37.clone();
                    {
                        let source_ml_source2_array_read_index102 = source_ml_source1_temporary_index37.clone();
                        { let ml_reference_array141 = source_ml_source2_array_read_value101.clone(); let ml_reference_position141 = source_ml_source2_array_read_index102.clone(); let ml_reference_projection141 = self::ml_array::at(&*ml_reference_array141.borrow(), ml_reference_position141.to_index()); ml_reference_projection141 }
                    }
                };
                {
                    let position = self::ml_array::index_of(&{ let ml_reference_value142 = functions.borrow().clone(); ml_reference_value142 }, &scope, 0f64, false, |a, b| a == b);
                    if ((position < (0f64)) || (((self::read_required_number_fact(bodies.clone(), position)) < (0f64)) && (scope != main_))) {
                        self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Return(false)
                    } else {
                        {
                            let source_ml_source1_temporary_index37_2 = source_ml_source1_temporary_index37.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index37 = source_ml_source1_temporary_index37_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Done
        };
    }
}

pub fn source_ml_source1_temporary_bind_calls_loop45(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values42: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index42: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index42 < ({ let ml_reference_array143 = source_ml_source1_temporary_values42.clone(); let ml_reference_projection143 = self::ml::Big::from_u128(ml_reference_array143.borrow().len() as u128); ml_reference_projection143 })) {
            {
                let root = {
                    let source_ml_source2_array_read_value103 = source_ml_source1_temporary_values42.clone();
                    {
                        let source_ml_source2_array_read_index104 = source_ml_source1_temporary_index42.clone();
                        { let ml_reference_array144 = source_ml_source2_array_read_value103.clone(); let ml_reference_position144 = source_ml_source2_array_read_index104.clone(); let ml_reference_projection144 = self::ml_array::at(&*ml_reference_array144.borrow(), ml_reference_position144.to_index()); ml_reference_projection144 }
                    }
                };
                {
                    let roots_2 = self::add_value(roots.clone(), root);
                    {
                        let source_ml_source1_temporary_index42_2 = source_ml_source1_temporary_index42.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index42) = (roots_2.clone(), source_ml_source1_temporary_index42_2.clone());
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
