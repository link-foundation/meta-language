// Generated from complete js/src/translation/disjoint-collection-iteration-rules.js. Do not edit by hand.
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result;
use super::accepts_effects;
use super::add_fact;
use super::add_field;
use super::add_roots;
use super::add_value;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_disjoint_collection_iteration_identity_kind;
use super::read_required_number_fact;
use super::read_roots;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop60;
use super::text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57(
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
) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result {
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
                                            let source_ml_source2_concat_value41 = accepted;
                                            {
                                                let source_ml_source2_concat_value42 =
                                                    vec![snapshot];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value41,
                                                    ),
                                                    source_ml_source2_concat_value42,
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
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(vec![2f64, snapshot])
                                }
                            } else {
                                {
                                    let node = 0f64;
                                    {
                                        let aliases_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59(roles, starts, counts, labels, targets, size, width, span, facts, snapshot, aliases, node);
                                        {
                                            let node_2 = 0f64;
                                            {
                                                let scopes_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop60(roles, starts, counts, labels, targets, size, width, span, facts, &(aliases_2), scopes, node_2);
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
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(vec![2f64, snapshot])
                                                } else {
                                                    {
                                                        let accepted_3 = {
                                                            let source_ml_source2_concat_value43 =
                                                                accepted;
                                                            {
                                                                let source_ml_source2_concat_value44 =
                                                                    vec![snapshot];
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value43), source_ml_source2_concat_value44)
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
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(accepted)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done(Vec<f64>, bool),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop65(
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
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin66 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin66Next(Vec<f64>, bool),
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
                                        let facts_3 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop69(kinds, parameter_starts, parameter_counts, parameters, width, span, facts_2, position, parameter);
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
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop67(
    kinds: &[self::ml_string::Text],
    owners: &[f64],
    functions: &[f64],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    mut width: f64,
    mut span: f64,
    mut facts: Vec<f64>,
    source_ml_source1_temporary_values50: &[Vec<f64>],
    mut source_ml_source1_temporary_index50: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index50
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values50.len() as u128))
        {
            {
                let list = self::ml_array::at(
                    source_ml_source1_temporary_values50,
                    source_ml_source1_temporary_index50.to_index(),
                );
                {
                    let at = 0f64;
                    {
                        let facts_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop68(kinds, owners, functions, parameter_starts, parameter_counts, parameters, width, span, facts, &(list), at);
                        {
                            let source_ml_source1_temporary_index50_2 =
                                source_ml_source1_temporary_index50
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (facts, source_ml_source1_temporary_index50) =
                                    (facts_2, source_ml_source1_temporary_index50_2);
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
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop64Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop64Done(Vec<f64>, f64),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop64Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72(
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values47: &[f64],
    mut source_ml_source1_temporary_index47: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index47
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values47.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values47,
                    source_ml_source1_temporary_index47.to_index(),
                );
                {
                    let roots_2 = self::add_value(&(roots), root);
                    {
                        let source_ml_source1_temporary_index47_2 =
                            source_ml_source1_temporary_index47.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index47) =
                                (roots_2, source_ml_source1_temporary_index47_2);
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
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop71(
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values46: &[f64],
    mut source_ml_source1_temporary_index46: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index46
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values46.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_source1_temporary_values46,
                    source_ml_source1_temporary_index46.to_index(),
                );
                {
                    let source_ml_source1_temporary_values47 =
                        self::read_roots(facts, 1f64, child, width, span);
                    {
                        let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                        {
                            let roots_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72(roots, &(source_ml_source1_temporary_values47), source_ml_source1_temporary_index47);
                            {
                                let source_ml_source1_temporary_index46_2 =
                                    source_ml_source1_temporary_index46
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (roots, source_ml_source1_temporary_index46) =
                                        (roots_2, source_ml_source1_temporary_index46_2);
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop76(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut width: f64,
    mut field_width: f64,
    mut span: f64,
    mut facts: Vec<f64>,
    mut node: f64,
    role: &self::ml_string::Text,
    mut object: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            if (self::text_fact(labels, at)).starts_with(&self::ml_string::Text::from_units(vec![
                102, 105, 101, 108, 100, 58,
            ])) || (self::text_fact(labels, at)).starts_with(&self::ml_string::Text::from_units(
                vec![105, 116, 101, 109, 58],
            )) {
                {
                    let actual = if (*role)
                        == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))
                    {
                        self::read_roots(&(facts), 1f64, object, width, span)
                    } else {
                        vec![node]
                    };
                    {
                        let facts_2 = self::add_field(
                            &(facts),
                            &(actual),
                            (self::ml_array::index_of(
                                labels,
                                &self::text_fact(labels, at),
                                0f64,
                                false,
                                |a, b| a == b,
                            ) + 1f64),
                            &(self::read_roots(
                                &(facts),
                                1f64,
                                self::read_required_number_fact(targets, at),
                                width,
                                span,
                            )),
                            field_width,
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
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin75 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(Vec<f64>, f64),
    MlSource1TemporaryProveDisjointCollectionIterationsJoin75Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin74 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(Vec<f64>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin73 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin73Next(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop78(
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values49: &[f64],
    mut source_ml_source1_temporary_index49: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index49
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values49.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values49,
                    source_ml_source1_temporary_index49.to_index(),
                );
                {
                    let roots_2 = self::add_value(&(roots), root);
                    {
                        let source_ml_source1_temporary_index49_2 =
                            source_ml_source1_temporary_index49.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index49) =
                                (roots_2, source_ml_source1_temporary_index49_2);
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
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop77(
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values48: &[f64],
    mut source_ml_source1_temporary_index48: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index48
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values48.len() as u128))
        {
            {
                let target = self::ml_array::at(
                    source_ml_source1_temporary_values48,
                    source_ml_source1_temporary_index48.to_index(),
                );
                {
                    let source_ml_source1_temporary_values49 =
                        self::read_roots(facts, 5f64, target, width, span);
                    {
                        let source_ml_source1_temporary_index49 = self::ml::Big::from_i128(0);
                        {
                            let roots_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop78(roots, &(source_ml_source1_temporary_values49), source_ml_source1_temporary_index49);
                            {
                                let source_ml_source1_temporary_index48_2 =
                                    source_ml_source1_temporary_index48
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (roots, source_ml_source1_temporary_index48) =
                                        (roots_2, source_ml_source1_temporary_index48_2);
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin70 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin70Next(f64, Vec<f64>),
    MlSource1TemporaryProveDisjointCollectionIterationsJoin70Return(Vec<f64>),
}
