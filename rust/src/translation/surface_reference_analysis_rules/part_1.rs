// Generated from the complete checked JavaScript surface reference analysis. Do not edit.
pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any(self.class_name, self.message_units, self.message_present)
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (crate::translation::surface_reference_analysis_rules::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Number fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (crate::translation::surface_reference_analysis_rules::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required text fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (crate::translation::surface_reference_analysis_rules::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Boolean fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (crate::translation::surface_reference_analysis_rules::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Number fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (crate::translation::frontend_rules::ml_array::optional_at(
            values,
            crate::translation::frontend_rules::ml_array::number_index(index),
        ))
        .unwrap_or_else(
            crate::translation::surface_reference_analysis_rules::refuse_required_number_fact,
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (crate::translation::surface_reference_analysis_rules::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (crate::translation::frontend_rules::ml_array::optional_at(
            values,
            crate::translation::frontend_rules::ml_array::number_index(index),
        ))
        .unwrap_or_else(
            crate::translation::surface_reference_analysis_rules::refuse_required_text_fact,
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (crate::translation::surface_reference_analysis_rules::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Boolean fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (crate::translation::frontend_rules::ml_array::optional_at(
            values,
            crate::translation::frontend_rules::ml_array::number_index(index),
        ))
        .unwrap_or_else(
            crate::translation::surface_reference_analysis_rules::refuse_required_boolean_fact,
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_surface_reference_closure_token(kind: &str, value: &str, following: &str) -> bool {
    ((kind == "punct") && (value == "=>"))
        || (((kind == "identifier") && (value == "function")) && (following == "("))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_surface_reference_mutating_method(method: &str, methods: &[String]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s1 = crate::translation::surface_reference_analysis_rules::source_ml_temporary_accept_surface_reference_mutating_method_loop1(method, methods, index);
            match source_ml_source1_s1 {
                crate::translation::surface_reference_analysis_rules::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Result::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Done => {
                    false
                }
                crate::translation::surface_reference_analysis_rules::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Result::MlTemporaryAcceptSurfaceReferenceMutatingMethodLoop1Return(source_ml_temporary_result1) => {
                    source_ml_temporary_result1
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_surface_reference_token(
    window: &[String],
    closure: bool,
    mutating_method: bool,
) -> bool {
    {
        let kind = crate::translation::surface_reference_analysis_rules::read_required_text_fact(
            window, 0f64,
        );
        {
            let value =
                crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                    window, 1f64,
                );
            {
                let previous_kind =
                    crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                        window, 2f64,
                    );
                {
                    let previous = crate::translation::surface_reference_analysis_rules::read_required_text_fact(window, 3f64);
                    {
                        let before_previous = crate::translation::surface_reference_analysis_rules::read_required_text_fact(window, 4f64);
                        {
                            let following_kind = crate::translation::surface_reference_analysis_rules::read_required_text_fact(window, 5f64);
                            {
                                let following = crate::translation::surface_reference_analysis_rules::read_required_text_fact(window, 6f64);
                                {
                                    let after_following = crate::translation::surface_reference_analysis_rules::read_required_text_fact(window, 7f64);
                                    {
                                        let member = (previous == ".") || (previous == "?.");
                                        if (((kind == "identifier") && (value == "class"))
                                            && !member)
                                            || ((((kind == "identifier") && member)
                                                && (following == "("))
                                                && mutating_method)
                                        {
                                            true
                                        } else {
                                            {
                                                let assignment = (((((((((((((((value
                                                    == "=")
                                                    || (value == "+="))
                                                    || (value == "-="))
                                                    || (value == "*="))
                                                    || (value == "/="))
                                                    || (value == "%="))
                                                    || (value == "**="))
                                                    || (value == "<<="))
                                                    || (value == ">>="))
                                                    || (value == ">>>="))
                                                    || (value == "&="))
                                                    || (value == "|="))
                                                    || (value == "^="))
                                                    || (value == "&&="))
                                                    || (value == "||="))
                                                    || (value == "??=");
                                                {
                                                    let update = (value == "++") || (value == "--");
                                                    if (kind != "punct") || (!assignment && !update)
                                                    {
                                                        false
                                                    } else if ((previous == "]")
                                                        || ((previous_kind == "identifier")
                                                            && ((before_previous == ".")
                                                                || (before_previous == "?."))))
                                                        || ((update
                                                            && (following_kind == "identifier"))
                                                            && ((after_following == ".")
                                                                || (after_following == "[")))
                                                    {
                                                        true
                                                    } else {
                                                        closure
                                                            && ((value != "=")
                                                                || (((before_previous != "const")
                                                                    && (before_previous != "let"))
                                                                    && (before_previous != "var")))
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
pub fn read_surface_reference_field(
    columns: &[String],
    rows: &[f64],
    size: f64,
    node: f64,
    field: &str,
) -> f64 {
    if (node < 0f64) || (node >= size) {
        -1f64
    } else {
        {
            let offset =
                crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                    rows, node,
                );
            {
                let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + node));
                {
                    let source_ml_source1_s2 = crate::translation::surface_reference_analysis_rules::source_ml_temporary_read_surface_reference_field_loop2(columns, rows, size, field, offset, end);
                    match source_ml_source1_s2 {
                        crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceFieldLoop2Result::MlTemporaryReadSurfaceReferenceFieldLoop2Done => {
                            -1f64
                        }
                        crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceFieldLoop2Result::MlTemporaryReadSurfaceReferenceFieldLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_text(columns: &[String], size: f64, node: f64) -> String {
    if (node < 0f64) || (node >= size) {
        String::new()
    } else {
        crate::translation::surface_reference_analysis_rules::read_required_text_fact(
            columns,
            (size + node),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_surface_reference_truthy_value(columns: &[String], size: f64, node: f64) -> bool {
    if (node < 0f64) || (node >= size) {
        false
    } else {
        {
            let form =
                crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                    columns, node,
                );
            if (form == "object") || (form == "array") {
                true
            } else if form == "boolean" {
                (crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                    columns,
                    (size + node),
                )) == "true"
            } else if form == "number" {
                (((crate::translation::surface_reference_analysis_rules::read_required_text_fact(columns, (size + node))) != "0") && ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(columns, (size + node))) != "-0")) && ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(columns, (size + node))) != "NaN")
            } else {
                (form == "string") && (!(crate::translation::surface_reference_analysis_rules::read_required_text_fact(columns, (size + node))).is_empty())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_type(columns: &[String], node: f64) -> f64 {
    if (node >= 0f64)
        && (((crate::translation::surface_reference_analysis_rules::read_required_text_fact(
            columns, node,
        )) == "undefined")
            || ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                columns, node,
            )) == "null"))
    {
        -1f64
    } else {
        node
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_kind(
    columns: &[String],
    rows: &[f64],
    size: f64,
    node: f64,
) -> String {
    crate::translation::surface_reference_analysis_rules::read_surface_reference_text(
        columns,
        size,
        crate::translation::surface_reference_analysis_rules::read_surface_reference_field(
            columns, rows, size, node, "k",
        ),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_surface_reference_structure(
    columns: &[String],
    rows: &[f64],
    size: f64,
    type_: f64,
) -> bool {
    if type_ < 0f64 {
        type_ == -3f64
    } else {
        {
            let kind = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, type_, "kind"));
            if ((((kind == "array") || (kind == "record")) || (kind == "set")) || (kind == "map"))
                || (kind == "data")
            {
                true
            } else {
                (kind == "option") && crate::translation::surface_reference_analysis_rules::accept_surface_reference_structure(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, type_, "element"))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_path(
    columns: &[String],
    rows: &[f64],
    size: f64,
    path: f64,
) -> String {
    if path < 0f64 {
        String::new()
    } else {
        {
            let result = String::new();
            {
                let offset =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows, path,
                    );
                {
                    let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + path));
                    {
                        crate::translation::surface_reference_analysis_rules::source_ml_temporary_read_surface_reference_path_loop3(columns, rows, size, path, result, offset, end)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_surface_reference_item_signature(paths: &[String], path: &str) -> f64 {
    {
        let position = (paths.len() as f64) - 1f64;
        {
            crate::translation::surface_reference_analysis_rules::source_ml_temporary_find_surface_reference_item_signature_loop4(paths, path, position)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_surface_reference_signature(signatures: (Vec<f64>, Vec<String>), path: &str) -> f64 {
    {
        let position = crate::translation::surface_reference_analysis_rules::find_surface_reference_item_signature(&((signatures.clone()).1), path);
        if position >= 0f64 {
            crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                &((signatures.clone()).0),
                position,
            )
        } else {
            -1f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_scope(names: &[String], types: &[f64], name: &str) -> f64 {
    {
        let index = names.len() as f64;
        {
            let source_ml_source1_s3 = crate::translation::surface_reference_analysis_rules::source_ml_temporary_read_surface_reference_scope_loop5(names, types, name, index);
            match source_ml_source1_s3 {
                crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceScopeLoop5Result::MlTemporaryReadSurfaceReferenceScopeLoop5Done => {
                    -1f64
                }
                crate::translation::surface_reference_analysis_rules::MlTemporaryReadSurfaceReferenceScopeLoop5Result::MlTemporaryReadSurfaceReferenceScopeLoop5Return(source_ml_temporary_result5) => {
                    source_ml_temporary_result5
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn inspect_surface_reference_expression(
    columns: &[String],
    rows: &[f64],
    size: f64,
    signatures: (Vec<f64>, Vec<String>),
    node: f64,
    names: &[String],
    types: &[f64],
) -> Vec<f64> {
    if (node < 0f64)
        || (((crate::translation::surface_reference_analysis_rules::read_required_text_fact(
            columns, node,
        )) != "object")
            && ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                columns, node,
            )) != "array"))
    {
        vec![-1f64, 0f64]
    } else {
        {
            let kind =
                crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(
                    columns, rows, size, node,
                );
            if kind == "name" {
                {
                    let path = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "path");
                    if (path >= 0f64) && ((crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + path))) == 1f64) {
                        vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_scope(names, types, &(crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, ((size * 2f64) + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, path))))))), 0f64]
                    } else {
                        {
                            let signature = crate::translation::surface_reference_analysis_rules::find_surface_reference_signature(signatures.clone(), &(crate::translation::surface_reference_analysis_rules::read_surface_reference_path(columns, rows, size, path)));
                            {
                                let parameters = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, signature, "params");
                                if (((crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, signature)) == "fn") && (parameters >= 0f64)) && ((crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + parameters))) == 0f64) {
                                    vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, signature, "ret")), 0f64]
                                } else {
                                    vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, signature, "type")), 0f64]
                                }
                            }
                        }
                    }
                }
            } else if kind == "let" {
                {
                    let value = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, signatures.clone(), crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "value"), names, types);
                    {
                        let name = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "name"));
                        {
                            let body = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, signatures.clone(), crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "body"), &({
                                    let source_ml_source1_concat_value4 = names.to_vec();
                                    {
                                        let source_ml_source1_concat_value5 = vec![name];
                                        crate::translation::frontend_rules::ml_array::append(crate::translation::frontend_rules::ml_array::append(Vec::<String>::new(), source_ml_source1_concat_value4), source_ml_source1_concat_value5)
                                    }
                                }), &({
                                    let source_ml_source1_concat_value6 = types.to_vec();
                                    {
                                        let source_ml_source1_concat_value7 = vec![crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(value), 0f64)];
                                        crate::translation::frontend_rules::ml_array::append(crate::translation::frontend_rules::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value6), source_ml_source1_concat_value7)
                                    }
                                }));
                            vec![crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(body), 0f64), if ((crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(value), 1f64)) == 1f64) || ((crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(body), 1f64)) == 1f64) {
                                    1f64
                                } else {
                                    0f64
                                }]
                        }
                    }
                }
            } else if kind == "binary" {
                {
                    let left_node = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "left");
                    {
                        let right_node = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "right");
                        {
                            let left = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, signatures.clone(), left_node, names, types);
                            {
                                let right = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, signatures.clone(), right_node, names, types);
                                {
                                    let operation = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "op"));
                                    {
                                        let observed = (((crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(left), 1f64)) == 1f64) || ((crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(right), 1f64)) == 1f64)) || (((((operation == "eq") || (operation == "ne")) && ((crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, left_node)) != "none")) && ((crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, right_node)) != "none")) && (crate::translation::surface_reference_analysis_rules::accept_surface_reference_structure(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(left), 0f64)) || crate::translation::surface_reference_analysis_rules::accept_surface_reference_structure(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(right), 0f64))));
                                        {
                                            let own = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "type");
                                            {
                                                let result_type = {
                                                    let ml_library_inequality_1 = (crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, own)) != -1f64 ;
                                                    if ml_library_inequality_1 {
                                                        own
                                                    } else if (operation == "plus")
                                                        || (operation == "add")
                                                    {
                                                        crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(left), 0f64)
                                                    } else {
                                                        -1f64
                                                    }
                                                };
                                                vec![
                                                    result_type,
                                                    if observed { 1f64 } else { 0f64 },
                                                ]
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if kind == "lambda" {
                {
                    let inner_names = names.to_vec();
                    {
                        let inner_types = types.to_vec();
                        {
                            let params = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "params");
                            {
                                let source_ml_source1_s9 = if params >= 0f64 {
                                    {
                                        let offset = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, params);
                                        {
                                            let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + params));
                                            {
                                                let source_ml_source1_s10 = crate::translation::surface_reference_analysis_rules::source_ml_temporary_inspect_surface_reference_expression_loop7(columns, rows, size, inner_names, inner_types, offset, end);
                                                match source_ml_source1_s10 {
                                                                crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop7Result::MlTemporaryInspectSurfaceReferenceExpressionLoop7Done(inner_names_2, inner_types_2) => {
                                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin6::MlTemporaryInspectSurfaceReferenceExpressionJoin6Next(inner_names_2, inner_types_2)
                                                                }
                                                            }
                                            }
                                        }
                                    }
                                } else {
                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin6::MlTemporaryInspectSurfaceReferenceExpressionJoin6Next(inner_names, inner_types)
                                };
                                match source_ml_source1_s9 {
                                                crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin6::MlTemporaryInspectSurfaceReferenceExpressionJoin6Next(inner_names_3, inner_types_3) => {
                                                    {
                                                        let body_2 = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, signatures.clone(), crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "body"), &(inner_names_3), &(inner_types_3));
                                                        vec![-2f64, crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(body_2), 1f64)]
                                                    }
                                                }
                                            }
                            }
                        }
                    }
                }
            } else {
                {
                    let observed_2 = false;
                    {
                        let object_type = -1f64;
                        {
                            let then_type = -1f64;
                            {
                                let else_type = -1f64;
                                {
                                    let object_node = if (kind == "field") || (kind == "index") {
                                        crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "object")
                                    } else {
                                        -1f64
                                    };
                                    {
                                        let then_node = if kind == "if" {
                                            crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "then")
                                        } else {
                                            -1f64
                                        };
                                        {
                                            let else_node = if kind == "if" {
                                                crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "else")
                                            } else {
                                                -1f64
                                            };
                                            {
                                                let offset_2 = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, node);
                                                {
                                                    let end_2 = offset_2 + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + node));
                                                    {
                                                        let source_ml_source1_s11 = crate::translation::surface_reference_analysis_rules::source_ml_temporary_inspect_surface_reference_expression_loop8(columns, rows, size, signatures.clone(), names, types, observed_2, object_type, then_type, else_type, object_node, then_node, else_node, offset_2, end_2);
                                                        match source_ml_source1_s11 {
                                                                        crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop8Result::MlTemporaryInspectSurfaceReferenceExpressionLoop8Done(observed_3, object_type_2, then_type_2, else_type_2) => {
                                                                            {
                                                                                let own_2 = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "type");
                                                                                if crate::translation::surface_reference_analysis_rules::accept_surface_reference_truthy_value(columns, size, own_2) {
                                                                                    vec![own_2, if observed_3 {
                                                                                        1f64
                                                                                    } else {
                                                                                        0f64
                                                                                    }]
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source1_s12 = if kind == "app" {
                                                                                            {
                                                                                                let head = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "fn");
                                                                                                if (crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, head)) == "name" {
                                                                                                    {
                                                                                                        let path_2 = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, head, "path");
                                                                                                        {
                                                                                                            let signature_2 = crate::translation::surface_reference_analysis_rules::find_surface_reference_signature(signatures.clone(), &(crate::translation::surface_reference_analysis_rules::read_surface_reference_path(columns, rows, size, path_2)));
                                                                                                            crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin9::MlTemporaryInspectSurfaceReferenceExpressionJoin9Return(vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, signature_2, "ret")), if observed_3 {
                                                                                                                1f64
                                                                                                            } else {
                                                                                                                0f64
                                                                                                            }])
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin9::MlTemporaryInspectSurfaceReferenceExpressionJoin9Next
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin9::MlTemporaryInspectSurfaceReferenceExpressionJoin9Next
                                                                                        };
                                                                                        match source_ml_source1_s12 {
                                                                                            crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin9::MlTemporaryInspectSurfaceReferenceExpressionJoin9Next => {
                                                                                                if (kind == "field") || (kind == "index") {
                                                                                                    if kind == "index" {
                                                                                                        vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, object_type_2, "element")), if observed_3 {
                                                                                                            1f64
                                                                                                        } else {
                                                                                                            0f64
                                                                                                        }]
                                                                                                    } else {
                                                                                                        {
                                                                                                            let type_kind = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, object_type_2, "kind"));
                                                                                                            {
                                                                                                                let source_ml_source1_s14 = if type_kind == "record" {
                                                                                                                    {
                                                                                                                        let wanted = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "field"));
                                                                                                                        {
                                                                                                                            let fields = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, object_type_2, "fields");
                                                                                                                            if fields >= 0f64 {
                                                                                                                                {
                                                                                                                                    let position = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, fields);
                                                                                                                                    {
                                                                                                                                        let finish = position + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + fields));
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_s15 = crate::translation::surface_reference_analysis_rules::source_ml_temporary_inspect_surface_reference_expression_loop11(columns, rows, size, observed_3, &(wanted), position, finish);
                                                                                                                                            match source_ml_source1_s15 {
                                                                                                                                                crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop11Result::MlTemporaryInspectSurfaceReferenceExpressionLoop11Done => {
                                                                                                                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin10::MlTemporaryInspectSurfaceReferenceExpressionJoin10Next
                                                                                                                                                }
                                                                                                                                                crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionLoop11Result::MlTemporaryInspectSurfaceReferenceExpressionLoop11Return(source_ml_temporary_result11) => {
                                                                                                                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin10::MlTemporaryInspectSurfaceReferenceExpressionJoin10Return(source_ml_temporary_result11)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin10::MlTemporaryInspectSurfaceReferenceExpressionJoin10Next
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin10::MlTemporaryInspectSurfaceReferenceExpressionJoin10Next
                                                                                                                };
                                                                                                                match source_ml_source1_s14 {
                                                                                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin10::MlTemporaryInspectSurfaceReferenceExpressionJoin10Next => {
                                                                                                                        vec![-1f64, if observed_3 {
                                                                                                                            1f64
                                                                                                                        } else {
                                                                                                                            0f64
                                                                                                                        }]
                                                                                                                    }
                                                                                                                    crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin10::MlTemporaryInspectSurfaceReferenceExpressionJoin10Return(source_ml_temporary_result10) => {
                                                                                                                        source_ml_temporary_result10
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else if kind == "if" {
                                                                                                        vec![if then_type_2 != -1f64 {
                                                                                                            then_type_2
                                                                                                        } else {
                                                                                                            else_type_2
                                                                                                        }, if observed_3 {
                                                                                                            1f64
                                                                                                        } else {
                                                                                                            0f64
                                                                                                        }]
                                                                                                    } else if ((kind == "num") || (kind == "str")) || (kind == "bool") {
                                                                                                            vec![-2f64, if observed_3 {
                                                                                                                1f64
                                                                                                            } else {
                                                                                                                0f64
                                                                                                            }]
                                                                                                        } else if (((kind == "array") || (kind == "record")) || (kind == "set")) || (kind == "map") {
                                                                                                                vec![-3f64, if observed_3 {
                                                                                                                    1f64
                                                                                                                } else {
                                                                                                                    0f64
                                                                                                                }]
                                                                                                            } else {
                                                                                                                vec![-1f64, if observed_3 {
                                                                                                                    1f64
                                                                                                                } else {
                                                                                                                    0f64
                                                                                                                }]
                                                                                                            }
                                                                                            }
                                                                                            crate::translation::surface_reference_analysis_rules::MlTemporaryInspectSurfaceReferenceExpressionJoin9::MlTemporaryInspectSurfaceReferenceExpressionJoin9Return(source_ml_temporary_result11_2) => {
                                                                                                source_ml_temporary_result11_2
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
pub fn inspect_surface_reference_items(
    columns: &[String],
    rows: &[f64],
    size: f64,
    signatures: (Vec<f64>, Vec<String>),
    items: f64,
) -> bool {
    if items < 0f64 {
        false
    } else {
        {
            let observed = false;
            {
                let offset =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows, items,
                    );
                {
                    let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + items));
                    {
                        crate::translation::surface_reference_analysis_rules::source_ml_temporary_inspect_surface_reference_items_loop13(columns, rows, size, signatures.clone(), observed, offset, end)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn has_surface_structural_identity(
    columns: &[String],
    rows: &[f64],
    size: f64,
    program: f64,
) -> bool {
    {
        let items =
            crate::translation::surface_reference_analysis_rules::read_surface_reference_field(
                columns, rows, size, program, "items",
            );
        {
            let externals =
                crate::translation::surface_reference_analysis_rules::read_surface_reference_field(
                    columns,
                    rows,
                    size,
                    program,
                    "externals",
                );
            {
                let declared = crate::translation::surface_reference_analysis_rules::collect_surface_reference_item_signatures(columns, rows, size, items, "crate", {
                    let source_ml_source1_field16 = Vec::<String>::new();
                    {
                        let source_ml_source1_field17 = Vec::<f64>::new();
                        (source_ml_source1_field17, source_ml_source1_field16)
                    }
                }, false);
                {
                    let signatures = crate::translation::surface_reference_analysis_rules::collect_surface_reference_item_signatures(columns, rows, size, externals, "crate", declared, true);
                    {
                        let observed = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_items(columns, rows, size, signatures.clone(), items);
                        {
                            let main_ = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, program, "main");
                            {
                                let effects = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, main_, "effects");
                                if effects < 0f64 {
                                    observed
                                } else {
                                    {
                                        let names = Vec::<String>::new();
                                        {
                                            let types = Vec::<f64>::new();
                                            {
                                                let offset = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, effects);
                                                {
                                                    let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + effects));
                                                    {
                                                        crate::translation::surface_reference_analysis_rules::source_ml_temporary_has_surface_structural_identity_loop16(columns, rows, size, signatures.clone(), observed, names, types, offset, end)
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
pub fn select_surface_reference_allocation(selected: bool, kind: &str, existing: bool) -> bool {
    (selected && !existing)
        && ((((kind == "array") || (kind == "record")) || (kind == "map")) || (kind == "set"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn select_surface_reference_read(reference: bool) -> bool {
    reference
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_result_field(kind: &str) -> String {
    if kind == "referenceRead" {
        String::from("reference")
    } else {
        String::new()
    }
}
