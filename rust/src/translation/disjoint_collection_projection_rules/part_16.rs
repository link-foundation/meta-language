// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop95Result;
use super::append;
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
use super::read_required_number_fact;
use super::row;
use super::symbol;
use super::word;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop95(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut state: Vec<Vec<f64>>,
    mut owner: f64,
    mut params: f64,
    mut names: Vec<f64>,
    mut bindings: Vec<f64>,
    mut parameter: f64,
) -> self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop95Result {
    loop {
        if parameter < (self::count(counts, params)) {
            {
                let actual = self::read_collection_construction_element(
                    starts, counts, targets, params, parameter,
                );
                {
                    let state_2 = self::make(
                        &(state),
                        2f64,
                        -1f64,
                        self::kind(starts, counts, labels, targets, actual),
                        owner,
                    );
                    {
                        let binding = self::last(&(state_2));
                        {
                            let state_3 = self::append(
                                &(self::append(&(state_2), 9f64, owner)),
                                10f64,
                                binding,
                            );
                            {
                                let names_2 = {
                                    let source_ml_source2_concat_value90 = names;
                                    {
                                        let source_ml_source2_concat_value91 = vec![self::field(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            actual,
                                            &(self::ml_string::Text::from_units(vec![
                                                110, 97, 109, 101,
                                            ])),
                                        )];
                                        self::ml_array::append(
                                            &(self::ml_array::append(
                                                &(Vec::<f64>::new()),
                                                source_ml_source2_concat_value90,
                                            )),
                                            source_ml_source2_concat_value91,
                                        )
                                    }
                                };
                                {
                                    let bindings_2 = {
                                        let source_ml_source2_concat_value92 = bindings;
                                        {
                                            let source_ml_source2_concat_value93 = vec![binding];
                                            self::ml_array::append(
                                                &(self::ml_array::append(
                                                    &(Vec::<f64>::new()),
                                                    source_ml_source2_concat_value92,
                                                )),
                                                source_ml_source2_concat_value93,
                                            )
                                        }
                                    };
                                    {
                                        let parameter_2 = parameter + 1f64;
                                        {
                                            (state, names, bindings, parameter) =
                                                (state_3, names_2, bindings_2, parameter_2);
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
            self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop95Result::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop95Done(state, names, bindings)
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
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop94(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    markers: &[self::ml_string::Text],
    mut state: Vec<Vec<f64>>,
    mut declarations: f64,
    mut at: f64,
) -> Vec<Vec<f64>> {
    loop {
        return if at < (self::count(counts, declarations)) {
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
                )) != (self::ml_string::Text::from_units(vec![102, 110]))
                {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let owner = self::symbol(
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
                            let params = self::field(
                                starts,
                                counts,
                                labels,
                                targets,
                                declaration,
                                &(self::ml_string::Text::from_units(vec![
                                    112, 97, 114, 97, 109, 115,
                                ])),
                            );
                            {
                                let names = Vec::<f64>::new();
                                {
                                    let bindings = Vec::<f64>::new();
                                    {
                                        let parameter = 0f64;
                                        {
                                            let source_ml_source2_s94 = self::source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop95(starts, counts, labels, targets, state, owner, params, names, bindings, parameter);
                                            match source_ml_source2_s94 {
                                                self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop95Result::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop95Done(state_2, names_2, bindings_2) => {
                                                    {
                                                        let state_3 = self::expression(&(state_2), self::field(starts, counts, labels, targets, declaration, &(self::ml_string::Text::from_units(vec![98, 111, 100, 121]))), owner, &(names_2), &(bindings_2), texts, starts, counts, labels, targets, markers, 0f64);
                                                        {
                                                            let state_4 = self::append(&(self::append(&(state_3), 11f64, owner)), 12f64, self::last(&(state_3)));
                                                            {
                                                                let at_3 = at + 1f64;
                                                                {
                                                                    (state, at) = (state_4, at_3);
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
            }
        } else {
            state
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop97(
    state: &[Vec<f64>],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < ((self::row(state, 0f64)).len() as f64) {
            {
                let result_2 = {
                    let source_ml_source2_concat_value95 = result;
                    {
                        let source_ml_source2_concat_value96 = vec![
                            self::read_required_number_fact(&(self::row(state, 0f64)), at),
                            self::read_required_number_fact(&(self::row(state, 1f64)), at),
                            self::read_required_number_fact(&(self::row(state, 2f64)), at),
                            self::read_required_number_fact(&(self::row(state, 3f64)), at),
                        ];
                        self::ml_array::append(
                            &(self::ml_array::append(
                                &(Vec::<f64>::new()),
                                source_ml_source2_concat_value95,
                            )),
                            source_ml_source2_concat_value96,
                        )
                    }
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
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop98(
    state: &[Vec<f64>],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < ((self::row(state, 4f64)).len() as f64) {
            {
                let result_2 = {
                    let source_ml_source2_concat_value97 = result;
                    {
                        let source_ml_source2_concat_value98 = vec![
                            self::read_required_number_fact(&(self::row(state, 4f64)), at),
                            self::read_required_number_fact(&(self::row(state, 5f64)), at),
                            self::read_required_number_fact(&(self::row(state, 6f64)), at),
                            self::read_required_number_fact(&(self::row(state, 7f64)), at),
                        ];
                        self::ml_array::append(
                            &(self::ml_array::append(
                                &(Vec::<f64>::new()),
                                source_ml_source2_concat_value97,
                            )),
                            source_ml_source2_concat_value98,
                        )
                    }
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
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop99(
    state: &[Vec<f64>],
    mut result: Vec<f64>,
    source_ml_source1_temporary_values92: &[f64],
    mut source_ml_source1_temporary_index92: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index92
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values92.len() as u128))
        {
            {
                let owner = self::ml_array::at(
                    source_ml_source1_temporary_values92,
                    source_ml_source1_temporary_index92.to_index(),
                );
                {
                    let position = self::ml_array::index_of(
                        &self::row(state, 11f64),
                        &owner,
                        0f64,
                        false,
                        |a, b| a == b,
                    );
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value99 = result;
                            {
                                let source_ml_source2_concat_value100 = vec![
                                    owner,
                                    if position < 0f64 {
                                        -1f64
                                    } else {
                                        self::read_required_number_fact(
                                            &(self::row(state, 12f64)),
                                            position,
                                        )
                                    },
                                ];
                                self::ml_array::append(
                                    &(self::ml_array::append(
                                        &(Vec::<f64>::new()),
                                        source_ml_source2_concat_value99,
                                    )),
                                    source_ml_source2_concat_value100,
                                )
                            }
                        };
                        {
                            let source_ml_source1_temporary_index92_2 =
                                source_ml_source1_temporary_index92
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index92) =
                                    (result_2, source_ml_source1_temporary_index92_2);
                                continue;
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
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop100(
    state: &[Vec<f64>],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < ((self::row(state, 9f64)).len() as f64) {
            {
                let result_2 = {
                    let source_ml_source2_concat_value101 = result;
                    {
                        let source_ml_source2_concat_value102 = vec![
                            self::read_required_number_fact(&(self::row(state, 9f64)), at),
                            self::read_required_number_fact(&(self::row(state, 10f64)), at),
                        ];
                        self::ml_array::append(
                            &(self::ml_array::append(
                                &(Vec::<f64>::new()),
                                source_ml_source2_concat_value101,
                            )),
                            source_ml_source2_concat_value102,
                        )
                    }
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
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop101(
    state: &[Vec<f64>],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < ((self::row(state, 17f64)).len() as f64) {
            {
                let result_2 = {
                    let source_ml_source2_concat_value103 = result;
                    {
                        let source_ml_source2_concat_value104 = vec![
                            self::read_required_number_fact(&(self::row(state, 17f64)), at),
                            self::read_required_number_fact(&(self::row(state, 18f64)), at),
                        ];
                        self::ml_array::append(
                            &(self::ml_array::append(
                                &(Vec::<f64>::new()),
                                source_ml_source2_concat_value103,
                            )),
                            source_ml_source2_concat_value104,
                        )
                    }
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
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop102(
    state: &[Vec<f64>],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < ((self::row(state, 19f64)).len() as f64) {
            {
                let result_2 = {
                    let source_ml_source2_concat_value105 = result;
                    {
                        let source_ml_source2_concat_value106 = vec![
                            self::read_required_number_fact(&(self::row(state, 19f64)), at),
                            self::read_required_number_fact(&(self::row(state, 20f64)), at),
                        ];
                        self::ml_array::append(
                            &(self::ml_array::append(
                                &(Vec::<f64>::new()),
                                source_ml_source2_concat_value105,
                            )),
                            source_ml_source2_concat_value106,
                        )
                    }
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
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Result {
    MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Done(Vec<Vec<f64>>),
    MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_project_disjoint_collection_iteration_facts_loop96(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    markers: &[self::ml_string::Text],
    mut state: Vec<Vec<f64>>,
    mut main_: f64,
    mut names: Vec<f64>,
    mut bindings: Vec<f64>,
    mut effects: f64,
    mut at: f64,
) -> self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Result {
    loop {
        return if at < (self::count(counts, effects)) {
            {
                let effect = self::read_collection_construction_element(
                    starts, counts, targets, effects, at,
                );
                {
                    let shape = self::word(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        effect,
                        &(self::ml_string::Text::from_units(vec![107])),
                    );
                    if shape == (self::ml_string::Text::from_units(vec![108, 101, 116])) {
                        {
                            let value = self::field(
                                starts,
                                counts,
                                labels,
                                targets,
                                effect,
                                &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                            );
                            {
                                let state_2 = self::expression(
                                    &(state),
                                    value,
                                    main_,
                                    &(names),
                                    &(bindings),
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    markers,
                                    0f64,
                                );
                                {
                                    let actual = self::last(&(state_2));
                                    {
                                        let state_3 = self::make(
                                            &(state_2),
                                            4f64,
                                            -1f64,
                                            self::kind(starts, counts, labels, targets, value),
                                            main_,
                                        );
                                        {
                                            let binding = self::last(&(state_3));
                                            {
                                                let state_4 = self::edge(
                                                    &(state_3),
                                                    binding,
                                                    1f64,
                                                    -1f64,
                                                    actual,
                                                );
                                                {
                                                    let names_2 = {
                                                        let source_ml_source2_concat_value107 =
                                                            names;
                                                        {
                                                            let source_ml_source2_concat_value108 = vec![self::field(starts, counts, labels, targets, effect, &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])))];
                                                            self::ml_array::append(&(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value107)), source_ml_source2_concat_value108)
                                                        }
                                                    };
                                                    {
                                                        let bindings_2 = {
                                                            let source_ml_source2_concat_value109 =
                                                                bindings;
                                                            {
                                                                let source_ml_source2_concat_value110 =
                                                                    vec![binding];
                                                                self::ml_array::append(&(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value109)), source_ml_source2_concat_value110)
                                                            }
                                                        };
                                                        {
                                                            let at_2 = at + 1f64;
                                                            {
                                                                (state, names, bindings, at) = (
                                                                    state_4, names_2, bindings_2,
                                                                    at_2,
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
                    {
                        if (shape
                            == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116])))
                            || (shape
                                == (self::ml_string::Text::from_units(vec![
                                    111, 117, 116, 112, 117, 116,
                                ])))
                        {
                            {
                                let state_5 = self::expression(
                                    &(state),
                                    self::field(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        effect,
                                        &(self::ml_string::Text::from_units(vec![
                                            101, 120, 112, 114,
                                        ])),
                                    ),
                                    main_,
                                    &(names),
                                    &(bindings),
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    markers,
                                    0f64,
                                );
                                {
                                    let at_3 = at + 1f64;
                                    {
                                        (state, at) = (state_5, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if shape
                                == (self::ml_string::Text::from_units(vec![
                                    97, 115, 115, 101, 114, 116,
                                ]))
                            {
                                self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Result::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Return(vec![1f64])
                            } else {
                                self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Result::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Return(vec![1f64])
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Result::MlSource1TemporaryProjectDisjointCollectionIterationFactsLoop96Done(state)
        };
    }
}
