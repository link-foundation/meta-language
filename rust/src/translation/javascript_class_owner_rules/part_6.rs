// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::accept_java_script_class_owner_observation;
use super::carries;
use super::contains_java_script_closed_class_brand;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::fresh;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::word;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_names_loop15(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut result: Vec<self::ml_string::Text>,
    mut fields: f64,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (self::dependency_function1(counts, fields)) {
            {
                let result_2 = {
                    let source_ml_source2_concat_value49 = result;
                    {
                        let source_ml_source2_concat_value50 = vec![self::word(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::dependency_function4(starts, counts, targets, fields, at),
                            &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source2_concat_value49,
                            ),
                            source_ml_source2_concat_value50,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCarriesLoop16Result {
    MlSource1TemporaryCarriesLoop16Done,
    MlSource1TemporaryCarriesLoop16Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_carries_loop16(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    owner: &self::ml_string::Text,
    mut depth: f64,
    mut elements: f64,
    mut at: f64,
) -> self::MlSource1TemporaryCarriesLoop16Result {
    loop {
        return if at < (self::dependency_function1(counts, elements)) {
            if self::carries(
                forms,
                texts,
                starts,
                counts,
                labels,
                targets,
                self::dependency_function4(starts, counts, targets, elements, at),
                owner,
                (depth + 1f64),
            ) {
                self::MlSource1TemporaryCarriesLoop16Result::MlSource1TemporaryCarriesLoop16Return(
                    true,
                )
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
            self::MlSource1TemporaryCarriesLoop16Result::MlSource1TemporaryCarriesLoop16Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFreshLoop17Result {
    MlSource1TemporaryFreshLoop17Done,
    MlSource1TemporaryFreshLoop17Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_fresh_loop17(
    bindings: &[self::ml_string::Text],
    values: &[f64],
    name: &self::ml_string::Text,
    mut at: f64,
) -> self::MlSource1TemporaryFreshLoop17Result {
    loop {
        return if at >= 0f64 {
            if (self::dependency_function2(bindings, at)) == (*name) {
                self::MlSource1TemporaryFreshLoop17Result::MlSource1TemporaryFreshLoop17Return(
                    self::dependency_function1(values, at),
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
            self::MlSource1TemporaryFreshLoop17Result::MlSource1TemporaryFreshLoop17Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFreshLoop18Result {
    MlSource1TemporaryFreshLoop18Done(f64),
    MlSource1TemporaryFreshLoop18Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_fresh_loop18(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    owner: &self::ml_string::Text,
    bindings: &[self::ml_string::Text],
    values: &[f64],
    mut depth: f64,
    mut origin: f64,
    mut items: f64,
    mut at: f64,
) -> self::MlSource1TemporaryFreshLoop18Result {
    loop {
        return if at < (self::dependency_function1(counts, items)) {
            {
                let value = self::fresh(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::dependency_function4(starts, counts, targets, items, at),
                    owner,
                    bindings,
                    values,
                    (depth + 1f64),
                );
                if (value == -1f64) || (((value >= 0f64) && (origin >= 0f64)) && (value != origin))
                {
                    self::MlSource1TemporaryFreshLoop18Result::MlSource1TemporaryFreshLoop18Return(
                        -1f64,
                    )
                } else {
                    if value >= 0f64 {
                        {
                            let origin_2 = value;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (origin, at) = (origin_2, at_2);
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
        } else {
            self::MlSource1TemporaryFreshLoop18Result::MlSource1TemporaryFreshLoop18Done(origin)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop28(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut operand: f64,
    mut values: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (self::dependency_function1(counts, operand)) {
            {
                let values_2 = {
                    let source_ml_source2_concat_value51 = values;
                    {
                        let source_ml_source2_concat_value52 = vec![self::dependency_function4(
                            starts, counts, targets, operand, at,
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value51,
                            ),
                            source_ml_source2_concat_value52,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (values, at) = (values_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { values };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop32(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut pending: Vec<f64>,
    mut actual: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::dependency_function1(starts, actual)
                + self::dependency_function1(counts, actual))
        {
            if !(self::ml_array::includes(
                &vec![
                    self::ml_string::Text::from_units(vec![115, 112, 97, 110]),
                    self::ml_string::Text::from_units(vec![
                        115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114,
                    ]),
                ],
                &self::dependency_function2(labels, at),
                0f64,
                false,
                |a, b| a == b,
            )) {
                {
                    let pending_2 = {
                        let source_ml_source2_concat_value53 = pending;
                        {
                            let source_ml_source2_concat_value54 =
                                vec![self::dependency_function1(targets, at)];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value53,
                                ),
                                source_ml_source2_concat_value54,
                            )
                        }
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (pending, at) = (pending_2, at_2);
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
        return { pending };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop31(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    constructors: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    mut identity: bool,
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result {
    loop {
        return {
            let ml_library_condition_2 = {
                let ml_value = pending.len() as f64;
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_2 {
                {
                    let actual =
                        self::dependency_function1(&(pending), ((pending.len() as f64) - 1f64));
                    {
                        let pending_2 = self::ml_array::slice(&pending, 0f64, -1f64);
                        if (actual < 0f64)
                            || self::ml_array::includes(
                                &seen,
                                &actual,
                                0f64,
                                false,
                                |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                            )
                        {
                            {
                                pending = pending_2;
                                continue;
                            }
                        }
                        {
                            if (seen.len() as f64) > 100_000f64 {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let seen_2 = {
                                        let source_ml_source2_concat_value55 = seen;
                                        {
                                            let source_ml_source2_concat_value56 = vec![actual];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value55,
                                                ),
                                                source_ml_source2_concat_value56,
                                            )
                                        }
                                    };
                                    {
                                        let owner = self::word(
                                            forms,
                                            texts,
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            actual,
                                            &(self::ml_string::Text::from_units(vec![
                                                115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115,
                                                79, 119, 110, 101, 114,
                                            ])),
                                        );
                                        {
                                            let position = self::ml_array::index_of(
                                                constructors,
                                                &owner,
                                                0f64,
                                                false,
                                                |a, b| a == b,
                                            );
                                            if (position >= 0f64) && !(self::accept_java_script_class_owner_observation(&(self::dependency_function2(representations, position)), &(if identity {
                                            self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
                                        } else {
                                            self::ml_string::Text::from_units(vec![115, 116, 114, 117, 99, 116, 117, 114, 97, 108])
                                        }))) {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 97, 100, 109, 105, 116, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 100, 101, 98, 117, 103, 32, 111, 114, 32, 112, 114, 111, 111, 102, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110]))
                                        } else {
                                            {
                                                let at = self::dependency_function1(starts, actual);
                                                {
                                                    let pending_3 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop32(starts, counts, labels, targets, pending_2, actual, at);
                                                    {
                                                        (pending, seen) = (pending_3, seen_2);
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
            } else {
                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop30(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    mut identity: bool,
    mut type_: f64,
    mut owner: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result {
    loop {
        return if owner < (constructors.len() as f64) {
            if !(self::accept_java_script_class_owner_observation(
                &(self::dependency_function2(representations, owner)),
                &(if identity {
                    self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
                } else {
                    self::ml_string::Text::from_units(vec![
                        115, 116, 114, 117, 99, 116, 117, 114, 97, 108,
                    ])
                }),
            )) && self::contains_java_script_closed_class_brand(
                texts,
                starts,
                counts,
                labels,
                targets,
                type_,
                declarations,
                &(vec![self::dependency_function2(brands, owner)]),
            ) {
                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 97, 100, 109, 105, 116, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 100, 101, 98, 117, 103, 32, 111, 114, 32, 112, 114, 111, 111, 102, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110]))
            } else {
                {
                    let owner_2 = owner + 1f64;
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop29(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    mut identity: bool,
    source_ml_source1_temporary_values21: &[f64],
    mut source_ml_source1_temporary_index21: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result {
    loop {
        return if source_ml_source1_temporary_index21
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values21.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_source1_temporary_values21,
                    source_ml_source1_temporary_index21.to_index(),
                );
                {
                    let type_ = self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        value,
                        &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                    );
                    if type_ < 0f64 {
                        {
                            let source_ml_source1_temporary_index21_2 =
                                source_ml_source1_temporary_index21
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index21 =
                                    source_ml_source1_temporary_index21_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let owner = 0f64;
                            {
                                let source_ml_source2_s57 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop30(texts, starts, counts, labels, targets, declarations, constructors, brands, representations, identity, type_, owner);
                                match source_ml_source2_s57 {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done => {
                                        {
                                            let pending = vec![type_];
                                            {
                                                let seen = Vec::<f64>::new();
                                                {
                                                    let source_ml_source2_s58 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop31(forms, texts, starts, counts, labels, targets, constructors, representations, identity, pending, seen);
                                                    match source_ml_source2_s58 {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done => {
                                                            {
                                                                let source_ml_source1_temporary_index21_3 = source_ml_source1_temporary_index21.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index21 = source_ml_source1_temporary_index21_3;
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(source_ml_source1_temporary_result31) => {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Return(source_ml_source1_temporary_result31)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(source_ml_source1_temporary_result32) => {
                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Return(source_ml_source1_temporary_result32)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop27(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    mut node: f64,
    mut identity: bool,
    source_ml_source1_temporary_values20: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index20: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Result {
    loop {
        return if source_ml_source1_temporary_index20
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values20.len() as u128))
        {
            {
                let key = self::ml_array::at(
                    source_ml_source1_temporary_values20,
                    source_ml_source1_temporary_index20.to_index(),
                );
                if key.0.is_empty() {
                    {
                        let source_ml_source1_temporary_index20_4 =
                            source_ml_source1_temporary_index20.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index20 =
                                source_ml_source1_temporary_index20_4;
                            continue;
                        }
                    }
                }
                {
                    {
                        let operand = self::dependency_function3(
                            starts,
                            counts,
                            labels,
                            targets,
                            node,
                            &(key),
                        );
                        if operand < 0f64 {
                            {
                                let source_ml_source1_temporary_index20_2 =
                                    source_ml_source1_temporary_index20
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index20 =
                                        source_ml_source1_temporary_index20_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let values = if (self::dependency_function2(forms, operand))
                                    == (self::ml_string::Text::from_units(vec![
                                        97, 114, 114, 97, 121,
                                    ])) {
                                    Vec::<f64>::new()
                                } else {
                                    vec![operand]
                                };
                                {
                                    let values_3 = if (self::dependency_function2(forms, operand))
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121,
                                        ])) {
                                        {
                                            let at = 0f64;
                                            {
                                                self::source_ml_source1_temporary_read_java_script_class_owner_error_loop28(starts, counts, targets, operand, values, at)
                                            }
                                        }
                                    } else {
                                        values
                                    };
                                    {
                                        let source_ml_source1_temporary_values21 = values_3;
                                        {
                                            let source_ml_source1_temporary_index21 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s59 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop29(forms, texts, starts, counts, labels, targets, declarations, constructors, brands, representations, identity, &(source_ml_source1_temporary_values21), source_ml_source1_temporary_index21);
                                                match source_ml_source2_s59 {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done => {
                                                        {
                                                            let source_ml_source1_temporary_index20_3 = source_ml_source1_temporary_index20.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                source_ml_source1_temporary_index20 = source_ml_source1_temporary_index20_3;
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Return(source_ml_source1_temporary_result29) => {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Return(source_ml_source1_temporary_result29)
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Done
        };
    }
}
