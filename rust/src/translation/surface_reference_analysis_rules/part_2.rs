// Generated from the complete checked JavaScript surface reference analysis. Do not edit.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_surface_reference_operation(
    format: &str,
    operation: &str,
    value: String,
    temporary: &str,
) -> String {
    if (format == "JavaScript")
        && ((operation == "referenceAllocate") || (operation == "referenceRead"))
    {
        value
    } else if (format == "Rust") && (operation == "referenceAllocate") {
        format!("std::rc::Rc::new(std::cell::RefCell::new({value}))")
    } else if (format == "Rust") && (operation == "referenceRead") {
        format!(
            "{{ let {} = {value}.borrow().clone(); {} }}",
            temporary.to_owned(),
            temporary.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_refusal() -> String {
    String::from("shared reference state needs an explicit target state model")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_emission_operation(
    kind: &str,
    stateful_receiver: bool,
    reference_allocation: bool,
) -> String {
    if stateful_receiver
        && (((((((((kind == "index") || (kind == "optionalIndex")) || (kind == "length"))
            || (kind == "recordField"))
            || (kind == "tupleIndex"))
            || (kind == "method"))
            || (kind == "collectionSize"))
            || (kind == "collectionRead"))
            || (kind == "collectionItems"))
    {
        String::from("reference operation")
    } else if reference_allocation
        && (((((kind == "array") || (kind == "record")) || (kind == "map")) || (kind == "set"))
            || (kind == "append"))
    {
        String::from("reference allocation")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_emission_refusal(operation: &str) -> String {
    if operation == "reference operation" {
        String::from(
            "this operation needs the shared reference and slot runtime before target emission",
        )
    } else if operation == "reference allocation" {
        String::from(
            "this allocation needs the shared reference and slot runtime before target emission",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_surface_reference_token_after_elimination(
    window: &[String],
    closure: bool,
    mutating_method: bool,
    eliminated: bool,
) -> bool {
    {
        let required =
            crate::translation::surface_reference_analysis_rules::require_surface_reference_token(
                window,
                closure,
                mutating_method,
            );
        (required
            && !(((eliminated
                && ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                    window, 0f64,
                )) == "punct"))
                && ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                    window, 1f64,
                )) == "="))
                && ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                    window, 3f64,
                )) == "]")))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_projection_form(
    operation: &str,
    kind: &str,
    reference: bool,
    selected: bool,
) -> String {
    if operation == "referenceAllocate" {
        String::from("allocation")
    } else if operation == "referenceRead" {
            String::new()
        } else if crate::translation::surface_reference_analysis_rules::select_surface_reference_allocation(selected, kind, reference) {
                String::from("referencePayload")
            } else {
                String::new()
            }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_payload_operand(operation: &str) -> String {
    if operation == "referencePayload" {
        String::from("value")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_surface_reference_payload_field(field: &str) -> bool {
    ((field == "k") || (field == "value")) || (field == "span")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_payload_error(kind: &str, reference: bool) -> String {
    if crate::translation::surface_reference_analysis_rules::select_surface_reference_allocation(
        true, kind, reference,
    ) {
        String::new()
    } else {
        String::from("a reference payload requires its nonreference structural value")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn collect_surface_reference_item_signatures(
    columns: &[String],
    rows: &[f64],
    size: f64,
    items: f64,
    prefix: &str,
    signatures: (Vec<f64>, Vec<String>),
    external: bool,
) -> (Vec<f64>, Vec<String>) {
    if items < 0f64 {
        signatures.clone()
    } else {
        {
            let result = signatures.clone();
            {
                let offset =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows, items,
                    );
                {
                    let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + items));
                    {
                        crate::translation::surface_reference_analysis_rules::source_ml_temporary_collect_surface_reference_item_signatures_loop18(columns, rows, size, prefix, external, result, offset, end)
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Result {
    MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Done,
    MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Return(bool),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_surface_reference_mutating_method_loop1(method: &str, methods: &[String], mut index: f64) -> crate::translation::surface_reference_analysis_rules::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Result{
    loop {
        return if index < (methods.len() as f64) {
            if (crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                methods, index,
            )) == method
            {
                crate::translation::surface_reference_analysis_rules::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Result::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Return(true)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::surface_reference_analysis_rules::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Result::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadSurfaceReferenceFieldLoop2Result {
    MlTemporaryReadSurfaceReferenceFieldLoop2Done,
    MlTemporaryReadSurfaceReferenceFieldLoop2Return(f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_read_surface_reference_field_loop2(columns: &[String], rows: &[f64], mut size: f64, field: &str, mut offset: f64, mut end: f64) -> crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceFieldLoop2Result{
    loop {
        return if offset < end {
            if (crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                columns,
                ((size * 2f64) + offset),
            )) == field
            {
                crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceFieldLoop2Result::MlTemporaryReadSurfaceReferenceFieldLoop2Return(crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, ((size * 2f64) + offset)))
            } else {
                {
                    let offset_2 = offset + 1f64;
                    {
                        offset = offset_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceFieldLoop2Result::MlTemporaryReadSurfaceReferenceFieldLoop2Done
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_read_surface_reference_path_loop3(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut path: f64,
    mut result: String,
    mut offset: f64,
    mut end: f64,
) -> String {
    loop {
        if offset < end {
            {
                let result_3 = if offset > (crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, path)) {
                    {
                        format!("{result}.")
                    }
                } else {
                    result
                };
                {
                    let result_4 = format!("{result_3}{}", crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, ((size * 2f64) + offset))));
                    {
                        let offset_2 = offset + 1f64;
                        {
                            (result, offset) = (result_4, offset_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_find_surface_reference_item_signature_loop4(
    paths: &[String],
    path: &str,
    mut position: f64,
) -> f64 {
    loop {
        if (position >= 0f64)
            && ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                paths, position,
            )) != path)
        {
            {
                let position_2 = position - 1f64;
                {
                    position = position_2;
                    continue;
                }
            }
        }
        return { position };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadSurfaceReferenceScopeLoop5Result {
    MlTemporaryReadSurfaceReferenceScopeLoop5Done,
    MlTemporaryReadSurfaceReferenceScopeLoop5Return(f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_read_surface_reference_scope_loop5(names: &[String], types: &[f64], name: &str, mut index: f64) -> crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceScopeLoop5Result{
    loop {
        return if index > 0f64 {
            {
                let index_2 = index - 1f64;
                if (crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                    names, index_2,
                )) == name
                {
                    crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceScopeLoop5Result::MlTemporaryReadSurfaceReferenceScopeLoop5Return(crate::translation::surface_reference_analysis_rules::read_required_number_fact(types, index_2))
                } else {
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceScopeLoop5Result::MlTemporaryReadSurfaceReferenceScopeLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceExpressionJoin6 {
    MlTemporaryInspectSurfaceReferenceExpressionJoin6Next(Vec<String>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceExpressionLoop7Result {
    MlTemporaryInspectSurfaceReferenceExpressionLoop7Done(Vec<String>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_inspect_surface_reference_expression_loop7(columns: &[String], rows: &[f64], mut size: f64, mut inner_names: Vec<String>, mut inner_types: Vec<f64>, mut offset: f64, mut end: f64) -> crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop7Result{
    loop {
        if offset < end {
            {
                let param =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + offset),
                    );
                {
                    let inner_names_2 = {
                        let source_ml_source1_concat_value18 = inner_names;
                        {
                            let source_ml_source1_concat_value19 = vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, param, "name"))];
                            crate::translation::frontend_rules::ml_array::append(
                                crate::translation::frontend_rules::ml_array::append(
                                    Vec::<String>::new(),
                                    source_ml_source1_concat_value18,
                                ),
                                source_ml_source1_concat_value19,
                            )
                        }
                    };
                    {
                        let inner_types_2 = {
                            let source_ml_source1_concat_value20 = inner_types;
                            {
                                let source_ml_source1_concat_value21 = vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, param, "type")];
                                crate::translation::frontend_rules::ml_array::append(
                                    crate::translation::frontend_rules::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value20,
                                    ),
                                    source_ml_source1_concat_value21,
                                )
                            }
                        };
                        {
                            let offset_2 = offset + 1f64;
                            {
                                (inner_names, inner_types, offset) =
                                    (inner_names_2, inner_types_2, offset_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop7Result::MlTemporaryInspectSurfaceReferenceExpressionLoop7Done(inner_names, inner_types)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceExpressionJoin10 {
    MlTemporaryInspectSurfaceReferenceExpressionJoin10Next,
    MlTemporaryInspectSurfaceReferenceExpressionJoin10Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceExpressionLoop11Result {
    MlTemporaryInspectSurfaceReferenceExpressionLoop11Done,
    MlTemporaryInspectSurfaceReferenceExpressionLoop11Return(Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_inspect_surface_reference_expression_loop11(columns: &[String], rows: &[f64], mut size: f64, mut observed: bool, wanted: &str, mut position: f64, mut finish: f64) -> crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop11Result{
    loop {
        return if position < finish {
            {
                let field =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + position),
                    );
                if (crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, field, "name"))) == wanted {
                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop11Result::MlTemporaryInspectSurfaceReferenceExpressionLoop11Return(vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, field, "type")), if observed {
                        1f64
                    } else {
                        0f64
                    }])
                } else {
                    {
                        let position_2 = position + 1f64;
                        {
                            position = position_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop11Result::MlTemporaryInspectSurfaceReferenceExpressionLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceExpressionJoin9 {
    MlTemporaryInspectSurfaceReferenceExpressionJoin9Next,
    MlTemporaryInspectSurfaceReferenceExpressionJoin9Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceExpressionLoop8Result {
    MlTemporaryInspectSurfaceReferenceExpressionLoop8Done(bool, f64, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceExpressionJoin12 {
    MlTemporaryInspectSurfaceReferenceExpressionJoin12Next(bool, f64, f64, f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_inspect_surface_reference_expression_loop8(columns: &[String], rows: &[f64], mut size: f64, mut signatures: (Vec<f64>, Vec<String>), names: &[String], types: &[f64], mut observed: bool, mut object_type: f64, mut then_type: f64, mut else_type: f64, mut object_node: f64, mut then_node: f64, mut else_node: f64, mut offset: f64, mut end: f64) -> crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop8Result{
    loop {
        return if offset < end {
            {
                let field =
                    crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                        columns,
                        ((size * 2f64) + offset),
                    );
                {
                    let source_ml_source1_s22 = if ((field != "type") && (field != "span"))
                        && (field != "data")
                    {
                        {
                            let child_node = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, ((size * 2f64) + offset));
                            {
                                let child = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, signatures.clone(), child_node, names, types);
                                {
                                    let observed_3 = if (crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(child), 1f64)) == 1f64 {
                                        {
                                            true
                                        }
                                    } else {
                                        observed
                                    };
                                    {
                                        let object_type_3 = if child_node == object_node {
                                            {
                                                crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(child), 0f64)
                                            }
                                        } else {
                                            object_type
                                        };
                                        {
                                            let then_type_3 = if child_node == then_node {
                                                {
                                                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(child), 0f64)
                                                }
                                            } else {
                                                then_type
                                            };
                                            if child_node == else_node {
                                                {
                                                    let else_type_2 = crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(child), 0f64);
                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin12::MlTemporaryInspectSurfaceReferenceExpressionJoin12Next(observed_3, object_type_3, then_type_3, else_type_2)
                                                }
                                            } else {
                                                crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin12::MlTemporaryInspectSurfaceReferenceExpressionJoin12Next(observed_3, object_type_3, then_type_3, else_type)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin12::MlTemporaryInspectSurfaceReferenceExpressionJoin12Next(observed, object_type, then_type, else_type)
                    };
                    match source_ml_source1_s22 {
                        crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin12::MlTemporaryInspectSurfaceReferenceExpressionJoin12Next(observed_4, object_type_4, then_type_4, else_type_3) => {
                            {
                                let offset_2 = offset + 1f64;
                                {
                                    (observed, object_type, then_type, else_type, offset) = (observed_4, object_type_4, then_type_4, else_type_3, offset_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop8Result::MlTemporaryInspectSurfaceReferenceExpressionLoop8Done(observed, object_type, then_type, else_type)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceItemsJoin14 {
    MlTemporaryInspectSurfaceReferenceItemsJoin14Next(Vec<String>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryInspectSurfaceReferenceItemsLoop15Result {
    MlTemporaryInspectSurfaceReferenceItemsLoop15Done(Vec<String>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_inspect_surface_reference_items_loop15(columns: &[String], rows: &[f64], mut size: f64, mut names: Vec<String>, mut types: Vec<f64>, mut position: f64, mut finish: f64) -> crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceItemsLoop15Result{
    loop {
        if position < finish {
            {
                let param =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + position),
                    );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value23 = names;
                        {
                            let source_ml_source1_concat_value24 = vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, param, "name"))];
                            crate::translation::frontend_rules::ml_array::append(
                                crate::translation::frontend_rules::ml_array::append(
                                    Vec::<String>::new(),
                                    source_ml_source1_concat_value23,
                                ),
                                source_ml_source1_concat_value24,
                            )
                        }
                    };
                    {
                        let types_2 = {
                            let source_ml_source1_concat_value25 = types;
                            {
                                let source_ml_source1_concat_value26 = vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, param, "type")];
                                crate::translation::frontend_rules::ml_array::append(
                                    crate::translation::frontend_rules::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value25,
                                    ),
                                    source_ml_source1_concat_value26,
                                )
                            }
                        };
                        {
                            let position_2 = position + 1f64;
                            {
                                (names, types, position) = (names_2, types_2, position_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceItemsLoop15Result::MlTemporaryInspectSurfaceReferenceItemsLoop15Done(names, types)
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_inspect_surface_reference_items_loop13(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut signatures: (Vec<f64>, Vec<String>),
    mut observed: bool,
    mut offset: f64,
    mut end: f64,
) -> bool {
    loop {
        if offset < end {
            {
                let item =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + offset),
                    );
                {
                    let kind = crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, item);
                    {
                        let observed_3 = if (kind == "module") && crate::translation::surface_reference_analysis_rules::inspect_surface_reference_items(columns, rows, size, signatures.clone(), crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "items")) {
                            {
                                true
                            }
                        } else {
                            observed
                        };
                        {
                            let observed_5 = if kind == "fn" {
                                {
                                    let names = Vec::<String>::new();
                                    {
                                        let types = Vec::<f64>::new();
                                        {
                                            let params = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "params");
                                            {
                                                let source_ml_source1_s28 = if params >= 0f64 {
                                                    {
                                                        let position = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, params);
                                                        {
                                                            let finish = position + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + params));
                                                            {
                                                                let source_ml_source1_s29 = crate::translation::surface_reference_analysis_rules::source_ml_temporary_inspect_surface_reference_items_loop15(columns, rows, size, names, types, position, finish);
                                                                match source_ml_source1_s29 {
                                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceItemsLoop15Result::MlTemporaryInspectSurfaceReferenceItemsLoop15Done(names_2, types_2) => {
                                                                        crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceItemsJoin14::MlTemporaryInspectSurfaceReferenceItemsJoin14Next(names_2, types_2)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceItemsJoin14::MlTemporaryInspectSurfaceReferenceItemsJoin14Next(names, types)
                                                };
                                                match source_ml_source1_s28 {
                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceItemsJoin14::MlTemporaryInspectSurfaceReferenceItemsJoin14Next(names_3, types_3) => {
                                                        {
                                                            let result = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, signatures.clone(), crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "body"), &(names_3), &(types_3));
                                                            if (crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(result), 1f64)) == 1f64 {
                                                                {
                                                                    true
                                                                }
                                                            } else {
                                                                observed_3
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                observed_3
                            };
                            {
                                let offset_2 = offset + 1f64;
                                {
                                    (observed, offset) = (observed_5, offset_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { observed };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryHasSurfaceStructuralIdentityJoin17 {
    MlTemporaryHasSurfaceStructuralIdentityJoin17Next(Vec<String>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_has_surface_structural_identity_loop16(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut signatures: (Vec<f64>, Vec<String>),
    mut observed: bool,
    mut names: Vec<String>,
    mut types: Vec<f64>,
    mut offset: f64,
    mut end: f64,
) -> bool {
    loop {
        return if offset < end {
            {
                let effect =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + offset),
                    );
                {
                    let binding = (crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, effect)) == "let";
                    {
                        let node = if binding {
                            crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, effect, "value")
                        } else {
                            effect
                        };
                        {
                            let result = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, signatures.clone(), node, &(names), &(types));
                            {
                                let observed_3 = if (crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(result), 1f64)) == 1f64 {
                                    {
                                        true
                                    }
                                } else {
                                    observed
                                };
                                {
                                    let source_ml_source1_s34 = if binding {
                                        {
                                            let names_2 = {
                                                let source_ml_source1_concat_value35 = names;
                                                {
                                                    let source_ml_source1_concat_value36 = vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, effect, "name"))];
                                                    crate::translation::frontend_rules::ml_array::append(crate::translation::frontend_rules::ml_array::append(Vec::<String>::new(), source_ml_source1_concat_value35), source_ml_source1_concat_value36)
                                                }
                                            };
                                            {
                                                let types_2 = {
                                                    let source_ml_source1_concat_value37 = types;
                                                    {
                                                        let source_ml_source1_concat_value38 = vec![crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(result), 0f64)];
                                                        crate::translation::frontend_rules::ml_array::append(crate::translation::frontend_rules::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value37), source_ml_source1_concat_value38)
                                                    }
                                                };
                                                crate::translation::surface_reference_analysis_rules::MlTemporaryHasSurfaceStructuralIdentityJoin17::MlTemporaryHasSurfaceStructuralIdentityJoin17Next(names_2, types_2)
                                            }
                                        }
                                    } else {
                                        crate::translation::surface_reference_analysis_rules::MlTemporaryHasSurfaceStructuralIdentityJoin17::MlTemporaryHasSurfaceStructuralIdentityJoin17Next(names, types)
                                    };
                                    match source_ml_source1_s34 {
                                        crate::translation::surface_reference_analysis_rules::MlTemporaryHasSurfaceStructuralIdentityJoin17::MlTemporaryHasSurfaceStructuralIdentityJoin17Next(names_3, types_3) => {
                                            {
                                                let offset_2 = offset + 1f64;
                                                {
                                                    (observed, names, types, offset) = (observed_3, names_3, types_3, offset_2);
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
            observed
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn source_ml_temporary_collect_surface_reference_item_signatures_loop18(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    prefix: &str,
    mut external: bool,
    mut result: (Vec<f64>, Vec<String>),
    mut offset: f64,
    mut end: f64,
) -> (Vec<f64>, Vec<String>) {
    loop {
        if offset < end {
            {
                let item =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + offset),
                    );
                {
                    let kind = crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, item);
                    {
                        let name = format!("{}.{}", prefix.to_owned(), crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "name")));
                        {
                            let result_4 = if external || (kind == "fn") {
                                {
                                    {
                                        let source_ml_source1_field41 = {
                                            let source_ml_source1_concat_value39 =
                                                (result.clone()).1;
                                            {
                                                let source_ml_source1_concat_value40 =
                                                    vec![name.clone()];
                                                crate::translation::frontend_rules::ml_array::append(crate::translation::frontend_rules::ml_array::append(Vec::<String>::new(), source_ml_source1_concat_value39), source_ml_source1_concat_value40)
                                            }
                                        };
                                        {
                                            let source_ml_source1_field44 = {
                                                let source_ml_source1_concat_value42 =
                                                    (result.clone()).0;
                                                {
                                                    let source_ml_source1_concat_value43 =
                                                        vec![item];
                                                    crate::translation::frontend_rules::ml_array::append(crate::translation::frontend_rules::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value42), source_ml_source1_concat_value43)
                                                }
                                            };
                                            (source_ml_source1_field44, source_ml_source1_field41)
                                        }
                                    }
                                }
                            } else if kind == "module" {
                                {
                                    let nested = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "items");
                                    {
                                        crate::translation::surface_reference_analysis_rules::collect_surface_reference_item_signatures(columns, rows, size, nested, &(name), result.clone(), false)
                                    }
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let offset_2 = offset + 1f64;
                                {
                                    (result, offset) = (result_4, offset_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result.clone() };
    }
}
