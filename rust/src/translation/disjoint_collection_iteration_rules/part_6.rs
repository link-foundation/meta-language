// Generated from complete js/src/translation/disjoint-collection-iteration-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptsEffectsLoop38Result;
use super::MlSource1TemporaryAcceptsEffectsLoop39Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result;
use super::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result;
use super::add_fact;
use super::add_value;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_roots;
use super::render_array_callback_argument;
use super::source_ml_source1_temporary_accepts_effects_loop38;
use super::source_ml_source1_temporary_accepts_effects_loop39;
use super::source_ml_source1_temporary_bind_calls_loop43;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop54;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57;
use super::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58;
use super::source_ml_source1_temporary_reachable_loop34;

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
                self::source_ml_source1_temporary_reachable_loop34(
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
                    let source_ml_source1_temporary_values37 = reached.clone();
                    {
                        let source_ml_source1_temporary_index37 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s22 =
                                self::source_ml_source1_temporary_accepts_effects_loop38(
                                    functions,
                                    bodies,
                                    main_,
                                    &(source_ml_source1_temporary_values37),
                                    source_ml_source1_temporary_index37,
                                );
                            match source_ml_source2_s22 {
                                self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Done => {
                                    {
                                        let node = 0f64;
                                        {
                                            let source_ml_source2_s23 = self::source_ml_source1_temporary_accepts_effects_loop39(facts, &(roles), operations, &(kinds), owners, starts, counts, labels, targets, functions.to_vec(), source, width, span, &(roots), &(reached), node);
                                            match source_ml_source2_s23 {
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Done => {
                                                    true
                                                }
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(source_ml_source1_temporary_result39) => {
                                                    source_ml_source1_temporary_result39
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Return(source_ml_source1_temporary_result40) => {
                                    source_ml_source1_temporary_result40
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
            let source_ml_source1_temporary_values41 = callees.to_vec();
            {
                let source_ml_source1_temporary_index41 = self::ml::Big::from_i128(0);
                {
                    self::source_ml_source1_temporary_bind_calls_loop43(
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
                        &(source_ml_source1_temporary_values41),
                        source_ml_source1_temporary_index41,
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
                                let source_ml_source2_s24 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop54(owners, starts, counts, targets, size, node);
                                match source_ml_source2_s24 {
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Done => {
                                        if { let ml_values3 = targets.to_vec(); let ml_callback3 = { let size = size; std::rc::Rc::new(move |target: f64| -> bool { (!(self::ml_math::is_integer(target)) || (target < 0f64)) || (target >= size) }) }; ml_values3.iter().cloned().enumerate().any(|(ml_index3, ml_value3)| ml_callback3(ml_value3)) } || { let ml_values4 = parameters.to_vec(); let ml_callback4 = { let size = size; std::rc::Rc::new(move |target: f64| -> bool { (!(self::ml_math::is_integer(target)) || (target < 0f64)) || (target >= size) }) }; ml_values4.iter().cloned().enumerate().any(|(ml_index4, ml_value4)| ml_callback4(ml_value4)) } {
                                            vec![1f64]
                                        } else {
                                            {
                                                let at = 0f64;
                                                {
                                                    let source_ml_source2_s25 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55(functions, parameter_starts, parameter_counts, parameters, bodies, size, at);
                                                    match source_ml_source2_s25 {
                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done => {
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
                                                                                        let source_ml_source2_s26 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56(roles, operations, kinds, owners, starts, counts, labels, targets, functions, parameter_starts, parameter_counts, parameters, bodies, main_, snapshots, callbacks, size, width, field_width, span, facts, work, settled, seeded, pass);
                                                                                        match source_ml_source2_s26 {
                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done(facts_2, settled_2) => {
                                                                                                if settled_2 {
                                                                                                    {
                                                                                                        let accepted = Vec::<f64>::new();
                                                                                                        {
                                                                                                            let at_2 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_s27 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57(roles, operations, kinds, owners, starts, counts, labels, targets, functions, bodies, main_, snapshots, size, width, span, &(facts_2), accepted, at_2);
                                                                                                                match source_ml_source2_s27 {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(accepted_2) => {
                                                                                                                        {
                                                                                                                            let safe = Vec::<f64>::new();
                                                                                                                            {
                                                                                                                                let at_3 = 0f64;
                                                                                                                                {
                                                                                                                                    let safe_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58(roles, operations, kinds, owners, starts, counts, labels, targets, functions, bodies, main_, callbacks, width, span, &(facts_2), safe, at_3);
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_concat_value28 = vec![0f64, (accepted_2.len() as f64)];
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_concat_value29 = accepted_2;
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_concat_value30 = vec![(safe_2.len() as f64)];
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_concat_value31 = safe_2;
                                                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value28), source_ml_source2_concat_value29), source_ml_source2_concat_value30), source_ml_source2_concat_value31)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(source_ml_source1_temporary_result58) => {
                                                                                                                        source_ml_source1_temporary_result58
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    vec![1f64]
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(source_ml_source1_temporary_result63) => {
                                                                                                source_ml_source1_temporary_result63
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
                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(source_ml_source1_temporary_result78) => {
                                                            source_ml_source1_temporary_result78
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Return(source_ml_source1_temporary_result78_2) => {
                                        source_ml_source1_temporary_result78_2
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodResultJoin2 {
    MlSource1TemporaryReadReferenceMethodResultJoin2Next,
    MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodResultJoin1 {
    MlSource1TemporaryReadReferenceMethodResultJoin1Next,
    MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin5 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin5Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin4 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin4Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin3 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin3Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_reference_method_expression_loop6(
    arguments_: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
    mut prefix: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let prefix_2 = prefix.concat(
                    &((((((self::ml_string::Text::from_units(vec![
                        108, 101, 116, 32, 109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114,
                        103, 117, 109, 101, 110, 116,
                    ]))
                    .concat(suffix))
                    .concat(&self::ml_string::Text::from_units(vec![95])))
                    .concat(&self::ml_string::Text::from_text(
                        &self::ml_number::js_number(index),
                    )))
                    .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                    .concat(
                        &(self::ml_array::optional_at(
                            arguments_,
                            self::ml_array::number_index(index),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    ))
                    .concat(&self::ml_string::Text::from_units(vec![59, 32])),
                );
                {
                    let index_2 = index + 1f64;
                    {
                        (prefix, index) = (prefix_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { prefix.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_reference_method_expression_loop7(
    arguments_: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
    mut names: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let names_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (((self::ml_string::Text::from_units(vec![
                                109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117,
                                109, 101, 110, 116,
                            ]))
                            .concat(suffix))
                            .concat(&self::ml_string::Text::from_units(vec![95])))
                            .concat(&self::ml_string::Text::from_text(
                                &self::ml_number::js_number(index),
                            )),
                        ],
                        names,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (names, index) = (names_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_callback_call_loop8(
    operation: &self::ml_string::Text,
    mut parameter_count: f64,
    suffix: &self::ml_string::Text,
    mut clone_element: bool,
    mut arguments_text: self::ml_string::Text,
    mut position: f64,
    mut maximum: f64,
) -> self::ml_string::Text {
    loop {
        if (position < parameter_count) && (position < maximum) {
            {
                let arguments_text_3 = if position > 0f64 {
                    { arguments_text.concat(&self::ml_string::Text::from_units(vec![44, 32])) }
                } else {
                    arguments_text
                };
                {
                    let arguments_text_4 =
                        arguments_text_3.concat(&self::render_array_callback_argument(
                            operation,
                            position,
                            suffix,
                            clone_element,
                        ));
                    {
                        let position_2 = position + 1f64;
                        {
                            (arguments_text, position) = (arguments_text_4, position_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { arguments_text };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_state_method_suffix_loop9(
    text: &self::ml_string::Text,
    mut suffix: f64,
) -> f64 {
    loop {
        if text.contains(
            &(self::ml_string::Text::from_units(vec![
                109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114,
            ]))
            .concat(&self::ml_string::Text::from_text(
                &self::ml_number::js_number(suffix),
            )),
        ) || text.contains(
            &((self::ml_string::Text::from_units(vec![
                109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116,
            ]))
            .concat(&self::ml_string::Text::from_text(
                &self::ml_number::js_number(suffix),
            )))
            .concat(&self::ml_string::Text::from_units(vec![95])),
        ) {
            {
                let suffix_2 = suffix + 1f64;
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        }
        return { suffix };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_state_method_expression_loop10(
    arguments_: &[self::ml_string::Text],
    types: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
    mut output: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let output_2 = output.concat(
                    &((((((((self::ml_string::Text::from_units(vec![
                        108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103,
                        117, 109, 101, 110, 116,
                    ]))
                    .concat(suffix))
                    .concat(&self::ml_string::Text::from_units(vec![95])))
                    .concat(&self::ml_string::Text::from_text(
                        &self::ml_number::js_number(index),
                    )))
                    .concat(&self::ml_string::Text::from_units(vec![58, 32])))
                    .concat(
                        &(self::ml_array::optional_at(types, self::ml_array::number_index(index)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    ))
                    .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                    .concat(
                        &(self::ml_array::optional_at(
                            arguments_,
                            self::ml_array::number_index(index),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    ))
                    .concat(&self::ml_string::Text::from_units(vec![59, 32])),
                );
                {
                    let index_2 = index + 1f64;
                    {
                        (output, index) = (output_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { output.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_state_method_expression_loop11(
    arguments_: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
    mut names: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let names_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (((self::ml_string::Text::from_units(vec![
                                109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109,
                                101, 110, 116,
                            ]))
                            .concat(suffix))
                            .concat(&self::ml_string::Text::from_units(vec![95])))
                            .concat(&self::ml_string::Text::from_text(
                                &self::ml_number::js_number(index),
                            )),
                        ],
                        names,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (names, index) = (names_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_state_method_expression_loop12(
    arguments_: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    mut inserted: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let inserted_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(
                                names,
                                self::ml_array::number_index(index),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                        ],
                        inserted,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (inserted, index) = (inserted_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { inserted };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_roots_loop14(
    mut tag: f64,
    mut node: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values13: &[f64],
    mut source_ml_source1_temporary_index13: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index13
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values13.len() as u128))
        {
            {
                let fact = self::ml_array::at(
                    source_ml_source1_temporary_values13,
                    source_ml_source1_temporary_index13.to_index(),
                );
                if ((fact / span).floor()) == tag {
                    {
                        let value = fact % span;
                        if ((value / width).floor()) == node {
                            {
                                let result_2 = self::add_value(&(result), ((value % width) - 1f64));
                                {
                                    let source_ml_source1_temporary_index13_2 =
                                        source_ml_source1_temporary_index13
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index13) =
                                            (result_2, source_ml_source1_temporary_index13_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let source_ml_source1_temporary_index13_3 =
                                    source_ml_source1_temporary_index13
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index13 =
                                        source_ml_source1_temporary_index13_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index13_4 =
                            source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index13 =
                                source_ml_source1_temporary_index13_4;
                            continue;
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
pub fn source_ml_source1_temporary_add_roots_loop17(
    mut present: Vec<f64>,
    source_ml_source1_temporary_values15: &[f64],
    mut source_ml_source1_temporary_index15: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index15
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values15.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values15,
                    source_ml_source1_temporary_index15.to_index(),
                );
                {
                    let present_2 = self::add_value(&(present), root);
                    {
                        let source_ml_source1_temporary_index15_2 =
                            source_ml_source1_temporary_index15.add(&self::ml::Big::from_i128(1));
                        {
                            (present, source_ml_source1_temporary_index15) =
                                (present_2, source_ml_source1_temporary_index15_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { present };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_add_roots_loop18(
    mut tag: f64,
    mut node: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values16: &[f64],
    mut source_ml_source1_temporary_index16: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index16
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values16.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values16,
                    source_ml_source1_temporary_index16.to_index(),
                );
                {
                    let result_2 = self::add_fact(
                        &(result),
                        ((((tag * span) + (node * width)) + root) + 1f64),
                    );
                    {
                        let source_ml_source1_temporary_index16_2 =
                            source_ml_source1_temporary_index16.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index16) =
                                (result_2, source_ml_source1_temporary_index16_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}
