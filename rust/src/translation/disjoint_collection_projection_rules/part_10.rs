// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::MlSource1TemporaryExpressionJoin83;
use super::MlSource1TemporaryExpressionLoop85Result;
use super::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Result;
use super::accept_collection_copy_mapper_snapshot;
use super::accept_disjoint_collection_iteration_read_expression;
use super::append;
use super::callable;
use super::edge;
use super::field;
use super::finish;
use super::kind;
use super::last;
use super::make;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::ml_string_number;
use super::ml_utf16_text;
use super::read_array_callback_result_form;
use super::read_disjoint_collection_iteration_marker_role;
use super::read_program_expression_operand_fields;
use super::read_reference_callback_operand_fields;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::replace;
use super::row;
use super::selected_edge;
use super::source_ml_source1_temporary_expression_loop84;
use super::source_ml_source1_temporary_expression_loop85;
use super::source_ml_source1_temporary_expression_loop86;
use super::source_ml_source1_temporary_expression_loop89;
use super::source_ml_source1_temporary_expression_loop90;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop93;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop94;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop96;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop97;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop98;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop99;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop100;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop101;
use super::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop102;
use super::text;
use super::word;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn expression(
    state: &[Vec<f64>],
    source: f64,
    owner: f64,
    names: &[f64],
    bindings: &[f64],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    markers: &[self::ml_string::Text],
    depth: f64,
) -> Vec<Vec<f64>> {
    {
        let ml_library_inequality_1 =
            (self::read_required_number_fact(&(self::row(state, 16f64)), 0f64)) != 0f64;
        if ml_library_inequality_1 {
            state.to_vec()
        } else if depth > 256f64 {
            self::replace(state, 16f64, vec![1f64])
        } else {
            {
                let shape = self::word(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    source,
                    &(self::ml_string::Text::from_units(vec![107])),
                );
                {
                    let type_ = self::kind(starts, counts, labels, targets, source);
                    {
                        let operation = self::field(
                            starts,
                            counts,
                            labels,
                            targets,
                            source,
                            &(self::ml_string::Text::from_units(vec![111, 112])),
                        );
                        {
                            let role = 0f64;
                            {
                                let operation_value = operation;
                                {
                                    let source_ml_source2_s37 = if shape == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
                                        {
                                            let role_2 = 3f64;
                                            self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_2, operation_value)
                                        }
                                    } else if (shape == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 65, 108, 108, 111, 99, 97, 116, 101]))) || (shape == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 65, 108, 108, 111, 99, 97, 116, 101]))) {
                                            {
                                                let role_3 = 5f64;
                                                self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_3, operation_value)
                                            }
                                        } else if shape == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 82, 101, 97, 100])) {
                                                {
                                                    let role_4 = 6f64;
                                                    self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_4, operation_value)
                                                }
                                            } else if shape == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])) {
                                                    {
                                                        let role_5 = 7f64;
                                                        {
                                                            let operation_value_2 = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])));
                                                            self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_5, operation_value_2)
                                                        }
                                                    }
                                                } else if shape == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101, 73, 110, 100, 101, 120])) {
                                                        {
                                                            let role_6 = 8f64;
                                                            {
                                                                let operation_value_3 = -2f64 - self::ml_string_number::convert_java_script_string_number_units(&(self::word(texts, starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                                                                self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_6, operation_value_3)
                                                            }
                                                        }
                                                    } else if shape == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 82, 101, 102])) {
                                                            {
                                                                let role_7 = 9f64;
                                                                self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_7, operation_value)
                                                            }
                                                        } else if (shape == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || (shape == (self::ml_string::Text::from_units(vec![114, 101, 99, 117, 114]))) {
                                                                {
                                                                    let role_8 = 10f64;
                                                                    self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_8, operation_value)
                                                                }
                                                            } else if shape == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])) {
                                                                    {
                                                                        let role_9 = 11f64;
                                                                        self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_9, operation_value)
                                                                    }
                                                                } else if shape == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])) {
                                                                        {
                                                                            let role_10 = 12f64;
                                                                            self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_10, operation_value)
                                                                        }
                                                                    } else if shape == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
                                                                            {
                                                                                let role_11 = 13f64;
                                                                                self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_11, operation_value)
                                                                            }
                                                                        } else if (((shape == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || (shape == (self::ml_string::Text::from_units(vec![109, 97, 112])))) || (shape == (self::ml_string::Text::from_units(vec![115, 101, 116])))) || (shape == (self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 109, 115]))) {
                                                                                {
                                                                                    let role_12 = 14f64;
                                                                                    self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_12, operation_value)
                                                                                }
                                                                            } else if shape == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])) {
                                                                                    {
                                                                                        let role_13 = 15f64;
                                                                                        self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_13, operation_value)
                                                                                    }
                                                                                } else if shape == (self::ml_string::Text::from_units(vec![99, 116, 111, 114])) {
                                                                                        {
                                                                                            let role_14 = 16f64;
                                                                                            self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_14, operation_value)
                                                                                        }
                                                                                    } else if shape == (self::ml_string::Text::from_units(vec![99, 101, 108, 108, 87, 114, 105, 116, 101])) {
                                                                                            {
                                                                                                let role_15 = 18f64;
                                                                                                self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_15, operation_value)
                                                                                            }
                                                                                        } else if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 82, 101, 97, 100]), self::ml_string::Text::from_units(vec![99, 101, 108, 108, 82, 101, 97, 100]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 70, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 111, 106, 101, 99, 116])], &shape.clone(), 0f64, false, |a, b| a == b) {
                                                                                                {
                                                                                                    let role_16 = 20f64;
                                                                                                    self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_16, operation_value)
                                                                                                }
                                                                                            } else if (self::accept_disjoint_collection_iteration_read_expression(&(shape))) {
                                                                                                    self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role, operation_value)
                                                                                                } else {
                                                                                                    {
                                                                                                        let role_17 = 19f64;
                                                                                                        self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_17, operation_value)
                                                                                                    }
                                                                                                };
                                    match source_ml_source2_s37 {
                                        self::MlSource1TemporaryExpressionJoin83::MlSource1TemporaryExpressionJoin83Next(role_18, operation_value_4) => {
                                            {
                                                let role_20 = if shape == (self::ml_string::Text::from_units(vec![108, 97, 109, 98, 100, 97])) {
                                                    {
                                                        1f64
                                                    }
                                                } else {
                                                    role_18
                                                };
                                                {
                                                    let marker = self::ml_array::index_of(markers, &self::text(texts, operation), 0f64, false, |a, b| a == b);
                                                    {
                                                        let consumer = self::read_disjoint_collection_iteration_marker_role(&(self::text(texts, operation)));
                                                        {
                                                            let role_22 = if (shape == (self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 109, 115]))) && (marker >= 0f64) {
                                                                {
                                                                    if consumer == (self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114])) {
                                                                        21f64
                                                                    } else {
                                                                        17f64
                                                                    }
                                                                }
                                                            } else {
                                                                role_20
                                                            };
                                                            if ((shape == (self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 109, 115]))) && (consumer != (self::ml_string::Text::from_units(vec![])))) && (marker < 0f64) {
                                                                self::replace(state, 16f64, vec![1f64])
                                                            } else {
                                                                {
                                                                    let state_2 = self::make(state, role_22, operation_value_4, type_, owner);
                                                                    {
                                                                        let node = self::last(&(state_2));
                                                                        if shape == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
                                                                            {
                                                                                let name = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])));
                                                                                {
                                                                                    let binding = -1f64;
                                                                                    {
                                                                                        let at = (names.len() as f64) - 1f64;
                                                                                        {
                                                                                            let binding_2 = self::source_ml_source1_temporary_expression_loop84(names, bindings, texts, name, binding, at);
                                                                                            {
                                                                                                let state_5 = if binding_2 >= 0f64 {
                                                                                                    {
                                                                                                        self::edge(&(state_2), node, 1f64, -1f64, binding_2)
                                                                                                    }
                                                                                                } else {
                                                                                                    {
                                                                                                        self::replace(&(state_2), 0f64, { let ml_values8 = self::row(&(state_2), 0f64); let ml_callback8 = { let node = node; std::rc::Rc::new(move |value: f64, index: f64| -> f64 { if index == node {
                                                                                                            19f64
                                                                                                        } else {
                                                                                                            value
                                                                                                        } }) }; ml_values8.iter().cloned().enumerate().map(|(ml_index8, ml_value8)| ml_callback8(ml_value8, (ml_index8 as f64))).collect::<Vec<_>>() })
                                                                                                    }
                                                                                                };
                                                                                                self::finish(&(state_5), node)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let state_9 = if ((shape == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 82, 101, 102]))) || (shape == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))) || (shape == (self::ml_string::Text::from_units(vec![114, 101, 99, 117, 114]))) {
                                                                                    if shape == (self::ml_string::Text::from_units(vec![114, 101, 99, 117, 114])) {
                                                                                        {
                                                                                            self::edge(&(state_2), node, 4f64, -1f64, owner)
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let state_7 = self::callable(&(state_2), self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![102, 110]))), texts);
                                                                                            {
                                                                                                self::edge(&(state_7), node, 4f64, -1f64, self::last(&(state_7)))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    state_2
                                                                                };
                                                                                if shape == (self::ml_string::Text::from_units(vec![108, 101, 116])) {
                                                                                    {
                                                                                        let value = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                                                                        {
                                                                                            let state_10 = self::expression(&(state_9), value, owner, names, bindings, texts, starts, counts, labels, targets, markers, (depth + 1f64));
                                                                                            {
                                                                                                let actual = self::last(&(state_10));
                                                                                                {
                                                                                                    let state_11 = self::make(&(state_10), 4f64, -1f64, self::kind(starts, counts, labels, targets, value), owner);
                                                                                                    {
                                                                                                        let binding_3 = self::last(&(state_11));
                                                                                                        {
                                                                                                            let state_12 = self::edge(&(state_11), binding_3, 1f64, -1f64, actual);
                                                                                                            {
                                                                                                                let state_13 = self::expression(&(state_12), self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![98, 111, 100, 121]))), owner, &({
                                                                                                                    let source_ml_source2_concat_value38 = names.to_vec();
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value39 = vec![self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])))];
                                                                                                                        self::ml_array::append(&(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value38)), source_ml_source2_concat_value39)
                                                                                                                    }
                                                                                                                }), &({
                                                                                                                    let source_ml_source2_concat_value40 = bindings.to_vec();
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value41 = vec![binding_3];
                                                                                                                        self::ml_array::append(&(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value40)), source_ml_source2_concat_value41)
                                                                                                                    }
                                                                                                                }), texts, starts, counts, labels, targets, markers, (depth + 1f64));
                                                                                                                self::finish(&(self::edge(&(state_13), node, 3f64, -1f64, self::last(&(state_13)))), node)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else if shape == (self::ml_string::Text::from_units(vec![108, 97, 109, 98, 100, 97])) {
                                                                                        {
                                                                                            let state_14 = self::append(&(state_9), 8f64, node);
                                                                                            {
                                                                                                let local_names = names.to_vec();
                                                                                                {
                                                                                                    let local_bindings = bindings.to_vec();
                                                                                                    {
                                                                                                        let params = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115])));
                                                                                                        {
                                                                                                            let at_2 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_s42 = self::source_ml_source1_temporary_expression_loop85(state_14, starts, counts, labels, targets, node, local_names, local_bindings, params, at_2);
                                                                                                                match source_ml_source2_s42 {
                                                                                                                    self::MlSource1TemporaryExpressionLoop85Result::MlSource1TemporaryExpressionLoop85Done(state_15, local_names_2, local_bindings_2) => {
                                                                                                                        {
                                                                                                                            let state_16 = self::expression(&(state_15), self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![98, 111, 100, 121]))), node, &(local_names_2), &(local_bindings_2), texts, starts, counts, labels, targets, markers, (depth + 1f64));
                                                                                                                            self::finish(&(self::append(&(self::append(&(state_16), 11f64, node)), 12f64, self::last(&(state_16)))), node)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else if shape == (self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104])) {
                                                                                            {
                                                                                                let scrutinee = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![115, 99, 114, 117, 116, 105, 110, 101, 101])));
                                                                                                {
                                                                                                    let state_17 = self::expression(&(state_9), scrutinee, owner, names, bindings, texts, starts, counts, labels, targets, markers, (depth + 1f64));
                                                                                                    {
                                                                                                        let value_2 = self::last(&(state_17));
                                                                                                        {
                                                                                                            let cases = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![99, 97, 115, 101, 115])));
                                                                                                            {
                                                                                                                let at_3 = 0f64;
                                                                                                                {
                                                                                                                    let state_18 = self::source_ml_source1_temporary_expression_loop86(state_17, owner, names, bindings, texts, starts, counts, labels, targets, markers, depth, node, scrutinee, value_2, cases, at_3);
                                                                                                                    self::finish(&(state_18), node)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else if (((shape == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) || (shape == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) || (shape == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (shape == (self::ml_string::Text::from_units(vec![99, 116, 111, 114]))) {
                                                                                                {
                                                                                                    let field_name = if shape == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
                                                                                                        self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])
                                                                                                    } else if shape == (self::ml_string::Text::from_units(vec![99, 116, 111, 114])) {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 115])
                                                                                                        } else {
                                                                                                            self::ml_string::Text::from_units(vec![105, 116, 101, 109, 115])
                                                                                                        };
                                                                                                    {
                                                                                                        let values = self::field(starts, counts, labels, targets, source, &(field_name));
                                                                                                        {
                                                                                                            let at_4 = 0f64;
                                                                                                            {
                                                                                                                let state_19 = self::source_ml_source1_temporary_expression_loop89(state_9, owner, names, bindings, texts, starts, counts, labels, targets, markers, depth, &(shape), node, values, at_4);
                                                                                                                {
                                                                                                                    let state_21 = if shape == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
                                                                                                                        {
                                                                                                                            let order = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114])));
                                                                                                                            if order >= 0f64 {
                                                                                                                                {
                                                                                                                                    self::expression(&(state_19), order, owner, names, bindings, texts, starts, counts, labels, targets, markers, (depth + 1f64))
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                state_19
                                                                                                                            }
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        state_19
                                                                                                                    };
                                                                                                                    self::finish(&(state_21), node)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                {
                                                                                                    let operand_fields = {
                                                                                                        let source_ml_source2_string_unit_operand43 = self::read_reference_callback_operand_fields(&(shape), &(self::read_program_expression_operand_fields(&(shape))));
                                                                                                        {
                                                                                                            let source_ml_source2_string_unit_operand44 = self::ml_string::Text::from_units(vec![32]);
                                                                                                            {
                                                                                                                let source_ml_source2_string_split_limit45 = 4_294_967_295f64;
                                                                                                                {
                                                                                                                    let source_ml_source2_string_split_separator46 = source_ml_source2_string_unit_operand44;
                                                                                                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source2_string_unit_operand43), Some(&source_ml_source2_string_split_separator46), source_ml_source2_string_split_limit45)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values82 = operand_fields;
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index82 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let state_22 = self::source_ml_source1_temporary_expression_loop90(state_9, source, owner, names, bindings, texts, starts, counts, labels, targets, markers, depth, &(shape), operation, node, &(source_ml_source1_temporary_values82), source_ml_source1_temporary_index82);
                                                                                                                {
                                                                                                                    let state_24 = if (role_22 == 17f64) || (role_22 == 21f64) {
                                                                                                                        {
                                                                                                                            let receiver = self::selected_edge(&(state_22), node, 2f64);
                                                                                                                            {
                                                                                                                                self::append(&(self::append(&(state_22), 17f64, node)), 18f64, receiver)
                                                                                                                            }
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        state_22
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let state_26 = if shape == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])) {
                                                                                                                            {
                                                                                                                                let object = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                                                                                                                {
                                                                                                                                    let object_shape = self::word(texts, starts, counts, labels, targets, object, &(self::ml_string::Text::from_units(vec![107])));
                                                                                                                                    {
                                                                                                                                        let object_kind = self::text(texts, self::kind(starts, counts, labels, targets, object));
                                                                                                                                        {
                                                                                                                                            let op = self::text(texts, operation);
                                                                                                                                            {
                                                                                                                                                let protected_source = -1f64;
                                                                                                                                                {
                                                                                                                                                    let protected_source_3 = if ((object_shape == (self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101, 82, 101, 97, 100]))) && (object_kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) && ((self::read_array_callback_result_form(&(op))) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                                        {
                                                                                                                                                            self::selected_edge(&(state_24), node, 2f64)
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        protected_source
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let protected_source_5 = if (object_shape == (self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 109, 115]))) && (op == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                                                                                                            {
                                                                                                                                                                let source_ = self::field(starts, counts, labels, targets, object, &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                                                                                                                                                {
                                                                                                                                                                    let actual_kind = self::text(texts, self::kind(starts, counts, labels, targets, source_));
                                                                                                                                                                    {
                                                                                                                                                                        let actual_operation = self::word(texts, starts, counts, labels, targets, object, &(self::ml_string::Text::from_units(vec![111, 112])));
                                                                                                                                                                        if self::accept_collection_copy_mapper_snapshot(&(actual_kind), &(actual_operation)) {
                                                                                                                                                                            {
                                                                                                                                                                                self::selected_edge(&(state_24), self::selected_edge(&(state_24), node, 2f64), 2f64)
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            protected_source_3
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            protected_source_3
                                                                                                                                                        };
                                                                                                                                                        if protected_source_5 >= 0f64 {
                                                                                                                                                            {
                                                                                                                                                                self::append(&(self::append(&(state_24), 19f64, node)), 20f64, protected_source_5)
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            state_24
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            state_24
                                                                                                                        };
                                                                                                                        self::finish(&(state_26), node)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_projection_role(role: f64) -> self::ml_string::Text {
    (self::ml_array::optional_at(
        &vec![
            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]),
            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114]),
            self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 98, 108, 101]),
            self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103]),
            self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![114, 101, 97, 100]),
            self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]),
            self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]),
            self::ml_string::Text::from_units(vec![
                102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99,
                101,
            ]),
            self::ml_string::Text::from_units(vec![99, 97, 108, 108]),
            self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]),
            self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]),
            self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]),
            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]),
            self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]),
            self::ml_string::Text::from_units(vec![100, 97, 116, 97]),
            self::ml_string::Text::from_units(vec![115, 110, 97, 112, 115, 104, 111, 116]),
            self::ml_string::Text::from_units(vec![99, 101, 108, 108, 32, 119, 114, 105, 116, 101]),
            self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]),
            self::ml_string::Text::from_units(vec![
                111, 112, 97, 113, 117, 101, 32, 114, 101, 97, 100,
            ]),
            self::ml_string::Text::from_units(vec![
                101, 97, 103, 101, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116,
            ]),
        ],
        self::ml_array::number_index(role),
    ))
    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_projection_operation(
    role: f64,
    operation: f64,
    text_2: &self::ml_string::Text,
) -> self::ml_string::Text {
    if role == 7f64 {
        (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 58])).concat(text_2)
    } else if role == 8f64 {
        (self::ml_string::Text::from_units(vec![105, 116, 101, 109, 58])).concat(&{
            let source_ml_source2_string_argument47 = -2f64 - operation;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source2_string_argument47,
            ))
        })
    } else {
        text_2.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_projection_kind(
    role: f64,
    value: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (role == 1f64) || (role == 9f64) {
        self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])
    } else {
        {
            let source_ml_source2_logical48 = value.clone();
            if source_ml_source2_logical48.0.is_empty() {
                self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
            } else {
                source_ml_source2_logical48.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_disjoint_collection_projection_edge(
    role: f64,
    auxiliary: f64,
    text_2: &self::ml_string::Text,
) -> self::ml_string::Text {
    if role == 1f64 {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])
    } else if role == 2f64 {
        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
    } else if role == 3f64 {
        self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116])
    } else if role == 4f64 {
        self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101])
    } else if role == 5f64 {
        self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116])
    } else if role == 6f64 {
        (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 58])).concat(text_2)
    } else if role == 7f64 {
        (self::ml_string::Text::from_units(vec![105, 116, 101, 109, 58])).concat(&{
            let source_ml_source2_string_argument49 = auxiliary;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source2_string_argument49,
            ))
        })
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_disjoint_collection_iteration_facts(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    root: f64,
    markers: &[self::ml_string::Text],
) -> Vec<f64> {
    if ((((root < 0f64) || (root >= (starts.len() as f64)))
        || ((starts.len() as f64) > 1_048_576f64))
        || ((starts.len() as f64) != (counts.len() as f64)))
        || ((labels.len() as f64) != (targets.len() as f64))
    {
        vec![1f64]
    } else {
        {
            let state = vec![
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                vec![-1f64],
                vec![0f64],
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
                Vec::<f64>::new(),
            ];
            {
                let declarations = self::field(
                    starts,
                    counts,
                    labels,
                    targets,
                    root,
                    &(self::ml_string::Text::from_units(vec![
                        100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115,
                    ])),
                );
                {
                    let at = 0f64;
                    {
                        let state_2 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop93(texts, starts, counts, labels, targets, state, declarations, at);
                        {
                            let state_3 = self::make(&(state_2), 1f64, -1f64, -1f64, -1f64);
                            {
                                let main_ = self::last(&(state_3));
                                {
                                    let state_4 = self::append(&(state_3), 8f64, main_);
                                    {
                                        let at_2 = 0f64;
                                        {
                                            let state_5 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop94(texts, starts, counts, labels, targets, markers, state_4, declarations, at_2);
                                            {
                                                let names = Vec::<f64>::new();
                                                {
                                                    let bindings = Vec::<f64>::new();
                                                    {
                                                        let effects = self::field(starts, counts, labels, targets, self::field(starts, counts, labels, targets, root, &(self::ml_string::Text::from_units(vec![109, 97, 105, 110]))), &(self::ml_string::Text::from_units(vec![101, 102, 102, 101, 99, 116, 115])));
                                                        {
                                                            let at_3 = 0f64;
                                                            {
                                                                let source_ml_source2_s50 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop96(texts, starts, counts, labels, targets, markers, state_5, main_, names, bindings, effects, at_3);
                                                                match source_ml_source2_s50 {
                                                                    self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Result::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Done(state_6) => {
                                                                        if ((self::read_required_number_fact(&(self::row(&(state_6), 16f64)), 0f64)) != 0f64) || (((self::row(&(state_6), 17f64)).len() as f64) != (markers.len() as f64)) {
                                                                            vec![1f64]
                                                                        } else {
                                                                            {
                                                                                let result = vec![0f64, ((self::row(&(state_6), 0f64)).len() as f64), ((self::row(&(state_6), 4f64)).len() as f64), ((self::row(&(state_6), 8f64)).len() as f64), ((self::row(&(state_6), 9f64)).len() as f64), main_, ((self::row(&(state_6), 17f64)).len() as f64), ((self::row(&(state_6), 19f64)).len() as f64)];
                                                                                {
                                                                                    let at_4 = 0f64;
                                                                                    {
                                                                                        let result_2 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop97(&(state_6), result, at_4);
                                                                                        {
                                                                                            let at_5 = 0f64;
                                                                                            {
                                                                                                let result_3 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop98(&(state_6), result_2, at_5);
                                                                                                {
                                                                                                    let source_ml_source1_temporary_values92 = self::row(&(state_6), 8f64);
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_index92 = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let result_4 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop99(&(state_6), result_3, &(source_ml_source1_temporary_values92), source_ml_source1_temporary_index92);
                                                                                                            {
                                                                                                                let at_6 = 0f64;
                                                                                                                {
                                                                                                                    let result_5 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop100(&(state_6), result_4, at_6);
                                                                                                                    {
                                                                                                                        let at_7 = 0f64;
                                                                                                                        {
                                                                                                                            let result_6 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop101(&(state_6), result_5, at_7);
                                                                                                                            {
                                                                                                                                let at_8 = 0f64;
                                                                                                                                {
                                                                                                                                    self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop102(&(state_6), result_6, at_8)
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
                                                                    }
                                                                    self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Result::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Return(source_ml_source1_temporary_result102) => {
                                                                        source_ml_source1_temporary_result102
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn count(values: &[f64], index: f64) -> f64 {
    if (index < 0f64) || (index >= (values.len() as f64)) {
        0f64
    } else {
        self::read_required_number_fact(values, index)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadCollectionConstructionFieldLoop1Result {
    MlSource1TemporaryReadCollectionConstructionFieldLoop1Done,
    MlSource1TemporaryReadCollectionConstructionFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_collection_construction_field_loop1(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    field_2: &self::ml_string::Text,
    mut start: f64,
    mut count_2: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result {
    loop {
        return if at < (start + count_2) {
            if (self::read_required_text_fact(labels, at)) == (*field_2) {
                self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result::MlSource1TemporaryReadCollectionConstructionFieldLoop1Return(self::read_required_number_fact(targets, at))
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result::MlSource1TemporaryReadCollectionConstructionFieldLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodResultJoin3 {
    MlSource1TemporaryReadReferenceMethodResultJoin3Next,
    MlSource1TemporaryReadReferenceMethodResultJoin3Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodResultJoin2 {
    MlSource1TemporaryReadReferenceMethodResultJoin2Next,
    MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin6 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin6Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin6Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin5 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin5Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin4 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin4Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_reference_method_expression_loop7(
    arguments_: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
    mut prefix: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let prefix_2 = prefix.concat(
                    &((((((self::ml_string::Text::from_units(vec![
                        108, 101, 116, 32, 109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114,
                        103, 117, 109, 101, 110, 116,
                    ]))
                    .concat(suffix))
                    .concat(&self::ml_string::Text::from_units(vec![95])))
                    .concat(&self::ml_string::Text::from_text(
                        &self::ml_number::js_number(index),
                    )))
                    .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                    .concat(
                        &(self::ml_array::optional_at(
                            arguments_,
                            self::ml_array::number_index(index),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    ))
                    .concat(&self::ml_string::Text::from_units(vec![59, 32])),
                );
                {
                    let index_2 = index + 1f64;
                    {
                        (prefix, index) = (prefix_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { prefix.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_reference_method_expression_loop8(
    arguments_: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
    mut names: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let names_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (((self::ml_string::Text::from_units(vec![
                                109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117,
                                109, 101, 110, 116,
                            ]))
                            .concat(suffix))
                            .concat(&self::ml_string::Text::from_units(vec![95])))
                            .concat(&self::ml_string::Text::from_text(
                                &self::ml_number::js_number(index),
                            )),
                        ],
                        names,
                    );
                    self::ml_array::append(&(ml_append_left), ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (names, index) = (names_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}
