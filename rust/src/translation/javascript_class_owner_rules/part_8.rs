// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::names;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop37;
use super::strings;
use super::word;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop40(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    type_keys: &[self::ml_string::Text],
    mut expected_fields: f64,
    mut actual: f64,
    name: &self::ml_string::Text,
    mut other: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Result {
    loop {
        return if other < (self::dependency_function1(counts, expected_fields)) {
            {
                let expected =
                    self::dependency_function4(starts, counts, targets, expected_fields, other);
                if ((self::word(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    expected,
                    &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                )) == (*name))
                    && ((self::dependency_function2(
                        type_keys,
                        self::dependency_function3(
                            starts,
                            counts,
                            labels,
                            targets,
                            actual,
                            &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                        ),
                    )) != (self::dependency_function2(
                        type_keys,
                        self::dependency_function3(
                            starts,
                            counts,
                            labels,
                            targets,
                            expected,
                            &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                        ),
                    )))
                {
                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 102, 105, 101, 108, 100, 32, 99, 104, 97, 110, 103, 101, 100, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 116, 121, 112, 101]))
                } else {
                    {
                        let other_2 = other + 1f64;
                        {
                            other = other_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop39(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    type_keys: &[self::ml_string::Text],
    mut actual_fields: f64,
    mut expected_fields: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Result {
    loop {
        return if at < (self::dependency_function1(counts, actual_fields)) {
            {
                let actual = self::dependency_function4(starts, counts, targets, actual_fields, at);
                {
                    let name = self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        actual,
                        &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                    );
                    {
                        let other = 0f64;
                        {
                            let source_ml_source2_s79 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop40(forms, texts, starts, counts, labels, targets, type_keys, expected_fields, actual, &(name), other);
                            match source_ml_source2_s79 {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Done => {
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            at = at_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop40Return(source_ml_source1_temporary_result40) => {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Return(source_ml_source1_temporary_result40)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop38(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    brands: &[self::ml_string::Text],
    mut position: f64,
    mut branded_fields: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result {
    loop {
        return if at < (self::dependency_function1(counts, branded_fields)) {
            {
                let row = self::dependency_function4(starts, counts, targets, branded_fields, at);
                if ((self::word(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    row,
                    &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                )) == (self::dependency_function2(brands, position)))
                    && ((self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::dependency_function3(
                            starts,
                            counts,
                            labels,
                            targets,
                            row,
                            &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                        ),
                        &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
                    )) != (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))
                {
                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 98, 114, 97, 110, 100, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 85, 110, 105, 116, 32, 102, 105, 101, 108, 100]))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop23(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    type_keys: &[self::ml_string::Text],
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    owner_nodes: &[f64],
    constructor_nodes: &[f64],
    mut node: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result {
    loop {
        return if node < (starts.len() as f64) {
            if (self::word(
                forms,
                texts,
                starts,
                counts,
                labels,
                targets,
                node,
                &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
            )) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))
            {
                {
                    let owner = self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![
                            115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101,
                            114,
                        ])),
                    );
                    {
                        let fields =
                            self::names(forms, texts, starts, counts, labels, targets, node);
                        {
                            let position = self::ml_array::index_of(
                                constructors,
                                &owner.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            );
                            {
                                let present = {
                                    let ml_values1 = brands.to_vec();
                                    let ml_callback1 = {
                                        let fields = fields.clone();
                                        std::rc::Rc::new(
                                            move |brand: self::ml_string::Text| -> bool {
                                                self::ml_array::includes(
                                                    &fields,
                                                    &brand.clone(),
                                                    0f64,
                                                    false,
                                                    |a, b| a == b,
                                                )
                                            },
                                        )
                                    };
                                    ml_values1
                                        .iter()
                                        .cloned()
                                        .enumerate()
                                        .any(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
                                };
                                if ((self::dependency_function3(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node,
                                    &(self::ml_string::Text::from_units(vec![
                                        115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79,
                                        119, 110, 101, 114,
                                    ])),
                                )) >= 0f64)
                                    && (owner == (self::ml_string::Text::from_units(vec![])))
                                {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 116, 121, 112, 101, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32, 110, 111, 110, 101, 109, 112, 116, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
                                } else {
                                    if (owner == (self::ml_string::Text::from_units(vec![])))
                                        && !present
                                    {
                                        {
                                            let node_2 = node + 1f64;
                                            {
                                                node = node_2;
                                                continue;
                                            }
                                        }
                                    }
                                    {
                                        if position < 0f64 {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 116, 121, 112, 101, 32, 99, 97, 110, 110, 111, 116, 32, 100, 114, 111, 112, 32, 105, 116, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 119, 105, 116, 110, 101, 115, 115]))
                                        } else {
                                            {
                                                let definition = self::dependency_function1(
                                                    owner_nodes,
                                                    position,
                                                );
                                                {
                                                    let expected = {
                                                        let source_ml_source2_concat_value80 = self::strings(forms, texts, starts, counts, labels, targets, self::dependency_function3(starts, counts, labels, targets, definition, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]))));
                                                        {
                                                            let source_ml_source2_concat_value81 =
                                                                vec![self::dependency_function2(
                                                                    brands, position,
                                                                )];
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value80), source_ml_source2_concat_value81)
                                                        }
                                                    };
                                                    {
                                                        let methods = self::dependency_function3(
                                                            starts,
                                                            counts,
                                                            labels,
                                                            targets,
                                                            definition,
                                                            &(self::ml_string::Text::from_units(
                                                                vec![
                                                                    109, 101, 116, 104, 111, 100,
                                                                    115,
                                                                ],
                                                            )),
                                                        );
                                                        {
                                                            let complete = expected;
                                                            {
                                                                let complete_3 = if (self::dependency_function2(representations, position)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115])) {
                                                                    {
                                                                        let at = 0f64;
                                                                        {
                                                                            self::source_ml_source1_temporary_read_java_script_class_owner_error_loop37(forms, texts, starts, counts, labels, targets, methods, complete, at)
                                                                        }
                                                                    }
                                                                } else {
                                                                    complete
                                                                };
                                                                if ((fields.len() as f64)
                                                                    != (complete_3.len() as f64))
                                                                    || {
                                                                        let ml_values2 = complete_3;
                                                                        let ml_callback2 = {
                                                                            let fields =
                                                                                fields.clone();
                                                                            std::rc::Rc::new(move |name: self::ml_string::Text| -> bool { !(self::ml_array::includes(&fields, &name.clone(), 0f64, false, |a, b| a == b)) })
                                                                        };
                                                                        ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2))
                                                                    }
                                                                {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 114, 101, 99, 111, 114, 100, 32, 99, 111, 110, 116, 114, 97, 100, 105, 99, 116, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115]))
                                                                } else {
                                                                    {
                                                                        let branded_fields = self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let source_ml_source2_s82 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop38(forms, texts, starts, counts, labels, targets, brands, position, branded_fields, at_2);
                                                                                match source_ml_source2_s82 {
                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done => {
                                                                                        {
                                                                                            let reference = self::dependency_function3(starts, counts, labels, targets, self::dependency_function1(constructor_nodes, position), &(self::ml_string::Text::from_units(vec![114, 101, 116])));
                                                                                            if (self::word(forms, texts, starts, counts, labels, targets, reference, &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
                                                                                                {
                                                                                                    let actual_fields = self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                                                                    {
                                                                                                        let expected_fields = self::dependency_function3(starts, counts, labels, targets, reference, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                                                                        {
                                                                                                            let at_3 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_s83 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop39(forms, texts, starts, counts, labels, targets, type_keys, actual_fields, expected_fields, at_3);
                                                                                                                match source_ml_source2_s83 {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Done => {
                                                                                                                        {
                                                                                                                            let node_3 = node + 1f64;
                                                                                                                            {
                                                                                                                                node = node_3;
                                                                                                                                continue;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop39Return(source_ml_source1_temporary_result39) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Return(source_ml_source1_temporary_result39.clone())
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
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(source_ml_source1_temporary_result40) => {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Return(source_ml_source1_temporary_result40.clone())
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22 {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Next(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<f64>,
        Vec<f64>,
    ),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<f64>,
        Vec<f64>,
    ),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop43(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    constructor: &self::ml_string::Text,
    mut actual: f64,
    mut declaration: f64,
) -> f64 {
    loop {
        if declaration < (self::dependency_function1(counts, declarations)) {
            {
                let row =
                    self::dependency_function4(starts, counts, targets, declarations, declaration);
                if ((self::word(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    row,
                    &(self::ml_string::Text::from_units(vec![
                        102, 117, 108, 108, 78, 97, 109, 101,
                    ])),
                )) == (*constructor))
                    && ((self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        row,
                        &(self::ml_string::Text::from_units(vec![107])),
                    )) == (self::ml_string::Text::from_units(vec![102, 110])))
                {
                    {
                        let actual_2 = row;
                        {
                            let declaration_2 = declaration + 1f64;
                            {
                                (actual, declaration) = (actual_2, declaration_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let declaration_3 = declaration + 1f64;
                        {
                            declaration = declaration_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { actual };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop45(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    constructor: &self::ml_string::Text,
    helper: &self::ml_string::Text,
    mut found: bool,
    mut declaration: f64,
) -> bool {
    loop {
        if declaration < (self::dependency_function1(counts, declarations)) {
            {
                let row =
                    self::dependency_function4(starts, counts, targets, declarations, declaration);
                if ((self::word(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    row,
                    &(self::ml_string::Text::from_units(vec![
                        102, 117, 108, 108, 78, 97, 109, 101,
                    ])),
                )) != (*helper))
                    || ((self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        row,
                        &(self::ml_string::Text::from_units(vec![107])),
                    )) != (self::ml_string::Text::from_units(vec![102, 110])))
                {
                    {
                        let declaration_2 = declaration + 1f64;
                        {
                            declaration = declaration_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let params = self::dependency_function3(
                            starts,
                            counts,
                            labels,
                            targets,
                            row,
                            &(self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115])),
                        );
                        {
                            let parameter = if params < 0f64 {
                                -1f64
                            } else {
                                self::dependency_function4(starts, counts, targets, params, 0f64)
                            };
                            {
                                let type_ = self::dependency_function3(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    parameter,
                                    &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                                );
                                if (self::word(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    type_,
                                    &(self::ml_string::Text::from_units(vec![
                                        115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79,
                                        119, 110, 101, 114,
                                    ])),
                                )) == (*constructor)
                                {
                                    {
                                        let found_2 = true;
                                        {
                                            let declaration_3 = declaration + 1f64;
                                            {
                                                (found, declaration) = (found_2, declaration_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let declaration_4 = declaration + 1f64;
                                        {
                                            declaration = declaration_4;
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
        return { found };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop44(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    constructor: &self::ml_string::Text,
    source_ml_source1_temporary_values19: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index19: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result {
    loop {
        return if source_ml_source1_temporary_index19
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values19.len() as u128))
        {
            {
                let helper = self::ml_array::at(
                    source_ml_source1_temporary_values19,
                    source_ml_source1_temporary_index19.to_index(),
                );
                {
                    let found = false;
                    {
                        let declaration = 0f64;
                        {
                            let found_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop45(forms, texts, starts, counts, labels, targets, declarations, constructor, &(helper), found, declaration);
                            if found_2 {
                                {
                                    let source_ml_source1_temporary_index19_2 =
                                        source_ml_source1_temporary_index19
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index19 =
                                            source_ml_source1_temporary_index19_2;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 104, 101, 108, 112, 101, 114, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101]))
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Done(Vec<self::ml_string::Text>),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Return(self::ml_string::Text),
}
