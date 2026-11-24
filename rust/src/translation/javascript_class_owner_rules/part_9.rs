// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::ml_array;
use super::ml_string;
use super::names;
use super::read_record_field_write_name;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop40;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop41;
use super::strings;
use super::word;

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
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    owner_nodes: &[f64],
    owner: &self::ml_string::Text,
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result {
    loop {
        return {
            let ml_library_condition_1 = {
                let ml_value = pending.len() as f64;
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_1 {
                {
                    let direct =
                        self::dependency_function1(&(pending), ((pending.len() as f64) - 1f64));
                    {
                        let node =
                            self::dependency_function1(&(pending), ((pending.len() as f64) - 2f64));
                        {
                            let pending_2 = self::ml_array::slice(&pending, 0f64, -2f64);
                            if (node < 0f64)
                                || self::ml_array::includes(
                                    &seen,
                                    &((node * 2f64) + direct),
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
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 98, 111, 100, 121, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let seen_2 = {
                                            let source_ml_source2_concat_value86 = seen;
                                            {
                                                let source_ml_source2_concat_value87 =
                                                    vec![((node * 2f64) + direct)];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value86,
                                                    ),
                                                    source_ml_source2_concat_value87,
                                                )
                                            }
                                        };
                                        {
                                            let kind = self::word(
                                                forms,
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                node,
                                                &(self::ml_string::Text::from_units(vec![107])),
                                            );
                                            {
                                                let type_ = self::dependency_function3(
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    node,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        116, 121, 112, 101,
                                                    ])),
                                                );
                                                {
                                                    let class_owner = self::word(
                                                        forms,
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        type_,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            115, 111, 117, 114, 99, 101, 67, 108,
                                                            97, 115, 115, 79, 119, 110, 101, 114,
                                                        ])),
                                                    );
                                                    if ((kind
                                                        == (self::ml_string::Text::from_units(
                                                            vec![114, 101, 99, 111, 114, 100],
                                                        )))
                                                        && (class_owner
                                                            != (self::ml_string::Text::from_units(
                                                                vec![],
                                                            ))))
                                                        && (class_owner != (*owner))
                                                    {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 115, 32, 98, 101, 108, 111, 110, 103, 32, 111, 110, 108, 121, 32, 116, 111, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                    } else {
                                                        {
                                                            let object = self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                                            {
                                                                let object_type = self::dependency_function3(starts, counts, labels, targets, object, &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])));
                                                                {
                                                                    let instance = self::ml_array::index_of(constructors, &self::word(forms, texts, starts, counts, labels, targets, object_type, &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))), 0f64, false, |a, b| a == b);
                                                                    {
                                                                        let source_ml_source2_s89 =
                                                                            if instance >= 0f64 {
                                                                                {
                                                                                    let definition = self::dependency_function1(owner_nodes, instance);
                                                                                    {
                                                                                        let brand = self::dependency_function2(brands, instance);
                                                                                        {
                                                                                            let name = if kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])) {
                                                                                        self::word(forms, texts, starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])))
                                                                                    } else if kind == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])) {
                                                                                            self::read_record_field_write_name(&(self::word(forms, texts, starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![111, 112])))))
                                                                                        } else {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        };
                                                                                            if name == brand {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(self::ml_string::Text::from_units(vec![97, 32, 115, 121, 110, 116, 104, 101, 116, 105, 99, 32, 99, 108, 97, 115, 115, 32, 98, 114, 97, 110, 100, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 111, 98, 115, 101, 114, 118, 97, 98, 108, 101, 32, 112, 117, 98, 108, 105, 99, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100]))
                                                                                    } else {
                                                                                        {
                                                                                            let methods = self::dependency_function3(starts, counts, labels, targets, definition, &(self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115])));
                                                                                            {
                                                                                                let at = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s90 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop41(forms, texts, starts, counts, labels, targets, constructors, representations, owner, direct, node, &(kind), object, instance, &(name), methods, at);
                                                                                                    match source_ml_source2_s90 {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(source_ml_source1_temporary_result41) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(source_ml_source1_temporary_result41.clone())
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
                                                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next
                                                                            };
                                                                        match source_ml_source2_s89 {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next => {
                                                                            {
                                                                                let at_2 = self::dependency_function1(starts, node);
                                                                                {
                                                                                    let pending_3 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop40(starts, counts, labels, targets, pending_2, node, &(kind), at_2);
                                                                                    {
                                                                                        (pending, seen) = (pending_3, seen_2);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(source_ml_source1_temporary_result40) => {
                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(source_ml_source1_temporary_result40.clone())
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
                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done
            }
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop30(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    owner_nodes: &[f64],
    roots: &[f64],
    scopes: &[self::ml_string::Text],
    mut scope: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result {
    loop {
        return if scope < (roots.len() as f64) {
            {
                let owner = self::dependency_function2(scopes, scope);
                {
                    let pending = vec![self::dependency_function1(roots, scope), 0f64];
                    {
                        let seen = Vec::<f64>::new();
                        {
                            let source_ml_source2_s91 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop38(forms, texts, starts, counts, labels, targets, constructors, brands, representations, owner_nodes, &(owner), pending, seen);
                            match source_ml_source2_s91 {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done => {
                                    {
                                        let scope_2 = scope + 1f64;
                                        {
                                            scope = scope_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(source_ml_source1_temporary_result38) => {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(source_ml_source1_temporary_result38)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(Vec<f64>, Vec<self::ml_string::Text>),
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
    mut roots: Vec<f64>,
    mut scopes: Vec<self::ml_string::Text>,
    mut declaration: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result {
    loop {
        if (declarations >= 0f64)
            && (declaration < (self::dependency_function1(counts, declarations)))
        {
            {
                let row =
                    self::dependency_function4(starts, counts, targets, declarations, declaration);
                {
                    let roots_2 = {
                        let source_ml_source2_concat_value92 = roots;
                        {
                            let source_ml_source2_concat_value93 = vec![row];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value92,
                                ),
                                source_ml_source2_concat_value93,
                            )
                        }
                    };
                    {
                        let scopes_2 = {
                            let source_ml_source2_concat_value94 = scopes;
                            {
                                let source_ml_source2_concat_value95 = vec![self::word(
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
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value94,
                                    ),
                                    source_ml_source2_concat_value95,
                                )
                            }
                        };
                        {
                            let declaration_2 = declaration + 1f64;
                            {
                                (roots, scopes, declaration) = (roots_2, scopes_2, declaration_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(roots, scopes)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop42(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut methods: f64,
    mut complete: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (self::dependency_function1(counts, methods)) {
            {
                let complete_2 = {
                    let source_ml_source2_concat_value96 = complete;
                    {
                        let source_ml_source2_concat_value97 = vec![self::word(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::dependency_function4(starts, counts, targets, methods, at),
                            &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source2_concat_value96,
                            ),
                            source_ml_source2_concat_value97,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (complete, at) = (complete_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { complete };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(self::ml_string::Text),
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
    type_keys: &[self::ml_string::Text],
    mut expected_fields: f64,
    mut actual: f64,
    name: &self::ml_string::Text,
    mut other: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result {
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
                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 102, 105, 101, 108, 100, 32, 99, 104, 97, 110, 103, 101, 100, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 116, 121, 112, 101]))
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop44(
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
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result {
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
                            let source_ml_source2_s98 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop45(forms, texts, starts, counts, labels, targets, type_keys, expected_fields, actual, &(name), other);
                            match source_ml_source2_s98 {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Done => {
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            at = at_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop45Return(source_ml_source1_temporary_result45) => {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(source_ml_source1_temporary_result45)
                                }
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
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(self::ml_string::Text),
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
    brands: &[self::ml_string::Text],
    mut position: f64,
    mut branded_fields: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result {
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
                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 98, 114, 97, 110, 100, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 85, 110, 105, 116, 32, 102, 105, 101, 108, 100]))
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop28(
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
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result {
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
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 116, 121, 112, 101, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32, 110, 111, 110, 101, 109, 112, 116, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
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
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 116, 121, 112, 101, 32, 99, 97, 110, 110, 111, 116, 32, 100, 114, 111, 112, 32, 105, 116, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 119, 105, 116, 110, 101, 115, 115]))
                                        } else {
                                            {
                                                let definition = self::dependency_function1(
                                                    owner_nodes,
                                                    position,
                                                );
                                                {
                                                    let expected = {
                                                        let source_ml_source2_concat_value99 = self::strings(forms, texts, starts, counts, labels, targets, self::dependency_function3(starts, counts, labels, targets, definition, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]))));
                                                        {
                                                            let source_ml_source2_concat_value100 =
                                                                vec![self::dependency_function2(
                                                                    brands, position,
                                                                )];
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value99), source_ml_source2_concat_value100)
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
                                                                            self::source_ml_source1_temporary_read_java_script_class_owner_error_loop42(forms, texts, starts, counts, labels, targets, methods, complete, at)
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
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 114, 101, 99, 111, 114, 100, 32, 99, 111, 110, 116, 114, 97, 100, 105, 99, 116, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115]))
                                                                } else {
                                                                    {
                                                                        let branded_fields = self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let source_ml_source2_s101 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop43(forms, texts, starts, counts, labels, targets, brands, position, branded_fields, at_2);
                                                                                match source_ml_source2_s101 {
                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Done => {
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
                                                                                                                let source_ml_source2_s102 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop44(forms, texts, starts, counts, labels, targets, type_keys, actual_fields, expected_fields, at_3);
                                                                                                                match source_ml_source2_s102 {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done => {
                                                                                                                        {
                                                                                                                            let node_3 = node + 1f64;
                                                                                                                            {
                                                                                                                                node = node_3;
                                                                                                                                continue;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(source_ml_source1_temporary_result44) => {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result44.clone())
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
                                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop43Return(source_ml_source1_temporary_result45) => {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result45.clone())
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27 {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<f64>,
        Vec<f64>,
    ),
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text),
}
