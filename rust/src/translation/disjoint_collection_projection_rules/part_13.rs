// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result;
use super::accepts_effects;
use super::add_fact;
use super::add_roots;
use super::add_value;
use super::edges;
use super::first_number;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_disjoint_collection_iteration_identity_kind;
use super::read_required_number_fact;
use super::read_roots;
use super::text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop60(
    roles: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut size: f64,
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    mut snapshot: f64,
    mut aliases: Vec<f64>,
    mut node: f64,
) -> Vec<f64> {
    loop {
        if node < size {
            if (self::text_fact(roles, node))
                == (self::ml_string::Text::from_units(vec![
                    97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                ]))
            {
                {
                    let child = self::first_number(
                        &(self::edges(
                            starts,
                            counts,
                            labels,
                            targets,
                            node,
                            &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                        )),
                    );
                    if (child >= 0f64)
                        && self::ml_array::includes(
                            &self::read_roots(facts, 1f64, child, width, span),
                            &snapshot,
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        )
                    {
                        {
                            let aliases_2 = self::add_value(&(aliases), node);
                            {
                                let node_2 = node + 1f64;
                                {
                                    (aliases, node) = (aliases_2, node_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let node_3 = node + 1f64;
                            {
                                node = node_3;
                                continue;
                            }
                        }
                    }
                }
            }
            {
                {
                    let node_4 = node + 1f64;
                    {
                        node = node_4;
                        continue;
                    }
                }
            }
        }
        return { aliases };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop63(
    aliases: &[f64],
    mut passes_alias: bool,
    source_ml_source1_temporary_values53: &[f64],
    mut source_ml_source1_temporary_index53: self::ml::Big,
) -> bool {
    loop {
        return if source_ml_source1_temporary_index53
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values53.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values53,
                    source_ml_source1_temporary_index53.to_index(),
                );
                if self::ml_array::includes(aliases, &root, 0f64, false, |a: &f64, b: &f64| {
                    a == b || (a.is_nan() && b.is_nan())
                }) {
                    { true }
                } else {
                    {
                        let source_ml_source1_temporary_index53_2 =
                            source_ml_source1_temporary_index53.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index53 =
                                source_ml_source1_temporary_index53_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            passes_alias
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop62(
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    aliases: &[f64],
    mut passes_alias: bool,
    source_ml_source1_temporary_values52: &[f64],
    mut source_ml_source1_temporary_index52: self::ml::Big,
) -> bool {
    loop {
        return if source_ml_source1_temporary_index52
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values52.len() as u128))
        {
            {
                let argument = self::ml_array::at(
                    source_ml_source1_temporary_values52,
                    source_ml_source1_temporary_index52.to_index(),
                );
                {
                    let source_ml_source1_temporary_values53 =
                        self::read_roots(facts, 1f64, argument, width, span);
                    {
                        let source_ml_source1_temporary_index53 = self::ml::Big::from_i128(0);
                        {
                            let passes_alias_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop63(aliases, passes_alias, &(source_ml_source1_temporary_values53), source_ml_source1_temporary_index53);
                            if passes_alias_2 {
                                passes_alias_2
                            } else {
                                {
                                    let source_ml_source1_temporary_index52_2 =
                                        source_ml_source1_temporary_index52
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (passes_alias, source_ml_source1_temporary_index52) =
                                            (passes_alias_2, source_ml_source1_temporary_index52_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            passes_alias
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop64(
    mut scopes: Vec<f64>,
    source_ml_source1_temporary_values54: &[f64],
    mut source_ml_source1_temporary_index54: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index54
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values54.len() as u128))
        {
            {
                let callee = self::ml_array::at(
                    source_ml_source1_temporary_values54,
                    source_ml_source1_temporary_index54.to_index(),
                );
                {
                    let scopes_2 = self::add_value(&(scopes), callee);
                    {
                        let source_ml_source1_temporary_index54_2 =
                            source_ml_source1_temporary_index54.add(&self::ml::Big::from_i128(1));
                        {
                            (scopes, source_ml_source1_temporary_index54) =
                                (scopes_2, source_ml_source1_temporary_index54_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { scopes };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop61(
    roles: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut size: f64,
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    aliases: &[f64],
    mut scopes: Vec<f64>,
    mut node: f64,
) -> Vec<f64> {
    loop {
        if node < size {
            if ((self::text_fact(roles, node))
                == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
                || ((self::text_fact(roles, node))
                    == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])))
            {
                {
                    let actual = self::edges(
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![
                            97, 114, 103, 117, 109, 101, 110, 116,
                        ])),
                    );
                    {
                        let passes_alias = false;
                        {
                            let source_ml_source1_temporary_values52 = actual;
                            {
                                let source_ml_source1_temporary_index52 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let passes_alias_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop62(width, span, facts, aliases, passes_alias, &(source_ml_source1_temporary_values52), source_ml_source1_temporary_index52);
                                    if passes_alias_2 {
                                        {
                                            let targets_ = self::edges(
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                node,
                                                &(self::ml_string::Text::from_units(vec![
                                                    99, 97, 108, 108, 101, 101,
                                                ])),
                                            );
                                            {
                                                let callees = if (self::text_fact(roles, node))
                                                    == (self::ml_string::Text::from_units(vec![
                                                        99, 97, 108, 108,
                                                    ])) {
                                                    targets_
                                                } else {
                                                    {
                                                        let ml_library_condition_2 = {
                                                            let ml_value = targets_.len() as f64;
                                                            ml_value != 0f64 && !ml_value.is_nan()
                                                        };
                                                        if ml_library_condition_2 {
                                                            self::read_roots(
                                                                facts,
                                                                1f64,
                                                                self::read_required_number_fact(
                                                                    &(targets_),
                                                                    0f64,
                                                                ),
                                                                width,
                                                                span,
                                                            )
                                                        } else {
                                                            Vec::<f64>::new()
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_temporary_values54 =
                                                        callees;
                                                    {
                                                        let source_ml_source1_temporary_index54 =
                                                            self::ml::Big::from_i128(0);
                                                        {
                                                            let scopes_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop64(scopes, &(source_ml_source1_temporary_values54), source_ml_source1_temporary_index54);
                                                            {
                                                                let node_2 = node + 1f64;
                                                                {
                                                                    (scopes, node) =
                                                                        (scopes_2, node_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    {
                                        {
                                            let node_3 = node + 1f64;
                                            {
                                                node = node_3;
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
            {
                {
                    let node_4 = node + 1f64;
                    {
                        node = node_4;
                        continue;
                    }
                }
            }
        }
        return { scopes };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58(
    roles: &[self::ml_string::Text],
    operations: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    owners: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    functions: &[f64],
    bodies: &[f64],
    mut main_: f64,
    snapshots: &[f64],
    mut size: f64,
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    mut accepted: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result {
    loop {
        return if at < (snapshots.len() as f64) {
            {
                let snapshot = self::read_required_number_fact(snapshots, at);
                {
                    let source = self::read_required_number_fact(snapshots, (at + 1f64));
                    {
                        let aliases = vec![snapshot];
                        {
                            let scopes = Vec::<f64>::new();
                            if (self::text_fact(roles, snapshot))
                                == (self::ml_string::Text::from_units(vec![
                                    101, 97, 103, 101, 114, 32, 115, 110, 97, 112, 115, 104, 111,
                                    116,
                                ]))
                            {
                                if (self::accepts_effects(
                                    facts,
                                    roles.to_vec(),
                                    operations,
                                    kinds.to_vec(),
                                    owners,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    functions,
                                    bodies,
                                    &(Vec::<f64>::new()),
                                    source,
                                    true,
                                    main_,
                                    width,
                                    span,
                                )) {
                                    {
                                        let accepted_2 = {
                                            let source_ml_source2_concat_value60 = accepted;
                                            {
                                                let source_ml_source2_concat_value61 =
                                                    vec![snapshot];
                                                self::ml_array::append(
                                                    &(self::ml_array::append(
                                                        &(Vec::<f64>::new()),
                                                        source_ml_source2_concat_value60,
                                                    )),
                                                    source_ml_source2_concat_value61,
                                                )
                                            }
                                        };
                                        {
                                            let at_2 = at + 2f64;
                                            {
                                                (accepted, at) = (accepted_2, at_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Return(vec![2f64, snapshot])
                                }
                            } else {
                                {
                                    let node = 0f64;
                                    {
                                        let aliases_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop60(roles, starts, counts, labels, targets, size, width, span, facts, snapshot, aliases, node);
                                        {
                                            let node_2 = 0f64;
                                            {
                                                let scopes_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop61(roles, starts, counts, labels, targets, size, width, span, facts, &(aliases_2), scopes, node_2);
                                                if ((scopes_2.len() as f64) == 0f64)
                                                    || !(self::accepts_effects(
                                                        facts,
                                                        roles.to_vec(),
                                                        operations,
                                                        kinds.to_vec(),
                                                        owners,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        functions,
                                                        bodies,
                                                        &(scopes_2),
                                                        source,
                                                        true,
                                                        main_,
                                                        width,
                                                        span,
                                                    ))
                                                {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Return(vec![2f64, snapshot])
                                                } else {
                                                    {
                                                        let accepted_3 = {
                                                            let source_ml_source2_concat_value62 =
                                                                accepted;
                                                            {
                                                                let source_ml_source2_concat_value63 =
                                                                    vec![snapshot];
                                                                self::ml_array::append(&(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value62)), source_ml_source2_concat_value63)
                                                            }
                                                        };
                                                        {
                                                            let at_3 = at + 2f64;
                                                            {
                                                                (accepted, at) = (accepted_3, at_3);
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
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Done(accepted)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(Vec<f64>, bool),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop66(
    functions: &[f64],
    bodies: &[f64],
    mut width: f64,
    mut span: f64,
    mut facts: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (functions.len() as f64) {
            if ((self::read_required_number_fact(bodies, at)) >= 0f64)
                && self::ml_array::includes(
                    &facts,
                    &((4f64 * span) + self::read_required_number_fact(functions, at)),
                    0f64,
                    false,
                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                )
            {
                {
                    let facts_2 = self::add_roots(
                        &(facts),
                        5f64,
                        self::read_required_number_fact(functions, at),
                        &(self::read_roots(
                            &(facts),
                            1f64,
                            self::read_required_number_fact(bodies, at),
                            width,
                            span,
                        )),
                        width,
                        span,
                    );
                    {
                        let at_2 = at + 1f64;
                        {
                            (facts, at) = (facts_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { facts };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin67 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin67Next(Vec<f64>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop70(
    kinds: &[self::ml_string::Text],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    mut width: f64,
    mut span: f64,
    mut facts: Vec<f64>,
    mut position: f64,
    mut parameter: f64,
) -> Vec<f64> {
    loop {
        if parameter
            < (self::read_required_number_fact(parameter_starts, position)
                + self::read_required_number_fact(parameter_counts, position))
        {
            if self::read_disjoint_collection_iteration_identity_kind(
                &(self::text_fact(
                    kinds,
                    self::read_required_number_fact(parameters, parameter),
                )),
            ) {
                {
                    let facts_2 = self::add_roots(
                        &(facts),
                        1f64,
                        self::read_required_number_fact(parameters, parameter),
                        &(vec![-1f64]),
                        width,
                        span,
                    );
                    {
                        let parameter_2 = parameter + 1f64;
                        {
                            (facts, parameter) = (facts_2, parameter_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let parameter_3 = parameter + 1f64;
                    {
                        parameter = parameter_3;
                        continue;
                    }
                }
            }
        }
        return { facts };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop69(
    kinds: &[self::ml_string::Text],
    owners: &[f64],
    functions: &[f64],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    mut width: f64,
    mut span: f64,
    mut facts: Vec<f64>,
    list: &[f64],
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (list.len() as f64) {
            {
                let owner = self::read_required_number_fact(
                    owners,
                    self::read_required_number_fact(list, at),
                );
                if (owner >= 0f64)
                    && !(self::ml_array::includes(
                        &facts,
                        &((4f64 * span) + owner),
                        0f64,
                        false,
                        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                    ))
                {
                    {
                        let facts_2 = self::add_fact(&(facts), ((4f64 * span) + owner));
                        {
                            let position =
                                self::ml_array::index_of(functions, &owner, 0f64, false, |a, b| {
                                    a == b
                                });
                            if position >= 0f64 {
                                {
                                    let parameter =
                                        self::read_required_number_fact(parameter_starts, position);
                                    {
                                        let facts_3 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop70(kinds, parameter_starts, parameter_counts, parameters, width, span, facts_2, position, parameter);
                                        {
                                            let at_2 = at + 2f64;
                                            {
                                                (facts, at) = (facts_3, at_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let at_3 = at + 2f64;
                                    {
                                        (facts, at) = (facts_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let at_4 = at + 2f64;
                        {
                            at = at_4;
                            continue;
                        }
                    }
                }
            }
        }
        return { facts };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop68(
    kinds: &[self::ml_string::Text],
    owners: &[f64],
    functions: &[f64],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    mut width: f64,
    mut span: f64,
    mut facts: Vec<f64>,
    source_ml_source1_temporary_values51: &[Vec<f64>],
    mut source_ml_source1_temporary_index51: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index51
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values51.len() as u128))
        {
            {
                let list = self::ml_array::at(
                    source_ml_source1_temporary_values51,
                    source_ml_source1_temporary_index51.to_index(),
                );
                {
                    let at = 0f64;
                    {
                        let facts_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop69(kinds, owners, functions, parameter_starts, parameter_counts, parameters, width, span, facts, &(list), at);
                        {
                            let source_ml_source1_temporary_index51_2 =
                                source_ml_source1_temporary_index51
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (facts, source_ml_source1_temporary_index51) =
                                    (facts_2, source_ml_source1_temporary_index51_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { facts };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop65Done(Vec<f64>, f64),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop65Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop73(
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values48: &[f64],
    mut source_ml_source1_temporary_index48: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index48
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values48.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values48,
                    source_ml_source1_temporary_index48.to_index(),
                );
                {
                    let roots_2 = self::add_value(&(roots), root);
                    {
                        let source_ml_source1_temporary_index48_2 =
                            source_ml_source1_temporary_index48.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index48) =
                                (roots_2, source_ml_source1_temporary_index48_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { roots };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72(
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values47: &[f64],
    mut source_ml_source1_temporary_index47: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index47
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values47.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_source1_temporary_values47,
                    source_ml_source1_temporary_index47.to_index(),
                );
                {
                    let source_ml_source1_temporary_values48 =
                        self::read_roots(facts, 1f64, child, width, span);
                    {
                        let source_ml_source1_temporary_index48 = self::ml::Big::from_i128(0);
                        {
                            let roots_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop73(roots, &(source_ml_source1_temporary_values48), source_ml_source1_temporary_index48);
                            {
                                let source_ml_source1_temporary_index47_2 =
                                    source_ml_source1_temporary_index47
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (roots, source_ml_source1_temporary_index47) =
                                        (roots_2, source_ml_source1_temporary_index47_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { roots };
    }
}
