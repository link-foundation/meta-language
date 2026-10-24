// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableAppendNameLoop44Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop47Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop48Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop49Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop50Result;
use super::MlSource1TemporaryPlanCallableClosureScopeLoop51Result;
use super::MlSource1TemporaryPlanCallableStatementCapturesLoop45Result;
use super::MlSource1TemporaryReadCallableCaptureConflictsLoop29Result;
use super::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Result;
use super::MlSource1TemporaryReadCallableClosureParameterErrorLoop42Result;
use super::callable_field;
use super::callable_tag;
use super::callable_truthy;
use super::ml_array;
use super::ml_number;
use super::read_callable_statement_uses;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_callable_append_name_loop44;
use super::source_ml_source1_temporary_callable_outer_assignments_loop34;
use super::source_ml_source1_temporary_plan_callable_application_constraints_loop38;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop47;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop48;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop49;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop50;
use super::source_ml_source1_temporary_plan_callable_closure_scope_loop51;
use super::source_ml_source1_temporary_plan_callable_function_type_loop36;
use super::source_ml_source1_temporary_plan_callable_parameter_ownership_loop39;
use super::source_ml_source1_temporary_plan_callable_parameter_types_loop35;
use super::source_ml_source1_temporary_plan_callable_statement_captures_loop45;
use super::source_ml_source1_temporary_plan_callable_surface_parameters_loop52;
use super::source_ml_source1_temporary_plan_callable_type_constraints_loop37;
use super::source_ml_source1_temporary_read_callable_capture_conflicts_loop29;
use super::source_ml_source1_temporary_read_callable_capture_conflicts_loop30;
use super::source_ml_source1_temporary_read_callable_closure_parameter_error_loop41;
use super::source_ml_source1_temporary_read_callable_closure_parameter_error_loop42;
use super::source_ml_source1_temporary_read_callable_function_children_loop40;
use super::source_ml_source1_temporary_render_callable_parameters_loop53;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_capture_conflicts(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> Vec<f64> {
    {
        let observations =
            self::callable_outer_assignments(forms, texts, starts, counts, labels, targets, node);
        {
            let assigned = Vec::<String>::new();
            {
                let closures = Vec::<f64>::new();
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s48 = self::source_ml_source1_temporary_read_callable_capture_conflicts_loop29(texts, &(observations), assigned, closures, at);
                        match source_ml_source2_s48 {
                            self::MlSource1TemporaryReadCallableCaptureConflictsLoop29Result::MlSource1TemporaryReadCallableCaptureConflictsLoop29Done(assigned_2, closures_2, at_2) => {
                                {
                                    let result = Vec::<f64>::new();
                                    {
                                        let at_3 = 0f64;
                                        {
                                            self::source_ml_source1_temporary_read_callable_capture_conflicts_loop30(texts, starts, counts, labels, targets, &(assigned_2), &(closures_2), at_3, result)
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
pub fn callable_outer_assignments(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> Vec<f64> {
    if (node < 0f64)
        || (((self::read_required_text_fact(forms, node)) != "object")
            && ((self::read_required_text_fact(forms, node)) != "array"))
    {
        Vec::<f64>::new()
    } else if (self::callable_tag(texts, starts, counts, labels, targets, node, "k")) == "lambda" {
        vec![1f64, node]
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let result_3 =
                    if (self::callable_tag(texts, starts, counts, labels, targets, node, "s"))
                        == "assign"
                    {
                        {
                            vec![
                                0f64,
                                self::callable_field(starts, counts, labels, targets, node, "name"),
                            ]
                        }
                    } else {
                        result
                    };
                {
                    let at = self::read_required_number_fact(starts, node);
                    {
                        self::source_ml_source1_temporary_callable_outer_assignments_loop34(
                            forms, texts, starts, counts, labels, targets, node, result_3, at,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_callable_function_reference(
    surface_kind: &str,
    expected_kind: &str,
    declaration_kind: &str,
) -> bool {
    (declaration_kind == "fn")
        && ((surface_kind == "functionValue")
            || ((surface_kind == "name") && (expected_kind == "function")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_callable_computed_application(
    head_kind: &str,
    path_length: f64,
    local_kind: &str,
) -> bool {
    ((head_kind != "name") && (head_kind != "dotCtor"))
        || (((head_kind == "name") && (path_length == 1f64)) && (local_kind == "function"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_application_error(
    kind: &str,
    parameters: f64,
    arguments_count: f64,
) -> String {
    if kind == "function" {
        if parameters == arguments_count {
            String::new()
        } else {
            format!(
                "function expects {} arguments",
                self::ml_number::js_number(parameters)
            )
        }
    } else {
        String::from("the called value is not a function")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_parameter_types(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    parameters: f64,
    expected: f64,
) -> Vec<f64> {
    {
        let expected_kind =
            self::callable_tag(texts, starts, counts, labels, targets, expected, "kind");
        {
            let expected_params =
                self::callable_field(starts, counts, labels, targets, expected, "params");
            {
                let result = Vec::<f64>::new();
                {
                    let at = 0f64;
                    {
                        self::source_ml_source1_temporary_plan_callable_parameter_types_loop35(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            parameters,
                            &(expected_kind),
                            expected_params,
                            result,
                            at,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_function_type(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    parameters: f64,
    result: f64,
) -> Vec<f64> {
    {
        let plan = vec![result];
        {
            let at = 0f64;
            {
                self::source_ml_source1_temporary_plan_callable_function_type_loop36(
                    starts, counts, labels, targets, parameters, plan, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_type_constraints(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    left: f64,
    right: f64,
) -> Vec<f64> {
    {
        let left_params = self::callable_field(starts, counts, labels, targets, left, "params");
        {
            let right_params =
                self::callable_field(starts, counts, labels, targets, right, "params");
            if ((left_params < 0f64) || (right_params < 0f64))
                || ((self::read_required_number_fact(counts, left_params))
                    != (self::read_required_number_fact(counts, right_params)))
            {
                vec![-1f64]
            } else {
                {
                    let pairs = Vec::<f64>::new();
                    {
                        let at = 0f64;
                        {
                            let pairs_2 = self::source_ml_source1_temporary_plan_callable_type_constraints_loop37(starts, counts, targets, left_params, right_params, pairs, at);
                            {
                                let source_ml_source2_concat_value49 = pairs_2;
                                {
                                    let source_ml_source2_concat_value50 = vec![
                                        self::callable_field(
                                            starts, counts, labels, targets, left, "ret",
                                        ),
                                        self::callable_field(
                                            starts, counts, labels, targets, right, "ret",
                                        ),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value49,
                                        ),
                                        source_ml_source2_concat_value50,
                                    )
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_application_constraints(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    fn_: f64,
    arguments_node: f64,
    candidate: f64,
) -> Vec<f64> {
    {
        let params = self::callable_field(starts, counts, labels, targets, fn_, "params");
        if ((((self::callable_tag(texts, starts, counts, labels, targets, fn_, "kind"))
            != "function")
            || (params < 0f64))
            || (arguments_node < 0f64))
            || ((self::read_required_number_fact(counts, params))
                != (self::read_required_number_fact(counts, arguments_node)))
        {
            vec![-1f64, fn_, candidate]
        } else {
            {
                let plan = vec![self::callable_field(
                    starts, counts, labels, targets, fn_, "ret",
                )];
                {
                    let at = 0f64;
                    {
                        self::source_ml_source1_temporary_plan_callable_application_constraints_loop38(starts, counts, targets, arguments_node, params, plan, at)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_callable_structural_inference(kind: &str, type_kind: &str, operation: &str) -> bool {
    if ((((kind == "lambda") || (kind == "functionValue")) || (type_kind == "function"))
        || ((kind == "stringTest") && (operation == "includes")))
        || (((((((((((((((((((kind == "none") || (kind == "coalesce"))
            || (kind == "optionalField"))
            || (kind == "record"))
            || (kind == "method"))
            || (kind == "map"))
            || (kind == "set"))
            || (kind == "arrayFrom"))
            || (kind == "arrayIsArray"))
            || (kind == "copySlice"))
            || (kind == "forOfIterable"))
            || (kind == "stringPoints"))
            || (kind == "cellAllocate"))
            || (kind == "cellRead"))
            || (kind == "cellWrite"))
            || (kind == "referenceWrite"))
            || (kind == "index"))
            || (kind == "tuple"))
            || (kind == "tupleIndex"))
    {
        true
    } else {
        ((((type_kind == "option") || (type_kind == "tuple")) || (type_kind == "record"))
            || (type_kind == "map"))
            || (type_kind == "set")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_callable_named_inference(
    head_kind: &str,
    path_length: f64,
    local_present: bool,
) -> bool {
    (head_kind == "name") && ((path_length != 1f64) || !local_present)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_callable_application_inference_result(
    kind: &str,
    parameters: f64,
    arguments_count: f64,
) -> bool {
    (kind == "function")
        && ((parameters).partial_cmp(&(arguments_count)) == Some(std::cmp::Ordering::Equal))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_parameter_ownership(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    owner: f64,
) -> Vec<f64> {
    {
        let parameters = self::callable_field(starts, counts, labels, targets, owner, "params");
        {
            let plan = Vec::<f64>::new();
            {
                let at = 0f64;
                {
                    self::source_ml_source1_temporary_plan_callable_parameter_ownership_loop39(
                        starts, counts, targets, owner, parameters, plan, at,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_function_children(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    type_: f64,
) -> Vec<f64> {
    {
        let params = self::callable_field(starts, counts, labels, targets, type_, "params");
        {
            let children = Vec::<f64>::new();
            {
                let at = 0f64;
                {
                    let children_2 =
                        self::source_ml_source1_temporary_read_callable_function_children_loop40(
                            starts, counts, targets, params, children, at,
                        );
                    {
                        let source_ml_source2_concat_value51 = children_2;
                        {
                            let source_ml_source2_concat_value52 = vec![self::callable_field(
                                starts, counts, labels, targets, type_, "ret",
                            )];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value51,
                                ),
                                source_ml_source2_concat_value52,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_lambda_inference(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    lambda: f64,
    expected: f64,
) -> Vec<f64> {
    {
        let explicit = self::callable_field(starts, counts, labels, targets, lambda, "ret");
        {
            let present = self::callable_truthy(forms, texts, explicit);
            {
                let contextual =
                    (self::callable_tag(texts, starts, counts, labels, targets, expected, "kind"))
                        == "function";
                {
                    let ret = if present {
                        explicit
                    } else if contextual {
                        self::callable_field(starts, counts, labels, targets, expected, "ret")
                    } else {
                        -1f64
                    };
                    {
                        let params = self::plan_callable_parameter_types(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(starts, counts, labels, targets, lambda, "params"),
                            expected,
                        );
                        {
                            let source_ml_source2_concat_value53 =
                                vec![ret, if present { 1f64 } else { 0f64 }];
                            {
                                let source_ml_source2_concat_value54 = params;
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value53,
                                    ),
                                    source_ml_source2_concat_value54,
                                )
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
pub fn read_callable_closure_prefix_error(
    expression: bool,
    token_kind: &str,
    token_value: &str,
) -> String {
    if expression && (token_kind == "identifier") {
        String::from("named function expression")
    } else if token_value == "*" {
        String::from("generator function")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_closure_parameter_error(names: &[String], defaults: &[bool]) -> String {
    {
        let at = 0f64;
        {
            let source_ml_source2_s55 =
                self::source_ml_source1_temporary_read_callable_closure_parameter_error_loop41(
                    names, at,
                );
            match source_ml_source2_s55 {
                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Done(at_2) => {
                    {
                        let at_3 = 0f64;
                        {
                            let source_ml_source2_s56 = self::source_ml_source1_temporary_read_callable_closure_parameter_error_loop42(defaults, at_3);
                            match source_ml_source2_s56 {
                                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop42Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop42Done => {
                                    String::new()
                                }
                                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop42Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop42Return(source_ml_source1_temporary_result42) => {
                                    source_ml_source1_temporary_result42
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Return(source_ml_source1_temporary_result42_2) => {
                    source_ml_source1_temporary_result42_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_append_name(texts: &[String], identities: &[f64], identity: f64) -> Vec<f64> {
    {
        let at = 0f64;
        {
            let source_ml_source2_s57 =
                self::source_ml_source1_temporary_callable_append_name_loop44(
                    texts, identities, identity, at,
                );
            match source_ml_source2_s57 {
                self::MlSource1TemporaryCallableAppendNameLoop44Result::MlSource1TemporaryCallableAppendNameLoop44Done => {
                    {
                        let source_ml_source2_concat_value58 = identities.to_vec();
                        {
                            let source_ml_source2_concat_value59 = vec![identity];
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value58), source_ml_source2_concat_value59)
                        }
                    }
                }
                self::MlSource1TemporaryCallableAppendNameLoop44Result::MlSource1TemporaryCallableAppendNameLoop44Return(source_ml_source1_temporary_result44) => {
                    source_ml_source1_temporary_result44
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_statement_captures(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    statements: f64,
    bound: &[String],
    outer: &[String],
) -> Vec<f64> {
    {
        let events = self::read_callable_statement_uses(
            forms, texts, starts, counts, labels, targets, statements, bound,
        );
        {
            let assigned = Vec::<f64>::new();
            {
                let free = Vec::<f64>::new();
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s60 = self::source_ml_source1_temporary_plan_callable_statement_captures_loop45(texts, outer, &(events), assigned, free, at);
                        match source_ml_source2_s60 {
                            self::MlSource1TemporaryPlanCallableStatementCapturesLoop45Result::MlSource1TemporaryPlanCallableStatementCapturesLoop45Done(assigned_2, free_2) => {
                                {
                                    let source_ml_source2_concat_value61 = vec![(assigned_2.len() as f64)];
                                    {
                                        let source_ml_source2_concat_value62 = assigned_2;
                                        {
                                            let source_ml_source2_concat_value63 = free_2;
                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value61), source_ml_source2_concat_value62), source_ml_source2_concat_value63)
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
pub fn plan_callable_closure_scope(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    outer: f64,
    tokens: f64,
) -> Vec<f64> {
    {
        let old_locals = self::callable_field(starts, counts, labels, targets, outer, "locals");
        {
            let old_temporal = self::callable_field(starts, counts, labels, targets, outer, "tdz");
            {
                let old_mutable =
                    self::callable_field(starts, counts, labels, targets, outer, "mutable");
                {
                    let locals = Vec::<f64>::new();
                    {
                        let temporal = Vec::<f64>::new();
                        {
                            let mutable = Vec::<f64>::new();
                            {
                                let names = Vec::<String>::new();
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source2_s64 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop47(texts, starts, counts, labels, targets, tokens, names, at);
                                        match source_ml_source2_s64 {
                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop47Result::MlSource1TemporaryPlanCallableClosureScopeLoop47Done(names_2, at_2) => {
                                                {
                                                    let at_3 = 0f64;
                                                    {
                                                        let source_ml_source2_s65 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop48(texts, starts, counts, targets, old_locals, locals, at_3);
                                                        match source_ml_source2_s65 {
                                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop48Result::MlSource1TemporaryPlanCallableClosureScopeLoop48Done(locals_2, at_4) => {
                                                                {
                                                                    let at_5 = 0f64;
                                                                    {
                                                                        let source_ml_source2_s66 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop49(texts, starts, counts, targets, old_temporal, temporal, &(names_2), at_5);
                                                                        match source_ml_source2_s66 {
                                                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop49Result::MlSource1TemporaryPlanCallableClosureScopeLoop49Done(temporal_2, at_6) => {
                                                                                {
                                                                                    let at_7 = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s67 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop50(texts, starts, counts, targets, old_mutable, mutable, at_7);
                                                                                        match source_ml_source2_s67 {
                                                                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop50Result::MlSource1TemporaryPlanCallableClosureScopeLoop50Done(mutable_2, at_8) => {
                                                                                                {
                                                                                                    let at_9 = 0f64;
                                                                                                    {
                                                                                                        let source_ml_source2_s68 = self::source_ml_source1_temporary_plan_callable_closure_scope_loop51(texts, starts, counts, labels, targets, tokens, locals_2, mutable_2, at_9);
                                                                                                        match source_ml_source2_s68 {
                                                                                                            self::MlSource1TemporaryPlanCallableClosureScopeLoop51Result::MlSource1TemporaryPlanCallableClosureScopeLoop51Done(locals_3, mutable_3) => {
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value69 = vec![(locals_3.len() as f64), (temporal_2.len() as f64), (mutable_3.len() as f64)];
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value70 = locals_3;
                                                                                                                        {
                                                                                                                            let source_ml_source2_concat_value71 = temporal_2;
                                                                                                                            {
                                                                                                                                let source_ml_source2_concat_value72 = mutable_3;
                                                                                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value69), source_ml_source2_concat_value70), source_ml_source2_concat_value71), source_ml_source2_concat_value72)
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_surface_parameters(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    tokens: f64,
) -> Vec<f64> {
    {
        let plan = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                self::source_ml_source1_temporary_plan_callable_surface_parameters_loop52(
                    starts, counts, labels, targets, tokens, plan, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_callable_assigned_capture_field(count: f64) -> bool {
    count > 0f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_surface_constructor(form: &str, known_function: bool) -> String {
    if form == "closure" {
        String::from("lambda")
    } else if form == "computed-call" {
        String::from("app")
    } else if (form == "name") && known_function {
        String::from("functionValue")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_function_reference(format: &str, fn_: &str) -> String {
    if format == "Rust" {
        format!("std::rc::Rc::new({})", fn_.to_owned())
    } else {
        fn_.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_application(format: &str, fn_: &str, args: &[String]) -> String {
    if (format == "Rust") || (format == "JavaScript") {
        format!(
            "({})({})",
            fn_.to_owned(),
            args.join((String::from(", ")).as_str())
        )
    } else {
        format!(
            "({} {})",
            fn_.to_owned(),
            args.join((String::from(" ")).as_str())
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_lambda(
    format: &str,
    names: &[String],
    types: &[String],
    ret: &str,
    body: &str,
    copies: &[String],
) -> String {
    if format == "JavaScript" {
        format!(
            "(({}) => {})",
            names.join((String::from(", ")).as_str()),
            body.to_owned()
        )
    } else {
        {
            let parameters = self::render_callable_parameters(format, names, types);
            if format == "Rust" {
                {
                    let closure = format!(
                        "std::rc::Rc::new(move |{}| -> {} {{ {} }})",
                        parameters.join((String::from(", ")).as_str()),
                        ret.to_owned(),
                        body.to_owned()
                    );
                    {
                        let ml_library_condition_1 = {
                            let ml_value = copies.len() as f64;
                            ml_value != 0f64 && !ml_value.is_nan()
                        };
                        if ml_library_condition_1 {
                            format!(
                                "{{ {} {closure} }}",
                                copies.join((String::from(" ")).as_str())
                            )
                        } else {
                            closure
                        }
                    }
                }
            } else {
                format!(
                    "(fun {} => {})",
                    parameters.join((String::from(" ")).as_str()),
                    body.to_owned()
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_parameters(format: &str, names: &[String], types: &[String]) -> Vec<String> {
    {
        let parameters = Vec::<String>::new();
        {
            let at = 0f64;
            {
                self::source_ml_source1_temporary_render_callable_parameters_loop53(
                    format, names, types, parameters, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_function_type(format: &str, parameters: &[String], ret: &str) -> String {
    format!(
        "({})",
        ({
            let source_ml_source2_concat_value73 = parameters.to_vec();
            {
                let source_ml_source2_concat_value74 = vec![ret.to_owned()];
                self::ml_array::append(
                    self::ml_array::append(Vec::<String>::new(), source_ml_source2_concat_value73),
                    source_ml_source2_concat_value74,
                )
            }
        })
        .join(
            (if format == "Lean" {
                String::from(" → ")
            } else {
                String::from(" -> ")
            })
            .as_str()
        )
    )
}
