// Generated from complete js/src/translation/disjoint-collection-iteration-rules.js. Do not edit by hand.
use super::add_fact;
use super::add_field;
use super::add_value;
use super::called;
use super::edges;
use super::first_number;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_disjoint_collection_iteration_callback;
use super::read_disjoint_collection_iteration_method_effect;
use super::read_required_number_fact;
use super::read_roots;
use super::separate_roots;
use super::text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_edges_loop19(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    label: &self::ml_string::Text,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            if (self::text_fact(labels, at)) == (*label) {
                {
                    let result_2 = {
                        let source_ml_source2_concat_value32 = result;
                        {
                            let source_ml_source2_concat_value33 =
                                vec![self::read_required_number_fact(targets, at)];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value32,
                                ),
                                source_ml_source2_concat_value33,
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
pub fn source_ml_source1_temporary_read_field_loop23(
    mut field: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut root: f64,
    source_ml_source1_temporary_values21: &[f64],
    mut source_ml_source1_temporary_index21: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index21
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values21.len() as u128))
        {
            {
                let fact = self::ml_array::at(
                    source_ml_source1_temporary_values21,
                    source_ml_source1_temporary_index21.to_index(),
                );
                if ((fact / span).floor()) == 2f64 {
                    {
                        let value = fact % span;
                        {
                            let owner = (value / (field_width * width)).floor();
                            {
                                let actual = (value / width).floor() % field_width;
                                if (owner == root) && ((actual == 0f64) || (actual == field)) {
                                    {
                                        let result_2 =
                                            self::add_value(&(result), ((value % width) - 1f64));
                                        {
                                            let source_ml_source1_temporary_index21_2 =
                                                source_ml_source1_temporary_index21
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (result, source_ml_source1_temporary_index21) = (
                                                    result_2,
                                                    source_ml_source1_temporary_index21_2,
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let source_ml_source1_temporary_index21_3 =
                                            source_ml_source1_temporary_index21
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index21 =
                                                source_ml_source1_temporary_index21_3;
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
                        let source_ml_source1_temporary_index21_4 =
                            source_ml_source1_temporary_index21.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index21 =
                                source_ml_source1_temporary_index21_4;
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_field_loop22(
    facts: &[f64],
    mut field: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values20: &[f64],
    mut source_ml_source1_temporary_index20: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index20
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values20.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values20,
                    source_ml_source1_temporary_index20.to_index(),
                );
                if root < 0f64 {
                    {
                        let result_2 = self::add_value(&(result), -1f64);
                        {
                            let source_ml_source1_temporary_index20_2 =
                                source_ml_source1_temporary_index20
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index20) =
                                    (result_2, source_ml_source1_temporary_index20_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_values21 = facts.to_vec();
                        {
                            let source_ml_source1_temporary_index21 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_source1_temporary_read_field_loop23(
                                    field,
                                    field_width,
                                    width,
                                    span,
                                    result,
                                    root,
                                    &(source_ml_source1_temporary_values21),
                                    source_ml_source1_temporary_index21,
                                );
                                {
                                    let source_ml_source1_temporary_index20_3 =
                                        source_ml_source1_temporary_index20
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index20) =
                                            (result_3, source_ml_source1_temporary_index20_3);
                                        continue;
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
pub fn source_ml_source1_temporary_add_field_loop27(
    mut field: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut owner: f64,
    source_ml_source1_temporary_values25: &[f64],
    mut source_ml_source1_temporary_index25: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index25
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values25.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values25,
                    source_ml_source1_temporary_index25.to_index(),
                );
                {
                    let result_2 = self::add_fact(
                        &(result),
                        ((((2f64 * span) + (((owner * field_width) + field) * width)) + root)
                            + 1f64),
                    );
                    {
                        let source_ml_source1_temporary_index25_2 =
                            source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index25) =
                                (result_2, source_ml_source1_temporary_index25_2);
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_add_field_loop26(
    mut field: f64,
    roots: &[f64],
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values24: &[f64],
    mut source_ml_source1_temporary_index24: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index24
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values24.len() as u128))
        {
            {
                let owner = self::ml_array::at(
                    source_ml_source1_temporary_values24,
                    source_ml_source1_temporary_index24.to_index(),
                );
                if owner >= 0f64 {
                    {
                        let source_ml_source1_temporary_values25 = roots.to_vec();
                        {
                            let source_ml_source1_temporary_index25 = self::ml::Big::from_i128(0);
                            {
                                let result_2 = self::source_ml_source1_temporary_add_field_loop27(
                                    field,
                                    field_width,
                                    width,
                                    span,
                                    result,
                                    owner,
                                    &(source_ml_source1_temporary_values25),
                                    source_ml_source1_temporary_index25,
                                );
                                {
                                    let source_ml_source1_temporary_index24_2 =
                                        source_ml_source1_temporary_index24
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index24) =
                                            (result_2, source_ml_source1_temporary_index24_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index24_3 =
                            source_ml_source1_temporary_index24.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index24 =
                                source_ml_source1_temporary_index24_3;
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_copy_fields_loop31(
    mut destination: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut root: f64,
    source_ml_source1_temporary_values29: &[f64],
    mut source_ml_source1_temporary_index29: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index29
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values29.len() as u128))
        {
            {
                let fact = self::ml_array::at(
                    source_ml_source1_temporary_values29,
                    source_ml_source1_temporary_index29.to_index(),
                );
                if ((fact / span).floor()) == 2f64 {
                    {
                        let value = fact % span;
                        if ((value / (field_width * width)).floor()) == root {
                            {
                                let result_2 = self::add_fact(
                                    &(result),
                                    (((2f64 * span) + ((destination * field_width) * width))
                                        + (value % (field_width * width))),
                                );
                                {
                                    let source_ml_source1_temporary_index29_2 =
                                        source_ml_source1_temporary_index29
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index29) =
                                            (result_2, source_ml_source1_temporary_index29_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let source_ml_source1_temporary_index29_3 =
                                    source_ml_source1_temporary_index29
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index29 =
                                        source_ml_source1_temporary_index29_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index29_4 =
                            source_ml_source1_temporary_index29.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index29 =
                                source_ml_source1_temporary_index29_4;
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_copy_fields_loop30(
    facts: &[f64],
    mut destination: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values28: &[f64],
    mut source_ml_source1_temporary_index28: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index28
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values28.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values28,
                    source_ml_source1_temporary_index28.to_index(),
                );
                if root < 0f64 {
                    {
                        let result_2 = self::add_field(
                            &(result),
                            &(vec![destination]),
                            0f64,
                            &(vec![-1f64]),
                            field_width,
                            width,
                            span,
                        );
                        {
                            let source_ml_source1_temporary_index28_2 =
                                source_ml_source1_temporary_index28
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index28) =
                                    (result_2, source_ml_source1_temporary_index28_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_values29 = facts.to_vec();
                        {
                            let source_ml_source1_temporary_index29 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_source1_temporary_copy_fields_loop31(
                                    destination,
                                    field_width,
                                    width,
                                    span,
                                    result,
                                    root,
                                    &(source_ml_source1_temporary_values29),
                                    source_ml_source1_temporary_index29,
                                );
                                {
                                    let source_ml_source1_temporary_index28_3 =
                                        source_ml_source1_temporary_index28
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index28) =
                                            (result_3, source_ml_source1_temporary_index28_3);
                                        continue;
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReachableLoop35Result {
    MlSource1TemporaryReachableLoop35Done(Vec<f64>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReachableLoop36Result {
    MlSource1TemporaryReachableLoop36Done(Vec<f64>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_reachable_loop36(
    mut result: Vec<f64>,
    mut progress: bool,
    source_ml_source1_temporary_values33: &[f64],
    mut source_ml_source1_temporary_index33: self::ml::Big,
) -> self::MlSource1TemporaryReachableLoop36Result {
    loop {
        if source_ml_source1_temporary_index33
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values33.len() as u128))
        {
            {
                let target = self::ml_array::at(
                    source_ml_source1_temporary_values33,
                    source_ml_source1_temporary_index33.to_index(),
                );
                if !(self::ml_array::includes(&result, &target, 0f64, false, |a: &f64, b: &f64| {
                    a == b || (a.is_nan() && b.is_nan())
                })) {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value34 = result;
                            {
                                let source_ml_source2_concat_value35 = vec![target];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value34,
                                    ),
                                    source_ml_source2_concat_value35,
                                )
                            }
                        };
                        {
                            let progress_2 = true;
                            {
                                let source_ml_source1_temporary_index33_2 =
                                    source_ml_source1_temporary_index33
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (result, progress, source_ml_source1_temporary_index33) = (
                                        result_2,
                                        progress_2,
                                        source_ml_source1_temporary_index33_2,
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index33_3 =
                            source_ml_source1_temporary_index33.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index33 =
                                source_ml_source1_temporary_index33_3;
                            continue;
                        }
                    }
                }
            }
        }
        return {
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
    source_ml_source1_temporary_values32: &[f64],
    mut source_ml_source1_temporary_index32: self::ml::Big,
) -> self::MlSource1TemporaryReachableLoop35Result {
    loop {
        return if source_ml_source1_temporary_index32
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values32.len() as u128))
        {
            {
                let owner = self::ml_array::at(
                    source_ml_source1_temporary_values32,
                    source_ml_source1_temporary_index32.to_index(),
                );
                {
                    let source_ml_source1_temporary_values33 =
                        self::called(facts, owner, width, span);
                    {
                        let source_ml_source1_temporary_index33 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s36 =
                                self::source_ml_source1_temporary_reachable_loop36(
                                    result,
                                    progress,
                                    &(source_ml_source1_temporary_values33),
                                    source_ml_source1_temporary_index33,
                                );
                            match source_ml_source2_s36 {
                                self::MlSource1TemporaryReachableLoop36Result::MlSource1TemporaryReachableLoop36Done(result_2, progress_2) => {
                                    {
                                        let source_ml_source1_temporary_index32_2 = source_ml_source1_temporary_index32.add(&self::ml::Big::from_i128(1));
                                        {
                                            (result, progress, source_ml_source1_temporary_index32) = (result_2, progress_2, source_ml_source1_temporary_index32_2);
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
            self::MlSource1TemporaryReachableLoop35Result::MlSource1TemporaryReachableLoop35Done(
                result, progress,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_reachable_loop34(
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
                    let source_ml_source1_temporary_values32 = result.clone();
                    {
                        let source_ml_source1_temporary_index32 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s37 =
                                self::source_ml_source1_temporary_reachable_loop35(
                                    facts,
                                    width,
                                    span,
                                    result,
                                    progress_2,
                                    &(source_ml_source1_temporary_values32),
                                    source_ml_source1_temporary_index32,
                                );
                            match source_ml_source2_s37 {
                                self::MlSource1TemporaryReachableLoop35Result::MlSource1TemporaryReachableLoop35Done(result_2, progress_3) => {
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
pub enum MlSource1TemporaryAcceptsEffectsLoop39Result {
    MlSource1TemporaryAcceptsEffectsLoop39Done,
    MlSource1TemporaryAcceptsEffectsLoop39Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptsEffectsJoin40 {
    MlSource1TemporaryAcceptsEffectsJoin40Next,
    MlSource1TemporaryAcceptsEffectsJoin40Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_accepts_effects_loop39(
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
) -> self::MlSource1TemporaryAcceptsEffectsLoop39Result {
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
                    self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                } else {
                    {
                        let source_ml_source2_s38 = if ((self::text_fact(roles, node))
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
                                            let ml_values7 = callees_4;
                                            let ml_callback7 = {
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
                                            ml_values7.iter().cloned().enumerate().any(
                                                |(ml_index7, ml_value7)| ml_callback7(ml_value7),
                                            )
                                        } {
                                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Return(false)
                                        } else {
                                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next
                        };
                        match source_ml_source2_s38 {
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Next => {
                                if (self::text_fact(roles, node)) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])) {
                                    {
                                        let object = self::first_number(&(self::edges(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])))));
                                        {
                                            let effect = self::read_disjoint_collection_iteration_method_effect(&(self::text_fact(kinds, object)), &(self::text_fact(operations, node)));
                                            if effect == (self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])) {
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                            } else if (effect == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![99, 101, 108, 108])], &self::text_fact(kinds, object), 0f64, false, |a, b| a == b)) {
                                                    self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                                } else if (effect == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && (((self::text_fact(kinds, object)) == (self::text_fact(kinds, source))) && !(self::separate_roots(roots, self::read_roots(facts, 1f64, object, width, span)))) {
                                                        self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
                                                    } else {
                                                        {
                                                            let arguments_ = self::edges(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116])));
                                                            if self::read_disjoint_collection_iteration_callback(&(self::text_fact(operations, node))) && ((arguments_.len() as f64) != 0f64) {
                                                                {
                                                                    let callees_5 = self::read_roots(facts, 1f64, self::read_required_number_fact(&(arguments_), 0f64), width, span);
                                                                    if ((callees_5.len() as f64) == 0f64) || { let ml_values8 = callees_5; let ml_callback8 = { let functions = functions.clone(); std::rc::Rc::new(move |target: f64| -> bool { !(self::ml_array::includes(&functions, &target, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values8.iter().cloned().enumerate().any(|(ml_index8, ml_value8)| ml_callback8(ml_value8)) } {
                                                                        self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(false)
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
                            self::MlSource1TemporaryAcceptsEffectsJoin40::MlSource1TemporaryAcceptsEffectsJoin40Return(source_ml_source1_temporary_result40) => {
                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(source_ml_source1_temporary_result40)
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
            self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Done
        };
    }
}
