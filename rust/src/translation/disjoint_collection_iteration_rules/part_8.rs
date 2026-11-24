// Generated from complete js/src/translation/disjoint-collection-iteration-rules.js. Do not edit by hand.
use super::accept_disjoint_collection_iteration_callback_head;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsLoop38Result {
    MlSource1TemporaryAcceptsEffectsLoop38Done,
    MlSource1TemporaryAcceptsEffectsLoop38Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accepts_effects_loop38(
    functions: &[f64],
    bodies: &[f64],
    mut main_: f64,
    source_ml_source1_temporary_values37: &[f64],
    mut source_ml_source1_temporary_index37: self::ml::Big,
) -> self::MlSource1TemporaryAcceptsEffectsLoop38Result {
    loop {
        return if source_ml_source1_temporary_index37
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values37.len() as u128))
        {
            {
                let scope = self::ml_array::at(
                    source_ml_source1_temporary_values37,
                    source_ml_source1_temporary_index37.to_index(),
                );
                {
                    let position =
                        self::ml_array::index_of(functions, &scope, 0f64, false, |a, b| a == b);
                    if (position < 0f64)
                        || (((self::read_required_number_fact(bodies, position)) < 0f64)
                            && (scope != main_))
                    {
                        self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Return(false)
                    } else {
                        {
                            let source_ml_source1_temporary_index37_2 =
                                source_ml_source1_temporary_index37
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index37 =
                                    source_ml_source1_temporary_index37_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_bind_calls_loop45(
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values42: &[f64],
    mut source_ml_source1_temporary_index42: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index42
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values42.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values42,
                    source_ml_source1_temporary_index42.to_index(),
                );
                {
                    let roots_2 = self::add_value(&(roots), root);
                    {
                        let source_ml_source1_temporary_index42_2 =
                            source_ml_source1_temporary_index42.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index42) =
                                (roots_2, source_ml_source1_temporary_index42_2);
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_bind_calls_loop44(
    arguments_: &[f64],
    callback: &self::ml_string::Text,
    mut receiver: f64,
    kinds: &[self::ml_string::Text],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut callee: f64,
    mut position: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (self::read_required_number_fact(parameter_counts, position)) {
            {
                let parameter = self::read_required_number_fact(
                    parameters,
                    (self::read_required_number_fact(parameter_starts, position) + at),
                );
                {
                    let roots = Vec::<f64>::new();
                    {
                        let roots_7 = if (*callback) == (self::ml_string::Text::from_units(vec![]))
                        {
                            {
                                if at < (arguments_.len() as f64) {
                                    self::read_roots(
                                        &(result),
                                        1f64,
                                        self::read_required_number_fact(arguments_, at),
                                        width,
                                        span,
                                    )
                                } else {
                                    Vec::<f64>::new()
                                }
                            }
                        } else if self::read_disjoint_collection_iteration_identity_kind(
                            &(self::text_fact(kinds, parameter)),
                        ) {
                            if ((*callback)
                                == (self::ml_string::Text::from_units(vec![
                                    114, 101, 100, 117, 99, 101,
                                ])))
                                && (at == 0f64)
                            {
                                {
                                    let roots_3 = if (arguments_.len() as f64) > 1f64 {
                                        self::read_roots(
                                            &(result),
                                            1f64,
                                            self::read_required_number_fact(arguments_, 1f64),
                                            width,
                                            span,
                                        )
                                    } else {
                                        Vec::<f64>::new()
                                    };
                                    {
                                        let source_ml_source1_temporary_values42 =
                                            self::read_roots(&(result), 5f64, callee, width, span);
                                        {
                                            let source_ml_source1_temporary_index42 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                self::source_ml_source1_temporary_bind_calls_loop45(
                                                    roots_3,
                                                    &(source_ml_source1_temporary_values42),
                                                    source_ml_source1_temporary_index42,
                                                )
                                            }
                                        }
                                    }
                                }
                            } else if at
                                == (if (*callback)
                                    == (self::ml_string::Text::from_units(vec![
                                        114, 101, 100, 117, 99, 101,
                                    ])) {
                                    3f64
                                } else {
                                    2f64
                                })
                            {
                                { self::read_roots(&(result), 1f64, receiver, width, span) }
                            } else {
                                { vec![-1f64] }
                            }
                        } else {
                            roots
                        };
                        {
                            let result_2 = self::add_roots(
                                &(result),
                                1f64,
                                parameter,
                                &(roots_7),
                                width,
                                span,
                            );
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_bind_calls_loop43(
    mut owner: f64,
    arguments_: &[f64],
    callback: &self::ml_string::Text,
    mut receiver: f64,
    kinds: &[self::ml_string::Text],
    functions: &[f64],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    bodies: &[f64],
    mut main_: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values41: &[f64],
    mut source_ml_source1_temporary_index41: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index41
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values41.len() as u128))
        {
            {
                let callee = self::ml_array::at(
                    source_ml_source1_temporary_values41,
                    source_ml_source1_temporary_index41.to_index(),
                );
                {
                    let position =
                        self::ml_array::index_of(functions, &callee, 0f64, false, |a, b| a == b);
                    if position < 0f64 {
                        {
                            let source_ml_source1_temporary_index41_2 =
                                source_ml_source1_temporary_index41
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index41 =
                                    source_ml_source1_temporary_index41_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let result_2 = self::add_fact(&(result), ((4f64 * span) + callee));
                            {
                                let result_3 = self::add_roots(
                                    &(result_2),
                                    3f64,
                                    owner,
                                    &(vec![callee]),
                                    width,
                                    span,
                                );
                                {
                                    let result_5 =
                                        if ((self::read_required_number_fact(bodies, position))
                                            < 0f64)
                                            && (callee != main_)
                                        {
                                            {
                                                self::add_roots(
                                                    &(result_3),
                                                    5f64,
                                                    callee,
                                                    &(vec![-1f64]),
                                                    width,
                                                    span,
                                                )
                                            }
                                        } else {
                                            result_3
                                        };
                                    {
                                        let at = 0f64;
                                        {
                                            let result_6 =
                                                self::source_ml_source1_temporary_bind_calls_loop44(
                                                    arguments_,
                                                    callback,
                                                    receiver,
                                                    kinds,
                                                    parameter_starts,
                                                    parameter_counts,
                                                    parameters,
                                                    width,
                                                    span,
                                                    result_5,
                                                    callee,
                                                    position,
                                                    at,
                                                );
                                            {
                                                let source_ml_source1_temporary_index41_3 =
                                                    source_ml_source1_temporary_index41
                                                        .add(&self::ml::Big::from_i128(1));
                                                {
                                                    (result, source_ml_source1_temporary_index41) = (
                                                        result_6,
                                                        source_ml_source1_temporary_index41_3,
                                                    );
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
        return { result };
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
    callbacks: &[f64],
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    mut safe: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (callbacks.len() as f64) {
            {
                let node = self::read_required_number_fact(callbacks, at);
                {
                    let source = self::read_required_number_fact(callbacks, (at + 1f64));
                    {
                        let callback = self::first_number(
                            &(self::edges(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![
                                    97, 114, 103, 117, 109, 101, 110, 116,
                                ])),
                            )),
                        );
                        if (callback < 0f64)
                            || !(self::accept_disjoint_collection_iteration_callback_head(
                                &(self::text_fact(roles, callback)),
                            ))
                        {
                            {
                                let at_2 = at + 2f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let scopes = self::read_roots(facts, 1f64, callback, width, span);
                                if ((scopes.len() as f64) != 0f64)
                                    && self::accepts_effects(
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
                                        &(scopes),
                                        source,
                                        false,
                                        main_,
                                        width,
                                        span,
                                    )
                                {
                                    {
                                        let safe_2 = {
                                            let source_ml_source2_concat_value39 = safe;
                                            {
                                                let source_ml_source2_concat_value40 = vec![node];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value39,
                                                    ),
                                                    source_ml_source2_concat_value40,
                                                )
                                            }
                                        };
                                        {
                                            let at_3 = at + 2f64;
                                            {
                                                (safe, at) = (safe_2, at_3);
                                                continue;
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
                    }
                }
            }
        }
        return { safe };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(Vec<f64>),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59(
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
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop62(
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
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values52,
                    source_ml_source1_temporary_index52.to_index(),
                );
                if self::ml_array::includes(aliases, &root, 0f64, false, |a: &f64, b: &f64| {
                    a == b || (a.is_nan() && b.is_nan())
                }) {
                    { true }
                } else {
                    {
                        let source_ml_source1_temporary_index52_2 =
                            source_ml_source1_temporary_index52.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index52 =
                                source_ml_source1_temporary_index52_2;
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
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop61(
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    aliases: &[f64],
    mut passes_alias: bool,
    source_ml_source1_temporary_values51: &[f64],
    mut source_ml_source1_temporary_index51: self::ml::Big,
) -> bool {
    loop {
        return if source_ml_source1_temporary_index51
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values51.len() as u128))
        {
            {
                let argument = self::ml_array::at(
                    source_ml_source1_temporary_values51,
                    source_ml_source1_temporary_index51.to_index(),
                );
                {
                    let source_ml_source1_temporary_values52 =
                        self::read_roots(facts, 1f64, argument, width, span);
                    {
                        let source_ml_source1_temporary_index52 = self::ml::Big::from_i128(0);
                        {
                            let passes_alias_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop62(aliases, passes_alias, &(source_ml_source1_temporary_values52), source_ml_source1_temporary_index52);
                            if passes_alias_2 {
                                passes_alias_2
                            } else {
                                {
                                    let source_ml_source1_temporary_index51_2 =
                                        source_ml_source1_temporary_index51
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (passes_alias, source_ml_source1_temporary_index51) =
                                            (passes_alias_2, source_ml_source1_temporary_index51_2);
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
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop63(
    mut scopes: Vec<f64>,
    source_ml_source1_temporary_values53: &[f64],
    mut source_ml_source1_temporary_index53: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index53
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values53.len() as u128))
        {
            {
                let callee = self::ml_array::at(
                    source_ml_source1_temporary_values53,
                    source_ml_source1_temporary_index53.to_index(),
                );
                {
                    let scopes_2 = self::add_value(&(scopes), callee);
                    {
                        let source_ml_source1_temporary_index53_2 =
                            source_ml_source1_temporary_index53.add(&self::ml::Big::from_i128(1));
                        {
                            (scopes, source_ml_source1_temporary_index53) =
                                (scopes_2, source_ml_source1_temporary_index53_2);
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
                            let source_ml_source1_temporary_values51 = actual;
                            {
                                let source_ml_source1_temporary_index51 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let passes_alias_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop61(width, span, facts, aliases, passes_alias, &(source_ml_source1_temporary_values51), source_ml_source1_temporary_index51);
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
                                                    let source_ml_source1_temporary_values53 =
                                                        callees;
                                                    {
                                                        let source_ml_source1_temporary_index53 =
                                                            self::ml::Big::from_i128(0);
                                                        {
                                                            let scopes_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop63(scopes, &(source_ml_source1_temporary_values53), source_ml_source1_temporary_index53);
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
