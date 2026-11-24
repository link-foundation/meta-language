// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result;
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
use super::ml_number;
use super::ml_string;
use super::word;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_record_field_write_operator_loop13(
    operation: &self::ml_string::Text,
    source_ml_source1_temporary_values12: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index12: self::ml::Big,
) -> self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result {
    loop {
        return if source_ml_source1_temporary_index12
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values12.len() as u128))
        {
            {
                let operator = self::ml_array::at(
                    source_ml_source1_temporary_values12,
                    source_ml_source1_temporary_index12.to_index(),
                );
                if operation.starts_with(
                    &((self::ml_string::Text::from_units(vec![
                        119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
                    ]))
                    .concat(&operator))
                    .concat(&self::ml_string::Text::from_units(vec![58])),
                ) {
                    self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(operator.clone())
                } else {
                    {
                        let source_ml_source1_temporary_index12_2 =
                            source_ml_source1_temporary_index12.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index12 =
                                source_ml_source1_temporary_index12_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_record_field_write_suffix_loop14(
    text: &self::ml_string::Text,
    mut suffix: f64,
) -> f64 {
    loop {
        if (text.contains(
            &(self::ml_string::Text::from_units(vec![
                109, 108, 95, 102, 105, 101, 108, 100, 95, 111, 119, 110, 101, 114,
            ]))
            .concat(&self::ml_string::Text::from_text(
                &self::ml_number::js_number(suffix),
            )),
        ) || text.contains(
            &(self::ml_string::Text::from_units(vec![
                109, 108, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101,
            ]))
            .concat(&self::ml_string::Text::from_text(
                &self::ml_number::js_number(suffix),
            )),
        )) || text.contains(
            &(self::ml_string::Text::from_units(vec![
                109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114, 101, 118, 105, 111, 117, 115,
            ]))
            .concat(&self::ml_string::Text::from_text(
                &self::ml_number::js_number(suffix),
            )),
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
pub fn source_ml_source1_temporary_read_optional_record_field_order_followers_loop15(
    numeric_keys: &[f64],
    mut selected: f64,
    mut followers: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (numeric_keys.len() as f64) {
            {
                let key = (self::ml_array::optional_at(
                    numeric_keys,
                    self::ml_array::number_index(index),
                ))
                .unwrap_or_else(|| -1f64);
                if (key < 0f64) || (key > selected) {
                    {
                        let followers_2 = {
                            let (ml_append_right, ml_append_left) = (vec![index], followers);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (followers, index) = (followers_2, index_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let index_3 = index + 1f64;
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { followers };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result {
    MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(self::ml_string::Text),
    MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_class_record_owner_loop16(
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    fields: &[self::ml_string::Text],
    mut owner: self::ml_string::Text,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result {
    loop {
        return if at < (brands.len() as f64) {
            if self::ml_array::includes(
                fields,
                &self::dependency_function2(brands, at),
                0f64,
                false,
                |a, b| a == b,
            ) {
                {
                    let ml_library_inequality_1 =
                        owner != (self::ml_string::Text::from_units(vec![]));
                    if ml_library_inequality_1 {
                        self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(self::ml_string::Text::from_units(vec![35, 97, 109, 98, 105, 103, 117, 111, 117, 115]))
                    } else {
                        {
                            let owner_2 = self::dependency_function2(constructors, at);
                            {
                                let at_2 = at + 1f64;
                                {
                                    (owner, at) = (owner_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(owner.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExactLoop18Result {
    MlSource1TemporaryExactLoop18Done,
    MlSource1TemporaryExactLoop18Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_exact_loop18(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    source_ml_source1_temporary_values17: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index17: self::ml::Big,
) -> self::MlSource1TemporaryExactLoop18Result {
    loop {
        return if source_ml_source1_temporary_index17
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values17.len() as u128))
        {
            {
                let name = self::ml_array::at(
                    source_ml_source1_temporary_values17,
                    source_ml_source1_temporary_index17.to_index(),
                );
                if (self::dependency_function3(starts, counts, labels, targets, node, &(name)))
                    < 0f64
                {
                    self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Return(
                        false,
                    )
                } else {
                    {
                        let source_ml_source1_temporary_index17_2 =
                            source_ml_source1_temporary_index17.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index17 =
                                source_ml_source1_temporary_index17_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStringsLoop19Result {
    MlSource1TemporaryStringsLoop19Done(Vec<self::ml_string::Text>),
    MlSource1TemporaryStringsLoop19Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_strings_loop19(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut node: f64,
    mut result: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlSource1TemporaryStringsLoop19Result {
    loop {
        return if at < (self::dependency_function1(counts, node)) {
            {
                let value = self::dependency_function4(starts, counts, targets, node, at);
                if ((self::dependency_function2(forms, value))
                    != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
                    || ((self::dependency_function2(texts, value))
                        == (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Return(vec![self::ml_string::Text::from_units(vec![])])
                } else {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value66 = result;
                            {
                                let source_ml_source2_concat_value67 =
                                    vec![self::dependency_function2(texts, value)];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value66,
                                    ),
                                    source_ml_source2_concat_value67,
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
            }
        } else {
            self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Done(result)
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
pub fn source_ml_source1_temporary_names_loop20(
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
                    let source_ml_source2_concat_value68 = result;
                    {
                        let source_ml_source2_concat_value69 = vec![self::word(
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
                                source_ml_source2_concat_value68,
                            ),
                            source_ml_source2_concat_value69,
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
pub enum MlSource1TemporaryCarriesLoop21Result {
    MlSource1TemporaryCarriesLoop21Done,
    MlSource1TemporaryCarriesLoop21Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_carries_loop21(
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
) -> self::MlSource1TemporaryCarriesLoop21Result {
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
                self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Return(
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
            self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFreshLoop22Result {
    MlSource1TemporaryFreshLoop22Done,
    MlSource1TemporaryFreshLoop22Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_fresh_loop22(
    bindings: &[self::ml_string::Text],
    values: &[f64],
    name: &self::ml_string::Text,
    mut at: f64,
) -> self::MlSource1TemporaryFreshLoop22Result {
    loop {
        return if at >= 0f64 {
            if (self::dependency_function2(bindings, at)) == (*name) {
                self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Return(
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
            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFreshLoop23Result {
    MlSource1TemporaryFreshLoop23Done(f64),
    MlSource1TemporaryFreshLoop23Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_fresh_loop23(
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
) -> self::MlSource1TemporaryFreshLoop23Result {
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
                if value == -3f64 {
                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Return(
                        -3f64,
                    )
                } else if (value == -1f64)
                    || (((value >= 0f64) && (origin >= 0f64)) && (value != origin))
                {
                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Return(
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
            self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Done(origin)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop33(
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
                    let source_ml_source2_concat_value70 = values;
                    {
                        let source_ml_source2_concat_value71 = vec![self::dependency_function4(
                            starts, counts, targets, operand, at,
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value70,
                            ),
                            source_ml_source2_concat_value71,
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
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop37(
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
                        let source_ml_source2_concat_value72 = pending;
                        {
                            let source_ml_source2_concat_value73 =
                                vec![self::dependency_function1(targets, at)];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value72,
                                ),
                                source_ml_source2_concat_value73,
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop36(
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
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result {
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
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let seen_2 = {
                                        let source_ml_source2_concat_value74 = seen;
                                        {
                                            let source_ml_source2_concat_value75 = vec![actual];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value74,
                                                ),
                                                source_ml_source2_concat_value75,
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
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 97, 100, 109, 105, 116, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 100, 101, 98, 117, 103, 32, 111, 114, 32, 112, 114, 111, 111, 102, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110]))
                                        } else {
                                            {
                                                let at = self::dependency_function1(starts, actual);
                                                {
                                                    let pending_3 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop37(starts, counts, labels, targets, pending_2, actual, at);
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
                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop35(
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
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result {
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
                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 97, 100, 109, 105, 116, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 100, 101, 98, 117, 103, 32, 111, 114, 32, 112, 114, 111, 111, 102, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110]))
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done
        };
    }
}
