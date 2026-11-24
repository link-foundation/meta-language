// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::MlSource1TemporaryExpressionLoop85Result;
use super::append;
use super::callable;
use super::count;
use super::edge;
use super::expression;
use super::field;
use super::kind;
use super::last;
use super::make;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_collection_construction_element;
use super::read_disjoint_collection_iteration_write_field;
use super::text;
use super::word;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_expression_loop85(
    mut state: Vec<Vec<f64>>,
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    mut local_names: Vec<f64>,
    mut local_bindings: Vec<f64>,
    mut params: f64,
    mut at: f64,
) -> self::MlSource1TemporaryExpressionLoop85Result {
    loop {
        if at < (self::count(counts, params)) {
            {
                let param =
                    self::read_collection_construction_element(starts, counts, targets, params, at);
                {
                    let state_2 = self::make(
                        &(state),
                        2f64,
                        -1f64,
                        self::kind(starts, counts, labels, targets, param),
                        node,
                    );
                    {
                        let binding = self::last(&(state_2));
                        {
                            let state_3 = self::append(
                                &(self::append(&(state_2), 9f64, node)),
                                10f64,
                                binding,
                            );
                            {
                                let local_names_2 = {
                                    let source_ml_source2_concat_value70 = local_names;
                                    {
                                        let source_ml_source2_concat_value71 = vec![self::field(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            param,
                                            &(self::ml_string::Text::from_units(vec![
                                                110, 97, 109, 101,
                                            ])),
                                        )];
                                        self::ml_array::append(
                                            &(self::ml_array::append(
                                                &(Vec::<f64>::new()),
                                                source_ml_source2_concat_value70,
                                            )),
                                            source_ml_source2_concat_value71,
                                        )
                                    }
                                };
                                {
                                    let local_bindings_2 = {
                                        let source_ml_source2_concat_value72 = local_bindings;
                                        {
                                            let source_ml_source2_concat_value73 = vec![binding];
                                            self::ml_array::append(
                                                &(self::ml_array::append(
                                                    &(Vec::<f64>::new()),
                                                    source_ml_source2_concat_value72,
                                                )),
                                                source_ml_source2_concat_value73,
                                            )
                                        }
                                    };
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            (state, local_names, local_bindings, at) =
                                                (state_3, local_names_2, local_bindings_2, at_2);
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
        return {
            self::MlSource1TemporaryExpressionLoop85Result::MlSource1TemporaryExpressionLoop85Done(
                state,
                local_names,
                local_bindings,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExpressionJoin87 {
    MlSource1TemporaryExpressionJoin87Next(Vec<Vec<f64>>, Vec<f64>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExpressionLoop88Result {
    MlSource1TemporaryExpressionLoop88Done(Vec<Vec<f64>>, Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_expression_loop88(
    mut state: Vec<Vec<f64>>,
    mut owner: f64,
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut value: f64,
    mut local_names: Vec<f64>,
    mut local_bindings: Vec<f64>,
    mut actual: f64,
    mut bind: f64,
) -> self::MlSource1TemporaryExpressionLoop88Result {
    loop {
        if bind < (self::count(counts, actual)) {
            {
                let name = self::read_collection_construction_element(
                    starts, counts, targets, actual, bind,
                );
                if (self::text(texts, name)) == (self::ml_string::Text::from_units(vec![])) {
                    {
                        let bind_2 = bind + 1f64;
                        {
                            bind = bind_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let state_2 = self::make(&(state), 8f64, (-2f64 - bind), -1f64, owner);
                        {
                            let binding = self::last(&(state_2));
                            {
                                let state_3 = self::edge(&(state_2), binding, 2f64, -1f64, value);
                                {
                                    let local_names_2 = {
                                        let source_ml_source2_concat_value74 = local_names;
                                        {
                                            let source_ml_source2_concat_value75 = vec![name];
                                            self::ml_array::append(
                                                &(self::ml_array::append(
                                                    &(Vec::<f64>::new()),
                                                    source_ml_source2_concat_value74,
                                                )),
                                                source_ml_source2_concat_value75,
                                            )
                                        }
                                    };
                                    {
                                        let local_bindings_2 = {
                                            let source_ml_source2_concat_value76 = local_bindings;
                                            {
                                                let source_ml_source2_concat_value77 =
                                                    vec![binding];
                                                self::ml_array::append(
                                                    &(self::ml_array::append(
                                                        &(Vec::<f64>::new()),
                                                        source_ml_source2_concat_value76,
                                                    )),
                                                    source_ml_source2_concat_value77,
                                                )
                                            }
                                        };
                                        {
                                            let bind_3 = bind + 1f64;
                                            {
                                                (state, local_names, local_bindings, bind) = (
                                                    state_3,
                                                    local_names_2,
                                                    local_bindings_2,
                                                    bind_3,
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
        return {
            self::MlSource1TemporaryExpressionLoop88Result::MlSource1TemporaryExpressionLoop88Done(
                state,
                local_names,
                local_bindings,
            )
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
pub fn source_ml_source1_temporary_expression_loop86(
    mut state: Vec<Vec<f64>>,
    mut owner: f64,
    names: &[f64],
    bindings: &[f64],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    markers: &[self::ml_string::Text],
    mut depth: f64,
    mut node: f64,
    mut scrutinee: f64,
    mut value: f64,
    mut cases: f64,
    mut at: f64,
) -> Vec<Vec<f64>> {
    loop {
        return if at < (self::count(counts, cases)) {
            {
                let arm =
                    self::read_collection_construction_element(starts, counts, targets, cases, at);
                {
                    let pattern = self::field(
                        starts,
                        counts,
                        labels,
                        targets,
                        arm,
                        &(self::ml_string::Text::from_units(vec![
                            112, 97, 116, 116, 101, 114, 110,
                        ])),
                    );
                    {
                        let form = self::word(
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            pattern,
                            &(self::ml_string::Text::from_units(vec![107])),
                        );
                        {
                            let local_names = names.to_vec();
                            {
                                let local_bindings = bindings.to_vec();
                                {
                                    let source_ml_source2_s83 = if form
                                        == (self::ml_string::Text::from_units(vec![
                                            98, 105, 110, 100,
                                        ])) {
                                        {
                                            let state_2 = self::make(
                                                &(state),
                                                4f64,
                                                -1f64,
                                                self::kind(
                                                    starts, counts, labels, targets, scrutinee,
                                                ),
                                                owner,
                                            );
                                            {
                                                let binding = self::last(&(state_2));
                                                {
                                                    let state_3 = self::edge(
                                                        &(state_2),
                                                        binding,
                                                        1f64,
                                                        -1f64,
                                                        value,
                                                    );
                                                    {
                                                        let local_names_2 = {
                                                            let source_ml_source2_concat_value84 =
                                                                local_names;
                                                            {
                                                                let source_ml_source2_concat_value85 = vec![self::field(starts, counts, labels, targets, pattern, &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])))];
                                                                self::ml_array::append(&(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value84)), source_ml_source2_concat_value85)
                                                            }
                                                        };
                                                        {
                                                            let local_bindings_2 = {
                                                                let source_ml_source2_concat_value86 =
                                                                    local_bindings;
                                                                {
                                                                    let source_ml_source2_concat_value87 =
                                                                        vec![binding];
                                                                    self::ml_array::append(&(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value86)), source_ml_source2_concat_value87)
                                                                }
                                                            };
                                                            self::MlSource1TemporaryExpressionJoin87::MlSource1TemporaryExpressionJoin87Next(state_3, local_names_2, local_bindings_2)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else if form
                                        == (self::ml_string::Text::from_units(vec![
                                            99, 116, 111, 114,
                                        ]))
                                    {
                                        {
                                            let actual = self::field(
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                pattern,
                                                &(self::ml_string::Text::from_units(vec![
                                                    98, 105, 110, 100, 115,
                                                ])),
                                            );
                                            {
                                                let bind = 0f64;
                                                {
                                                    let source_ml_source2_s88 = self::source_ml_source1_temporary_expression_loop88(state, owner, texts, starts, counts, targets, value, local_names, local_bindings, actual, bind);
                                                    match source_ml_source2_s88 {
                                                            self::MlSource1TemporaryExpressionLoop88Result::MlSource1TemporaryExpressionLoop88Done(state_4, local_names_3, local_bindings_3) => {
                                                                self::MlSource1TemporaryExpressionJoin87::MlSource1TemporaryExpressionJoin87Next(state_4, local_names_3, local_bindings_3)
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryExpressionJoin87::MlSource1TemporaryExpressionJoin87Next(state, local_names, local_bindings)
                                    };
                                    match source_ml_source2_s83 {
                                        self::MlSource1TemporaryExpressionJoin87::MlSource1TemporaryExpressionJoin87Next(state_5, local_names_4, local_bindings_4) => {
                                            {
                                                let state_6 = self::expression(&(state_5), self::field(starts, counts, labels, targets, arm, &(self::ml_string::Text::from_units(vec![98, 111, 100, 121]))), owner, &(local_names_4), &(local_bindings_4), texts, starts, counts, labels, targets, markers, (depth + 1f64));
                                                {
                                                    let state_7 = self::edge(&(state_6), node, 3f64, -1f64, self::last(&(state_6)));
                                                    {
                                                        let at_2 = at + 1f64;
                                                        {
                                                            (state, at) = (state_7, at_2);
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
        } else {
            state
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
pub fn source_ml_source1_temporary_expression_loop89(
    mut state: Vec<Vec<f64>>,
    mut owner: f64,
    names: &[f64],
    bindings: &[f64],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    markers: &[self::ml_string::Text],
    mut depth: f64,
    shape: &self::ml_string::Text,
    mut node: f64,
    mut values: f64,
    mut at: f64,
) -> Vec<Vec<f64>> {
    loop {
        if at < (self::count(counts, values)) {
            {
                let item =
                    self::read_collection_construction_element(starts, counts, targets, values, at);
                {
                    let value = if (*shape)
                        == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))
                    {
                        self::field(
                            starts,
                            counts,
                            labels,
                            targets,
                            item,
                            &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                        )
                    } else {
                        item
                    };
                    {
                        let state_2 = self::expression(
                            &(state),
                            value,
                            owner,
                            names,
                            bindings,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            markers,
                            (depth + 1f64),
                        );
                        if (*shape)
                            != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))
                        {
                            {
                                let state_3 = self::edge(
                                    &(state_2),
                                    node,
                                    if (*shape)
                                        == (self::ml_string::Text::from_units(vec![
                                            114, 101, 99, 111, 114, 100,
                                        ]))
                                    {
                                        6f64
                                    } else {
                                        7f64
                                    },
                                    if (*shape)
                                        == (self::ml_string::Text::from_units(vec![
                                            114, 101, 99, 111, 114, 100,
                                        ]))
                                    {
                                        self::field(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            item,
                                            &(self::ml_string::Text::from_units(vec![
                                                110, 97, 109, 101,
                                            ])),
                                        )
                                    } else {
                                        at
                                    },
                                    self::last(&(state_2)),
                                );
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (state, at) = (state_3, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let at_3 = at + 1f64;
                                {
                                    (state, at) = (state_2, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { state };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExpressionLoop91Result {
    MlSource1TemporaryExpressionLoop91Done(Vec<Vec<f64>>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_expression_loop91(
    mut state: Vec<Vec<f64>>,
    mut owner: f64,
    names: &[f64],
    bindings: &[f64],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    markers: &[self::ml_string::Text],
    mut depth: f64,
    mut node: f64,
    mut value: f64,
    mut first: f64,
    mut at: f64,
) -> self::MlSource1TemporaryExpressionLoop91Result {
    loop {
        if at < (self::count(counts, value)) {
            {
                let state_2 = self::expression(
                    &(state),
                    self::read_collection_construction_element(starts, counts, targets, value, at),
                    owner,
                    names,
                    bindings,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    markers,
                    (depth + 1f64),
                );
                {
                    let state_3 = self::edge(&(state_2), node, 5f64, -1f64, self::last(&(state_2)));
                    if at == 0f64 {
                        {
                            let first_2 = self::last(&(state_3));
                            {
                                let at_2 = at + 1f64;
                                {
                                    (state, first, at) = (state_3, first_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let at_3 = at + 1f64;
                            {
                                (state, at) = (state_3, at_3);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryExpressionLoop91Result::MlSource1TemporaryExpressionLoop91Done(
                state, first,
            )
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
pub fn source_ml_source1_temporary_expression_loop90(
    mut state: Vec<Vec<f64>>,
    mut source: f64,
    mut owner: f64,
    names: &[f64],
    bindings: &[f64],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    markers: &[self::ml_string::Text],
    mut depth: f64,
    shape: &self::ml_string::Text,
    mut operation: f64,
    mut node: f64,
    source_ml_source1_temporary_values82: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index82: self::ml::Big,
) -> Vec<Vec<f64>> {
    loop {
        return if source_ml_source1_temporary_index82
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values82.len() as u128))
        {
            {
                let operand = self::ml_array::at(
                    source_ml_source1_temporary_values82,
                    source_ml_source1_temporary_index82.to_index(),
                );
                if operand == (self::ml_string::Text::from_units(vec![])) {
                    {
                        let source_ml_source1_temporary_index82_2 =
                            source_ml_source1_temporary_index82.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index82 =
                                source_ml_source1_temporary_index82_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let value =
                            self::field(starts, counts, labels, targets, source, &(operand));
                        if value < 0f64 {
                            {
                                let source_ml_source1_temporary_index82_3 =
                                    source_ml_source1_temporary_index82
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index82 =
                                        source_ml_source1_temporary_index82_3;
                                    continue;
                                }
                            }
                        }
                        {
                            if operand
                                == (self::ml_string::Text::from_units(vec![97, 114, 103, 115]))
                            {
                                {
                                    let first = -1f64;
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source2_s89 =
                                                self::source_ml_source1_temporary_expression_loop91(
                                                    state, owner, names, bindings, texts, starts,
                                                    counts, labels, targets, markers, depth, node,
                                                    value, first, at,
                                                );
                                            match source_ml_source2_s89 {
                                                self::MlSource1TemporaryExpressionLoop91Result::MlSource1TemporaryExpressionLoop91Done(state_2, first_2) => {
                                                    if ((*shape) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && (first_2 >= 0f64) {
                                                        {
                                                            let index = self::read_disjoint_collection_iteration_write_field(&(self::text(texts, operation)));
                                                            {
                                                                let object = self::field(starts, counts, labels, targets, source, &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                                                {
                                                                    let fields = self::field(starts, counts, labels, targets, self::field(starts, counts, labels, targets, object, &(self::ml_string::Text::from_units(vec![116, 121, 112, 101]))), &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                                    if (index >= 0f64) && (index < (self::count(counts, fields))) {
                                                                        {
                                                                            let descriptor = self::read_collection_construction_element(starts, counts, targets, fields, index);
                                                                            {
                                                                                let state_3 = self::edge(&(state_2), node, 6f64, self::field(starts, counts, labels, targets, descriptor, &(self::ml_string::Text::from_units(vec![110, 97, 109, 101]))), first_2);
                                                                                {
                                                                                    let source_ml_source1_temporary_index82_4 = source_ml_source1_temporary_index82.add(&self::ml::Big::from_i128(1));
                                                                                    {
                                                                                        (state, source_ml_source1_temporary_index82) = (state_3, source_ml_source1_temporary_index82_4);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
{
                                                                        {
                                                                            let source_ml_source1_temporary_index82_5 = source_ml_source1_temporary_index82.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (state, source_ml_source1_temporary_index82) = (state_2, source_ml_source1_temporary_index82_5);
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
{
                                                        {
                                                            let source_ml_source1_temporary_index82_6 = source_ml_source1_temporary_index82.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                (state, source_ml_source1_temporary_index82) = (state_2, source_ml_source1_temporary_index82_6);
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
                                {
                                    let state_4 = self::expression(
                                        &(state),
                                        value,
                                        owner,
                                        names,
                                        bindings,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        markers,
                                        (depth + 1f64),
                                    );
                                    {
                                        let label = if operand
                                            == (self::ml_string::Text::from_units(vec![102, 110]))
                                        {
                                            4f64
                                        } else if ((operand
                                            == (self::ml_string::Text::from_units(vec![
                                                114, 101, 102, 101, 114, 101, 110, 99, 101,
                                            ])))
                                            || ((operand
                                                == (self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101,
                                                ])))
                                                && ((*shape)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        114, 101, 102, 101, 114, 101, 110, 99, 101,
                                                        65, 108, 108, 111, 99, 97, 116, 101,
                                                    ])))))
                                            || ((operand
                                                == (self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101,
                                                ])))
                                                && ((*shape)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        99, 101, 108, 108, 65, 108, 108, 111, 99,
                                                        97, 116, 101,
                                                    ]))))
                                        {
                                            1f64
                                        } else if (operand
                                            == (self::ml_string::Text::from_units(vec![
                                                111, 98, 106, 101, 99, 116,
                                            ])))
                                            || (operand
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 101, 108, 108,
                                                ])))
                                        {
                                            2f64
                                        } else {
                                            3f64
                                        };
                                        {
                                            let state_5 = self::edge(
                                                &(state_4),
                                                node,
                                                label,
                                                -1f64,
                                                self::last(&(state_4)),
                                            );
                                            {
                                                let source_ml_source1_temporary_index82_7 =
                                                    source_ml_source1_temporary_index82
                                                        .add(&self::ml::Big::from_i128(1));
                                                {
                                                    (state, source_ml_source1_temporary_index82) = (
                                                        state_5,
                                                        source_ml_source1_temporary_index82_7,
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
        } else {
            state
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExpressionJoin83 {
    MlSource1TemporaryExpressionJoin83Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop93(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut state: Vec<Vec<f64>>,
    mut declarations: f64,
    mut at: f64,
) -> Vec<Vec<f64>> {
    loop {
        if at < (self::count(counts, declarations)) {
            {
                let declaration = self::read_collection_construction_element(
                    starts,
                    counts,
                    targets,
                    declarations,
                    at,
                );
                if (self::word(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    declaration,
                    &(self::ml_string::Text::from_units(vec![107])),
                )) == (self::ml_string::Text::from_units(vec![102, 110]))
                {
                    {
                        let state_2 = self::callable(
                            &(state),
                            self::field(
                                starts,
                                counts,
                                labels,
                                targets,
                                declaration,
                                &(self::ml_string::Text::from_units(vec![
                                    102, 117, 108, 108, 78, 97, 109, 101,
                                ])),
                            ),
                            texts,
                        );
                        {
                            let at_2 = at + 1f64;
                            {
                                (state, at) = (state_2, at_2);
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
        }
        return { state };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop95Result {
    MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop95Done(
        Vec<Vec<f64>>,
        Vec<f64>,
        Vec<f64>,
    ),
}
