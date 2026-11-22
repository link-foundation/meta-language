// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::MlSource1TemporaryProveDisjointCollectionIterationsJoin67;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result;
use super::add_field;
use super::add_roots;
use super::add_value;
use super::bind_calls;
use super::copy_fields;
use super::edges;
use super::first_number;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_array_state_method_result;
use super::read_disjoint_collection_iteration_callback;
use super::read_disjoint_collection_iteration_identity_kind;
use super::read_field;
use super::read_reference_method_result;
use super::read_required_number_fact;
use super::read_roots;
use super::row;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop66;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop68;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72;
use super::text;
use super::text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop77(
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
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin76 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin76Next(Vec<f64>, f64),
    MlSource1TemporaryProveDisjointCollectionIterationsJoin76Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin75 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(Vec<f64>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin74 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop79(
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values50: &[f64],
    mut source_ml_source1_temporary_index50: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index50
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values50.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values50,
                    source_ml_source1_temporary_index50.to_index(),
                );
                {
                    let roots_2 = self::add_value(&(roots), root);
                    {
                        let source_ml_source1_temporary_index50_2 =
                            source_ml_source1_temporary_index50.add(&self::ml::Big::from_i128(1));
                        {
                            (roots, source_ml_source1_temporary_index50) =
                                (roots_2, source_ml_source1_temporary_index50_2);
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
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop78(
    mut width: f64,
    mut span: f64,
    facts: &[f64],
    mut roots: Vec<f64>,
    source_ml_source1_temporary_values49: &[f64],
    mut source_ml_source1_temporary_index49: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index49
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values49.len() as u128))
        {
            {
                let target = self::ml_array::at(
                    source_ml_source1_temporary_values49,
                    source_ml_source1_temporary_index49.to_index(),
                );
                {
                    let source_ml_source1_temporary_values50 =
                        self::read_roots(facts, 5f64, target, width, span);
                    {
                        let source_ml_source1_temporary_index50 = self::ml::Big::from_i128(0);
                        {
                            let roots_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop79(roots, &(source_ml_source1_temporary_values50), source_ml_source1_temporary_index50);
                            {
                                let source_ml_source1_temporary_index49_2 =
                                    source_ml_source1_temporary_index49
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (roots, source_ml_source1_temporary_index49) =
                                        (roots_2, source_ml_source1_temporary_index49_2);
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
pub enum MlSource1TemporaryProveDisjointCollectionIterationsJoin71 {
    MlSource1TemporaryProveDisjointCollectionIterationsJoin71Next(f64, Vec<f64>),
    MlSource1TemporaryProveDisjointCollectionIterationsJoin71Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop65(
    roles: &[self::ml_string::Text],
    operations: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    owners: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    functions: &[f64],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    bodies: &[f64],
    mut main_: f64,
    mut size: f64,
    mut width: f64,
    mut field_width: f64,
    mut span: f64,
    mut facts: Vec<f64>,
    mut work: f64,
    mut node: f64,
) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result {
    loop {
        return if node < size {
            {
                let work_2 = work
                    + (((facts.len() as f64) + 1f64)
                        * (self::read_required_number_fact(counts, node) + 4f64));
                if (((self::read_required_number_fact(&(facts), 0f64)) == -1f64)
                    || (work_2 > 268_435_456f64))
                    || ((facts.len() as f64) > 65_536f64)
                {
                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Return(vec![1f64])
                } else {
                    {
                        let facts_3 = if (self::text_fact(roles, node))
                            == (self::ml_string::Text::from_units(vec![
                                102, 117, 110, 99, 116, 105, 111, 110,
                            ])) {
                            { self::add_roots(&(facts), 1f64, node, &(vec![node]), width, span) }
                        } else {
                            facts
                        };
                        if !(self::ml_array::includes(
                            &facts_3,
                            &((4f64 * span) + self::read_required_number_fact(owners, node)),
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        )) {
                            {
                                let node_2 = node + 1f64;
                                {
                                    (facts, work, node) = (facts_3, work_2, node_2);
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let role = self::text_fact(roles, node);
                                {
                                    let object = self::first_number(
                                        &(self::edges(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            node,
                                            &(self::ml_string::Text::from_units(vec![
                                                111, 98, 106, 101, 99, 116,
                                            ])),
                                        )),
                                    );
                                    {
                                        let value = self::first_number(
                                            &(self::edges(
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                node,
                                                &(self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101,
                                                ])),
                                            )),
                                        );
                                        {
                                            let arguments_ = self::edges(
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
                                                let roots = Vec::<f64>::new();
                                                {
                                                    let roots_3 = if ((((((role
                                                        == (self::ml_string::Text::from_units(
                                                            vec![
                                                                97, 108, 108, 111, 99, 97, 116,
                                                                105, 111, 110,
                                                            ],
                                                        )))
                                                        || (role
                                                            == (self::ml_string::Text::from_units(
                                                                vec![114, 101, 99, 111, 114, 100],
                                                            ))))
                                                        || (role
                                                            == (self::ml_string::Text::from_units(
                                                                vec![97, 114, 114, 97, 121],
                                                            ))))
                                                        || (role
                                                            == (self::ml_string::Text::from_units(
                                                                vec![116, 117, 112, 108, 101],
                                                            ))))
                                                        || (role
                                                            == (self::ml_string::Text::from_units(
                                                                vec![100, 97, 116, 97],
                                                            ))))
                                                        || (role
                                                            == (self::ml_string::Text::from_units(
                                                                vec![
                                                                    115, 110, 97, 112, 115, 104,
                                                                    111, 116,
                                                                ],
                                                            ))))
                                                        || (role
                                                            == (self::ml_string::Text::from_units(
                                                                vec![
                                                                    101, 97, 103, 101, 114, 32,
                                                                    115, 110, 97, 112, 115, 104,
                                                                    111, 116,
                                                                ],
                                                            ))) {
                                                        { vec![node] }
                                                    } else {
                                                        roots
                                                    };
                                                    {
                                                        let roots_5 = if ((role == (self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 98, 108, 101]))) || (role == (self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103])))) || (role == (self::ml_string::Text::from_units(vec![114, 101, 97, 100]))) {
                                                            {
                                                                if value < 0f64 {
                                                                    Vec::<f64>::new()
                                                                } else {
                                                                    self::read_roots(&(facts_3), 1f64, value, width, span)
                                                                }
                                                            }
                                                        } else {
                                                            roots_3
                                                        };
                                                        {
                                                            let source_ml_source2_s64 = if role == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])) {
                                                                {
                                                                    let actual = self::read_roots(&(facts_3), 1f64, object, width, span);
                                                                    {
                                                                        let work_3 = work_2 + (((facts_3.len() as f64) + 1f64) * (actual.len() as f64));
                                                                        if work_3 > 268_435_456f64 {
                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsJoin71::MlSource1TemporaryProveDisjointCollectionIterationsJoin71Return(vec![1f64])
                                                                        } else {
                                                                            {
                                                                                let roots_6 = self::read_field(&(facts_3), &(actual), (self::ml_array::index_of(labels, &self::text_fact(operations, node), 0f64, false, |a, b| a == b) + 1f64), field_width, width, span);
                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin71::MlSource1TemporaryProveDisjointCollectionIterationsJoin71Next(work_3, roots_6)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin71::MlSource1TemporaryProveDisjointCollectionIterationsJoin71Next(work_2, roots_5)
                                                            };
                                                            match source_ml_source2_s64 {
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin71::MlSource1TemporaryProveDisjointCollectionIterationsJoin71Next(work_4, roots_7) => {
                                                                    {
                                                                        let roots_9 = if role == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101])) {
                                                                            {
                                                                                self::edges(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101])))
                                                                            }
                                                                        } else {
                                                                            roots_7
                                                                        };
                                                                        {
                                                                            let roots_11 = if role == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                                                                {
                                                                                    let source_ml_source1_temporary_values47 = self::edges(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116])));
                                                                                    {
                                                                                        let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop72(width, span, &(facts_3), roots_9, &(source_ml_source1_temporary_values47), source_ml_source1_temporary_index47)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                roots_9
                                                                            };
                                                                            {
                                                                                let source_ml_source2_s65 = if (role == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) || (role == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) {
                                                                                    {
                                                                                        let callee = self::edges(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 101, 101])));
                                                                                        {
                                                                                            let actual_2 = callee.clone();
                                                                                            {
                                                                                                let actual_4 = if role == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])) {
                                                                                                    {
                                                                                                        {
let ml_library_condition_1 =  { let ml_value = callee.len() as f64; ml_value != 0f64 && !ml_value.is_nan() };
if ml_library_condition_1 {
                                                                                                            self::read_roots(&(facts_3), 1f64, self::read_required_number_fact(&(callee), 0f64), width, span)
                                                                                                        } else {
                                                                                                            Vec::<f64>::new()
                                                                                                        }
}
                                                                                                    }
                                                                                                } else {
                                                                                                    actual_2
                                                                                                };
                                                                                                {
                                                                                                    let facts_4 = self::bind_calls(&(facts_3), self::read_required_number_fact(owners, node), &(actual_4), &(arguments_), &(self::ml_string::Text::from_units(vec![])), -1f64, kinds, functions, parameter_starts, parameter_counts, parameters, bodies, main_, width, span);
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_values49 = actual_4;
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index49 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let roots_12 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop78(width, span, &(facts_4), roots_11, &(source_ml_source1_temporary_values49), source_ml_source1_temporary_index49);
                                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_4, roots_12)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_3, roots_11)
                                                                                };
                                                                                match source_ml_source2_s65 {
                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin74::MlSource1TemporaryProveDisjointCollectionIterationsJoin74Next(facts_5, roots_13) => {
                                                                                        {
                                                                                            let source_ml_source2_s66 = if role == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])) {
                                                                                                {
                                                                                                    let roots_16 = if ((self::read_reference_method_result(&(self::text_fact(kinds, object)), &(self::text_fact(operations, node)))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))) || (((self::text_fact(kinds, object)) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) && (((self::read_array_state_method_result(&(self::text_fact(operations, node)))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))) || ((self::text_fact(operations, node)) == (self::ml_string::Text::from_units(vec![115, 111, 114, 116]))))) {
                                                                                                        {
                                                                                                            self::read_roots(&(facts_5), 1f64, object, width, span)
                                                                                                        }
                                                                                                    } else if self::read_disjoint_collection_iteration_identity_kind(&(self::text_fact(kinds, node))) {
                                                                                                            {
                                                                                                                vec![-1f64]
                                                                                                            }
                                                                                                        } else {
                                                                                                            roots_13
                                                                                                        };
                                                                                                    if self::read_disjoint_collection_iteration_callback(&(self::text_fact(operations, node))) && ((arguments_.len() as f64) != 0f64) {
                                                                                                        {
                                                                                                            let facts_6 = self::bind_calls(&(facts_5), self::read_required_number_fact(owners, node), &(self::read_roots(&(facts_5), 1f64, self::read_required_number_fact(&(arguments_), 0f64), width, span)), &(arguments_), &(self::text_fact(operations, node)), object, kinds, functions, parameter_starts, parameter_counts, parameters, bodies, main_, width, span);
                                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_6, roots_16)
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_5, roots_16)
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_5, roots_13)
                                                                                            };
                                                                                            match source_ml_source2_s66 {
                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin75::MlSource1TemporaryProveDisjointCollectionIterationsJoin75Next(facts_7, roots_17) => {
                                                                                                    {
                                                                                                        let roots_19 = if ((role == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110]))) || (role == (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 114, 101, 97, 100])))) && self::read_disjoint_collection_iteration_identity_kind(&(self::text_fact(kinds, node))) {
                                                                                                            {
                                                                                                                vec![-1f64]
                                                                                                            }
                                                                                                        } else {
                                                                                                            roots_17
                                                                                                        };
                                                                                                        {
                                                                                                            let facts_9 = if (role != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) && self::read_disjoint_collection_iteration_identity_kind(&(self::text_fact(kinds, node))) {
                                                                                                                {
                                                                                                                    self::add_roots(&(facts_7), 1f64, node, &(roots_19), width, span)
                                                                                                                }
                                                                                                            } else {
                                                                                                                facts_7
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_s67 = if role == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110])) {
                                                                                                                    {
                                                                                                                        let actual_5 = self::read_roots(&(facts_9), 1f64, value, width, span);
                                                                                                                        {
                                                                                                                            let work_5 = work_4 + (((facts_9.len() as f64) + 1f64) * (actual_5.len() as f64));
                                                                                                                            if work_5 > 268_435_456f64 {
                                                                                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin76::MlSource1TemporaryProveDisjointCollectionIterationsJoin76Return(vec![1f64])
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let facts_10 = self::copy_fields(&(facts_9), &(actual_5), node, field_width, width, span);
                                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin76::MlSource1TemporaryProveDisjointCollectionIterationsJoin76Next(facts_10, work_5)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin76::MlSource1TemporaryProveDisjointCollectionIterationsJoin76Next(facts_9, work_4)
                                                                                                                };
                                                                                                                match source_ml_source2_s67 {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin76::MlSource1TemporaryProveDisjointCollectionIterationsJoin76Next(facts_11, work_6) => {
                                                                                                                        if (((role == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) || (role == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (role == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))) || (role == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) {
                                                                                                                            {
                                                                                                                                let at = self::read_required_number_fact(starts, node);
                                                                                                                                {
                                                                                                                                    let facts_12 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop77(starts, counts, labels, targets, width, field_width, span, facts_11, node, &(role), object, at);
                                                                                                                                    {
                                                                                                                                        let node_3 = node + 1f64;
                                                                                                                                        {
                                                                                                                                            (facts, work, node) = (facts_12, work_6, node_3);
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
                                                                                                                                    (facts, work, node) = (facts_11, work_6, node_4);
                                                                                                                                    continue;
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin76::MlSource1TemporaryProveDisjointCollectionIterationsJoin76Return(source_ml_source1_temporary_result77) => {
                                                                                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Return(source_ml_source1_temporary_result77)
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
                                                                self::MlSource1TemporaryProveDisjointCollectionIterationsJoin71::MlSource1TemporaryProveDisjointCollectionIterationsJoin71Return(source_ml_source1_temporary_result79) => {
                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Return(source_ml_source1_temporary_result79)
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
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Done(facts, work)
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
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    bodies: &[f64],
    mut main_: f64,
    snapshots: &[f64],
    callbacks: &[f64],
    mut size: f64,
    mut width: f64,
    mut field_width: f64,
    mut span: f64,
    mut facts: Vec<f64>,
    mut work: f64,
    mut settled: bool,
    mut seeded: bool,
    mut pass: f64,
) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result {
    loop {
        return if (pass < 128f64) && !settled {
            {
                let before = facts.len() as f64;
                {
                    let node = 0f64;
                    {
                        let source_ml_source2_s68 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop65(roles, operations, kinds, owners, starts, counts, labels, targets, functions, parameter_starts, parameter_counts, parameters, bodies, main_, size, width, field_width, span, facts, work, node);
                        match source_ml_source2_s68 {
                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Done(facts_2, work_2) => {
                                {
                                    let at = 0f64;
                                    {
                                        let facts_3 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop66(functions, bodies, width, span, facts_2, at);
                                        if (facts_3.len() as f64) == before {
                                            {
                                                let source_ml_source2_s69 = if seeded {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin67::MlSource1TemporaryProveDisjointCollectionIterationsJoin67Next(facts_3, seeded)
                                                } else {
                                                    {
                                                        let seeded_2 = true;
                                                        {
                                                            let source_ml_source1_temporary_values51 = vec![snapshots.to_vec(), callbacks.to_vec()];
                                                            {
                                                                let source_ml_source1_temporary_index51 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let facts_4 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop68(kinds, owners, functions, parameter_starts, parameter_counts, parameters, width, span, facts_3, &(source_ml_source1_temporary_values51), source_ml_source1_temporary_index51);
                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin67::MlSource1TemporaryProveDisjointCollectionIterationsJoin67Next(facts_4, seeded_2)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                match source_ml_source2_s69 {
                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsJoin67::MlSource1TemporaryProveDisjointCollectionIterationsJoin67Next(facts_5, seeded_3) => {
                                                        {
                                                            let settled_2 = (facts_5.len() as f64) == before;
                                                            {
                                                                let pass_2 = pass + 1f64;
                                                                {
                                                                    (facts, work, settled, seeded, pass) = (facts_5, work_2, settled_2, seeded_3, pass_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let pass_3 = pass + 1f64;
                                                {
                                                    (facts, work, pass) = (facts_3, work_2, pass_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop65Return(source_ml_source1_temporary_result70) => {
                                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(source_ml_source1_temporary_result70)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(facts, settled)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done,
    MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56(
    functions: &[f64],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    bodies: &[f64],
    mut size: f64,
    mut at: f64,
) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result {
    loop {
        return if at < (functions.len() as f64) {
            if (((((((((!(self::ml_math::is_integer(self::read_required_number_fact(
                functions, at,
            ))) || !(self::ml_math::is_integer(
                self::read_required_number_fact(parameter_starts, at),
            ))) || !(self::ml_math::is_integer(self::read_required_number_fact(
                parameter_counts,
                at,
            )))) || !(self::ml_math::is_integer(self::read_required_number_fact(
                bodies, at,
            )))) || ((self::read_required_number_fact(functions, at)) < 0f64))
                || ((self::read_required_number_fact(functions, at)) >= size))
                || ((self::read_required_number_fact(parameter_starts, at)) < 0f64))
                || ((self::read_required_number_fact(parameter_counts, at)) < 0f64))
                || ((self::read_required_number_fact(parameter_starts, at)
                    + self::read_required_number_fact(parameter_counts, at))
                    > (parameters.len() as f64)))
                || ((self::read_required_number_fact(bodies, at)) < -1f64))
                || ((self::read_required_number_fact(bodies, at)) >= size)
            {
                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(vec![1f64])
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
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result {
    MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done,
    MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55(
    owners: &[f64],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut size: f64,
    mut node: f64,
) -> self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result {
    loop {
        return if node < size {
            if ((((((!(self::ml_math::is_integer(self::read_required_number_fact(
                starts, node,
            ))) || !(self::ml_math::is_integer(self::read_required_number_fact(
                counts, node,
            )))) || !(self::ml_math::is_integer(self::read_required_number_fact(
                owners, node,
            )))) || ((self::read_required_number_fact(starts, node)) < 0f64))
                || ((self::read_required_number_fact(counts, node)) < 0f64))
                || ((self::read_required_number_fact(starts, node)
                    + self::read_required_number_fact(counts, node))
                    > (targets.len() as f64)))
                || ((self::read_required_number_fact(owners, node)) < -1f64))
                || ((self::read_required_number_fact(owners, node)) >= size)
            {
                self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(vec![1f64])
            } else {
                {
                    let node_2 = node + 1f64;
                    {
                        node = node_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporarySelectedEdgeLoop80Result {
    MlSource1TemporarySelectedEdgeLoop80Done,
    MlSource1TemporarySelectedEdgeLoop80Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_selected_edge_loop80(
    state: &[Vec<f64>],
    mut owner: f64,
    mut label: f64,
    mut at: f64,
) -> self::MlSource1TemporarySelectedEdgeLoop80Result {
    loop {
        return if at < ((self::row(state, 4f64)).len() as f64) {
            if ((self::read_required_number_fact(&(self::row(state, 4f64)), at)) == owner)
                && ((self::read_required_number_fact(&(self::row(state, 5f64)), at)) == label)
            {
                self::MlSource1TemporarySelectedEdgeLoop80Result::MlSource1TemporarySelectedEdgeLoop80Return(self::read_required_number_fact(&(self::row(state, 7f64)), at))
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
            self::MlSource1TemporarySelectedEdgeLoop80Result::MlSource1TemporarySelectedEdgeLoop80Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporarySymbolLoop81Result {
    MlSource1TemporarySymbolLoop81Done,
    MlSource1TemporarySymbolLoop81Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_symbol_loop81(
    state: &[Vec<f64>],
    mut name: f64,
    texts: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlSource1TemporarySymbolLoop81Result {
    loop {
        return if at >= 0f64 {
            if (self::text(
                texts,
                self::read_required_number_fact(&(self::row(state, 13f64)), at),
            )) == (self::text(texts, name))
            {
                self::MlSource1TemporarySymbolLoop81Result::MlSource1TemporarySymbolLoop81Return(
                    self::read_required_number_fact(&(self::row(state, 14f64)), at),
                )
            } else {
                {
                    let at_2 = at - 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporarySymbolLoop81Result::MlSource1TemporarySymbolLoop81Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_expression_loop84(
    names: &[f64],
    bindings: &[f64],
    texts: &[self::ml_string::Text],
    mut name: f64,
    mut binding: f64,
    mut at: f64,
) -> f64 {
    loop {
        return if at >= 0f64 {
            if (self::text(texts, self::read_required_number_fact(names, at)))
                == (self::text(texts, name))
            {
                { self::read_required_number_fact(bindings, at) }
            } else {
                {
                    let at_2 = at - 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            binding
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExpressionLoop85Result {
    MlSource1TemporaryExpressionLoop85Done(Vec<Vec<f64>>, Vec<f64>, Vec<f64>),
}
