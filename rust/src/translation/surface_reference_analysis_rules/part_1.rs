// Generated from the complete checked JavaScript surface reference analysis. Do not edit.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Number fact column position")
    } else {
        (crate::translation::frontend_rules::ml_array::optional_at(
            values,
            crate::translation::frontend_rules::ml_array::number_index(index),
        ))
        .unwrap_or_else(|| {
            crate::translation::surface_reference_analysis_rules::refuse_required_number_fact()
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required text fact column position")
    } else {
        (crate::translation::frontend_rules::ml_array::optional_at(
            values,
            crate::translation::frontend_rules::ml_array::number_index(index),
        ))
        .unwrap_or_else(|| {
            crate::translation::surface_reference_analysis_rules::refuse_required_text_fact()
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Boolean fact column position")
    } else {
        (crate::translation::frontend_rules::ml_array::optional_at(
            values,
            crate::translation::frontend_rules::ml_array::number_index(index),
        ))
        .unwrap_or_else(|| {
            crate::translation::surface_reference_analysis_rules::refuse_required_boolean_fact()
        })
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
            let ml_s1 = crate::translation::surface_reference_analysis_rules::ml_accept_surface_reference_mutating_method_loop1(method, methods, index);
            match ml_s1 {
                crate::translation::surface_reference_analysis_rules::MlAcceptSurfaceReferenceMutatingMethodLoop1Result::MlAcceptSurfaceReferenceMutatingMethodLoop1Done => {
                    false
                }
                crate::translation::surface_reference_analysis_rules::MlAcceptSurfaceReferenceMutatingMethodLoop1Result::MlAcceptSurfaceReferenceMutatingMethodLoop1Return(ml_result1) => {
                    ml_result1
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
                                        if ((kind == "identifier") && (value == "class")) && !member
                                        {
                                            true
                                        } else if (((kind == "identifier") && member)
                                            && (following == "("))
                                            && mutating_method
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
                                                    } else if (previous == "]")
                                                        || ((previous_kind == "identifier")
                                                            && ((before_previous == ".")
                                                                || (before_previous == "?.")))
                                                    {
                                                        true
                                                    } else if (update
                                                        && (following_kind == "identifier"))
                                                        && ((after_following == ".")
                                                            || (after_following == "["))
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
                    let ml_s2 = crate::translation::surface_reference_analysis_rules::ml_read_surface_reference_field_loop2(columns, rows, size, field, offset, end);
                    match ml_s2 {
                        crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceFieldLoop2Result::MlReadSurfaceReferenceFieldLoop2Done => {
                            -1f64
                        }
                        crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceFieldLoop2Result::MlReadSurfaceReferenceFieldLoop2Return(ml_result2) => {
                            ml_result2
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
        String::from("")
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
                (form == "string") && ((crate::translation::surface_reference_analysis_rules::read_required_text_fact(columns, (size + node))) != "")
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
        String::from("")
    } else {
        {
            let result = String::from("");
            {
                let offset =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows, path,
                    );
                {
                    let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + path));
                    {
                        let result_2 = crate::translation::surface_reference_analysis_rules::ml_read_surface_reference_path_loop3(columns, rows, size, path, result, offset, end);
                        result_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_surface_reference_item_signature(
    columns: &[String],
    rows: &[f64],
    size: f64,
    items: f64,
    prefix: &str,
    path: &str,
) -> f64 {
    if items < 0f64 {
        -1f64
    } else {
        {
            let found = -1f64;
            {
                let offset =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows, items,
                    );
                {
                    let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + items));
                    {
                        crate::translation::surface_reference_analysis_rules::ml_find_surface_reference_item_signature_loop4(columns, rows, size, prefix, path, found, offset, end)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_surface_reference_signature(
    columns: &[String],
    rows: &[f64],
    size: f64,
    program: f64,
    path: &str,
) -> f64 {
    {
        let found = crate::translation::surface_reference_analysis_rules::find_surface_reference_item_signature(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, program, "items"), "crate", path);
        {
            let externals =
                crate::translation::surface_reference_analysis_rules::read_surface_reference_field(
                    columns,
                    rows,
                    size,
                    program,
                    "externals",
                );
            if externals < 0f64 {
                found
            } else {
                {
                    let offset = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, externals);
                    {
                        let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + externals));
                        {
                            crate::translation::surface_reference_analysis_rules::ml_find_surface_reference_signature_loop5(columns, rows, size, path, found, offset, end)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_scope(names: &[String], types: &[f64], name: &str) -> f64 {
    {
        let index = names.len() as f64;
        {
            let ml_s3 = crate::translation::surface_reference_analysis_rules::ml_read_surface_reference_scope_loop6(names, types, name, index);
            match ml_s3 {
                crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceScopeLoop6Result::MlReadSurfaceReferenceScopeLoop6Done => {
                    -1f64
                }
                crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceScopeLoop6Result::MlReadSurfaceReferenceScopeLoop6Return(ml_result6) => {
                    ml_result6
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
    program: f64,
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
                            let signature = crate::translation::surface_reference_analysis_rules::find_surface_reference_signature(columns, rows, size, program, &(crate::translation::surface_reference_analysis_rules::read_surface_reference_path(columns, rows, size, path)));
                            vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, signature, "type")), 0f64]
                        }
                    }
                }
            } else if kind == "let" {
                {
                    let value = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "value"), names, types);
                    {
                        let name = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "name"));
                        {
                            let body = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "body"), &({ let (ml_append_right, ml_append_left) = (vec![name], names.to_vec()); crate::translation::frontend_rules::ml_array::append(ml_append_left, ml_append_right) }), &({ let (ml_append_right, ml_append_left) = (vec![crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(value), 0f64)], types.to_vec()); crate::translation::frontend_rules::ml_array::append(ml_append_left, ml_append_right) }));
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
                            let left = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, left_node, names, types);
                            {
                                let right = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, right_node, names, types);
                                {
                                    let operation = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "op"));
                                    {
                                        let observed = (((crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(left), 1f64)) == 1f64) || ((crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(right), 1f64)) == 1f64)) || (((((operation == "eq") || (operation == "ne")) && ((crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, left_node)) != "none")) && ((crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, right_node)) != "none")) && (crate::translation::surface_reference_analysis_rules::accept_surface_reference_structure(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(left), 0f64)) || crate::translation::surface_reference_analysis_rules::accept_surface_reference_structure(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(right), 0f64))));
                                        {
                                            let own = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "type");
                                            {
                                                let type_ = if (crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, own)) != -1f64 {
                                                            own
                                                        } else if (operation == "plus") || (operation == "add") {
                                                                crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(left), 0f64)
                                                            } else {
                                                                -1f64
                                                            };
                                                vec![type_, if observed { 1f64 } else { 0f64 }]
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
                                let ml_s5 = if params >= 0f64 {
                                    {
                                        let offset = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, params);
                                        {
                                            let end = offset + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + params));
                                            {
                                                let ml_s6 = crate::translation::surface_reference_analysis_rules::ml_inspect_surface_reference_expression_loop8(columns, rows, size, inner_names, inner_types, offset, end);
                                                match ml_s6 {
                                                                crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionLoop8Result::MlInspectSurfaceReferenceExpressionLoop8Done(inner_names_2, inner_types_2) => {
                                                                    crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin7::MlInspectSurfaceReferenceExpressionJoin7Next(inner_names_2, inner_types_2)
                                                                }
                                                            }
                                            }
                                        }
                                    }
                                } else {
                                    crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin7::MlInspectSurfaceReferenceExpressionJoin7Next(inner_names, inner_types)
                                };
                                match ml_s5 {
                                                crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin7::MlInspectSurfaceReferenceExpressionJoin7Next(inner_names_3, inner_types_3) => {
                                                    {
                                                        let body_2 = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "body"), &(inner_names_3), &(inner_types_3));
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
                        let offset_2 = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, node);
                        {
                            let end_2 = offset_2 + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + node));
                            {
                                let observed_3 = crate::translation::surface_reference_analysis_rules::ml_inspect_surface_reference_expression_loop9(columns, rows, size, program, names, types, observed_2, offset_2, end_2);
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
                                                        let ml_s7 = if kind == "app" {
                                                            {
                                                                let head = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "fn");
                                                                if (crate::translation::surface_reference_analysis_rules::read_surface_reference_kind(columns, rows, size, head)) == "name" {
                                                                    {
                                                                        let path_2 = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, head, "path");
                                                                        {
                                                                            let signature_2 = crate::translation::surface_reference_analysis_rules::find_surface_reference_signature(columns, rows, size, program, &(crate::translation::surface_reference_analysis_rules::read_surface_reference_path(columns, rows, size, path_2)));
                                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin10::MlInspectSurfaceReferenceExpressionJoin10Return(vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, signature_2, "ret")), if observed_3 {
                                                                                1f64
                                                                            } else {
                                                                                0f64
                                                                            }])
                                                                        }
                                                                    }
                                                                } else {
                                                                    crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin10::MlInspectSurfaceReferenceExpressionJoin10Next
                                                                }
                                                            }
                                                        } else {
                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin10::MlInspectSurfaceReferenceExpressionJoin10Next
                                                        };
                                                        match ml_s7 {
                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin10::MlInspectSurfaceReferenceExpressionJoin10Next => {
                                                                if (kind == "field") || (kind == "index") {
                                                                    {
                                                                        let object = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "object"), names, types);
                                                                        {
                                                                            let observed_5 = if (crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(object), 1f64)) == 1f64 {
                                                                                {
                                                                                    true
                                                                                }
                                                                            } else {
                                                                                observed_3
                                                                            };
                                                                            if kind == "index" {
                                                                                vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(object), 0f64), "element")), if observed_5 {
                                                                                    1f64
                                                                                } else {
                                                                                    0f64
                                                                                }]
                                                                            } else {
                                                                                {
                                                                                    let type_kind = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(object), 0f64), "kind"));
                                                                                    {
                                                                                        let ml_s9 = if type_kind == "record" {
                                                                                            {
                                                                                                let wanted = crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "field"));
                                                                                                {
                                                                                                    let fields = crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(object), 0f64), "fields");
                                                                                                    if fields >= 0f64 {
                                                                                                        {
                                                                                                            let position = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, fields);
                                                                                                            {
                                                                                                                let finish = position + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + fields));
                                                                                                                {
                                                                                                                    let ml_s10 = crate::translation::surface_reference_analysis_rules::ml_inspect_surface_reference_expression_loop12(columns, rows, size, observed_5, &(wanted), position, finish);
                                                                                                                    match ml_s10 {
                                                                                                                        crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionLoop12Result::MlInspectSurfaceReferenceExpressionLoop12Done => {
                                                                                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin11::MlInspectSurfaceReferenceExpressionJoin11Next
                                                                                                                        }
                                                                                                                        crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionLoop12Result::MlInspectSurfaceReferenceExpressionLoop12Return(ml_result12) => {
                                                                                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin11::MlInspectSurfaceReferenceExpressionJoin11Return(ml_result12)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin11::MlInspectSurfaceReferenceExpressionJoin11Next
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin11::MlInspectSurfaceReferenceExpressionJoin11Next
                                                                                        };
                                                                                        match ml_s9 {
                                                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin11::MlInspectSurfaceReferenceExpressionJoin11Next => {
                                                                                                vec![-1f64, if observed_5 {
                                                                                                    1f64
                                                                                                } else {
                                                                                                    0f64
                                                                                                }]
                                                                                            }
                                                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin11::MlInspectSurfaceReferenceExpressionJoin11Return(ml_result11) => {
                                                                                                ml_result11
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else if kind == "if" {
                                                                        {
                                                                            let yes = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "then"), names, types);
                                                                            {
                                                                                let observed_7 = if (crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(yes), 1f64)) == 1f64 {
                                                                                    {
                                                                                        true
                                                                                    }
                                                                                } else {
                                                                                    observed_3
                                                                                };
                                                                                if (crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(yes), 0f64)) != -1f64 {
                                                                                    vec![crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(yes), 0f64), if observed_7 {
                                                                                        1f64
                                                                                    } else {
                                                                                        0f64
                                                                                    }]
                                                                                } else {
                                                                                    {
                                                                                        let no = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, node, "else"), names, types);
                                                                                        vec![crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(no), 0f64), if observed_7 || ((crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(no), 1f64)) == 1f64) {
                                                                                            1f64
                                                                                        } else {
                                                                                            0f64
                                                                                        }]
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                                            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionJoin10::MlInspectSurfaceReferenceExpressionJoin10Return(ml_result12_2) => {
                                                                ml_result12_2
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
    program: f64,
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
                        crate::translation::surface_reference_analysis_rules::ml_inspect_surface_reference_items_loop13(columns, rows, size, program, observed, offset, end)
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
        let observed =
            crate::translation::surface_reference_analysis_rules::inspect_surface_reference_items(
                columns,
                rows,
                size,
                program,
                crate::translation::surface_reference_analysis_rules::read_surface_reference_field(
                    columns, rows, size, program, "items",
                ),
            );
        {
            let main_ =
                crate::translation::surface_reference_analysis_rules::read_surface_reference_field(
                    columns, rows, size, program, "main",
                );
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
                                        crate::translation::surface_reference_analysis_rules::ml_has_surface_structural_identity_loop16(columns, rows, size, program, observed, names, types, offset, end)
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
pub fn select_surface_reference_read(reference: bool) -> bool {
    reference
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_result_field(kind: &str) -> String {
    if kind == "referenceRead" {
        String::from("reference")
    } else {
        String::from("")
    }
}

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
        value.clone()
    } else if (format == "Rust") && (operation == "referenceAllocate") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("std::rc::Rc::new(std::cell::RefCell::new(")),
                value
            )),
            (String::from("))"))
        )
    } else if (format == "Rust") && (operation == "referenceRead") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!("{}{}", (String::from("{ let ")), temporary.to_owned())),
                            (String::from(" = "))
                        )),
                        value
                    )),
                    (String::from(".borrow().clone(); "))
                )),
                temporary.to_owned()
            )),
            (String::from(" }"))
        )
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_surface_reference_refusal() -> String {
    String::from("shared reference state needs an explicit target state model")
}
