// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptsEffectsLoop39Result;
use super::MlSource1TemporaryAcceptsEffectsLoop40Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result;
use super::MlSource1TemporarySelectedEdgeLoop80Result;
use super::MlSource1TemporarySymbolLoop81Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_collection_construction_field;
use super::read_collection_construction_kind;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_roots;
use super::source_ml_source1_temporary_accepts_effects_loop39;
use super::source_ml_source1_temporary_accepts_effects_loop40;
use super::source_ml_source1_temporary_bind_calls_loop44;
use super::source_ml_source1_temporary_copy_fields_loop31;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59;
use super::source_ml_source1_temporary_reachable_loop35;
use super::source_ml_source1_temporary_selected_edge_loop80;
use super::source_ml_source1_temporary_symbol_loop81;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn copy_fields(
    facts: &[f64],
    roots: &[f64],
    destination: f64,
    field_width: f64,
    width: f64,
    span: f64,
) -> Vec<f64> {
    {
        let result = facts.to_vec();
        {
            let source_ml_source1_temporary_values29 = roots.to_vec();
            {
                let source_ml_source1_temporary_index29 = self::ml::Big::from_i128(0);
                {
                    self::source_ml_source1_temporary_copy_fields_loop31(
                        facts,
                        destination,
                        field_width,
                        width,
                        span,
                        result,
                        &(source_ml_source1_temporary_values29),
                        source_ml_source1_temporary_index29,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn called(facts: &[f64], source: f64, width: f64, span: f64) -> Vec<f64> {
    self::read_roots(facts, 3f64, source, width, span)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn reachable(facts: &[f64], initial: &[f64], width: f64, span: f64) -> Vec<f64> {
    {
        let result = initial.to_vec();
        {
            let progress = true;
            {
                self::source_ml_source1_temporary_reachable_loop35(
                    facts, width, span, result, progress,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn separate_roots(left: &[f64], right: Vec<f64>) -> bool {
    (((((left.len() as f64) > 0f64) && ((right.len() as f64) > 0f64))
        && !(self::ml_array::includes(left, &-1f64, 0f64, false, |a: &f64, b: &f64| {
            a == b || (a.is_nan() && b.is_nan())
        })))
        && !(self::ml_array::includes(&right, &-1f64, 0f64, false, |a: &f64, b: &f64| {
            a == b || (a.is_nan() && b.is_nan())
        })))
        && !({
            let ml_values1 = left.to_vec();
            let ml_callback1 = {
                let right = right.clone();
                std::rc::Rc::new(move |root: f64| -> bool {
                    self::ml_array::includes(&right, &root, 0f64, false, |a: &f64, b: &f64| {
                        a == b || (a.is_nan() && b.is_nan())
                    })
                })
            };
            ml_values1
                .iter()
                .cloned()
                .enumerate()
                .any(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
        })
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accepts_effects(
    facts: &[f64],
    roles: Vec<self::ml_string::Text>,
    operations: &[self::ml_string::Text],
    kinds: Vec<self::ml_string::Text>,
    owners: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    functions: &[f64],
    bodies: &[f64],
    scopes: &[f64],
    source: f64,
    allocation: bool,
    main_: f64,
    width: f64,
    span: f64,
) -> bool {
    {
        let roots = self::read_roots(facts, 1f64, source, width, span);
        if allocation
            && (((roots.len() as f64) == 0f64) || {
                let ml_values2 = roots.clone();
                let ml_callback2 = {
                    let roles = roles.clone();
                    let kinds = kinds.clone();
                    let source = source;
                    std::rc::Rc::new(move |root: f64| -> bool {
                        ((root < 0f64)
                            || ((self::text_fact(&(roles), root))
                                != (self::ml_string::Text::from_units(vec![
                                    97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                                ]))))
                            || ((self::text_fact(&(kinds), root))
                                != (self::text_fact(&(kinds), source)))
                    })
                };
                ml_values2
                    .iter()
                    .cloned()
                    .enumerate()
                    .any(|(ml_index2, ml_value2)| ml_callback2(ml_value2))
            })
        {
            false
        } else {
            {
                let reached = self::reachable(facts, scopes, width, span);
                {
                    let source_ml_source1_temporary_values38 = reached.clone();
                    {
                        let source_ml_source1_temporary_index38 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s23 =
                                self::source_ml_source1_temporary_accepts_effects_loop39(
                                    functions,
                                    bodies,
                                    main_,
                                    &(source_ml_source1_temporary_values38),
                                    source_ml_source1_temporary_index38,
                                );
                            match source_ml_source2_s23 {
                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Done => {
                                    {
                                        let node = 0f64;
                                        {
                                            let source_ml_source2_s24 = self::source_ml_source1_temporary_accepts_effects_loop40(facts, &(roles), operations, &(kinds), owners, starts, counts, labels, targets, functions.to_vec(), source, width, span, &(roots), &(reached), node);
                                            match source_ml_source2_s24 {
                                                self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Done => {
                                                    true
                                                }
                                                self::MlSource1TemporaryAcceptsEffectsLoop40Result::MlSource1TemporaryAcceptsEffectsLoop40Return(source_ml_source1_temporary_result40) => {
                                                    source_ml_source1_temporary_result40
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(source_ml_source1_temporary_result41) => {
                                    source_ml_source1_temporary_result41
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn bind_calls(
    facts: &[f64],
    owner: f64,
    callees: &[f64],
    arguments_: &[f64],
    callback: &self::ml_string::Text,
    receiver: f64,
    kinds: &[self::ml_string::Text],
    functions: &[f64],
    parameter_starts: &[f64],
    parameter_counts: &[f64],
    parameters: &[f64],
    bodies: &[f64],
    main_: f64,
    width: f64,
    span: f64,
) -> Vec<f64> {
    {
        let result = facts.to_vec();
        {
            let source_ml_source1_temporary_values42 = callees.to_vec();
            {
                let source_ml_source1_temporary_index42 = self::ml::Big::from_i128(0);
                {
                    self::source_ml_source1_temporary_bind_calls_loop44(
                        owner,
                        arguments_,
                        callback,
                        receiver,
                        kinds,
                        functions,
                        parameter_starts,
                        parameter_counts,
                        parameters,
                        bodies,
                        main_,
                        width,
                        span,
                        result,
                        &(source_ml_source1_temporary_values42),
                        source_ml_source1_temporary_index42,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn prove_disjoint_collection_iterations(
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
    main_: f64,
    snapshots: &[f64],
    callbacks: &[f64],
) -> Vec<f64> {
    {
        let size = roles.len() as f64;
        {
            let width = size + 1f64;
            {
                let field_width = (labels.len() as f64) + 1f64;
                {
                    let span = ((size * field_width) * width) + 1f64;
                    if ((((((((((((((size < 1f64) || (size > 4096f64))
                        || ((labels.len() as f64) > 32_768f64))
                        || ((operations.len() as f64) != size))
                        || ((kinds.len() as f64) != size))
                        || ((owners.len() as f64) != size))
                        || ((starts.len() as f64) != size))
                        || ((counts.len() as f64) != size))
                        || ((labels.len() as f64) != (targets.len() as f64)))
                        || ((functions.len() as f64) != (parameter_starts.len() as f64)))
                        || ((functions.len() as f64) != (parameter_counts.len() as f64)))
                        || ((functions.len() as f64) != (bodies.len() as f64)))
                        || !(self::ml_array::includes(
                            functions,
                            &main_,
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        )))
                        || (((snapshots.len() as f64) % 2f64) != 0f64))
                        || (((callbacks.len() as f64) % 2f64) != 0f64)
                    {
                        vec![1f64]
                    } else {
                        {
                            let node = 0f64;
                            {
                                let source_ml_source2_s25 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55(owners, starts, counts, targets, size, node);
                                match source_ml_source2_s25 {
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done => {
                                        if { let ml_values3 = targets.to_vec(); let ml_callback3 = { let size = size; std::rc::Rc::new(move |target: f64| -> bool { (!(self::ml_math::is_integer(target)) || (target < 0f64)) || (target >= size) }) }; ml_values3.iter().cloned().enumerate().any(|(ml_index3, ml_value3)| ml_callback3(ml_value3)) } || { let ml_values4 = parameters.to_vec(); let ml_callback4 = { let size = size; std::rc::Rc::new(move |target: f64| -> bool { (!(self::ml_math::is_integer(target)) || (target < 0f64)) || (target >= size) }) }; ml_values4.iter().cloned().enumerate().any(|(ml_index4, ml_value4)| ml_callback4(ml_value4)) } {
                                            vec![1f64]
                                        } else {
                                            {
                                                let at = 0f64;
                                                {
                                                    let source_ml_source2_s26 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56(functions, parameter_starts, parameter_counts, parameters, bodies, size, at);
                                                    match source_ml_source2_s26 {
                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done => {
                                                            if { let ml_values5 = snapshots.to_vec(); let ml_callback5 = { let size = size; std::rc::Rc::new(move |node_2: f64| -> bool { (!(self::ml_math::is_integer(node_2)) || (node_2 < 0f64)) || (node_2 >= size) }) }; ml_values5.iter().cloned().enumerate().any(|(ml_index5, ml_value5)| ml_callback5(ml_value5)) } || { let ml_values6 = callbacks.to_vec(); let ml_callback6 = { let size = size; std::rc::Rc::new(move |node_2: f64| -> bool { (!(self::ml_math::is_integer(node_2)) || (node_2 < 0f64)) || (node_2 >= size) }) }; ml_values6.iter().cloned().enumerate().any(|(ml_index6, ml_value6)| ml_callback6(ml_value6)) } {
                                                                vec![1f64]
                                                            } else {
                                                                {
                                                                    let facts = vec![((4f64 * span) + main_)];
                                                                    {
                                                                        let work = 0f64;
                                                                        {
                                                                            let settled = false;
                                                                            {
                                                                                let seeded = false;
                                                                                {
                                                                                    let pass = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s27 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57(roles, operations, kinds, owners, starts, counts, labels, targets, functions, parameter_starts, parameter_counts, parameters, bodies, main_, snapshots, callbacks, size, width, field_width, span, facts, work, settled, seeded, pass);
                                                                                        match source_ml_source2_s27 {
                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(facts_2, settled_2) => {
                                                                                                if settled_2 {
                                                                                                    {
                                                                                                        let accepted = Vec::<f64>::new();
                                                                                                        {
                                                                                                            let at_2 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_s28 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58(roles, operations, kinds, owners, starts, counts, labels, targets, functions, bodies, main_, snapshots, size, width, span, &(facts_2), accepted, at_2);
                                                                                                                match source_ml_source2_s28 {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Done(accepted_2) => {
                                                                                                                        {
                                                                                                                            let safe = Vec::<f64>::new();
                                                                                                                            {
                                                                                                                                let at_3 = 0f64;
                                                                                                                                {
                                                                                                                                    let safe_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop59(roles, operations, kinds, owners, starts, counts, labels, targets, functions, bodies, main_, callbacks, width, span, &(facts_2), safe, at_3);
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_concat_value29 = vec![0f64, (accepted_2.len() as f64)];
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_concat_value30 = accepted_2;
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_concat_value31 = vec![(safe_2.len() as f64)];
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_concat_value32 = safe_2;
                                                                                                                                                    self::ml_array::append(&(self::ml_array::append(&(self::ml_array::append(&(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value29)), source_ml_source2_concat_value30)), source_ml_source2_concat_value31)), source_ml_source2_concat_value32)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop58Return(source_ml_source1_temporary_result59) => {
                                                                                                                        source_ml_source1_temporary_result59
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    vec![1f64]
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(source_ml_source1_temporary_result64) => {
                                                                                                source_ml_source1_temporary_result64
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
                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(source_ml_source1_temporary_result79) => {
                                                            source_ml_source1_temporary_result79
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(source_ml_source1_temporary_result79_2) => {
                                        source_ml_source1_temporary_result79_2
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
pub fn first_number(values: &[f64]) -> f64 {
    if (values.len() as f64) == 0f64 {
        -1f64
    } else {
        self::read_required_number_fact(values, 0f64)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn text_fact(values: &[self::ml_string::Text], index: f64) -> self::ml_string::Text {
    if (index < 0f64) || (index >= (values.len() as f64)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::read_required_text_fact(values, index)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_copy_inference_role(
    target: &self::ml_string::Text,
    source: &self::ml_string::Text,
    settled: bool,
) -> self::ml_string::Text {
    if (((*target) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        || ((*target) == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
        && ((*source) == (*target))
    {
        self::ml_string::Text::from_units(vec![99, 111, 112, 121])
    } else if ((*source) == (self::ml_string::Text::from_units(vec![118, 97, 114]))) && !settled {
        self::ml_string::Text::from_units(vec![119, 97, 105, 116])
    } else {
        self::ml_string::Text::from_units(vec![
            108, 101, 103, 97, 99, 121, 32, 97, 114, 114, 97, 121,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_copy_source_normalization() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 70, 114, 111, 109])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_copy_snapshot_operation(
    kind_2: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
        self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])
    } else if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_collection_copy_array_from_snapshot(
    kind_2: &self::ml_string::Text,
    reference: bool,
) -> bool {
    reference
        && (((*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
            || ((*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_collection_copy_mapper_snapshot(
    kind_2: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> bool {
    {
        let actual = self::read_collection_copy_snapshot_operation(kind_2);
        ((actual != (self::ml_string::Text::from_units(vec![]))) && (actual == (*operation)))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_copy_mapper_error(snapshot: bool, readonly: bool) -> self::ml_string::Text {
    if snapshot && !readonly {
        self::ml_string::Text::from_units(vec![
            97, 32, 109, 117, 116, 97, 98, 108, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111,
            110, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 109, 97, 112, 112, 101, 114, 32,
            110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101,
            32, 114, 101, 97, 100, 111, 110, 108, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32,
            103, 114, 97, 112, 104, 59, 32, 102, 97, 99, 116, 111, 114, 105, 101, 115, 32, 97, 110,
            100, 32, 108, 105, 118, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 109, 117,
            116, 97, 116, 105, 111, 110, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114,
            32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32,
            105, 116, 101, 114, 97, 116, 111, 114, 32, 109, 111, 100, 101, 108,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_collection_copy_input(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    kind_2: &self::ml_string::Text,
    input: f64,
) -> Vec<f64> {
    if (((*kind_2) != (self::ml_string::Text::from_units(vec![109, 97, 112])))
        && ((*kind_2) != (self::ml_string::Text::from_units(vec![115, 101, 116]))))
        || ((self::read_collection_construction_kind(
            texts, starts, counts, labels, targets, input,
        )) != (*kind_2))
    {
        vec![0f64, -1f64, -1f64]
    } else {
        {
            let first = self::read_collection_construction_field(
                starts,
                counts,
                labels,
                targets,
                input,
                &(if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
                    self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
                } else {
                    self::ml_string::Text::from_units(vec![107, 101, 121])
                }),
            );
            {
                let second = if (*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112]))
                {
                    self::read_collection_construction_field(
                        starts,
                        counts,
                        labels,
                        targets,
                        input,
                        &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                    )
                } else {
                    -1f64
                };
                if (first < 0f64)
                    || (((*kind_2) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
                        && (second < 0f64))
                {
                    vec![0f64, -1f64, -1f64]
                } else {
                    vec![
                        if (*kind_2) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
                            1f64
                        } else {
                            2f64
                        },
                        first,
                        second,
                    ]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn row(state: &[Vec<f64>], column: f64) -> Vec<f64> {
    (self::ml_array::optional_at(state, self::ml_array::number_index(column)))
        .unwrap_or_else(|| Vec::<f64>::new())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace(state: &[Vec<f64>], column: f64, value: Vec<f64>) -> Vec<Vec<f64>> {
    {
        let ml_values7 = state.to_vec();
        let ml_callback7 = {
            let column = column;
            let value = value.clone();
            std::rc::Rc::new(move |previous: Vec<f64>, index: f64| -> Vec<f64> {
                if index == column {
                    value.clone()
                } else {
                    previous.clone()
                }
            })
        };
        ml_values7
            .iter()
            .cloned()
            .enumerate()
            .map(|(ml_index7, ml_value7)| ml_callback7(ml_value7, (ml_index7 as f64)))
            .collect::<Vec<_>>()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append(state: &[Vec<f64>], column: f64, value: f64) -> Vec<Vec<f64>> {
    self::replace(state, column, {
        let source_ml_source2_concat_value33 = self::row(state, column);
        {
            let source_ml_source2_concat_value34 = vec![value];
            self::ml_array::append(
                &(self::ml_array::append(&(Vec::<f64>::new()), source_ml_source2_concat_value33)),
                source_ml_source2_concat_value34,
            )
        }
    })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn last(state: &[Vec<f64>]) -> f64 {
    self::read_required_number_fact(&(self::row(state, 15f64)), 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish(state: &[Vec<f64>], node: f64) -> Vec<Vec<f64>> {
    self::replace(state, 15f64, vec![node])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn make(
    state: &[Vec<f64>],
    role: f64,
    operation: f64,
    kind_2: f64,
    owner: f64,
) -> Vec<Vec<f64>> {
    {
        let identity = (self::row(state, 0f64)).len() as f64;
        if identity >= 4096f64 {
            self::replace(state, 16f64, vec![1f64])
        } else {
            self::finish(
                &(self::append(
                    &(self::append(
                        &(self::append(&(self::append(state, 0f64, role)), 1f64, operation)),
                        2f64,
                        kind_2,
                    )),
                    3f64,
                    owner,
                )),
                identity,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn edge(
    state: &[Vec<f64>],
    owner: f64,
    label: f64,
    auxiliary: f64,
    target: f64,
) -> Vec<Vec<f64>> {
    self::append(
        &(self::append(
            &(self::append(&(self::append(state, 4f64, owner)), 5f64, label)),
            6f64,
            auxiliary,
        )),
        7f64,
        target,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn selected_edge(state: &[Vec<f64>], owner: f64, label: f64) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s35 =
                self::source_ml_source1_temporary_selected_edge_loop80(state, owner, label, at);
            match source_ml_source2_s35 {
                self::MlSource1TemporarySelectedEdgeLoop80Result::MlSource1TemporarySelectedEdgeLoop80Done => {
                    -1f64
                }
                self::MlSource1TemporarySelectedEdgeLoop80Result::MlSource1TemporarySelectedEdgeLoop80Return(source_ml_source1_temporary_result80) => {
                    source_ml_source1_temporary_result80
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn text(texts: &[self::ml_string::Text], identity: f64) -> self::ml_string::Text {
    if identity < 0f64 {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::read_required_text_fact(texts, identity)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn field(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    name: &self::ml_string::Text,
) -> f64 {
    self::read_collection_construction_field(starts, counts, labels, targets, node, name)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn word(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    name: &self::ml_string::Text,
) -> self::ml_string::Text {
    self::text(
        texts,
        self::field(starts, counts, labels, targets, node, name),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn kind(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
) -> f64 {
    self::field(
        starts,
        counts,
        labels,
        targets,
        self::field(
            starts,
            counts,
            labels,
            targets,
            node,
            &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
        ),
        &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn symbol(state: &[Vec<f64>], name: f64, texts: &[self::ml_string::Text]) -> f64 {
    {
        let at = ((self::row(state, 13f64)).len() as f64) - 1f64;
        {
            let source_ml_source2_s36 =
                self::source_ml_source1_temporary_symbol_loop81(state, name, texts, at);
            match source_ml_source2_s36 {
                self::MlSource1TemporarySymbolLoop81Result::MlSource1TemporarySymbolLoop81Done => {
                    -1f64
                }
                self::MlSource1TemporarySymbolLoop81Result::MlSource1TemporarySymbolLoop81Return(source_ml_source1_temporary_result81) => {
                    source_ml_source1_temporary_result81
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable(state: &[Vec<f64>], name: f64, texts: &[self::ml_string::Text]) -> Vec<Vec<f64>> {
    {
        let known = self::symbol(state, name, texts);
        if known >= 0f64 {
            self::finish(state, known)
        } else {
            {
                let state_2 = self::make(state, 1f64, -1f64, -1f64, -1f64);
                {
                    let root = self::last(&(state_2));
                    self::finish(
                        &(self::append(
                            &(self::append(&(self::append(&(state_2), 8f64, root)), 13f64, name)),
                            14f64,
                            root,
                        )),
                        root,
                    )
                }
            }
        }
    }
}
