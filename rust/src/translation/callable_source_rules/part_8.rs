// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Result;
use super::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Result;
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
pub fn source_ml_source1_temporary_read_callable_closure_parameter_error_loop48(
    names: &[String],
    mut at: f64,
    mut previous: f64,
) -> self::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Result {
    loop {
        return if previous < at {
            if (self::read_required_text_fact(names, previous))
                == (self::read_required_text_fact(names, at))
            {
                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Return(String::from("duplicate closure parameter"))
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
            self::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_callable_closure_parameter_error_loop46(
    names: &[String],
    mut at: f64,
) -> self::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Result {
    loop {
        return if at < (names.len() as f64) {
            {
                let previous = 0f64;
                {
                    let source_ml_source2_s189 = self::source_ml_source1_temporary_read_callable_closure_parameter_error_loop48(names, at, previous);
                    match source_ml_source2_s189 {
                        self::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Done => {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                        self::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop48Return(source_ml_source1_temporary_result48) => {
                            self::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Return(source_ml_source1_temporary_result48)
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop46Done(at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCallableAppendNameLoop49Result {
    MlSource1TemporaryCallableAppendNameLoop49Done,
    MlSource1TemporaryCallableAppendNameLoop49Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_append_name_loop49(
    texts: &[String],
    identities: &[f64],
    mut identity: f64,
    mut at: f64,
) -> self::MlSource1TemporaryCallableAppendNameLoop49Result {
    loop {
        return if at < (identities.len() as f64) {
            if (self::callable_text(texts, self::read_required_number_fact(identities, at)))
                == (self::callable_text(texts, identity))
            {
                self::MlSource1TemporaryCallableAppendNameLoop49Result::MlSource1TemporaryCallableAppendNameLoop49Return(identities.to_vec())
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
            self::MlSource1TemporaryCallableAppendNameLoop49Result::MlSource1TemporaryCallableAppendNameLoop49Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableStatementCapturesLoop50Result {
    MlSource1TemporaryPlanCallableStatementCapturesLoop50Done(Vec<f64>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableStatementCapturesJoin51 {
    MlSource1TemporaryPlanCallableStatementCapturesJoin51Next(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_statement_captures_loop50(
    texts: &[String],
    outer: &[String],
    events: &[f64],
    mut assigned: Vec<f64>,
    mut free: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableStatementCapturesLoop50Result {
    loop {
        return if at < (events.len() as f64) {
            {
                let name = self::read_required_number_fact(events, (at + 1f64));
                {
                    let source_ml_source2_s190 = if self::callable_bound(
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
                                    self::MlSource1TemporaryPlanCallableStatementCapturesJoin51::MlSource1TemporaryPlanCallableStatementCapturesJoin51Next(assigned_2, free_3)
                                }
                            } else {
                                self::MlSource1TemporaryPlanCallableStatementCapturesJoin51::MlSource1TemporaryPlanCallableStatementCapturesJoin51Next(assigned, free_3)
                            }
                        }
                    } else {
                        self::MlSource1TemporaryPlanCallableStatementCapturesJoin51::MlSource1TemporaryPlanCallableStatementCapturesJoin51Next(assigned, free)
                    };
                    match source_ml_source2_s190 {
                        self::MlSource1TemporaryPlanCallableStatementCapturesJoin51::MlSource1TemporaryPlanCallableStatementCapturesJoin51Next(assigned_3, free_4) => {
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
            self::MlSource1TemporaryPlanCallableStatementCapturesLoop50Result::MlSource1TemporaryPlanCallableStatementCapturesLoop50Done(assigned, free)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop56Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop56Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop56(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut tokens: f64,
    mut locals: Vec<f64>,
    mut mutable: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop56Result {
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop56Result::MlSource1TemporaryPlanCallableClosureScopeLoop56Done(locals, mutable)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop55Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop55Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop55(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_mutable: f64,
    mut mutable: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop55Result {
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop55Result::MlSource1TemporaryPlanCallableClosureScopeLoop55Done(mutable, at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop54Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop54Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop54(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_temporal: f64,
    mut temporal: Vec<f64>,
    names: &[String],
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop54Result {
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
                                    let source_ml_source2_concat_value191 = temporal;
                                    {
                                        let source_ml_source2_concat_value192 = vec![name];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value191,
                                            ),
                                            source_ml_source2_concat_value192,
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop54Result::MlSource1TemporaryPlanCallableClosureScopeLoop54Done(temporal, at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop53Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop53Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop53(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut old_locals: f64,
    mut locals: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop53Result {
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop53Result::MlSource1TemporaryPlanCallableClosureScopeLoop53Done(locals, at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableClosureScopeLoop52Result {
    MlSource1TemporaryPlanCallableClosureScopeLoop52Done(Vec<String>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_callable_closure_scope_loop52(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut tokens: f64,
    mut names: Vec<String>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableClosureScopeLoop52Result {
    loop {
        if (tokens >= 0f64) && (at < (self::read_required_number_fact(counts, tokens))) {
            {
                let token = self::callable_element(starts, counts, targets, tokens, at);
                {
                    let names_2 = {
                        let source_ml_source2_concat_value193 = names;
                        {
                            let source_ml_source2_concat_value194 = vec![self::callable_text(
                                texts,
                                self::callable_field(
                                    starts, counts, labels, targets, token, "value",
                                ),
                            )];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<String>::new(),
                                    source_ml_source2_concat_value193,
                                ),
                                source_ml_source2_concat_value194,
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
            self::MlSource1TemporaryPlanCallableClosureScopeLoop52Result::MlSource1TemporaryPlanCallableClosureScopeLoop52Done(names, at)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_surface_parameters_loop57(
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
                        let source_ml_source2_concat_value195 = plan;
                        {
                            let source_ml_source2_concat_value196 = vec![
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
                                    source_ml_source2_concat_value195,
                                ),
                                source_ml_source2_concat_value196,
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
pub fn source_ml_source1_temporary_render_callable_parameters_loop58(
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
                    let source_ml_source2_concat_value197 = parameters;
                    {
                        let source_ml_source2_concat_value198 = vec![if format == "Rust" {
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
                                source_ml_source2_concat_value197,
                            ),
                            source_ml_source2_concat_value198,
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
pub fn source_ml_source1_temporary_plan_callable_mutable_cells_loop60(
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
                            let source_ml_source2_concat_value199 = cells;
                            {
                                let source_ml_source2_concat_value200 =
                                    vec![self::read_required_number_fact(captures, at)];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value199,
                                    ),
                                    source_ml_source2_concat_value200,
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
pub enum MlSource1TemporaryPlanCallableMutableCellsLoop59Result {
    MlSource1TemporaryPlanCallableMutableCellsLoop59Done(Vec<f64>, Vec<String>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableMutableCellsJoin61 {
    MlSource1TemporaryPlanCallableMutableCellsJoin61Next(Vec<f64>, Vec<String>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_mutable_cells_loop59(
    texts: &[String],
    events: &[f64],
    mut captures: Vec<f64>,
    mut assigned: Vec<String>,
    mut at: f64,
) -> self::MlSource1TemporaryPlanCallableMutableCellsLoop59Result {
    loop {
        return if at < (events.len() as f64) {
            {
                let source_ml_source2_s203 = if (self::read_required_number_fact(events, at))
                    == 0f64
                {
                    {
                        let captures_2 = self::callable_append_name(
                            texts,
                            &(captures),
                            self::read_required_number_fact(events, (at + 1f64)),
                        );
                        self::MlSource1TemporaryPlanCallableMutableCellsJoin61::MlSource1TemporaryPlanCallableMutableCellsJoin61Next(captures_2, assigned)
                    }
                } else {
                    {
                        let assigned_2 = {
                            let source_ml_source2_concat_value204 = assigned;
                            {
                                let source_ml_source2_concat_value205 = vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(events, (at + 1f64)),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source2_concat_value204,
                                    ),
                                    source_ml_source2_concat_value205,
                                )
                            }
                        };
                        self::MlSource1TemporaryPlanCallableMutableCellsJoin61::MlSource1TemporaryPlanCallableMutableCellsJoin61Next(captures, assigned_2)
                    }
                };
                match source_ml_source2_s203 {
                    self::MlSource1TemporaryPlanCallableMutableCellsJoin61::MlSource1TemporaryPlanCallableMutableCellsJoin61Next(captures_3, assigned_3) => {
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
            self::MlSource1TemporaryPlanCallableMutableCellsLoop59Result::MlSource1TemporaryPlanCallableMutableCellsLoop59Done(captures, assigned, at)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_cell_observations_loop63(
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
                    let source_ml_source2_concat_value206 = events;
                    {
                        let source_ml_source2_concat_value207 = vec![
                            1f64,
                            self::callable_element(starts, counts, targets, assigned, at),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value206,
                            ),
                            source_ml_source2_concat_value207,
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
pub enum MlSource1TemporaryCallableCellObservationsLoop62Result {
    MlSource1TemporaryCallableCellObservationsLoop62Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_cell_observations_loop62(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut events: Vec<f64>,
    mut captured: f64,
    mut at: f64,
) -> self::MlSource1TemporaryCallableCellObservationsLoop62Result {
    loop {
        if (captured >= 0f64) && (at < (self::read_required_number_fact(counts, captured))) {
            {
                let events_2 = {
                    let source_ml_source2_concat_value208 = events;
                    {
                        let source_ml_source2_concat_value209 = vec![
                            0f64,
                            self::callable_element(starts, counts, targets, captured, at),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value208,
                            ),
                            source_ml_source2_concat_value209,
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
            self::MlSource1TemporaryCallableCellObservationsLoop62Result::MlSource1TemporaryCallableCellObservationsLoop62Done(events, at)
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
pub fn source_ml_source1_temporary_callable_cell_observations_loop64(
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
                                let source_ml_source2_concat_value210 = events;
                                {
                                    let source_ml_source2_concat_value211 =
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
                                            source_ml_source2_concat_value210,
                                        ),
                                        source_ml_source2_concat_value211,
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
pub fn source_ml_source1_temporary_plan_callable_source_renaming_loop65(
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
                                    let source_ml_source2_concat_value212 = bindings;
                                    {
                                        let source_ml_source2_concat_value213 = vec![
                                            source,
                                            target,
                                            if reference { 1f64 } else { 0f64 },
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value212,
                                            ),
                                            source_ml_source2_concat_value213,
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
pub enum MlSource1TemporaryCallableRenamingBindingLoop66Result {
    MlSource1TemporaryCallableRenamingBindingLoop66Done,
    MlSource1TemporaryCallableRenamingBindingLoop66Return(Vec<f64>),
}
