// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Result;
use super::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Result;
use super::callable_append_name;
use super::callable_bound;
use super::callable_cell_observations;
use super::callable_element;
use super::callable_field;
use super::callable_text;
use super::callable_truthy;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_callable_closure_parameter_error_loop43(
    names: &[String],
    mut at: f64,
    mut previous: f64,
) -> self::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Result {
    loop {
        return if previous < at {
            if (self::read_required_text_fact(names, previous))
                == (self::read_required_text_fact(names, at))
            {
                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Return(String::from("duplicate closure parameter"))
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
            self::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_callable_closure_parameter_error_loop41(
    names: &[String],
    mut at: f64,
) -> self::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Result {
    loop {
        return if at < (names.len() as f64) {
            {
                let previous = 0f64;
                {
                    let source_ml_source2_s164 = self::source_ml_source1_temporary_read_callable_closure_parameter_error_loop43(names, at, previous);
                    match source_ml_source2_s164 {
                        self::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Done => {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                        self::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop43Return(source_ml_source1_temporary_result43) => {
                            self::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Return(source_ml_source1_temporary_result43)
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop41Done(at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCallableAppendNameLoop44Result {
    MlSource1TemporaryCallableAppendNameLoop44Done,
    MlSource1TemporaryCallableAppendNameLoop44Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_append_name_loop44(
    texts: &[String],
    identities: &[f64],
    mut identity: f64,
    mut at: f64,
) -> self::MlSource1TemporaryCallableAppendNameLoop44Result {
    loop {
        return if at < (identities.len() as f64) {
            if (self::callable_text(texts, self::read_required_number_fact(identities, at)))
                == (self::callable_text(texts, identity))
            {
                self::MlSource1TemporaryCallableAppendNameLoop44Result::MlSource1TemporaryCallableAppendNameLoop44Return(identities.to_vec())
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
            self::MlSource1TemporaryCallableAppendNameLoop44Result::MlSource1TemporaryCallableAppendNameLoop44Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableStatementCapturesLoop45Result {
    MlSource1TemporaryPlanCallableStatementCapturesLoop45Done(Vec<f64>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableStatementCapturesJoin46 {
    MlSource1TemporaryPlanCallableStatementCapturesJoin46Next(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_statement_captures_loop45(
    texts: &[String],
    outer: &[String],
    events: &[f64],
    mut assigned: Vec<f64>,
    mut free: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableStatementCapturesLoop45Result {
    loop {
        return if at < (events.len() as f64) {
            {
                let name = self::read_required_number_fact(events, (at + 1f64));
                {
                    let source_ml_source2_s165 = if self::callable_bound(
                        outer,
                        &(self::callable_text(texts, name)),
                    ) {
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
                                    self::MlSource1TemporaryPlanCallableStatementCapturesJoin46::MlSource1TemporaryPlanCallableStatementCapturesJoin46Next(assigned_2, free_3)
                                }
                            } else {
                                self::MlSource1TemporaryPlanCallableStatementCapturesJoin46::MlSource1TemporaryPlanCallableStatementCapturesJoin46Next(assigned, free_3)
                            }
                        }
                    } else {
                        self::MlSource1TemporaryPlanCallableStatementCapturesJoin46::MlSource1TemporaryPlanCallableStatementCapturesJoin46Next(assigned, free)
                    };
                    match source_ml_source2_s165 {
                        self::MlSource1TemporaryPlanCallableStatementCapturesJoin46::MlSource1TemporaryPlanCallableStatementCapturesJoin46Next(assigned_3, free_4) => {
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
            self::MlSource1TemporaryPlanCallableStatementCapturesLoop45Result::MlSource1TemporaryPlanCallableStatementCapturesLoop45Done(assigned, free)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop51Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop51Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop51(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut tokens: f64,
    mut locals: Vec<f64>,
    mut mutable: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop51Result {
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop51Result::MlSource1TemporaryPlanCallableClosureScopeLoop51Done(locals, mutable)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop50Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop50Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop50(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_mutable: f64,
    mut mutable: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop50Result {
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop50Result::MlSource1TemporaryPlanCallableClosureScopeLoop50Done(mutable, at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop49Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop49Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop49(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_temporal: f64,
    mut temporal: Vec<f64>,
    names: &[String],
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop49Result {
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
                                    let source_ml_source2_concat_value166 = temporal;
                                    {
                                        let source_ml_source2_concat_value167 = vec![name];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value166,
                                            ),
                                            source_ml_source2_concat_value167,
                                        )
                                    }
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop49Result::MlSource1TemporaryPlanCallableClosureScopeLoop49Done(temporal, at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop48Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop48Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop48(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_locals: f64,
    mut locals: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop48Result {
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop48Result::MlSource1TemporaryPlanCallableClosureScopeLoop48Done(locals, at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop47Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop47Done(Vec<String>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop47(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut tokens: f64,
    mut names: Vec<String>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop47Result {
    loop {
        if (tokens >= 0f64) && (at < (self::read_required_number_fact(counts, tokens))) {
            {
                let token = self::callable_element(starts, counts, targets, tokens, at);
                {
                    let names_2 = {
                        let source_ml_source2_concat_value168 = names;
                        {
                            let source_ml_source2_concat_value169 = vec![self::callable_text(
                                texts,
                                self::callable_field(
                                    starts, counts, labels, targets, token, "value",
                                ),
                            )];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<String>::new(),
                                    source_ml_source2_concat_value168,
                                ),
                                source_ml_source2_concat_value169,
                            )
                        }
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop47Result::MlSource1TemporaryPlanCallableClosureScopeLoop47Done(names, at)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_surface_parameters_loop52(
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
                        let source_ml_source2_concat_value170 = plan;
                        {
                            let source_ml_source2_concat_value171 = vec![
                                self::callable_field(
                                    starts, counts, labels, targets, token, "value",
                                ),
                                self::callable_field(
                                    starts, counts, labels, targets, token, "start",
                                ),
                                self::callable_field(starts, counts, labels, targets, token, "end"),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value170,
                                ),
                                source_ml_source2_concat_value171,
                            )
                        }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_callable_parameters_loop53(
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
                    let source_ml_source2_concat_value172 = parameters;
                    {
                        let source_ml_source2_concat_value173 = vec![if format == "Rust" {
                            format!(
                                "{}: {}",
                                self::read_required_text_fact(names, at),
                                self::read_required_text_fact(types, at)
                            )
                        } else {
                            format!(
                                "({} : {})",
                                self::read_required_text_fact(names, at),
                                self::read_required_text_fact(types, at)
                            )
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<String>::new(),
                                source_ml_source2_concat_value172,
                            ),
                            source_ml_source2_concat_value173,
                        )
                    }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_mutable_cells_loop55(
    texts: &[String],
    captures: &[f64],
    assigned: &[String],
    mut at: f64,
    mut cells: Vec<f64>,
) -> Vec<f64> {
    loop {
        if at < (captures.len() as f64) {
            {
                let cells_3 = if self::callable_bound(
                    assigned,
                    &(self::callable_text(texts, self::read_required_number_fact(captures, at))),
                ) {
                    {
                        {
                            let source_ml_source2_concat_value174 = cells;
                            {
                                let source_ml_source2_concat_value175 =
                                    vec![self::read_required_number_fact(captures, at)];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value174,
                                    ),
                                    source_ml_source2_concat_value175,
                                )
                            }
                        }
                    }
                } else {
                    cells
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (at, cells) = (at_2, cells_3);
                        continue;
                    }
                }
            }
        }
        return { cells };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableMutableCellsLoop54Result {
    MlSource1TemporaryPlanCallableMutableCellsLoop54Done(Vec<f64>, Vec<String>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableMutableCellsJoin56 {
    MlSource1TemporaryPlanCallableMutableCellsJoin56Next(Vec<f64>, Vec<String>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_mutable_cells_loop54(
    texts: &[String],
    events: &[f64],
    mut captures: Vec<f64>,
    mut assigned: Vec<String>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableMutableCellsLoop54Result {
    loop {
        return if at < (events.len() as f64) {
            {
                let source_ml_source2_s178 = if (self::read_required_number_fact(events, at))
                    == 0f64
                {
                    {
                        let captures_2 = self::callable_append_name(
                            texts,
                            &(captures),
                            self::read_required_number_fact(events, (at + 1f64)),
                        );
                        self::MlSource1TemporaryPlanCallableMutableCellsJoin56::MlSource1TemporaryPlanCallableMutableCellsJoin56Next(captures_2, assigned)
                    }
                } else {
                    {
                        let assigned_2 = {
                            let source_ml_source2_concat_value179 = assigned;
                            {
                                let source_ml_source2_concat_value180 = vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(events, (at + 1f64)),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source2_concat_value179,
                                    ),
                                    source_ml_source2_concat_value180,
                                )
                            }
                        };
                        self::MlSource1TemporaryPlanCallableMutableCellsJoin56::MlSource1TemporaryPlanCallableMutableCellsJoin56Next(captures, assigned_2)
                    }
                };
                match source_ml_source2_s178 {
                    self::MlSource1TemporaryPlanCallableMutableCellsJoin56::MlSource1TemporaryPlanCallableMutableCellsJoin56Next(captures_3, assigned_3) => {
                        {
                            let at_2 = at + 2f64;
                            {
                                (captures, assigned, at) = (captures_3, assigned_3, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanCallableMutableCellsLoop54Result::MlSource1TemporaryPlanCallableMutableCellsLoop54Done(captures, assigned, at)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_cell_observations_loop58(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut events: Vec<f64>,
    mut assigned: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (assigned >= 0f64) && (at < (self::read_required_number_fact(counts, assigned))) {
            {
                let events_2 = {
                    let source_ml_source2_concat_value181 = events;
                    {
                        let source_ml_source2_concat_value182 = vec![
                            1f64,
                            self::callable_element(starts, counts, targets, assigned, at),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value181,
                            ),
                            source_ml_source2_concat_value182,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (events, at) = (events_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { events };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCallableCellObservationsLoop57Result {
    MlSource1TemporaryCallableCellObservationsLoop57Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_cell_observations_loop57(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut events: Vec<f64>,
    mut captured: f64,
    mut at: f64,
) -> self::MlSource1TemporaryCallableCellObservationsLoop57Result {
    loop {
        if (captured >= 0f64) && (at < (self::read_required_number_fact(counts, captured))) {
            {
                let events_2 = {
                    let source_ml_source2_concat_value183 = events;
                    {
                        let source_ml_source2_concat_value184 = vec![
                            0f64,
                            self::callable_element(starts, counts, targets, captured, at),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value183,
                            ),
                            source_ml_source2_concat_value184,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (events, at) = (events_2, at_2);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryCallableCellObservationsLoop57Result::MlSource1TemporaryCallableCellObservationsLoop57Done(events, at)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_cell_observations_loop59(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    mut events: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            {
                let label = self::read_required_text_fact(labels, at);
                {
                    let events_3 = if ((((label != "span") && (label != "type"))
                        && (label != "data"))
                        && (label != "captures"))
                        && (label != "assignedCaptures")
                    {
                        {
                            {
                                let source_ml_source2_concat_value185 = events;
                                {
                                    let source_ml_source2_concat_value186 =
                                        self::callable_cell_observations(
                                            forms,
                                            texts,
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            self::read_required_number_fact(targets, at),
                                        );
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value185,
                                        ),
                                        source_ml_source2_concat_value186,
                                    )
                                }
                            }
                        }
                    } else {
                        events
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (events, at) = (events_3, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { events };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_callable_source_renaming_loop60(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut scope: f64,
    mut bindings: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (scope >= 0f64) && (at < (self::read_required_number_fact(counts, scope))) {
            {
                let binding = self::callable_element(starts, counts, targets, scope, at);
                {
                    let source =
                        self::callable_field(starts, counts, labels, targets, binding, "source");
                    {
                        let target =
                            self::callable_field(starts, counts, labels, targets, binding, "ir");
                        {
                            let reference = self::callable_truthy(
                                forms,
                                texts,
                                self::callable_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    binding,
                                    "reference",
                                ),
                            );
                            {
                                let bindings_2 = {
                                    let source_ml_source2_concat_value187 = bindings;
                                    {
                                        let source_ml_source2_concat_value188 = vec![
                                            source,
                                            target,
                                            if reference { 1f64 } else { 0f64 },
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value187,
                                            ),
                                            source_ml_source2_concat_value188,
                                        )
                                    }
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (bindings, at) = (bindings_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { bindings };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCallableRenamingBindingLoop61Result {
    MlSource1TemporaryCallableRenamingBindingLoop61Done,
    MlSource1TemporaryCallableRenamingBindingLoop61Return(Vec<f64>),
}
