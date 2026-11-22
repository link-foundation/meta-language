// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result;
use super::carries;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::exact;
use super::fresh;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop43;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop44;
use super::strings;
use super::word;

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
    fields: &[self::ml_string::Text],
    mut methods: f64,
    mut method_names: Vec<self::ml_string::Text>,
    mut helpers: Vec<self::ml_string::Text>,
    mut method: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Result {
    loop {
        return if method < (self::dependency_function1(counts, methods)) {
            {
                let row = self::dependency_function4(starts, counts, targets, methods, method);
                {
                    let name = self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        row,
                        &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                    );
                    {
                        let helper = self::word(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            row,
                            &(self::ml_string::Text::from_units(vec![
                                104, 101, 108, 112, 101, 114,
                            ])),
                        );
                        if ((((!(self::exact(
                            starts,
                            counts,
                            labels,
                            targets,
                            row,
                            &(vec![
                                self::ml_string::Text::from_units(vec![110, 97, 109, 101]),
                                self::ml_string::Text::from_units(vec![
                                    104, 101, 108, 112, 101, 114,
                                ]),
                            ]),
                        )) || (name == (self::ml_string::Text::from_units(vec![]))))
                            || (helper == (self::ml_string::Text::from_units(vec![]))))
                            || self::ml_array::includes(
                                fields,
                                &name.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            ))
                            || self::ml_array::includes(
                                &method_names,
                                &name.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            ))
                            || self::ml_array::includes(
                                &helpers,
                                &helper.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )
                        {
                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 111, 119, 110, 101, 114, 115, 32, 97, 114, 101, 32, 99, 111, 110, 116, 114, 97, 100, 105, 99, 116, 111, 114, 121]))
                        } else {
                            {
                                let method_names_2 = {
                                    let source_ml_source2_concat_value84 = method_names;
                                    {
                                        let source_ml_source2_concat_value85 = vec![name.clone()];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source2_concat_value84,
                                            ),
                                            source_ml_source2_concat_value85,
                                        )
                                    }
                                };
                                {
                                    let helpers_2 = {
                                        let source_ml_source2_concat_value86 = helpers;
                                        {
                                            let source_ml_source2_concat_value87 =
                                                vec![helper.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source2_concat_value86,
                                                ),
                                                source_ml_source2_concat_value87,
                                            )
                                        }
                                    };
                                    {
                                        let method_2 = method + 1f64;
                                        {
                                            (method_names, helpers, method) =
                                                (method_names_2, helpers_2, method_2);
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Done(helpers)
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop41(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    mut constructors: Vec<self::ml_string::Text>,
    mut brands: Vec<self::ml_string::Text>,
    mut representations: Vec<self::ml_string::Text>,
    mut owner_nodes: Vec<f64>,
    mut constructor_nodes: Vec<f64>,
    mut owners: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result {
    loop {
        return if at < (self::dependency_function1(counts, owners)) {
            {
                let owner = self::dependency_function4(starts, counts, targets, owners, at);
                {
                    let constructor = self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        owner,
                        &(self::ml_string::Text::from_units(vec![
                            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                        ])),
                    );
                    {
                        let brand = self::word(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            owner,
                            &(self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100])),
                        );
                        {
                            let representation = self::word(
                                forms,
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                owner,
                                &(self::ml_string::Text::from_units(vec![
                                    114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111,
                                    110,
                                ])),
                            );
                            if ((((!(self::exact(
                                starts,
                                counts,
                                labels,
                                targets,
                                owner,
                                &(vec![
                                    self::ml_string::Text::from_units(vec![
                                        99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                                    ]),
                                    self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100]),
                                    self::ml_string::Text::from_units(vec![
                                        114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105,
                                        111, 110,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        102, 105, 101, 108, 100, 115,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        109, 101, 116, 104, 111, 100, 115,
                                    ]),
                                ]),
                            )) || (constructor
                                == (self::ml_string::Text::from_units(vec![]))))
                                || (brand == (self::ml_string::Text::from_units(vec![]))))
                                || self::ml_array::includes(
                                    &constructors,
                                    &constructor.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                ))
                                || self::ml_array::includes(
                                    &brands,
                                    &brand.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                ))
                                || !(self::ml_array::includes(
                                    &vec![
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104,
                                            101, 108, 112, 101, 114, 115,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111,
                                            116, 115,
                                        ]),
                                    ],
                                    &representation.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                ))
                            {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 98, 114, 97, 110, 100, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                            } else {
                                {
                                    let fields = self::strings(
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
                                            owner,
                                            &(self::ml_string::Text::from_units(vec![
                                                102, 105, 101, 108, 100, 115,
                                            ])),
                                        ),
                                    );
                                    if ((((fields.len() as f64) > 8192f64)
                                        || self::ml_array::includes(
                                            &fields,
                                            &self::ml_string::Text::from_units(vec![]),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        ))
                                        || (((self::ml_array::set(fields.clone(), |a, b| a == b))
                                            .len()
                                            as f64)
                                            != (fields.len() as f64)))
                                        || self::ml_array::includes(
                                            &fields,
                                            &brand.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        )
                                    {
                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 110, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116]))
                                    } else {
                                        {
                                            let methods = self::dependency_function3(
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                owner,
                                                &(self::ml_string::Text::from_units(vec![
                                                    109, 101, 116, 104, 111, 100, 115,
                                                ])),
                                            );
                                            if ((methods < 0f64)
                                                || ((self::dependency_function2(forms, methods))
                                                    != (self::ml_string::Text::from_units(vec![
                                                        97, 114, 114, 97, 121,
                                                    ]))))
                                                || ((self::dependency_function1(counts, methods))
                                                    > 8192f64)
                                            {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 104, 101, 108, 112, 101, 114, 115]))
                                            } else {
                                                {
                                                    let method_names =
                                                        Vec::<self::ml_string::Text>::new();
                                                    {
                                                        let helpers =
                                                            Vec::<self::ml_string::Text>::new();
                                                        {
                                                            let method = 0f64;
                                                            {
                                                                let source_ml_source2_s88 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop42(forms, texts, starts, counts, labels, targets, &(fields), methods, method_names, helpers, method);
                                                                match source_ml_source2_s88 {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Done(helpers_2) => {
                                                                        {
                                                                            let actual = -1f64;
                                                                            {
                                                                                let declaration = 0f64;
                                                                                {
                                                                                    let actual_2 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop43(forms, texts, starts, counts, labels, targets, declarations, &(constructor), actual, declaration);
                                                                                    if actual_2 < 0f64 {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))
                                                                                    } else {
                                                                                        {
                                                                                            let ret = self::dependency_function3(starts, counts, labels, targets, actual_2, &(self::ml_string::Text::from_units(vec![114, 101, 116])));
                                                                                            {
                                                                                                let body = self::dependency_function3(starts, counts, labels, targets, actual_2, &(self::ml_string::Text::from_units(vec![98, 111, 100, 121])));
                                                                                                {
                                                                                                    let reference = self::dependency_function3(starts, counts, labels, targets, ret, &(self::ml_string::Text::from_units(vec![114, 101, 102, 101, 114, 101, 110, 99, 101])));
                                                                                                    if ((self::word(forms, texts, starts, counts, labels, targets, ret, &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])))) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) && (((reference < 0f64) || ((self::dependency_function2(forms, reference)) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || ((self::dependency_function2(texts, reference)) != (self::ml_string::Text::from_units(vec![116, 114, 117, 101])))) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![97, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 115, 104, 97, 114, 101, 100, 32, 105, 110, 115, 116, 97, 110, 99, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
                                                                                                    } else if !(self::carries(forms, texts, starts, counts, labels, targets, ret, &(constructor), 0f64)) || ((self::fresh(forms, texts, starts, counts, labels, targets, body, &(constructor), &(Vec::<self::ml_string::Text>::new()), &(Vec::<f64>::new()), 0f64)) < 0f64) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 105, 116, 115, 32, 111, 119, 110, 32, 102, 114, 101, 115, 104, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                                                                                        } else {
                                                                                                            {
                                                                                                                let source_ml_source1_temporary_values19 = helpers_2;
                                                                                                                {
                                                                                                                    let source_ml_source1_temporary_index19 = self::ml::Big::from_i128(0);
                                                                                                                    {
                                                                                                                        let source_ml_source2_s89 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop44(forms, texts, starts, counts, labels, targets, declarations, &(constructor), &(source_ml_source1_temporary_values19), source_ml_source1_temporary_index19);
                                                                                                                        match source_ml_source2_s89 {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Done => {
                                                                                                                                {
                                                                                                                                    let constructors_2 = {
                                                                                                                                        let source_ml_source2_concat_value90 = constructors;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_concat_value91 = vec![constructor.clone()];
                                                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value90), source_ml_source2_concat_value91)
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let brands_2 = {
                                                                                                                                            let source_ml_source2_concat_value92 = brands;
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_concat_value93 = vec![brand.clone()];
                                                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value92), source_ml_source2_concat_value93)
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let representations_2 = {
                                                                                                                                                let source_ml_source2_concat_value94 = representations;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_concat_value95 = vec![representation.clone()];
                                                                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value94), source_ml_source2_concat_value95)
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let owner_nodes_2 = {
                                                                                                                                                    let source_ml_source2_concat_value96 = owner_nodes;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_concat_value97 = vec![owner];
                                                                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value96), source_ml_source2_concat_value97)
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let constructor_nodes_2 = {
                                                                                                                                                        let source_ml_source2_concat_value98 = constructor_nodes;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_concat_value99 = vec![actual_2];
                                                                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value98), source_ml_source2_concat_value99)
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let at_2 = at + 1f64;
                                                                                                                                                        {
                                                                                                                                                            (constructors, brands, representations, owner_nodes, constructor_nodes, at) = (constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2, at_2);
                                                                                                                                                            continue;
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop44Return(source_ml_source1_temporary_result44) => {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(source_ml_source1_temporary_result44.clone())
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
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop42Return(source_ml_source1_temporary_result45) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(source_ml_source1_temporary_result45.clone())
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done(constructors, brands, representations, owner_nodes, constructor_nodes)
        };
    }
}
