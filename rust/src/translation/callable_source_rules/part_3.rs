// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableStatementUsesLoop32Result;
use super::MlSource1TemporaryReadCallableCaptureConflictsLoop34Result;
use super::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Result;
use super::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Result;
use super::callable_bound;
use super::callable_expression_events;
use super::callable_field;
use super::callable_nested_events;
use super::callable_tag;
use super::callable_text;
use super::callable_truthy;
use super::ml_array;
use super::ml_number;
use super::read_callable_statement_role;
use super::read_callable_statement_uses;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_callable_outer_assignments_loop39;
use super::source_ml_source1_temporary_callable_statement_uses_loop31;
use super::source_ml_source1_temporary_callable_statement_uses_loop32;
use super::source_ml_source1_temporary_callable_statement_uses_loop33;
use super::source_ml_source1_temporary_plan_callable_application_constraints_loop43;
use super::source_ml_source1_temporary_plan_callable_function_type_loop41;
use super::source_ml_source1_temporary_plan_callable_parameter_ownership_loop44;
use super::source_ml_source1_temporary_plan_callable_parameter_types_loop40;
use super::source_ml_source1_temporary_plan_callable_type_constraints_loop42;
use super::source_ml_source1_temporary_read_callable_capture_conflicts_loop34;
use super::source_ml_source1_temporary_read_callable_capture_conflicts_loop35;
use super::source_ml_source1_temporary_read_callable_closure_parameter_error_loop46;
use super::source_ml_source1_temporary_read_callable_closure_parameter_error_loop47;
use super::source_ml_source1_temporary_read_callable_function_children_loop45;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn callable_statement_uses(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    statement: f64,
    bound: &[String],
) -> Vec<f64> {
    {
        let role = self::read_callable_statement_role(
            &(self::callable_tag(texts, starts, counts, labels, targets, statement, "s")),
        );
        {
            let name = self::callable_field(starts, counts, labels, targets, statement, "name");
            {
                let value =
                    self::callable_field(starts, counts, labels, targets, statement, "value");
                if role == "declare" {
                    {
                        let source_ml_source2_concat_value32 = self::callable_expression_events(
                            forms, texts, starts, counts, labels, targets, value, bound,
                        );
                        {
                            let source_ml_source2_concat_value33 = vec![2f64, name];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value32,
                                ),
                                source_ml_source2_concat_value33,
                            )
                        }
                    }
                } else if role == "assign" {
                    {
                        let reads = self::callable_expression_events(
                            forms, texts, starts, counts, labels, targets, value, bound,
                        );
                        if self::callable_bound(bound, &(self::callable_text(texts, name))) {
                            reads
                        } else {
                            {
                                let source_ml_source2_concat_value34 = reads;
                                {
                                    let source_ml_source2_concat_value35 =
                                        vec![0f64, name, 1f64, name];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value34,
                                        ),
                                        source_ml_source2_concat_value35,
                                    )
                                }
                            }
                        }
                    }
                } else if role == "read" {
                    self::callable_expression_events(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::callable_field(starts, counts, labels, targets, statement, "expr"),
                        bound,
                    )
                } else if role == "exception" {
                    self::callable_expression_events(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::callable_field(
                            starts,
                            counts,
                            labels,
                            targets,
                            statement,
                            "exception",
                        ),
                        bound,
                    )
                } else if role == "reference" {
                    {
                        let source_ml_source2_concat_value36 = self::callable_expression_events(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "object",
                            ),
                            bound,
                        );
                        {
                            let source_ml_source2_concat_value37 = self::callable_expression_events(
                                forms,
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                self::callable_field(
                                    starts, counts, labels, targets, statement, "index",
                                ),
                                bound,
                            );
                            {
                                let source_ml_source2_concat_value38 =
                                    self::callable_expression_events(
                                        forms, texts, starts, counts, labels, targets, value, bound,
                                    );
                                self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value36,
                                        ),
                                        source_ml_source2_concat_value37,
                                    ),
                                    source_ml_source2_concat_value38,
                                )
                            }
                        }
                    }
                } else if role == "block" {
                    {
                        let events = self::read_callable_statement_uses(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "body",
                            ),
                            bound,
                        );
                        if self::callable_truthy(
                            forms,
                            texts,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "inline",
                            ),
                        ) {
                            events
                        } else {
                            self::callable_nested_events(&(events))
                        }
                    }
                } else if (role == "branch") || (role == "loop") {
                    {
                        let reads_2 = self::callable_expression_events(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "cond",
                            ),
                            bound,
                        );
                        if role == "loop" {
                            {
                                let source_ml_source2_concat_value39 = reads_2;
                                {
                                    let source_ml_source2_concat_value40 =
                                        self::callable_nested_events(
                                            &(self::read_callable_statement_uses(
                                                forms,
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                self::callable_field(
                                                    starts, counts, labels, targets, statement,
                                                    "body",
                                                ),
                                                bound,
                                            )),
                                        );
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value39,
                                        ),
                                        source_ml_source2_concat_value40,
                                    )
                                }
                            }
                        } else {
                            {
                                let yes = self::read_callable_statement_uses(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::callable_field(
                                        starts, counts, labels, targets, statement, "then",
                                    ),
                                    bound,
                                );
                                {
                                    let no = self::read_callable_statement_uses(
                                        forms,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        self::callable_field(
                                            starts, counts, labels, targets, statement, "else",
                                        ),
                                        bound,
                                    );
                                    {
                                        let source_ml_source2_concat_value41 = reads_2;
                                        {
                                            let source_ml_source2_concat_value42 =
                                                self::callable_nested_events(&(yes));
                                            {
                                                let source_ml_source2_concat_value43 =
                                                    self::callable_nested_events(&(no));
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source2_concat_value41,
                                                        ),
                                                        source_ml_source2_concat_value42,
                                                    ),
                                                    source_ml_source2_concat_value43,
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if role == "switch" {
                    {
                        let events_2 = self::callable_expression_events(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts,
                                counts,
                                labels,
                                targets,
                                statement,
                                "discriminant",
                            ),
                            bound,
                        );
                        {
                            let clauses = self::callable_field(
                                starts, counts, labels, targets, statement, "clauses",
                            );
                            {
                                let at = 0f64;
                                {
                                    self::source_ml_source1_temporary_callable_statement_uses_loop31(
                                        forms, texts, starts, counts, labels, targets, bound,
                                        events_2, clauses, at,
                                    )
                                }
                            }
                        }
                    }
                } else if role == "for" {
                    {
                        let initial = self::read_callable_statement_uses(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "init",
                            ),
                            bound,
                        );
                        {
                            let inner = bound.to_vec();
                            {
                                let at_2 = 0f64;
                                {
                                    let source_ml_source2_s44 = self::source_ml_source1_temporary_callable_statement_uses_loop32(texts, &(initial), inner, at_2);
                                    match source_ml_source2_s44 {
                                                                        self::MlSource1TemporaryCallableStatementUsesLoop32Result::MlSource1TemporaryCallableStatementUsesLoop32Done(inner_2, at_3) => {
                                                                            {
                                                                                let events_4 = self::callable_nested_events(&(initial));
                                                                                {
                                                                                    let events_5 = {
                                                                                        let source_ml_source2_concat_value45 = events_4;
                                                                                        {
                                                                                            let source_ml_source2_concat_value46 = self::callable_expression_events(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "cond"), &(inner_2));
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value45), source_ml_source2_concat_value46)
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let update = self::read_callable_statement_uses(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "update"), &(inner_2));
                                                                                        {
                                                                                            let at_4 = 0f64;
                                                                                            {
                                                                                                let inner_3 = self::source_ml_source1_temporary_callable_statement_uses_loop33(texts, inner_2, at_4, &(update));
                                                                                                {
                                                                                                    let source_ml_source2_concat_value47 = events_5;
                                                                                                    {
                                                                                                        let source_ml_source2_concat_value48 = self::callable_nested_events(&(update));
                                                                                                        {
                                                                                                            let source_ml_source2_concat_value49 = self::callable_nested_events(&(self::read_callable_statement_uses(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "body"), &(inner_3))));
                                                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value47), source_ml_source2_concat_value48), source_ml_source2_concat_value49)
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
                    Vec::<f64>::new()
                }
            }
        }
    }
}

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
                        let source_ml_source2_s50 = self::source_ml_source1_temporary_read_callable_capture_conflicts_loop34(texts, &(observations), assigned, closures, at);
                        match source_ml_source2_s50 {
                            self::MlSource1TemporaryReadCallableCaptureConflictsLoop34Result::MlSource1TemporaryReadCallableCaptureConflictsLoop34Done(assigned_2, closures_2, at_2) => {
                                {
                                    let result = Vec::<f64>::new();
                                    {
                                        let at_3 = 0f64;
                                        {
                                            self::source_ml_source1_temporary_read_callable_capture_conflicts_loop35(texts, starts, counts, labels, targets, &(assigned_2), &(closures_2), at_3, result)
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
                        self::source_ml_source1_temporary_callable_outer_assignments_loop39(
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
            format!("function expects {} arguments", {
                let source_ml_source2_string_argument51 = parameters;
                self::ml_number::js_number(source_ml_source2_string_argument51)
            })
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
                        self::source_ml_source1_temporary_plan_callable_parameter_types_loop40(
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
                self::source_ml_source1_temporary_plan_callable_function_type_loop41(
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
                            let pairs_2 = self::source_ml_source1_temporary_plan_callable_type_constraints_loop42(starts, counts, targets, left_params, right_params, pairs, at);
                            {
                                let source_ml_source2_concat_value52 = pairs_2;
                                {
                                    let source_ml_source2_concat_value53 = vec![
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
                                            source_ml_source2_concat_value52,
                                        ),
                                        source_ml_source2_concat_value53,
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
                        self::source_ml_source1_temporary_plan_callable_application_constraints_loop43(starts, counts, targets, arguments_node, params, plan, at)
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
                    self::source_ml_source1_temporary_plan_callable_parameter_ownership_loop44(
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
                        self::source_ml_source1_temporary_read_callable_function_children_loop45(
                            starts, counts, targets, params, children, at,
                        );
                    {
                        let source_ml_source2_concat_value54 = children_2;
                        {
                            let source_ml_source2_concat_value55 = vec![self::callable_field(
                                starts, counts, labels, targets, type_, "ret",
                            )];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value54,
                                ),
                                source_ml_source2_concat_value55,
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
                            let source_ml_source2_concat_value56 =
                                vec![ret, if present { 1f64 } else { 0f64 }];
                            {
                                let source_ml_source2_concat_value57 = params;
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value56,
                                    ),
                                    source_ml_source2_concat_value57,
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
            let source_ml_source2_s58 =
                self::source_ml_source1_temporary_read_callable_closure_parameter_error_loop46(
                    names, at,
                );
            match source_ml_source2_s58 {
                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Done(at_2) => {
                    {
                        let at_3 = 0f64;
                        {
                            let source_ml_source2_s59 = self::source_ml_source1_temporary_read_callable_closure_parameter_error_loop47(defaults, at_3);
                            match source_ml_source2_s59 {
                                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Done => {
                                    String::new()
                                }
                                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Return(source_ml_source1_temporary_result47) => {
                                    source_ml_source1_temporary_result47
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Return(source_ml_source1_temporary_result47_2) => {
                    source_ml_source1_temporary_result47_2
                }
            }
        }
    }
}
