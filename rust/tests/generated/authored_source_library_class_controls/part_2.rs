pub fn render_source_generated_identifier(prefix: self::ml_string::Text, role: self::ml_string::Text, ordinal: f64) -> self::ml_string::Text {
    if (!(self::ml_math::is_safe_integer(ordinal)) || (ordinal < (0f64))) {
        panic!("{}", "source identifier generation ordinal exceeds its exact range")
    } else {
        (prefix.concat(&if role.starts_with(&self::ml_string::Text::from_units(vec![109, 108, 95])) {
            {
                let source_ml_source2_string_range_operand56 = role.clone();
                {
                    let source_ml_source2_string_range_operand57 = 3f64;
                    {
                        let source_ml_source2_string_range_position58 = source_ml_source2_string_range_operand57;
                        {
                            let source_ml_source2_string_range_position59 = f64::INFINITY;
                            source_ml_source2_string_range_operand56.slice(source_ml_source2_string_range_position58, source_ml_source2_string_range_position59)
                        }
                    }
                }
            }
        } else {
            role.clone()
        })).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(ordinal)))
    }
}

pub fn render_source_cell_parameter(prefix: self::ml_string::Text, name: self::ml_string::Text, reference: bool) -> self::ml_string::Text {
    if reference {
        (prefix.concat(&self::ml_string::Text::from_units(vec![99, 101, 108, 108, 95]))).concat(&name)
    } else {
        name.clone()
    }
}

pub fn dependency_function1(kind: self::ml_string::Text, name: self::ml_string::Text, duplicate: bool, reserved: Vec<self::ml_string::Text>) -> bool {
    if (((kind != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) || (name == (self::ml_string::Text::from_units(vec![])))) || duplicate) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s60 = self::source_ml_source1_temporary_dependency_function1_loop3(name.clone(), reserved, index);
                match source_ml_source2_s60.clone() {
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Done => {
                        true
                    }
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
                    }
                }
            }
        }
    }
}

pub fn read_source_library_declaration_binding_kind(keyword: self::ml_string::Text, kind: self::ml_string::Text, name: self::ml_string::Text, callable: bool) -> self::ml_string::Text {
    if !(self::dependency_function1(kind.clone(), name.clone(), false, Vec::<self::ml_string::Text>::new())) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (keyword == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])
        } else {
            if ((keyword == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]))) && callable) {
                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn accept_java_script_source_function_head_name(kind: self::ml_string::Text) -> bool {
    (kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))
}

pub fn read_source_library_function_declaration_offset(async_: self::ml_string::Text, second: self::ml_string::Text, line_break: bool) -> f64 {
    if (async_ == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
        0f64
    } else {
        if (((async_ == (self::ml_string::Text::from_units(vec![97, 115, 121, 110, 99]))) && (second == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) && !line_break) {
            1f64
        } else {
            (-1f64)
        }
    }
}

pub fn test_java_script_source_function_expression_term(term: self::ml_string::Text) -> bool {
    ((term == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 95, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110]))) || (term == (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 97, 116, 111, 114, 95, 102, 117, 110, 99, 116, 105, 111, 110]))))
}

pub fn test_java_script_source_keyword_role(identifier: bool, keyword: bool) -> bool {
    (!identifier && keyword)
}

pub fn read_java_script_source_function_scope_form(term: self::ml_string::Text) -> self::ml_string::Text {
    if self::test_java_script_source_function_expression_term(term.clone()) {
        self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110])
    } else {
        if ((term == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))) || (term == (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 97, 116, 111, 114, 95, 102, 117, 110, 99, 116, 105, 111, 110, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])))) {
            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])
        } else {
            if (term == (self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 95, 102, 117, 110, 99, 116, 105, 111, 110]))) {
                self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 102, 117, 110, 99, 116, 105, 111, 110])
            } else {
                if (term == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 95, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))) {
                    self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn accept_java_script_source_simple_parameter_list(terms: Vec<self::ml_string::Text>) -> bool {
    {
        let source_ml_source1_temporary_values4 = terms;
        {
            let source_ml_source1_temporary_index4 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s61 = self::source_ml_source1_temporary_accept_java_script_source_simple_parameter_list_loop5(source_ml_source1_temporary_values4, source_ml_source1_temporary_index4.clone());
                match source_ml_source2_s61.clone() {
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(source_ml_source1_temporary_result5) => {
                        source_ml_source1_temporary_result5
                    }
                }
            }
        }
    }
}

pub fn test_java_script_source_parameter_variable_dependency(first: self::ml_string::Text, second: self::ml_string::Text, body_parent: bool) -> bool {
    (body_parent && (((first == (self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114]))) && (second == (self::ml_string::Text::from_units(vec![118, 97, 114])))) || ((first == (self::ml_string::Text::from_units(vec![118, 97, 114]))) && (second == (self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114]))))))
}

pub fn test_java_script_source_variable_redeclaration(kind: self::ml_string::Text, existing: bool) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))) && existing)
}

pub fn test_java_script_source_loop_scope_term(term: self::ml_string::Text) -> bool {
    (((term == (self::ml_string::Text::from_units(vec![102, 111, 114, 95, 115, 116, 97, 116, 101, 109, 101, 110, 116]))) || (term == (self::ml_string::Text::from_units(vec![102, 111, 114, 95, 105, 110, 95, 115, 116, 97, 116, 101, 109, 101, 110, 116])))) || (term == (self::ml_string::Text::from_units(vec![102, 111, 114, 95, 111, 102, 95, 115, 116, 97, 116, 101, 109, 101, 110, 116]))))
}

pub fn test_java_script_source_catch_scope_term(term: self::ml_string::Text) -> bool {
    (term == (self::ml_string::Text::from_units(vec![99, 97, 116, 99, 104, 95, 99, 108, 97, 117, 115, 101])))
}

pub fn test_java_script_source_class_expression_term(term: self::ml_string::Text) -> bool {
    (term == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))
}

pub fn test_java_script_source_variable_environment_term(term: self::ml_string::Text) -> bool {
    ((term == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95, 98, 108, 111, 99, 107]))) || ((self::read_java_script_source_function_scope_form(term.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn test_rust_source_opaque_literal_term(term: self::ml_string::Text) -> bool {
    ((((term == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 95, 108, 105, 116, 101, 114, 97, 108]))) || (term == (self::ml_string::Text::from_units(vec![114, 97, 119, 95, 115, 116, 114, 105, 110, 103, 95, 108, 105, 116, 101, 114, 97, 108])))) || (term == (self::ml_string::Text::from_units(vec![99, 104, 97, 114, 97, 99, 116, 101, 114, 95, 108, 105, 116, 101, 114, 97, 108])))) || (term == (self::ml_string::Text::from_units(vec![99, 104, 97, 114, 95, 108, 105, 116, 101, 114, 97, 108]))))
}

pub fn test_java_script_source_opaque_literal_term(term: self::ml_string::Text) -> bool {
    (((term == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (term == (self::ml_string::Text::from_units(vec![114, 101, 103, 117, 108, 97, 114, 95, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110])))) || (term == (self::ml_string::Text::from_units(vec![114, 101, 103, 101, 120]))))
}

pub fn decode_java_script_identifier_escape(units: Vec<f64>, start: f64) -> Vec<f64> {
    if ((((!(self::ml_math::is_safe_integer(start)) || (start < (0f64))) || (((start + 2f64)) > ((units.len() as f64)))) || {
        let source_ml_source2_comparison_operand62 = self::ml_array::optional_at(&units, self::ml_array::number_index(start));
        {
            let source_ml_source2_comparison_operand63 = 92f64;
            !(match (source_ml_source2_comparison_operand62.clone(), source_ml_source2_comparison_operand63) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) || {
        let source_ml_source2_comparison_operand64 = self::ml_array::optional_at(&units, self::ml_array::number_index((start + 1f64)));
        {
            let source_ml_source2_comparison_operand65 = 117f64;
            !(match (source_ml_source2_comparison_operand64.clone(), source_ml_source2_comparison_operand65) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) {
        vec![(-1f64), self::ml_math::min((units.len() as f64), (start + 2f64))]
    } else {
        {
            let braced = {
                let source_ml_source2_comparison_operand66 = self::ml_array::optional_at(&units, self::ml_array::number_index((start + 2f64)));
                {
                    let source_ml_source2_comparison_operand67 = 123f64;
                    match (source_ml_source2_comparison_operand66.clone(), source_ml_source2_comparison_operand67) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }
            };
            {
                let index = (start + if braced {
                    3f64
                } else {
                    2f64
                });
                {
                    let digits = 0f64;
                    {
                        let scalar = 0f64;
                        {
                            let source_ml_source2_s68 = self::source_ml_source1_temporary_decode_java_script_identifier_escape_loop6(units.clone(), braced, index, digits, scalar);
                            match source_ml_source2_s68.clone() {
                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Done(index_2, digits_2, scalar_2) => {
                                    {
                                        let source_ml_source2_s71 = if braced {
                                            if ((digits_2 == (0f64)) || {
                                                let source_ml_source2_comparison_operand72 = self::ml_array::optional_at(&units, self::ml_array::number_index(index_2));
                                                {
                                                    let source_ml_source2_comparison_operand73 = 125f64;
                                                    !(match (source_ml_source2_comparison_operand72.clone(), source_ml_source2_comparison_operand73) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                }
                                            }) {
                                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Return(vec![(-1f64), self::ml_math::min((units.len() as f64), (index_2 + 1f64))])
                                            } else {
                                                {
                                                    let index_3 = (index_2 + 1f64);
                                                    self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Next(index_3)
                                                }
                                            }
                                        } else {
                                            if (digits_2 != (4f64)) {
                                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Return(vec![(-1f64), self::ml_math::min((units.len() as f64), (index_2 + 1f64))])
                                            } else {
                                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Next(index_2)
                                            }
                                        };
                                        match source_ml_source2_s71.clone() {
                                            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Next(index_4) => {
                                                if ((scalar_2 >= (55296f64)) && (scalar_2 <= (57343f64))) {
                                                    vec![(-1f64), index_4]
                                                } else {
                                                    vec![index_4, scalar_2]
                                                }
                                            }
                                            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Return(source_ml_source1_temporary_result7) => {
                                                source_ml_source1_temporary_result7
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Return(source_ml_source1_temporary_result7_2) => {
                                    source_ml_source1_temporary_result7_2
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_java_script_reserved_identifier(name: self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]), self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107]), self::ml_string::Text::from_units(vec![99, 97, 115, 101]), self::ml_string::Text::from_units(vec![99, 97, 116, 99, 104]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]), self::ml_string::Text::from_units(vec![100, 101, 98, 117, 103, 103, 101, 114]), self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]), self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![100, 111]), self::ml_string::Text::from_units(vec![101, 108, 115, 101]), self::ml_string::Text::from_units(vec![101, 110, 117, 109]), self::ml_string::Text::from_units(vec![101, 120, 112, 111, 114, 116]), self::ml_string::Text::from_units(vec![101, 120, 116, 101, 110, 100, 115]), self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]), self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121]), self::ml_string::Text::from_units(vec![102, 111, 114]), self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![105, 102]), self::ml_string::Text::from_units(vec![105, 109, 112, 111, 114, 116]), self::ml_string::Text::from_units(vec![105, 110]), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![110, 117, 108, 108]), self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]), self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]), self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104]), self::ml_string::Text::from_units(vec![116, 104, 105, 115]), self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]), self::ml_string::Text::from_units(vec![116, 114, 117, 101]), self::ml_string::Text::from_units(vec![116, 114, 121]), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]), self::ml_string::Text::from_units(vec![118, 97, 114]), self::ml_string::Text::from_units(vec![118, 111, 105, 100]), self::ml_string::Text::from_units(vec![119, 104, 105, 108, 101]), self::ml_string::Text::from_units(vec![119, 105, 116, 104]), self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![108, 101, 116]), self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]), self::ml_string::Text::from_units(vec![105, 109, 112, 108, 101, 109, 101, 110, 116, 115]), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 114, 102, 97, 99, 101]), self::ml_string::Text::from_units(vec![112, 97, 99, 107, 97, 103, 101]), self::ml_string::Text::from_units(vec![112, 114, 105, 118, 97, 116, 101]), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 101, 99, 116, 101, 100]), self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 99])], &name.clone(), 0f64, false, |a, b| a == b)
}

pub fn accept_java_script_source_keyword_token(name: self::ml_string::Text, raw_: self::ml_string::Text) -> bool {
    (!(raw_.contains(&self::ml_string::Text::from_units(vec![92]))) || !((self::accept_java_script_reserved_identifier(name.clone()) || self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 115]), self::ml_string::Text::from_units(vec![97, 115, 121, 110, 99]), self::ml_string::Text::from_units(vec![102, 114, 111, 109]), self::ml_string::Text::from_units(vec![103, 101, 116]), self::ml_string::Text::from_units(vec![109, 101, 116, 97]), self::ml_string::Text::from_units(vec![111, 102]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])], &name.clone(), 0f64, false, |a, b| a == b))))
}

pub fn read_java_script_source_token_goal_value(kind: self::ml_string::Text, name: self::ml_string::Text, raw_: self::ml_string::Text) -> self::ml_string::Text {
    if ((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && !(self::accept_java_script_source_keyword_token(name.clone(), raw_.clone()))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        name.clone()
    }
}

pub fn read_java_script_escaped_identifier_error(name: self::ml_string::Text, raw_: self::ml_string::Text, binding: bool) -> self::ml_string::Text {
    if !(raw_.contains(&self::ml_string::Text::from_units(vec![92]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if self::accept_java_script_reserved_identifier(name.clone()) {
            (self::ml_string::Text::from_units(vec![101, 115, 99, 97, 112, 101, 100, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 32, 105, 115, 32, 114, 101, 115, 101, 114, 118, 101, 100, 58, 32])).concat(&name)
        } else {
            if (binding && ((name == (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115]))) || (name == (self::ml_string::Text::from_units(vec![101, 118, 97, 108]))))) {
                (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 115, 116, 114, 105, 99, 116, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 58, 32])).concat(&name)
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn accept_java_script_binding_identifier_context(context: self::ml_string::Text) -> bool {
    self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 98, 105, 110, 100, 105, 110, 103]), self::ml_string::Text::from_units(vec![110, 97, 109, 101, 115, 112, 97, 99, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]), self::ml_string::Text::from_units(vec![108, 101, 116]), self::ml_string::Text::from_units(vec![102, 111, 114, 8230, 111, 102])], &context.clone(), 0f64, false, |a, b| a == b)
}

pub fn accept_java_script_number_identifier_boundary(identifier: bool, following: f64) -> bool {
    ((!identifier && (following != (92f64))) && !(((following >= (48f64)) && (following <= (57f64)))))
}

pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

pub fn dependency_function2(values: Vec<f64>, index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

pub fn dependency_function3(values: Vec<self::ml_string::Text>, index: f64) -> self::ml_string::Text {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

pub fn read_required_boolean_fact(values: Vec<bool>, index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

pub fn read_source_library_class_profile_error(profile: self::ml_string::Text, dependencies: bool, aliases: bool, shared: bool) -> self::ml_string::Text {
    if (profile == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if ((((profile == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]))) && dependencies) && aliases) && shared) {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115])
        }
    }
}

pub fn read_source_library_re_export_profile_error(profile: self::ml_string::Text, dependencies: bool, aliases: bool) -> self::ml_string::Text {
    if ((profile == (self::ml_string::Text::from_units(vec![]))) || (((profile == (self::ml_string::Text::from_units(vec![114, 101, 115, 111, 108, 118, 101, 100, 32, 110, 97, 109, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 112, 114, 111, 118, 105, 100, 101, 114, 115, 32, 49]))) && dependencies) && aliases)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100, 32, 114, 101, 45, 101, 120, 112, 111, 114, 116, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 97, 110, 100, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 101, 115])
    }
}

pub fn accept_source_library_re_export_name(kind: self::ml_string::Text, name: self::ml_string::Text) -> bool {
    (((self::dependency_function1(kind.clone(), name.clone(), false, Vec::<self::ml_string::Text>::new()) && !(self::accept_java_script_reserved_identifier(name.clone()))) && (name != (self::ml_string::Text::from_units(vec![101, 118, 97, 108])))) && (name != (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115]))))
}

pub fn accept_source_library_entry_export_owner(exported: self::ml_string::Text, compiled: self::ml_string::Text) -> bool {
    ((exported != (self::ml_string::Text::from_units(vec![]))) && (exported == compiled))
}

pub fn read_source_library_class_binding_kind(keyword: self::ml_string::Text, kind: self::ml_string::Text, name: self::ml_string::Text, callable: bool) -> self::ml_string::Text {
    if (((((keyword == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) && self::dependency_function1(kind.clone(), name.clone(), false, Vec::<self::ml_string::Text>::new())) && !(self::accept_java_script_reserved_identifier(name.clone()))) && (name != (self::ml_string::Text::from_units(vec![101, 118, 97, 108])))) && (name != (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115])))) {
        self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])
    } else {
        self::read_source_library_declaration_binding_kind(keyword.clone(), kind.clone(), name.clone(), callable)
    }
}

pub fn accept_source_library_class_export(keyword: self::ml_string::Text, kind: self::ml_string::Text, name: self::ml_string::Text, callable: bool) -> bool {
    ((self::read_source_library_class_binding_kind(keyword.clone(), kind.clone(), name.clone(), callable)) != (self::ml_string::Text::from_units(vec![])))
}

pub fn accept_source_library_class_declaration_term(term: self::ml_string::Text, kind: self::ml_string::Text) -> bool {
    if (kind == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) {
        (term == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])))
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
            ((term == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))) || (term == (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 97, 116, 111, 114, 95, 102, 117, 110, 99, 116, 105, 111, 110, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))))
        } else {
            ((kind == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]))) && (term == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))))
        }
    }
}

pub fn read_source_library_class_declaration_owner(names: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, witnessed: Vec<self::ml_string::Text>, first: f64, last: f64) -> self::ml_string::Text {
    {
        let owner = self::ml_string::Text::from_units(vec![]);
        if ((((((names.len() as f64)) != ((starts.len() as f64))) || (((names.len() as f64)) != ((ends.len() as f64)))) || (first < (0f64))) || (last < first)) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s74 = self::source_ml_source1_temporary_read_source_library_class_declaration_owner_loop8(names, starts, ends, witnessed, first, last, owner.clone(), at);
                    match source_ml_source2_s74.clone() {
                        self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(owner_2) => {
                            owner_2.clone()
                        }
                        self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(source_ml_source1_temporary_result8) => {
                            source_ml_source1_temporary_result8.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_source_library_class_helper_call(imported_owners: Vec<self::ml_string::Text>, constructor: self::ml_string::Text, helpers: Vec<self::ml_string::Text>, helper: self::ml_string::Text, receiver_owner: self::ml_string::Text) -> bool {
    ((((constructor != (self::ml_string::Text::from_units(vec![]))) && (constructor == receiver_owner)) && self::ml_array::includes(&imported_owners, &constructor.clone(), 0f64, false, |a, b| a == b)) && self::ml_array::includes(&helpers, &helper.clone(), 0f64, false, |a, b| a == b))
}

pub fn accept_source_library_class_static_call(imported_owners: Vec<self::ml_string::Text>, constructor: self::ml_string::Text, helpers: Vec<self::ml_string::Text>, helper: self::ml_string::Text) -> bool {
    (((constructor != (self::ml_string::Text::from_units(vec![]))) && self::ml_array::includes(&imported_owners, &constructor.clone(), 0f64, false, |a, b| a == b)) && self::ml_array::includes(&helpers, &helper.clone(), 0f64, false, |a, b| a == b))
}

pub fn project_source_library_class_edit_interval(starts: Vec<f64>, ends: Vec<f64>, lengths: Vec<f64>, first: f64, last: f64) -> Vec<f64> {
    if (((((((((starts.len() as f64)) != ((ends.len() as f64))) || (((starts.len() as f64)) != ((lengths.len() as f64)))) || (((starts.len() as f64)) > (32768f64))) || !(self::ml_math::is_safe_integer(first))) || !(self::ml_math::is_safe_integer(last))) || (first < (0f64))) || (last < first)) {
        Vec::<f64>::new()
    } else {
        {
            let delta = 0f64;
            {
                let previous = 0f64;
                {
                    let low = (-1f64);
                    {
                        let high = (-1f64);
                        {
                            let at = 0f64;
                            {
                                let source_ml_source2_s75 = self::source_ml_source1_temporary_project_source_library_class_edit_interval_loop9(starts, ends, lengths, first, last, delta, previous, low, high, at);
                                match source_ml_source2_s75.clone() {
                                    self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(delta_2, low_2, high_2) => {
                                        {
                                            let low_4 = if (low_2 < (0f64)) {
                                                {
                                                    let low_3 = (first - delta_2);
                                                    low_3
                                                }
                                            } else {
                                                low_2
                                            };
                                            {
                                                let high_4 = if (high_2 < (0f64)) {
                                                    {
                                                        let high_3 = (last - delta_2);
                                                        high_3
                                                    }
                                                } else {
                                                    high_2
                                                };
                                                if ((low_4 < (0f64)) || (high_4 < low_4)) {
                                                    Vec::<f64>::new()
                                                } else {
                                                    vec![low_4, high_4]
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(source_ml_source1_temporary_result9) => {
                                        source_ml_source1_temporary_result9
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result {
    MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(f64, f64),
    MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Return(f64),
}

pub fn source_ml_source1_temporary_read_source_identifier_prefix_ordinal_loop1(mut name: self::ml_string::Text, mut index: f64, mut ordinal: f64) -> self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result {
    loop {
        return if (index < (name.length())) {
            {
                let unit = {
                    let source_ml_source2_string_unit_operand76 = name.clone();
                    {
                        let source_ml_source2_string_unit_operand77 = index;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand83 = {
                                        let source_ml_source2_string_range_operand78 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand79 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand80 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position81 = source_ml_source2_string_range_operand79;
                                                    {
                                                        let source_ml_source2_string_range_position82 = source_ml_source2_string_range_operand80;
                                                        source_ml_source2_string_range_operand78.slice(source_ml_source2_string_range_position81, source_ml_source2_string_range_position82)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand84 = 0f64;
                                        source_ml_source2_string_unit_operand83.code_point_at(source_ml_source2_string_unit_operand84)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand76.clone(), source_ml_source2_string_unit_operand77)
                    }
                };
                if ((unit < (48f64)) || (unit > (57f64))) {
                    self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(index, ordinal)
                } else {
                    {
                        let ordinal_2 = (((ordinal * 10f64) + unit) - 48f64);
                        if !(self::ml_math::is_safe_integer(ordinal_2)) {
                            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Return((-1f64))
                        } else {
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (index, ordinal) = (index_2, ordinal_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Result::MlSource1TemporaryReadSourceIdentifierPrefixOrdinalLoop1Done(index, ordinal)
        };
    }
}

pub fn source_ml_source1_temporary_read_source_identifier_prefix_loop2(mut ordinals: Vec<f64>, mut next: f64, mut previous: f64, mut index: f64) -> f64 {
    loop {
        return if (index < ((ordinals.len() as f64))) {
            {
                let ordinal = (self::ml_array::optional_at(&ordinals, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(ordinal)) || (ordinal < (1f64))) || (ordinal < previous)) {
                    panic!("{}", "source identifier ordinals need a sorted positive integer column")
                } else {
                    {
                        let previous_2 = ordinal;
                        if (ordinal == next) {
                            {
                                let next_2 = (next + 1f64);
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (next, previous, index) = (next_2, previous_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = (index + 1f64);
                                {
                                    (previous, index) = (previous_2, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            next
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDependencyFunction1Loop3Result {
    MlSource1TemporaryDependencyFunction1Loop3Done,
    MlSource1TemporaryDependencyFunction1Loop3Return(bool),
}

pub fn source_ml_source1_temporary_dependency_function1_loop3(mut name: self::ml_string::Text, mut reserved: Vec<self::ml_string::Text>, mut index: f64) -> self::MlSource1TemporaryDependencyFunction1Loop3Result {
    loop {
        return if (index < ((reserved.len() as f64))) {
            {
                let word = self::ml_array::optional_at(&reserved, self::ml_array::number_index(index));
                if {
                    let source_ml_source2_comparison_operand85 = word.clone();
                    {
                        let source_ml_source2_comparison_operand86 = None::<()>;
                        match (source_ml_source2_comparison_operand85.clone(), source_ml_source2_comparison_operand86.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    }
                } {
                    panic!("{}", "source binding reserved names need a complete text column")
                } else {
                    if ((word.clone().expect("a proven present value")) == name) {
                        self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Return(false)
                    } else {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result {
    MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done,
    MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(bool),
}

pub fn source_ml_source1_temporary_accept_java_script_source_simple_parameter_list_loop5(mut source_ml_source1_temporary_values4: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_index4: self::ml::Big) -> self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result {
    loop {
        return if (source_ml_source1_temporary_index4 < (self::ml::Big::from_u128(source_ml_source1_temporary_values4.len() as u128))) {
            {
                let term = self::ml_array::at(&source_ml_source1_temporary_values4, source_ml_source1_temporary_index4.to_index());
                if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 95, 112, 97, 116, 116, 101, 114, 110]), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 95, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 95, 112, 97, 116, 116, 101, 114, 110]), self::ml_string::Text::from_units(vec![114, 101, 115, 116, 95, 112, 97, 116, 116, 101, 114, 110]), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 95, 112, 97, 116, 116, 101, 114, 110]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 95, 112, 97, 116, 116, 101, 114, 110])], &term.clone(), 0f64, false, |a, b| a == b) {
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(false)
                } else {
                    {
                        let source_ml_source1_temporary_index4_2 = source_ml_source1_temporary_index4.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index4 = source_ml_source1_temporary_index4_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7 {
    MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Next(f64),
    MlSource1TemporaryDecodeJavaScriptIdentifierEscapeJoin7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result {
    MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Done(f64, f64, f64),
    MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_decode_java_script_identifier_escape_loop6(mut units: Vec<f64>, mut braced: bool, mut index: f64, mut digits: f64, mut scalar: f64) -> self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result {
    loop {
        return if ((index < ((units.len() as f64))) && (braced || (digits < (4f64)))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                {
                    let digit = (-1f64);
                    {
                        let digit_5 = if ((unit >= (48f64)) && (unit <= (57f64))) {
                            {
                                let digit_2 = (unit - 48f64);
                                digit_2
                            }
                        } else {
                            if ((unit >= (65f64)) && (unit <= (70f64))) {
                                {
                                    let digit_3 = (unit - 55f64);
                                    digit_3
                                }
                            } else {
                                if ((unit >= (97f64)) && (unit <= (102f64))) {
                                    {
                                        let digit_4 = (unit - 87f64);
                                        digit_4
                                    }
                                } else {
                                    digit
                                }
                            }
                        };
                        if (digit_5 < (0f64)) {
                            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Done(index, digits, scalar)
                        } else {
                            {
                                let scalar_2 = ((scalar * 16f64) + digit_5);
                                {
                                    let digits_2 = (digits + 1f64);
                                    {
                                        let index_2 = (index + 1f64);
                                        if (scalar_2 > (1114111f64)) {
                                            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Return(vec![(-1f64), index_2])
                                        } else {
                                            {
                                                (index, digits, scalar) = (index_2, digits_2, scalar_2);
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
            self::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Result::MlSource1TemporaryDecodeJavaScriptIdentifierEscapeLoop6Done(index, digits, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result {
    MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(self::ml_string::Text),
    MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_source_library_class_declaration_owner_loop8(mut names: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut witnessed: Vec<self::ml_string::Text>, mut first: f64, mut last: f64, mut owner: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result {
    loop {
        return if (at < ((names.len() as f64))) {
            if (((first >= (self::dependency_function2(starts.clone(), at))) && (last <= (self::dependency_function2(ends.clone(), at)))) && self::ml_array::includes(&witnessed, &self::dependency_function3(names.clone(), at), 0f64, false, |a, b| a == b)) {
                if (owner != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Return(self::ml_string::Text::from_units(vec![35, 97, 109, 98, 105, 103, 117, 111, 117, 115]))
                } else {
                    {
                        let owner_2 = self::dependency_function3(names.clone(), at);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (owner, at) = (owner_2.clone(), at_2);
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
            self::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Result::MlSource1TemporaryReadSourceLibraryClassDeclarationOwnerLoop8Done(owner.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result {
    MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(f64, f64, f64),
    MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_project_source_library_class_edit_interval_loop9(mut starts: Vec<f64>, mut ends: Vec<f64>, mut lengths: Vec<f64>, mut first: f64, mut last: f64, mut delta: f64, mut previous: f64, mut low: f64, mut high: f64, mut at: f64) -> self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result {
    loop {
        return if (at < ((starts.len() as f64))) {
            {
                let start = self::dependency_function2(starts.clone(), at);
                {
                    let end = self::dependency_function2(ends.clone(), at);
                    {
                        let length = self::dependency_function2(lengths.clone(), at);
                        if (((((!(self::ml_math::is_safe_integer(start)) || !(self::ml_math::is_safe_integer(end))) || !(self::ml_math::is_safe_integer(length))) || (start < previous)) || (end < start)) || (length < (0f64))) {
                            self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Return(Vec::<f64>::new())
                        } else {
                            {
                                let changed_start = (start + delta);
                                {
                                    let changed_end = (changed_start + length);
                                    {
                                        let low_3 = if ((low < (0f64)) && (first < changed_start)) {
                                            {
                                                let low_2 = (first - delta);
                                                low_2
                                            }
                                        } else {
                                            low
                                        };
                                        {
                                            let high_3 = if ((high < (0f64)) && (last < changed_start)) {
                                                {
                                                    let high_2 = (last - delta);
                                                    high_2
                                                }
                                            } else {
                                                high
                                            };
                                            {
                                                let low_5 = if (((first >= changed_start) && (first <= changed_end)) && (low_3 < (0f64))) {
                                                    {
                                                        let low_4 = start;
                                                        low_4
                                                    }
                                                } else {
                                                    low_3
                                                };
                                                {
                                                    let high_5 = if (((last >= changed_start) && (last <= changed_end)) && (high_3 < (0f64))) {
                                                        {
                                                            let high_4 = end;
                                                            high_4
                                                        }
                                                    } else {
                                                        high_3
                                                    };
                                                    {
                                                        let delta_2 = (delta + (length - (end - start)));
                                                        {
                                                            let previous_2 = end;
                                                            {
                                                                let at_2 = (at + 1f64);
                                                                {
                                                                    (delta, previous, low, high, at) = (delta_2, previous_2, low_5, high_5, at_2);
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
            self::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Result::MlSource1TemporaryProjectSourceLibraryClassEditIntervalLoop9Done(delta, low, high)
        };
    }
}

fn ml_main() {
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![]), false, false, false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 1");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]), true, true, true)) == (self::ml_string::Text::from_units(vec![]))), "assertion 2");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]), false, true, true)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115]))), "assertion 3");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]), true, false, true)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115]))), "assertion 4");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 49]), true, true, false)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115]))), "assertion 5");
    assert!(((self::read_source_library_class_profile_error(self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110, 32, 99, 108, 97, 115, 115, 32, 112, 114, 111, 102, 105, 108, 101]), true, true, true)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 121, 44, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101, 115]))), "assertion 6");
    assert!(((self::read_source_library_class_binding_kind(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), false)) == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))), "assertion 7");
    assert!(((self::read_source_library_class_binding_kind(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![114, 117, 110]), true)) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]))), "assertion 8");
    assert!(((self::read_source_library_class_binding_kind(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 9");
    assert!(((self::accept_source_library_class_export(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), false)) == (true)), "assertion 10");
    assert!(((self::accept_source_library_class_export(self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), false)) == (false)), "assertion 11");
    assert!(((self::accept_source_library_class_declaration_term(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) == (true)), "assertion 12");
    assert!(((self::accept_source_library_class_declaration_term(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) == (false)), "assertion 13");
    assert!(((self::read_source_library_class_declaration_owner(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], vec![10f64], vec![90f64], vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], 20f64, 80f64)) == (self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))), "assertion 14");
    assert!(((self::read_source_library_class_declaration_owner(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], vec![10f64], vec![90f64], vec![self::ml_string::Text::from_units(vec![79, 116, 104, 101, 114])], 20f64, 80f64)) == (self::ml_string::Text::from_units(vec![]))), "assertion 15");
    assert!(((self::accept_source_library_class_helper_call(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114])], self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (true)), "assertion 16");
    assert!(((self::accept_source_library_class_helper_call(Vec::<self::ml_string::Text>::new(), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114])], self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (false)), "assertion 17");
    assert!(((self::accept_source_library_class_helper_call(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 72, 101, 108, 112, 101, 114])], self::ml_string::Text::from_units(vec![100, 105, 102, 102, 101, 114, 101, 110, 116]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (false)), "assertion 18");
    assert!(((self::accept_source_library_class_static_call(vec![self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101])], self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 83, 116, 97, 116, 105, 99])], self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 83, 116, 97, 116, 105, 99]))) == (true)), "assertion 19");
    assert!(((self::accept_source_library_class_static_call(Vec::<self::ml_string::Text>::new(), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), vec![self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 83, 116, 97, 116, 105, 99])], self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 83, 116, 97, 116, 105, 99]))) == (false)), "assertion 20");
    assert!(((self::project_source_library_class_edit_interval(vec![4f64], vec![9f64], vec![2f64], 7f64, 9f64)) == (vec![10f64, 12f64])), "assertion 21");
    assert!(((self::project_source_library_class_edit_interval(vec![4f64], vec![9f64], vec![2f64], 4f64, 5f64)) == (vec![4f64, 9f64])), "assertion 22");
    assert!(((self::project_source_library_class_edit_interval(vec![4f64], vec![10f64], vec![0f64], 4f64, 6f64)) == (vec![4f64, 12f64])), "assertion 23");
    assert!(((self::project_source_library_class_edit_interval(vec![9f64, 4f64], vec![10f64, 6f64], vec![1f64, 1f64], 1f64, 3f64)) == (Vec::<f64>::new())), "assertion 24");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![]), false, false)) == (self::ml_string::Text::from_units(vec![]))), "assertion 25");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![114, 101, 115, 111, 108, 118, 101, 100, 32, 110, 97, 109, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 112, 114, 111, 118, 105, 100, 101, 114, 115, 32, 49]), true, true)) == (self::ml_string::Text::from_units(vec![]))), "assertion 26");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![114, 101, 115, 111, 108, 118, 101, 100, 32, 110, 97, 109, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 112, 114, 111, 118, 105, 100, 101, 114, 115, 32, 49]), false, true)) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100, 32, 114, 101, 45, 101, 120, 112, 111, 114, 116, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 97, 110, 100, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 101, 115]))), "assertion 27");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![114, 101, 115, 111, 108, 118, 101, 100, 32, 110, 97, 109, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 112, 114, 111, 118, 105, 100, 101, 114, 115, 32, 49]), true, false)) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100, 32, 114, 101, 45, 101, 120, 112, 111, 114, 116, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 97, 110, 100, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 101, 115]))), "assertion 28");
    assert!(((self::read_source_library_re_export_profile_error(self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]), true, true)) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101, 100, 32, 114, 101, 45, 101, 120, 112, 111, 114, 116, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 97, 110, 100, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 97, 108, 105, 97, 115, 101, 115]))), "assertion 29");
    assert!(((self::accept_source_library_re_export_name(self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (true)), "assertion 30");
    assert!(((self::accept_source_library_re_export_name(self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) == (false)), "assertion 31");
    assert!(((self::accept_source_library_re_export_name(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (false)), "assertion 32");
    assert!(((self::accept_source_library_entry_export_owner(self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]))) == (true)), "assertion 33");
    assert!(((self::accept_source_library_entry_export_owner(self::ml_string::Text::from_units(vec![83, 99, 111, 112, 101]), self::ml_string::Text::from_units(vec![112, 114, 105, 118, 97, 116, 101, 80, 114, 111, 118, 105, 100, 101, 114]))) == (false)), "assertion 34");
    assert!(((self::accept_source_library_entry_export_owner(self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]))) == (false)), "assertion 35");
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
