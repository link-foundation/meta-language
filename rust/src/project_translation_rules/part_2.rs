// Generated from complete js/src/project-translation-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_project_translation_dependency_gaps(
    count: f64,
    failed: &[f64],
    importers: &[f64],
    providers: &[f64],
    limit: f64,
) -> Vec<f64> {
    {
        let parents = Vec::<f64>::new();
        {
            let roots = Vec::<f64>::new();
            {
                let queue = Vec::<f64>::new();
                {
                    let result = Vec::<f64>::new();
                    {
                        let work = 0f64;
                        {
                            let source_ml_temporary_values4 = failed.to_vec();
                            {
                                let source_ml_temporary_index4 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_s1 = self::source_ml_temporary_plan_project_translation_dependency_gaps_loop5(limit, parents, roots, queue, work, &(source_ml_temporary_values4), source_ml_temporary_index4);
                                    match source_ml_source1_s1 {
                                        self::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Done(parents_2, roots_2, queue_2, work_2) => {
                                            {
                                                let at = 0f64;
                                                {
                                                    let source_ml_source1_s2 = self::source_ml_temporary_plan_project_translation_dependency_gaps_loop6(importers, providers, limit, parents_2, roots_2, queue_2, work_2, at);
                                                    match source_ml_source1_s2 {
                                                        self::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Done(parents_3, roots_3, queue_3, work_3) => {
                                                            {
                                                                let index = 0f64;
                                                                {
                                                                    let source_ml_source1_s3 = self::source_ml_temporary_plan_project_translation_dependency_gaps_loop7(count, limit, &(parents_3), &(roots_3), &(queue_3), result, work_3, index);
                                                                    match source_ml_source1_s3 {
                                                                        self::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Done(result_2) => {
                                                                            result_2
                                                                        }
                                                                        self::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Return(source_ml_temporary_result7) => {
                                                                            source_ml_temporary_result7
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Return(source_ml_temporary_result7_2) => {
                                                            source_ml_temporary_result7_2
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Return(source_ml_temporary_result8) => {
                                            source_ml_temporary_result8
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
pub fn source_ml_temporary_read_project_translation_frontier_loop2(
    importers: &[self::ml_string::Text],
    providers: &[self::ml_string::Text],
    active: &[self::ml_string::Text],
    mut selected: Vec<self::ml_string::Text>,
    mut edge: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        return if edge < (importers.len() as f64) {
            {
                let provider =
                    self::ml_array::optional_at(providers, self::ml_array::number_index(edge));
                {
                    let importer =
                        self::ml_array::optional_at(importers, self::ml_array::number_index(edge));
                    if ({
                        let source_ml_source1_comparison_operand4 = provider.clone();
                        {
                            let source_ml_source1_comparison_operand5 = None::<()>;
                            !(match (
                                source_ml_source1_comparison_operand4,
                                source_ml_source1_comparison_operand5,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            })
                        }
                    } && {
                        let source_ml_source1_comparison_operand6 = importer.clone();
                        {
                            let source_ml_source1_comparison_operand7 = None::<()>;
                            !(match (
                                source_ml_source1_comparison_operand6,
                                source_ml_source1_comparison_operand7,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            })
                        }
                    }) && {
                        let ml_collection = active.to_vec();
                        let ml_search = provider
                            .clone()
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        ml_collection.iter().any(|ml_key| ml_key == &ml_search)
                    } {
                        {
                            let selected_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![importer.clone().unwrap_or_else(|| {
                                        self::ml_string::Text::from_units(vec![])
                                    })],
                                    selected,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let edge_2 = edge + 1f64;
                                {
                                    (selected, edge) = (selected_2, edge_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let edge_3 = edge + 1f64;
                            {
                                edge = edge_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            selected
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_project_translation_frontier_loop3(
    affected: &[self::ml_string::Text],
    mut next: Vec<self::ml_string::Text>,
    source_ml_temporary_values1: &[self::ml_string::Text],
    mut source_ml_temporary_index1: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        return if source_ml_temporary_index1
            < (self::ml::Big::from_u128(source_ml_temporary_values1.len() as u128))
        {
            {
                let identifier = self::ml_array::at(
                    source_ml_temporary_values1,
                    source_ml_temporary_index1.to_index(),
                );
                if {
                    let ml_collection = affected.to_vec();
                    let ml_search = identifier.clone();
                    ml_collection.iter().any(|ml_key| ml_key == &ml_search)
                } {
                    {
                        let next_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![identifier.clone()], next);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let source_ml_temporary_index1_2 =
                                source_ml_temporary_index1.add(&self::ml::Big::from_i128(1));
                            {
                                (next, source_ml_temporary_index1) =
                                    (next_2, source_ml_temporary_index1_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index1_3 =
                            source_ml_temporary_index1.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index1 = source_ml_temporary_index1_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            next
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanProjectTranslationDependencyGapsLoop7Result {
    MlTemporaryPlanProjectTranslationDependencyGapsLoop7Done(Vec<f64>),
    MlTemporaryPlanProjectTranslationDependencyGapsLoop7Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_project_translation_dependency_gaps_loop7(
    mut count: f64,
    mut limit: f64,
    parents: &[f64],
    roots: &[f64],
    queue: &[f64],
    mut result: Vec<f64>,
    mut work: f64,
    mut index: f64,
) -> self::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Result {
    loop {
        return if index < count {
            {
                let work_2 = work + (queue.len() as f64);
                if work_2 > limit {
                    self::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Return(vec![-1f64])
                } else {
                    {
                        let position =
                            self::ml_array::index_of(queue, &index, 0f64, false, |a, b| a == b);
                        if (position >= 0f64) && {
                            let source_ml_source1_comparison_operand8 = self::ml_array::optional_at(
                                parents,
                                self::ml_array::number_index(position),
                            );
                            {
                                let source_ml_source1_comparison_operand9 = index;
                                !(match (
                                    source_ml_source1_comparison_operand8,
                                    source_ml_source1_comparison_operand9,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                })
                            }
                        } {
                            {
                                let result_2 = {
                                    let (ml_append_right, ml_append_left) = (vec![index], result);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let result_3 = {
                                        let (ml_append_right, ml_append_left) = (
                                            vec![
                                                (self::ml_array::optional_at(
                                                    parents,
                                                    self::ml_array::number_index(position),
                                                ))
                                                .unwrap_or_else(|| -1f64),
                                            ],
                                            result_2,
                                        );
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let result_4 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![
                                                    (self::ml_array::optional_at(
                                                        roots,
                                                        self::ml_array::number_index(position),
                                                    ))
                                                    .unwrap_or_else(|| -1f64),
                                                ],
                                                result_3,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let index_2 = index + 1f64;
                                            {
                                                (result, work, index) = (result_4, work_2, index_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = index + 1f64;
                                {
                                    (work, index) = (work_2, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop7Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanProjectTranslationDependencyGapsLoop6Result {
    MlTemporaryPlanProjectTranslationDependencyGapsLoop6Done(Vec<f64>, Vec<f64>, Vec<f64>, f64),
    MlTemporaryPlanProjectTranslationDependencyGapsLoop6Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanProjectTranslationDependencyGapsLoop8Result {
    MlTemporaryPlanProjectTranslationDependencyGapsLoop8Done(Vec<f64>, Vec<f64>, Vec<f64>, f64),
    MlTemporaryPlanProjectTranslationDependencyGapsLoop8Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_project_translation_dependency_gaps_loop8(
    importers: &[f64],
    providers: &[f64],
    mut limit: f64,
    mut parents: Vec<f64>,
    mut roots: Vec<f64>,
    mut queue: Vec<f64>,
    mut work: f64,
    mut at: f64,
    mut current: f64,
    mut edge: f64,
) -> self::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Result {
    loop {
        return if edge < (providers.len() as f64) {
            {
                let work_2 = work + 1f64;
                if work_2 > limit {
                    self::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Return(vec![-1f64])
                } else if {
                    let source_ml_source1_comparison_operand10 =
                        self::ml_array::optional_at(providers, self::ml_array::number_index(edge));
                    {
                        let source_ml_source1_comparison_operand11 = current;
                        !(match (
                            source_ml_source1_comparison_operand10,
                            source_ml_source1_comparison_operand11,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } {
                    {
                        let edge_2 = edge + 1f64;
                        {
                            (work, edge) = (work_2, edge_2);
                            continue;
                        }
                    }
                } else {
                    {
                        let work_3 = work_2 + (queue.len() as f64);
                        if work_3 > limit {
                            self::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Return(vec![-1f64])
                        } else {
                            {
                                let importer = (self::ml_array::optional_at(
                                    importers,
                                    self::ml_array::number_index(edge),
                                ))
                                .unwrap_or_else(|| -1f64);
                                if !(self::ml_array::includes(
                                    &queue,
                                    &importer,
                                    0f64,
                                    false,
                                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                                )) {
                                    {
                                        let parents_2 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![current], parents);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let roots_2 = {
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![
                                                        (self::ml_array::optional_at(
                                                            &roots,
                                                            self::ml_array::number_index(at),
                                                        ))
                                                        .unwrap_or_else(|| -1f64),
                                                    ],
                                                    roots,
                                                );
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let queue_2 = {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![importer], queue);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                };
                                                {
                                                    let edge_3 = edge + 1f64;
                                                    {
                                                        (parents, roots, queue, work, edge) = (
                                                            parents_2, roots_2, queue_2, work_3,
                                                            edge_3,
                                                        );
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let edge_4 = edge + 1f64;
                                        {
                                            (work, edge) = (work_3, edge_4);
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
            self::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Done(parents, roots, queue, work)
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
pub fn source_ml_temporary_plan_project_translation_dependency_gaps_loop6(
    importers: &[f64],
    providers: &[f64],
    mut limit: f64,
    mut parents: Vec<f64>,
    mut roots: Vec<f64>,
    mut queue: Vec<f64>,
    mut work: f64,
    mut at: f64,
) -> self::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Result {
    loop {
        return if at < (queue.len() as f64) {
            {
                let current =
                    (self::ml_array::optional_at(&queue, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64);
                {
                    let edge = 0f64;
                    {
                        let source_ml_source1_s12 = self::source_ml_temporary_plan_project_translation_dependency_gaps_loop8(importers, providers, limit, parents, roots, queue, work, at, current, edge);
                        match source_ml_source1_s12 {
                            self::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Done(parents_2, roots_2, queue_2, work_2) => {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (parents, roots, queue, work, at) = (parents_2, roots_2, queue_2, work_2, at_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop8Return(source_ml_temporary_result8) => {
                                self::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Return(source_ml_temporary_result8)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop6Done(parents, roots, queue, work)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanProjectTranslationDependencyGapsLoop5Result {
    MlTemporaryPlanProjectTranslationDependencyGapsLoop5Done(Vec<f64>, Vec<f64>, Vec<f64>, f64),
    MlTemporaryPlanProjectTranslationDependencyGapsLoop5Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_project_translation_dependency_gaps_loop5(
    mut limit: f64,
    mut parents: Vec<f64>,
    mut roots: Vec<f64>,
    mut queue: Vec<f64>,
    mut work: f64,
    source_ml_temporary_values4: &[f64],
    mut source_ml_temporary_index4: self::ml::Big,
) -> self::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Result {
    loop {
        return if source_ml_temporary_index4
            < (self::ml::Big::from_u128(source_ml_temporary_values4.len() as u128))
        {
            {
                let index = self::ml_array::at(
                    source_ml_temporary_values4,
                    source_ml_temporary_index4.to_index(),
                );
                {
                    let work_2 = work + (queue.len() as f64);
                    if work_2 > limit {
                        self::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Return(vec![-1f64])
                    } else {
                        if !(self::ml_array::includes(
                            &queue,
                            &index,
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        )) {
                            {
                                let parents_2 = {
                                    let (ml_append_right, ml_append_left) = (vec![index], parents);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let roots_2 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![index], roots);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let queue_2 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![index], queue);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let source_ml_temporary_index4_2 =
                                                source_ml_temporary_index4
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (
                                                    parents,
                                                    roots,
                                                    queue,
                                                    work,
                                                    source_ml_temporary_index4,
                                                ) = (
                                                    parents_2,
                                                    roots_2,
                                                    queue_2,
                                                    work_2,
                                                    source_ml_temporary_index4_2,
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        {
                            {
                                let source_ml_temporary_index4_3 =
                                    source_ml_temporary_index4.add(&self::ml::Big::from_i128(1));
                                {
                                    (work, source_ml_temporary_index4) =
                                        (work_2, source_ml_temporary_index4_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Result::MlTemporaryPlanProjectTranslationDependencyGapsLoop5Done(parents, roots, queue, work)
        };
    }
}
