// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::exact;
use super::ml_array;
use super::ml_string;
use super::read_java_script_class_owner_model;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop28;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop29;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop30;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop31;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop46;
use super::word;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_class_owner_error(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    type_keys: &[self::ml_string::Text],
    root: f64,
) -> self::ml_string::Text {
    if ((((((starts.len() as f64) > 1_048_576f64)
        || ((starts.len() as f64) != (forms.len() as f64)))
        || ((starts.len() as f64) != (texts.len() as f64)))
        || ((starts.len() as f64) != (counts.len() as f64)))
        || ((starts.len() as f64) != (type_keys.len() as f64)))
        || ((labels.len() as f64) != (targets.len() as f64))
    {
        self::ml_string::Text::from_units(vec![
            99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112, 32, 101, 120,
            99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101,
            32, 103, 114, 97, 112, 104, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        {
            let witness = self::dependency_function3(
                starts,
                counts,
                labels,
                targets,
                root,
                &(self::ml_string::Text::from_units(vec![
                    115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114, 115,
                ])),
            );
            {
                let declarations = self::dependency_function3(
                    starts,
                    counts,
                    labels,
                    targets,
                    root,
                    &(self::ml_string::Text::from_units(vec![
                        100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115,
                    ])),
                );
                {
                    let constructors = Vec::<self::ml_string::Text>::new();
                    {
                        let brands = Vec::<self::ml_string::Text>::new();
                        {
                            let representations = Vec::<self::ml_string::Text>::new();
                            {
                                let owner_nodes = Vec::<f64>::new();
                                {
                                    let constructor_nodes = Vec::<f64>::new();
                                    {
                                        let source_ml_source2_s25 = if witness >= 0f64 {
                                            if (((((((self::dependency_function2(
                                                forms, witness,
                                            )) != (self::ml_string::Text::from_units(
                                                vec![111, 98, 106, 101, 99, 116],
                                            ))) || !(self::exact(
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                witness,
                                                &(vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        115, 99, 104, 101, 109, 97, 86, 101, 114,
                                                        115, 105, 111, 110,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        108, 97, 110, 103, 117, 97, 103, 101,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        109, 111, 100, 101, 108,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        111, 119, 110, 101, 114, 115,
                                                    ]),
                                                ]),
                                            ))) || ((self::dependency_function2(
                                                forms,
                                                self::dependency_function3(
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    witness,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        115, 99, 104, 101, 109, 97, 86, 101, 114,
                                                        115, 105, 111, 110,
                                                    ])),
                                                ),
                                            ))
                                                != (self::ml_string::Text::from_units(vec![
                                                    110, 117, 109, 98, 101, 114,
                                                ]))))
                                                || ((self::dependency_function2(
                                                    texts,
                                                    self::dependency_function3(
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        witness,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            115, 99, 104, 101, 109, 97, 86, 101,
                                                            114, 115, 105, 111, 110,
                                                        ])),
                                                    ),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![49],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    witness,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        108, 97, 110, 103, 117, 97, 103, 101,
                                                    ])),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![
                                                        74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
                                                    ],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    root,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        115, 111, 117, 114, 99, 101, 76, 97, 110,
                                                        103, 117, 97, 103, 101,
                                                    ])),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![
                                                        74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
                                                    ],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    witness,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        109, 111, 100, 101, 108,
                                                    ])),
                                                ))
                                                    != (self::read_java_script_class_owner_model()))
                                            {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text::from_units(vec![117, 110, 114, 101, 99, 111, 103, 110, 105, 122, 101, 100, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 119, 105, 116, 110, 101, 115, 115, 32, 104, 101, 97, 100, 101, 114]))
                                            } else {
                                                {
                                                    let owners = self::dependency_function3(
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        witness,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            111, 119, 110, 101, 114, 115,
                                                        ])),
                                                    );
                                                    if ((((owners < 0f64)
                                                        || ((self::dependency_function2(
                                                            forms, owners,
                                                        ))
                                                            != (self::ml_string::Text::from_units(
                                                                vec![97, 114, 114, 97, 121],
                                                            ))))
                                                        || ((self::dependency_function1(
                                                            counts, owners,
                                                        )) < 1f64))
                                                        || ((self::dependency_function1(
                                                            counts, owners,
                                                        )) > 64f64))
                                                        || (declarations < 0f64)
                                                    {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 97, 114, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100]))
                                                    } else {
                                                        {
                                                            let at = 0f64;
                                                            {
                                                                let source_ml_source2_s26 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop46(forms, texts, starts, counts, labels, targets, declarations, constructors, brands, representations, owner_nodes, constructor_nodes, owners, at);
                                                                match source_ml_source2_s26 {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Done(constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2)
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(source_ml_source1_temporary_result46) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(source_ml_source1_temporary_result46)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors, brands, representations, owner_nodes, constructor_nodes)
                                        };
                                        match source_ml_source2_s25 {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors_3, brands_3, representations_3, owner_nodes_3, constructor_nodes_3) => {
                                                {
                                                    let node = 0f64;
                                                    {
                                                        let source_ml_source2_s27 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop28(forms, texts, starts, counts, labels, targets, type_keys, &(constructors_3), &(brands_3), &(representations_3), &(owner_nodes_3), &(constructor_nodes_3), node);
                                                        match source_ml_source2_s27 {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done => {
                                                                if (constructors_3.len() as f64) == 0f64 {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                } else {
                                                                    {
                                                                        let roots = vec![self::dependency_function3(starts, counts, labels, targets, root, &(self::ml_string::Text::from_units(vec![109, 97, 105, 110])))];
                                                                        {
                                                                            let scopes = vec![self::ml_string::Text::from_units(vec![])];
                                                                            {
                                                                                let declaration = 0f64;
                                                                                {
                                                                                    let source_ml_source2_s28 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop29(forms, texts, starts, counts, labels, targets, declarations, roots, scopes, declaration);
                                                                                    match source_ml_source2_s28 {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(roots_2, scopes_2) => {
                                                                                            {
                                                                                                let scope = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s29 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop30(forms, texts, starts, counts, labels, targets, &(constructors_3), &(brands_3), &(representations_3), &(owner_nodes_3), &(roots_2), &(scopes_2), scope);
                                                                                                    match source_ml_source2_s29 {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done => {
                                                                                                            {
                                                                                                                let node_2 = 0f64;
                                                                                                                {
                                                                                                                    let source_ml_source2_s30 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop31(forms, texts, starts, counts, labels, targets, declarations, &(constructors_3), &(brands_3), &(representations_3), node_2);
                                                                                                                    match source_ml_source2_s30 {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done => {
                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(source_ml_source1_temporary_result31) => {
                                                                                                                            source_ml_source1_temporary_result31
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(source_ml_source1_temporary_result37) => {
                                                                                                            source_ml_source1_temporary_result37
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
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result41) => {
                                                                source_ml_source1_temporary_result41
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(source_ml_source1_temporary_result45) => {
                                                source_ml_source1_temporary_result45
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction3Loop1Result {
    MlSource1TemporaryDependencyFunction3Loop1Done,
    MlSource1TemporaryDependencyFunction3Loop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function3_loop1(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    field: &self::ml_string::Text,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryDependencyFunction3Loop1Result {
    loop {
        return if at < (start + count) {
            if (self::dependency_function2(labels, at)) == (*field) {
                self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Return(self::dependency_function1(targets, at))
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
            self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done,
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(Vec<f64>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    brands: &[self::ml_string::Text],
    mut pending: Vec<f64>,
    mut fields: f64,
    mut at: f64,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    loop {
        return if at < (self::dependency_function1(counts, fields)) {
            {
                let row = self::dependency_function4(starts, counts, targets, fields, at);
                {
                    let name = self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        row,
                        &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                    );
                    if (name < 0f64)
                        || (self::ml_array::includes(
                            brands,
                            &self::dependency_function2(texts, name),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))
                    {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                    } else {
                        {
                            let pending_2 = {
                                let source_ml_source2_concat_value31 = pending;
                                {
                                    let source_ml_source2_concat_value32 =
                                        vec![self::dependency_function3(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            row,
                                            &(self::ml_string::Text::from_units(vec![
                                                116, 121, 112, 101,
                                            ])),
                                        )];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value31,
                                        ),
                                        source_ml_source2_concat_value32,
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
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut pending: Vec<f64>,
    mut children: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (self::dependency_function1(counts, children)) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value33 = pending;
                    {
                        let source_ml_source2_concat_value34 = vec![self::dependency_function4(
                            starts, counts, targets, children, at,
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value33,
                            ),
                            source_ml_source2_concat_value34,
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
        return { pending };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(Vec<f64>, bool),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(Vec<f64>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut pending: Vec<f64>,
    mut fields: f64,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (self::dependency_function1(counts, fields)) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value35 = pending;
                    {
                        let source_ml_source2_concat_value36 = vec![self::dependency_function3(
                            starts,
                            counts,
                            labels,
                            targets,
                            self::dependency_function4(starts, counts, targets, fields, index),
                            &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value35,
                            ),
                            source_ml_source2_concat_value36,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (pending, index) = (pending_2, index_2);
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
pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut pending: Vec<f64>,
    mut constructors: f64,
    mut ctor: f64,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    loop {
        return if ctor < (self::dependency_function1(counts, constructors)) {
            {
                let fields = self::dependency_function3(
                    starts,
                    counts,
                    labels,
                    targets,
                    self::dependency_function4(starts, counts, targets, constructors, ctor),
                    &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])),
                );
                if fields < 0f64 {
                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(true)
                } else {
                    {
                        let index = 0f64;
                        {
                            let pending_2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(starts, counts, labels, targets, pending, fields, index);
                            {
                                let ctor_2 = ctor + 1f64;
                                {
                                    (pending, ctor) = (pending_2, ctor_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending)
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
pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    mut pending: Vec<f64>,
    mut name: f64,
    mut found: bool,
    mut at: f64,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    loop {
        return if at < (self::dependency_function1(counts, declarations)) {
            {
                let row = self::dependency_function4(starts, counts, targets, declarations, at);
                {
                    let full_name = self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        row,
                        &(self::ml_string::Text::from_units(vec![
                            102, 117, 108, 108, 78, 97, 109, 101,
                        ])),
                    );
                    if (full_name < 0f64)
                        || ((self::dependency_function2(texts, full_name))
                            != (self::dependency_function2(texts, name)))
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
                            let constructors = self::dependency_function3(
                                starts,
                                counts,
                                labels,
                                targets,
                                row,
                                &(self::ml_string::Text::from_units(vec![99, 116, 111, 114, 115])),
                            );
                            if constructors < 0f64 {
                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(true)
                            } else {
                                {
                                    let found_2 = true;
                                    {
                                        let ctor = 0f64;
                                        {
                                            let source_ml_source2_s37 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts, counts, labels, targets, pending, constructors, ctor);
                                            match source_ml_source2_s37 {
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending_2) => {
                                                    {
                                                        let at_3 = at + 1f64;
                                                        {
                                                            (pending, found, at) = (pending_2, found_2, at_3);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(source_ml_source1_temporary_result6) => {
                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result6)
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
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending, found)
        };
    }
}
