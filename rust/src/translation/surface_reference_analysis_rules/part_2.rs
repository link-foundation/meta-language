// Generated from the complete checked JavaScript surface reference analysis. Do not edit.
use super::*;

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
        String::from("")
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
        String::from("")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptSurfaceReferenceMutatingMethodLoop1Result {
    MlAcceptSurfaceReferenceMutatingMethodLoop1Done,
    MlAcceptSurfaceReferenceMutatingMethodLoop1Return(bool),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_accept_surface_reference_mutating_method_loop1(method: &str, methods: &[String], mut index: f64) -> crate::translation::surface_reference_analysis_rules::MlAcceptSurfaceReferenceMutatingMethodLoop1Result{
    loop {
        return if index < (methods.len() as f64) {
            if (crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                methods, index,
            )) == method
            {
                crate::translation::surface_reference_analysis_rules::MlAcceptSurfaceReferenceMutatingMethodLoop1Result::MlAcceptSurfaceReferenceMutatingMethodLoop1Return(true)
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
            crate::translation::surface_reference_analysis_rules::MlAcceptSurfaceReferenceMutatingMethodLoop1Result::MlAcceptSurfaceReferenceMutatingMethodLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadSurfaceReferenceFieldLoop2Result {
    MlReadSurfaceReferenceFieldLoop2Done,
    MlReadSurfaceReferenceFieldLoop2Return(f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_surface_reference_field_loop2(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    field: &str,
    mut offset: f64,
    mut end: f64,
) -> crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceFieldLoop2Result {
    loop {
        return if offset < end {
            if (crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                columns,
                ((size * 2f64) + offset),
            )) == field
            {
                crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceFieldLoop2Result::MlReadSurfaceReferenceFieldLoop2Return(crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, ((size * 2f64) + offset)))
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
            crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceFieldLoop2Result::MlReadSurfaceReferenceFieldLoop2Done
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_surface_reference_path_loop3(
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
                        let result_2 = format!("{}{}", result, (String::from(".")));
                        result_2
                    }
                } else {
                    result.clone()
                };
                {
                    let result_4 = format!("{}{}", result_3, (crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, ((size * 2f64) + offset)))));
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
        return { result.clone() };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]

pub fn ml_find_surface_reference_item_signature_loop4(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    prefix: &str,
    path: &str,
    mut found: f64,
    mut offset: f64,
    mut end: f64,
) -> f64 {
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
                        let name = format!("{}{}", (format!("{}{}", prefix, (String::from(".")))), (crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "name"))));
                        {
                            let found_3 = if (kind == "fn") && (name == path) {
                                { item }
                            } else {
                                found
                            };
                            {
                                let found_5 = if kind == "module" {
                                    {
                                        let nested = crate::translation::surface_reference_analysis_rules::find_surface_reference_item_signature(columns, rows, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "items"), &(name), path);
                                        if nested >= 0f64 { { nested } } else { found_3 }
                                    }
                                } else {
                                    found_3
                                };
                                {
                                    let offset_2 = offset + 1f64;
                                    {
                                        (found, offset) = (found_5, offset_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { found };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_find_surface_reference_signature_loop5(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    path: &str,
    mut found: f64,
    mut offset: f64,
    mut end: f64,
) -> f64 {
    loop {
        if offset < end {
            {
                let external =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + offset),
                    );
                {
                    let name = format!("{}{}", (String::from("crate.")), (crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, external, "name"))));
                    {
                        let found_3 = if name == path { { external } } else { found };
                        {
                            let offset_2 = offset + 1f64;
                            {
                                (found, offset) = (found_3, offset_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { found };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadSurfaceReferenceScopeLoop6Result {
    MlReadSurfaceReferenceScopeLoop6Done,
    MlReadSurfaceReferenceScopeLoop6Return(f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_surface_reference_scope_loop6(
    names: &[String],
    types: &[f64],
    name: &str,
    mut index: f64,
) -> crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceScopeLoop6Result {
    loop {
        return if index > 0f64 {
            {
                let index_2 = index - 1f64;
                if (crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                    names, index_2,
                )) == name
                {
                    crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceScopeLoop6Result::MlReadSurfaceReferenceScopeLoop6Return(crate::translation::surface_reference_analysis_rules::read_required_number_fact(types, index_2))
                } else {
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::surface_reference_analysis_rules::MlReadSurfaceReferenceScopeLoop6Result::MlReadSurfaceReferenceScopeLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInspectSurfaceReferenceExpressionJoin7 {
    MlInspectSurfaceReferenceExpressionJoin7Next(Vec<String>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInspectSurfaceReferenceExpressionLoop8Result {
    MlInspectSurfaceReferenceExpressionLoop8Done(Vec<String>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_inspect_surface_reference_expression_loop8(columns: &[String], rows: &[f64], mut size: f64, mut inner_names: Vec<String>, mut inner_types: Vec<f64>, mut offset: f64, mut end: f64) -> crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionLoop8Result{
    loop {
        if offset < end {
            {
                let param =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + offset),
                    );
                {
                    let inner_names_2 = crate::translation::frontend_rules::ml_array::append(inner_names.clone(), vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, param, "name"))]);
                    {
                        let inner_types_2 = crate::translation::frontend_rules::ml_array::append(inner_types.clone(), vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, param, "type")]);
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
            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionLoop8Result::MlInspectSurfaceReferenceExpressionLoop8Done(inner_names.clone(), inner_types.clone())
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]

pub fn ml_inspect_surface_reference_expression_loop9(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut program: f64,
    names: &[String],
    types: &[f64],
    mut observed: bool,
    mut offset: f64,
    mut end: f64,
) -> bool {
    loop {
        if offset < end {
            {
                let field =
                    crate::translation::surface_reference_analysis_rules::read_required_text_fact(
                        columns,
                        ((size * 2f64) + offset),
                    );
                {
                    let observed_3 = if ((field != "type") && (field != "span"))
                        && (field != "data")
                    {
                        {
                            let child = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, ((size * 2f64) + offset)), names, types);
                            if (crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(child), 1f64)) == 1f64 {
                                {
                                    true
                                }
                            } else {
                                observed
                            }
                        }
                    } else {
                        observed
                    };
                    {
                        let offset_2 = offset + 1f64;
                        {
                            (observed, offset) = (observed_3, offset_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { observed };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInspectSurfaceReferenceExpressionJoin11 {
    MlInspectSurfaceReferenceExpressionJoin11Next,
    MlInspectSurfaceReferenceExpressionJoin11Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInspectSurfaceReferenceExpressionLoop12Result {
    MlInspectSurfaceReferenceExpressionLoop12Done,
    MlInspectSurfaceReferenceExpressionLoop12Return(Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_inspect_surface_reference_expression_loop12(columns: &[String], rows: &[f64], mut size: f64, mut observed: bool, wanted: &str, mut position: f64, mut finish: f64) -> crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionLoop12Result{
    loop {
        return if position < finish {
            {
                let field =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + position),
                    );
                if (crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, field, "name"))) == wanted {
                    crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionLoop12Result::MlInspectSurfaceReferenceExpressionLoop12Return(vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_type(columns, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, field, "type")), if observed {
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
            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceExpressionLoop12Result::MlInspectSurfaceReferenceExpressionLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInspectSurfaceReferenceExpressionJoin10 {
    MlInspectSurfaceReferenceExpressionJoin10Next,
    MlInspectSurfaceReferenceExpressionJoin10Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInspectSurfaceReferenceItemsJoin14 {
    MlInspectSurfaceReferenceItemsJoin14Next(Vec<String>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlInspectSurfaceReferenceItemsLoop15Result {
    MlInspectSurfaceReferenceItemsLoop15Done(Vec<String>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_inspect_surface_reference_items_loop15(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut names: Vec<String>,
    mut types: Vec<f64>,
    mut position: f64,
    mut finish: f64,
) -> crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceItemsLoop15Result
{
    loop {
        if position < finish {
            {
                let param =
                    crate::translation::surface_reference_analysis_rules::read_required_number_fact(
                        rows,
                        ((size * 2f64) + position),
                    );
                {
                    let names_2 = crate::translation::frontend_rules::ml_array::append(names.clone(), vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, param, "name"))]);
                    {
                        let types_2 = crate::translation::frontend_rules::ml_array::append(types.clone(), vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, param, "type")]);
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
            crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceItemsLoop15Result::MlInspectSurfaceReferenceItemsLoop15Done(names.clone(), types.clone())
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_inspect_surface_reference_items_loop13(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut program: f64,
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
                        let observed_3 = if (kind == "module") && crate::translation::surface_reference_analysis_rules::inspect_surface_reference_items(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "items")) {
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
                                                let ml_s12 = if params >= 0f64 {
                                                    {
                                                        let position = crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, params);
                                                        {
                                                            let finish = position + crate::translation::surface_reference_analysis_rules::read_required_number_fact(rows, (size + params));
                                                            {
                                                                let ml_s13 = crate::translation::surface_reference_analysis_rules::ml_inspect_surface_reference_items_loop15(columns, rows, size, names.clone(), types.clone(), position, finish);
                                                                match ml_s13 {
                                                                    crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceItemsLoop15Result::MlInspectSurfaceReferenceItemsLoop15Done(names_2, types_2) => {
                                                                        crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceItemsJoin14::MlInspectSurfaceReferenceItemsJoin14Next(names_2.clone(), types_2.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceItemsJoin14::MlInspectSurfaceReferenceItemsJoin14Next(names.clone(), types.clone())
                                                };
                                                match ml_s12 {
                                                    crate::translation::surface_reference_analysis_rules::MlInspectSurfaceReferenceItemsJoin14::MlInspectSurfaceReferenceItemsJoin14Next(names_3, types_3) => {
                                                        {
                                                            let result = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, item, "body"), &(names_3), &(types_3));
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
pub enum MlHasSurfaceStructuralIdentityJoin17 {
    MlHasSurfaceStructuralIdentityJoin17Next(Vec<String>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]

pub fn ml_has_surface_structural_identity_loop16(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut program: f64,
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
                            let result = crate::translation::surface_reference_analysis_rules::inspect_surface_reference_expression(columns, rows, size, program, node, &(names), &(types));
                            {
                                let observed_3 = if (crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(result), 1f64)) == 1f64 {
                                    {
                                        true
                                    }
                                } else {
                                    observed
                                };
                                {
                                    let ml_s14 = if binding {
                                        {
                                            let names_2 = crate::translation::frontend_rules::ml_array::append(names.clone(), vec![crate::translation::surface_reference_analysis_rules::read_surface_reference_text(columns, size, crate::translation::surface_reference_analysis_rules::read_surface_reference_field(columns, rows, size, effect, "name"))]);
                                            {
                                                let types_2 = crate::translation::frontend_rules::ml_array::append(types.clone(), vec![crate::translation::surface_reference_analysis_rules::read_required_number_fact(&(result), 0f64)]);
                                                crate::translation::surface_reference_analysis_rules::MlHasSurfaceStructuralIdentityJoin17::MlHasSurfaceStructuralIdentityJoin17Next(names_2, types_2)
                                            }
                                        }
                                    } else {
                                        crate::translation::surface_reference_analysis_rules::MlHasSurfaceStructuralIdentityJoin17::MlHasSurfaceStructuralIdentityJoin17Next(names.clone(), types.clone())
                                    };
                                    match ml_s14 {
                                        crate::translation::surface_reference_analysis_rules::MlHasSurfaceStructuralIdentityJoin17::MlHasSurfaceStructuralIdentityJoin17Next(names_3, types_3) => {
                                            {
                                                let offset_2 = offset + 1f64;
                                                {
                                                    (observed, names, types, offset) = (observed_3, names_3.clone(), types_3.clone(), offset_2);
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
