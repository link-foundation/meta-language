// Generated from complete callable-source-rules.js through the checked translator.
// Do not edit by hand.
use super::*;

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_callable_outer_assignments_loop32(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            {
                let result_3 = if ((self::read_required_text_fact(labels, at)) != "span")
                    && ((self::read_required_text_fact(labels, at)) != "type")
                {
                    {
                        {
                            let (ml_append_right, ml_append_left) = (
                                self::callable_outer_assignments(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::read_required_number_fact(targets, at),
                                ),
                                result,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        }
                    }
                } else {
                    result
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (result, at) = (result_3, at_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_callable_parameter_types_loop33(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut parameters: f64,
    expected_kind: &str,
    mut expected_params: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (parameters >= 0f64) && (at < (self::read_required_number_fact(counts, parameters))) {
            {
                let parameter = self::callable_element(starts, counts, targets, parameters, at);
                {
                    let explicit =
                        self::callable_field(starts, counts, labels, targets, parameter, "type");
                    {
                        let type_ = if self::callable_truthy(forms, texts, explicit) {
                            explicit
                        } else if expected_kind == "function" {
                            self::callable_element(starts, counts, targets, expected_params, at)
                        } else {
                            -1f64
                        };
                        {
                            let result_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        parameter,
                                        type_,
                                        if self::callable_truthy(forms, texts, explicit) {
                                            1f64
                                        } else {
                                            0f64
                                        },
                                    ],
                                    result,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
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
pub fn ml_plan_callable_function_type_loop34(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut parameters: f64,
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (parameters >= 0f64) && (at < (self::read_required_number_fact(counts, parameters))) {
            {
                let parameter = self::callable_element(starts, counts, targets, parameters, at);
                {
                    let plan_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![self::callable_field(
                                starts, counts, labels, targets, parameter, "type",
                            )],
                            plan,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (plan, at) = (plan_2, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { plan };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_type_constraints_loop35(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut left_params: f64,
    mut right_params: f64,
    mut pairs: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (self::read_required_number_fact(counts, left_params)) {
            {
                let pairs_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::callable_element(starts, counts, targets, left_params, at),
                            self::callable_element(starts, counts, targets, right_params, at),
                        ],
                        pairs,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (pairs, at) = (pairs_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { pairs };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_application_constraints_loop36(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut arguments_node: f64,
    mut params: f64,
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (self::read_required_number_fact(counts, params)) {
            {
                let plan_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::callable_element(starts, counts, targets, arguments_node, at),
                            self::callable_element(starts, counts, targets, params, at),
                        ],
                        plan,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (plan, at) = (plan_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { plan };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_parameter_ownership_loop37(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut owner: f64,
    mut parameters: f64,
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (parameters >= 0f64) && (at < (self::read_required_number_fact(counts, parameters))) {
            {
                let plan_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            owner,
                            at,
                            self::callable_element(starts, counts, targets, parameters, at),
                        ],
                        plan,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (plan, at) = (plan_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { plan };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_callable_function_children_loop38(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut params: f64,
    mut children: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (params >= 0f64) && (at < (self::read_required_number_fact(counts, params))) {
            {
                let children_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::callable_element(starts, counts, targets, params, at)],
                        children,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (children, at) = (children_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { children };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadCallableClosureParameterErrorLoop40Result {
    MlReadCallableClosureParameterErrorLoop40Done,
    MlReadCallableClosureParameterErrorLoop40Return(String),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_callable_closure_parameter_error_loop40(
    defaults: &[bool],
    mut at: f64,
) -> self::MlReadCallableClosureParameterErrorLoop40Result {
    loop {
        return if at < (defaults.len() as f64) {
            if self::read_required_boolean_fact(defaults, at) {
                self::MlReadCallableClosureParameterErrorLoop40Result::MlReadCallableClosureParameterErrorLoop40Return(String::from("closure parameter default"))
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
            self::MlReadCallableClosureParameterErrorLoop40Result::MlReadCallableClosureParameterErrorLoop40Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadCallableClosureParameterErrorLoop39Result {
    MlReadCallableClosureParameterErrorLoop39Done(f64),
    MlReadCallableClosureParameterErrorLoop39Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadCallableClosureParameterErrorLoop41Result {
    MlReadCallableClosureParameterErrorLoop41Done,
    MlReadCallableClosureParameterErrorLoop41Return(String),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_callable_closure_parameter_error_loop41(
    names: &[String],
    mut at: f64,
    mut previous: f64,
) -> self::MlReadCallableClosureParameterErrorLoop41Result {
    loop {
        return if previous < at {
            if (self::read_required_text_fact(names, previous))
                == (self::read_required_text_fact(names, at))
            {
                self::MlReadCallableClosureParameterErrorLoop41Result::MlReadCallableClosureParameterErrorLoop41Return(String::from("duplicate closure parameter"))
            } else {
                {
                    let previous_2 = previous + 1f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlReadCallableClosureParameterErrorLoop41Result::MlReadCallableClosureParameterErrorLoop41Done
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_callable_closure_parameter_error_loop39(
    names: &[String],
    mut at: f64,
) -> self::MlReadCallableClosureParameterErrorLoop39Result {
    loop {
        return if at < (names.len() as f64) {
            {
                let previous = 0f64;
                {
                    let ml_s27 =
                        self::ml_read_callable_closure_parameter_error_loop41(names, at, previous);
                    match ml_s27 {
                        self::MlReadCallableClosureParameterErrorLoop41Result::MlReadCallableClosureParameterErrorLoop41Done => {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                        self::MlReadCallableClosureParameterErrorLoop41Result::MlReadCallableClosureParameterErrorLoop41Return(ml_result41) => {
                            self::MlReadCallableClosureParameterErrorLoop39Result::MlReadCallableClosureParameterErrorLoop39Return(ml_result41)
                        }
                    }
                }
            }
        } else {
            self::MlReadCallableClosureParameterErrorLoop39Result::MlReadCallableClosureParameterErrorLoop39Done(at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCallableAppendNameLoop42Result {
    MlCallableAppendNameLoop42Done,
    MlCallableAppendNameLoop42Return(Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_append_name_loop42(
    texts: &[String],
    identities: &[f64],
    mut identity: f64,
    mut at: f64,
) -> self::MlCallableAppendNameLoop42Result {
    loop {
        return if at < (identities.len() as f64) {
            if (self::callable_text(texts, self::read_required_number_fact(identities, at)))
                == (self::callable_text(texts, identity))
            {
                self::MlCallableAppendNameLoop42Result::MlCallableAppendNameLoop42Return(
                    identities.to_vec(),
                )
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
            self::MlCallableAppendNameLoop42Result::MlCallableAppendNameLoop42Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableStatementCapturesLoop43Result {
    MlPlanCallableStatementCapturesLoop43Done(Vec<f64>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableStatementCapturesJoin44 {
    MlPlanCallableStatementCapturesJoin44Next(Vec<f64>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_statement_captures_loop43(
    texts: &[String],
    outer: &[String],
    events: &[f64],
    mut assigned: Vec<f64>,
    mut free: Vec<f64>,
    mut at: f64,
) -> self::MlPlanCallableStatementCapturesLoop43Result {
    loop {
        return if at < (events.len() as f64) {
            {
                let name = self::read_required_number_fact(events, (at + 1f64));
                {
                    let ml_s28 = if self::callable_bound(outer, &(self::callable_text(texts, name)))
                    {
                        {
                            let free_3 = if (self::read_required_number_fact(events, at)) == 0f64 {
                                { self::callable_append_name(texts, &(free), name) }
                            } else {
                                free
                            };
                            if (self::read_required_number_fact(events, at)) == 1f64 {
                                {
                                    let assigned_2 =
                                        self::callable_append_name(texts, &(assigned), name);
                                    self::MlPlanCallableStatementCapturesJoin44::MlPlanCallableStatementCapturesJoin44Next(assigned_2, free_3)
                                }
                            } else {
                                self::MlPlanCallableStatementCapturesJoin44::MlPlanCallableStatementCapturesJoin44Next(assigned, free_3)
                            }
                        }
                    } else {
                        self::MlPlanCallableStatementCapturesJoin44::MlPlanCallableStatementCapturesJoin44Next(assigned, free)
                    };
                    match ml_s28 {
                        self::MlPlanCallableStatementCapturesJoin44::MlPlanCallableStatementCapturesJoin44Next(assigned_3, free_4) => {
                            {
                                let at_2 = at + 2f64;
                                {
                                    (assigned, free, at) = (assigned_3, free_4, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlPlanCallableStatementCapturesLoop43Result::MlPlanCallableStatementCapturesLoop43Done(assigned, free)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableClosureScopeLoop49Result {
    MlPlanCallableClosureScopeLoop49Done(Vec<f64>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_callable_closure_scope_loop49(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut tokens: f64,
    mut locals: Vec<f64>,
    mut mutable: Vec<f64>,
    mut at: f64,
) -> self::MlPlanCallableClosureScopeLoop49Result {
    loop {
        if (tokens >= 0f64) && (at < (self::read_required_number_fact(counts, tokens))) {
            {
                let token = self::callable_element(starts, counts, targets, tokens, at);
                {
                    let name =
                        self::callable_field(starts, counts, labels, targets, token, "value");
                    {
                        let locals_2 = self::callable_append_name(texts, &(locals), name);
                        {
                            let mutable_2 = self::callable_append_name(texts, &(mutable), name);
                            {
                                let at_2 = at + 1f64;
                                {
                                    (locals, mutable, at) = (locals_2, mutable_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlPlanCallableClosureScopeLoop49Result::MlPlanCallableClosureScopeLoop49Done(
                locals, mutable,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableClosureScopeLoop48Result {
    MlPlanCallableClosureScopeLoop48Done(Vec<f64>, f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_closure_scope_loop48(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_mutable: f64,
    mut mutable: Vec<f64>,
    mut at: f64,
) -> self::MlPlanCallableClosureScopeLoop48Result {
    loop {
        if (old_mutable >= 0f64) && (at < (self::read_required_number_fact(counts, old_mutable))) {
            {
                let mutable_2 = self::callable_append_name(
                    texts,
                    &(mutable),
                    self::callable_element(starts, counts, targets, old_mutable, at),
                );
                {
                    let at_2 = at + 1f64;
                    {
                        (mutable, at) = (mutable_2, at_2);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlPlanCallableClosureScopeLoop48Result::MlPlanCallableClosureScopeLoop48Done(
                mutable, at,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableClosureScopeLoop47Result {
    MlPlanCallableClosureScopeLoop47Done(Vec<f64>, f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_callable_closure_scope_loop47(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_temporal: f64,
    mut temporal: Vec<f64>,
    names: &[String],
    mut at: f64,
) -> self::MlPlanCallableClosureScopeLoop47Result {
    loop {
        if (old_temporal >= 0f64) && (at < (self::read_required_number_fact(counts, old_temporal)))
        {
            {
                let name = self::callable_element(starts, counts, targets, old_temporal, at);
                {
                    let temporal_3 =
                        if (self::callable_bound(names, &(self::callable_text(texts, name)))) {
                            temporal
                        } else {
                            {
                                {
                                    let (ml_append_right, ml_append_left) = (vec![name], temporal);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                }
                            }
                        };
                    {
                        let at_2 = at + 1f64;
                        {
                            (temporal, at) = (temporal_3, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlPlanCallableClosureScopeLoop47Result::MlPlanCallableClosureScopeLoop47Done(
                temporal, at,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableClosureScopeLoop46Result {
    MlPlanCallableClosureScopeLoop46Done(Vec<f64>, f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_closure_scope_loop46(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_locals: f64,
    mut locals: Vec<f64>,
    mut at: f64,
) -> self::MlPlanCallableClosureScopeLoop46Result {
    loop {
        if (old_locals >= 0f64) && (at < (self::read_required_number_fact(counts, old_locals))) {
            {
                let locals_2 = self::callable_append_name(
                    texts,
                    &(locals),
                    self::callable_element(starts, counts, targets, old_locals, at),
                );
                {
                    let at_2 = at + 1f64;
                    {
                        (locals, at) = (locals_2, at_2);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlPlanCallableClosureScopeLoop46Result::MlPlanCallableClosureScopeLoop46Done(
                locals, at,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableClosureScopeLoop45Result {
    MlPlanCallableClosureScopeLoop45Done(Vec<String>, f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_callable_closure_scope_loop45(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut tokens: f64,
    mut names: Vec<String>,
    mut at: f64,
) -> self::MlPlanCallableClosureScopeLoop45Result {
    loop {
        if (tokens >= 0f64) && (at < (self::read_required_number_fact(counts, tokens))) {
            {
                let token = self::callable_element(starts, counts, targets, tokens, at);
                {
                    let names_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![self::callable_text(
                                texts,
                                self::callable_field(
                                    starts, counts, labels, targets, token, "value",
                                ),
                            )],
                            names,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (names, at) = (names_2, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlPlanCallableClosureScopeLoop45Result::MlPlanCallableClosureScopeLoop45Done(
                names, at,
            )
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_surface_parameters_loop50(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut tokens: f64,
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (tokens >= 0f64) && (at < (self::read_required_number_fact(counts, tokens))) {
            {
                let token = self::callable_element(starts, counts, targets, tokens, at);
                {
                    let plan_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![
                                self::callable_field(
                                    starts, counts, labels, targets, token, "value",
                                ),
                                self::callable_field(
                                    starts, counts, labels, targets, token, "start",
                                ),
                                self::callable_field(starts, counts, labels, targets, token, "end"),
                            ],
                            plan,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (plan, at) = (plan_2, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { plan };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_render_callable_parameters_loop51(
    format: &str,
    names: &[String],
    types: &[String],
    mut parameters: Vec<String>,
    mut at: f64,
) -> Vec<String> {
    loop {
        if at < (names.len() as f64) {
            {
                let parameters_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![if format == "Rust" {
                            format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (self::read_required_text_fact(names, at)),
                                    (String::from(": "))
                                )),
                                (self::read_required_text_fact(types, at))
                            )
                        } else {
                            format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!(
                                            "{}{}",
                                            (String::from("(")),
                                            (self::read_required_text_fact(names, at))
                                        )),
                                        (String::from(" : "))
                                    )),
                                    (self::read_required_text_fact(types, at))
                                )),
                                (String::from(")"))
                            )
                        }],
                        parameters,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (parameters, at) = (parameters_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { parameters };
    }
}
