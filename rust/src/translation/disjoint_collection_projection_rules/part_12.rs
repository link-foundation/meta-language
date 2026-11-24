// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReachableLoop36Result;
use super::MlSource1TemporaryReachableLoop37Result;
use super::accept_disjoint_collection_iteration_callback_head;
use super::accepts_effects;
use super::add_fact;
use super::add_roots;
use super::add_value;
use super::called;
use super::edges;
use super::first_number;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_disjoint_collection_iteration_callback;
use super::read_disjoint_collection_iteration_identity_kind;
use super::read_disjoint_collection_iteration_method_effect;
use super::read_required_number_fact;
use super::read_roots;
use super::separate_roots;
use super::text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_reachable_loop37(
    mut result: Vec<f64>,
    mut progress: bool,
    source_ml_source1_temporary_values34: &[f64],
    mut source_ml_source1_temporary_index34: self::ml::Big,
) -> self::MlSource1TemporaryReachableLoop37Result {
    loop {
        if source_ml_source1_temporary_index34
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values34.len() as u128))
        {
            {
                let target = self::ml_array::at(
                    source_ml_source1_temporary_values34,
                    source_ml_source1_temporary_index34.to_index(),
                );
                if !(self::ml_array::includes(&result, &target, 0f64, false, |a: &f64, b: &f64| {
                    a == b || (a.is_nan() && b.is_nan())
                })) {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value53 = result;
                            {
                                let source_ml_source2_concat_value54 = vec![target];
                                self::ml_array::append(
                                    &(self::ml_array::append(
                                        &(Vec::<f64>::new()),
                                        source_ml_source2_concat_value53,
                                    )),
                                    source_ml_source2_concat_value54,
                                )
                            }
                        };
                        {
                            let progress_2 = true;
                            {
                                let source_ml_source1_temporary_index34_2 =
                                    source_ml_source1_temporary_index34
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (result, progress, source_ml_source1_temporary_index34) = (
                                        result_2,
                                        progress_2,
                                        source_ml_source1_temporary_index34_2,
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index34_3 =
                            source_ml_source1_temporary_index34.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index34 =
                                source_ml_source1_temporary_index34_3;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReachableLoop37Result::MlSource1TemporaryReachableLoop37Done(
                result, progress,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_reachable_loop36(
    facts: &[f64],
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut progress: bool,
    source_ml_source1_temporary_values33: &[f64],
    mut source_ml_source1_temporary_index33: self::ml::Big,
) -> self::MlSource1TemporaryReachableLoop36Result {
    loop {
        return if source_ml_source1_temporary_index33
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values33.len() as u128))
        {
            {
                let owner = self::ml_array::at(
                    source_ml_source1_temporary_values33,
                    source_ml_source1_temporary_index33.to_index(),
                );
                {
                    let source_ml_source1_temporary_values34 =
                        self::called(facts, owner, width, span);
                    {
                        let source_ml_source1_temporary_index34 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s55 =
                                self::source_ml_source1_temporary_reachable_loop37(
                                    result,
                                    progress,
                                    &(source_ml_source1_temporary_values34),
                                    source_ml_source1_temporary_index34,
                                );
                            match source_ml_source2_s55 {
                                self::MlSource1TemporaryReachableLoop37Result::MlSource1TemporaryReachableLoop37Done(result_2, progress_2) => {
                                    {
                                        let source_ml_source1_temporary_index33_2 = source_ml_source1_temporary_index33.add(&self::ml::Big::from_i128(1));
                                        {
                                            (result, progress, source_ml_source1_temporary_index33) = (result_2, progress_2, source_ml_source1_temporary_index33_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReachableLoop36Result::MlSource1TemporaryReachableLoop36Done(
                result, progress,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_reachable_loop35(
    facts: &[f64],
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut progress: bool,
) -> Vec<f64> {
    loop {
        return if progress {
            {
                let progress_2 = false;
                {
                    let source_ml_source1_temporary_values33 = result.clone();
                    {
                        let source_ml_source1_temporary_index33 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s56 =
                                self::source_ml_source1_temporary_reachable_loop36(
                                    facts,
                                    width,
                                    span,
                                    result,
                                    progress_2,
                                    &(source_ml_source1_temporary_values33),
                                    source_ml_source1_temporary_index33,
                                );
                            match source_ml_source2_s56 {
                                self::MlSource1TemporaryReachableLoop36Result::MlSource1TemporaryReachableLoop36Done(result_2, progress_3) => {
                                    {
                                        (result, progress) = (result_2, progress_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsLoop40Result {
    MlSource1TemporaryAcceptsEffectsLoop40Done,
    MlSource1TemporaryAcceptsEffectsLoop40Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsJoin41 {
    MlSource1TemporaryAcceptsEffectsJoin41Next,
    MlSource1TemporaryAcceptsEffectsJoin41Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_accepts_effects_loop40(
    facts: &[f64],
    roles: &[self::ml_string::Text],
    operations: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    owners: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut functions: Vec<f64>,
    mut source: f64,
    mut width: f64,
    mut span: f64,
    roots: &[f64],
    reached: &[f64],
    mut node: f64,
) -> self::MlSource1TemporaryAcceptsEffectsLoop40Result {
    loop {
        return if node < (roles.len() as f64) {
            if self::ml_array::includes(
                reached,
                &self::read_required_number_fact(owners, node),
                0f64,
                false,
                |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
            ) {
                if ((self::text_fact(roles, node))
                    == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])))
                    || ((self::text_fact(roles, node))
                        == (self::ml_string::Text::from_units(vec![
                            99, 101, 108, 108, 32, 119, 114, 105, 116, 101,
                        ])))
                {
                    self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Return(false)
                } else {
                    {
                        let source_ml_source2_s57 = if ((self::text_fact(roles, node))
                            == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
                            || ((self::text_fact(roles, node))
                                == (self::ml_string::Text::from_units(vec![
                                    97, 112, 112, 108, 121,
                                ]))) {
                            {
                                let actual = self::edges(
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
                                    let callees = Vec::<f64>::new();
                                    {
                                        let callees_4 = if (self::text_fact(roles, node))
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 97, 108, 108,
                                            ])) {
                                            { actual }
                                        } else {
                                            let ml_library_inequality_1 =
                                                (actual.len() as f64) != 0f64;
                                            if ml_library_inequality_1 {
                                                {
                                                    self::read_roots(
                                                        facts,
                                                        1f64,
                                                        self::read_required_number_fact(
                                                            &(actual),
                                                            0f64,
                                                        ),
                                                        width,
                                                        span,
                                                    )
                                                }
                                            } else {
                                                callees
                                            }
                                        };
                                        if ((callees_4.len() as f64) == 0f64) || {
                                            let ml_values9 = callees_4;
                                            let ml_callback9 = {
                                                let functions = functions.clone();
                                                std::rc::Rc::new(move |target: f64| -> bool {
                                                    !(self::ml_array::includes(
                                                        &functions,
                                                        &target,
                                                        0f64,
                                                        false,
                                                        |a: &f64, b: &f64| {
                                                            a == b || (a.is_nan() && b.is_nan())
                                                        },
                                                    ))
                                                })
                                            };
                                            ml_values9.iter().cloned().enumerate().any(
                                                |(ml_index9, ml_value9)| ml_callback9(ml_value9),
                                            )
                                        } {
                                            self::MlSource1TemporaryAcceptsEffectsJoin41::MlSource1TemporaryAcceptsEffectsJoin41Return(false)
                                        } else {
                                            self::MlSource1TemporaryAcceptsEffectsJoin41::MlSource1TemporaryAcceptsEffectsJoin41Next
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryAcceptsEffectsJoin41::MlSource1TemporaryAcceptsEffectsJoin41Next
                        };
                        match source_ml_source2_s57 {
                            self::MlSource1TemporaryAcceptsEffectsJoin41::MlSource1TemporaryAcceptsEffectsJoin41Next => {
                                if (self::text_fact(roles, node)) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])) {
                                    {
                                        let object = self::first_number(&(self::edges(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])))));
                                        {
                                            let effect = self::read_disjoint_collection_iteration_method_effect(&(self::text_fact(kinds, object)), &(self::text_fact(operations, node)));
                                            if effect == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])) {
                                                self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Return(false)
                                            } else if (effect == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![99, 101, 108, 108])], &self::text_fact(kinds, object), 0f64, false, |a, b| a == b)) {
                                                    self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Return(false)
                                                } else if (effect == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && (((self::text_fact(kinds, object)) == (self::text_fact(kinds, source))) && !(self::separate_roots(roots, self::read_roots(facts, 1f64, object, width, span)))) {
                                                        self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Return(false)
                                                    } else {
                                                        {
                                                            let arguments_ = self::edges(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116])));
                                                            if self::read_disjoint_collection_iteration_callback(&(self::text_fact(operations, node))) && ((arguments_.len() as f64) != 0f64) {
                                                                {
                                                                    let callees_5 = self::read_roots(facts, 1f64, self::read_required_number_fact(&(arguments_), 0f64), width, span);
                                                                    if ((callees_5.len() as f64) == 0f64) || { let ml_values10 = callees_5; let ml_callback10 = { let functions = functions.clone(); std::rc::Rc::new(move |target: f64| -> bool { !(self::ml_array::includes(&functions, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values10.iter().cloned().enumerate().any(|(ml_index10, ml_value10)| ml_callback10(ml_value10)) } {
                                                                        self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Return(false)
                                                                    } else {
                                                                        {
                                                                            let node_2 = node + 1f64;
                                                                            {
                                                                                node = node_2;
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
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
                                } else {
                                    {
                                        let node_4 = node + 1f64;
                                        {
                                            node = node_4;
                                            continue;
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryAcceptsEffectsJoin41::MlSource1TemporaryAcceptsEffectsJoin41Return(source_ml_source1_temporary_result41) => {
                                self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Return(source_ml_source1_temporary_result41)
                            }
                        }
                    }
                }
            } else {
                {
                    let node_5 = node + 1f64;
                    {
                        node = node_5;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsLoop39Result {
    MlSource1TemporaryAcceptsEffectsLoop39Done,
    MlSource1TemporaryAcceptsEffectsLoop39Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accepts_effects_loop39(
    functions: &[f64],
    bodies: &[f64],
    mut main_: f64,
    source_ml_source1_temporary_values38: &[f64],
    mut source_ml_source1_temporary_index38: self::ml::Big,
) -> self::MlSource1TemporaryAcceptsEffectsLoop39Result {
    loop {
        return if source_ml_source1_temporary_index38
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values38.len() as u128))
        {
            {
                let scope = self::ml_array::at(
                    source_ml_source1_temporary_values38,
                    source_ml_source1_temporary_index38.to_index(),
                );
                {
                    let position =
                        self::ml_array::index_of(functions, &scope, 0f64, false, |a, b| a == b);
                    if (position < 0f64)
                        || (((self::read_required_number_fact(bodies, position)) < 0f64)
                            && (scope != main_))
                    {
                        self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                    } else {
                        {
                            let source_ml_source1_temporary_index38_2 =
                                source_ml_source1_temporary_index38
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index38 =
                                    source_ml_source1_temporary_index38_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_bind_calls_loop46(
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values43: &[f64],
    mut source_ml_source1_temporary_index43: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index43
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values43.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values43,
                    source_ml_source1_temporary_index43.to_index(),
                );
                {
                    let roots_2 = self::add_value(&(roots), root);
                    {
                        let source_ml_source1_temporary_index43_2 =
                            source_ml_source1_temporary_index43.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index43) =
                                (roots_2, source_ml_source1_temporary_index43_2);
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
pub fn source_ml_source1_temporary_bind_calls_loop45(
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
                                        let source_ml_source1_temporary_values43 =
                                            self::read_roots(&(result), 5f64, callee, width, span);
                                        {
                                            let source_ml_source1_temporary_index43 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                self::source_ml_source1_temporary_bind_calls_loop46(
                                                    roots_3,
                                                    &(source_ml_source1_temporary_values43),
                                                    source_ml_source1_temporary_index43,
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
pub fn source_ml_source1_temporary_bind_calls_loop44(
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
    source_ml_source1_temporary_values42: &[f64],
    mut source_ml_source1_temporary_index42: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index42
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values42.len() as u128))
        {
            {
                let callee = self::ml_array::at(
                    source_ml_source1_temporary_values42,
                    source_ml_source1_temporary_index42.to_index(),
                );
                {
                    let position =
                        self::ml_array::index_of(functions, &callee, 0f64, false, |a, b| a == b);
                    if position < 0f64 {
                        {
                            let source_ml_source1_temporary_index42_2 =
                                source_ml_source1_temporary_index42
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index42 =
                                    source_ml_source1_temporary_index42_2;
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
                                                self::source_ml_source1_temporary_bind_calls_loop45(
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
                                                let source_ml_source1_temporary_index42_3 =
                                                    source_ml_source1_temporary_index42
                                                        .add(&self::ml::Big::from_i128(1));
                                                {
                                                    (result, source_ml_source1_temporary_index42) = (
                                                        result_6,
                                                        source_ml_source1_temporary_index42_3,
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
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59(
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
                                            let source_ml_source2_concat_value58 = safe;
                                            {
                                                let source_ml_source2_concat_value59 = vec![node];
                                                self::ml_array::append(
                                                    &(self::ml_array::append(
                                                        &(Vec::<f64>::new()),
                                                        source_ml_source2_concat_value58,
                                                    )),
                                                    source_ml_source2_concat_value59,
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
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop58Done(Vec<f64>),
    MlSource1TemporaryProveDisjointCollectionIterationsLoop58Return(Vec<f64>),
}
