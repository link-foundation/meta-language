// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryDependencyFunction3Loop1Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;

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
                                let source_ml_source2_concat_value30 = pending;
                                {
                                    let source_ml_source2_concat_value31 =
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
                                            source_ml_source2_concat_value30,
                                        ),
                                        source_ml_source2_concat_value31,
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
                    let source_ml_source2_concat_value32 = pending;
                    {
                        let source_ml_source2_concat_value33 = vec![self::dependency_function4(
                            starts, counts, targets, children, at,
                        )];
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
                    let source_ml_source2_concat_value34 = pending;
                    {
                        let source_ml_source2_concat_value35 = vec![self::dependency_function3(
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
                                source_ml_source2_concat_value34,
                            ),
                            source_ml_source2_concat_value35,
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
                                            let source_ml_source2_s36 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts, counts, labels, targets, pending, constructors, ctor);
                                            match source_ml_source2_s36 {
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    brands: &[self::ml_string::Text],
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    loop {
        return {
            let ml_library_condition_3 = {
                let ml_value = pending.len() as f64;
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_3 {
                if ((seen.len() as f64) >= 100_000f64) || ((pending.len() as f64) > 100_000f64) {
                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                } else {
                    {
                        let node =
                            self::dependency_function1(&(pending), ((pending.len() as f64) - 1f64));
                        {
                            let pending_2 = self::ml_array::slice(&pending, 0f64, -1f64);
                            if (node < 0f64) || (node >= (starts.len() as f64)) {
                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                            } else {
                                if self::ml_array::includes(
                                    &seen,
                                    &node,
                                    0f64,
                                    false,
                                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                                ) {
                                    {
                                        pending = pending_2;
                                        continue;
                                    }
                                }
                                {
                                    {
                                        let seen_2 = {
                                            let source_ml_source2_concat_value37 = seen;
                                            {
                                                let source_ml_source2_concat_value38 = vec![node];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value37,
                                                    ),
                                                    source_ml_source2_concat_value38,
                                                )
                                            }
                                        };
                                        {
                                            let owner = self::dependency_function3(
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                node,
                                                &(self::ml_string::Text::from_units(vec![
                                                    107, 105, 110, 100,
                                                ])),
                                            );
                                            if owner < 0f64 {
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                            } else {
                                                {
                                                    let kind =
                                                        self::dependency_function2(texts, owner);
                                                    if kind
                                                        == (self::ml_string::Text::from_units(
                                                            vec![114, 101, 99, 111, 114, 100],
                                                        ))
                                                    {
                                                        {
                                                            let fields = self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                            if fields < 0f64 {
                                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                            } else {
                                                                {
                                                                    let at = 0f64;
                                                                    {
                                                                        let source_ml_source2_s39 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts, starts, counts, labels, targets, brands, pending_2, fields, at);
                                                                        match source_ml_source2_s39 {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending_3) => {
                                                                            {
                                                                                (pending, seen) = (pending_3, seen_2);
                                                                                continue;
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(source_ml_source1_temporary_result3) => {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result3)
                                                                        }
                                                                    }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if self::ml_array::includes(
                                                            &vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![97, 114, 114, 97, 121],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        111, 112, 116, 105, 111,
                                                                        110,
                                                                    ],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![115, 101, 116],
                                                                ),
                                                                self::ml_string::Text::from_units(
                                                                    vec![99, 101, 108, 108],
                                                                ),
                                                            ],
                                                            &kind.clone(),
                                                            0f64,
                                                            false,
                                                            |a, b| a == b,
                                                        ) {
                                                            {
                                                                let pending_4 = {
                                                                    let source_ml_source2_concat_value40 =
                                                                        pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value41 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value40), source_ml_source2_concat_value41)
                                                                    }
                                                                };
                                                                {
                                                                    (pending, seen) =
                                                                        (pending_4, seen_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        {
                                                            if kind == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                            {
                                                                let pending_5 = {
                                                                    let source_ml_source2_concat_value42 = pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value43 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![107, 101, 121]))), self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value42), source_ml_source2_concat_value43)
                                                                    }
                                                                };
                                                                {
                                                                    (pending, seen) = (pending_5, seen_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                            {
                                                                if ((kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110])))) || (kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                                {
                                                                    let children = self::dependency_function3(starts, counts, labels, targets, node, &(if kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])) {
                                                                        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115])
                                                                    } else if kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110])) {
                                                                            self::ml_string::Text::from_units(vec![97, 108, 116, 101, 114, 110, 97, 116, 105, 118, 101, 115])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115])
                                                                        }));
                                                                    if children < 0f64 {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let pending_6 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(starts, counts, targets, pending_2, children, at_2);
                                                                                if kind == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])) {
                                                                                    {
                                                                                        let pending_7 = {
                                                                                            let source_ml_source2_concat_value44 = pending_6;
                                                                                            {
                                                                                                let source_ml_source2_concat_value45 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![114, 101, 116])))];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value44), source_ml_source2_concat_value45)
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            (pending, seen) = (pending_7, seen_2);
                                                                                            continue;
                                                                                        }
                                                                                    }
                                                                                }
{
                                                                                    {
                                                                                        (pending, seen) = (pending_6, seen_2);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else if kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])) {
                                                                    {
                                                                        let name = self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])));
                                                                        if (name < 0f64) || (declarations < 0f64) {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                        } else {
                                                                            {
                                                                                let found = false;
                                                                                {
                                                                                    let at_3 = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s46 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts, starts, counts, labels, targets, declarations, pending_2, name, found, at_3);
                                                                                        match source_ml_source2_s46 {
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending_8, found_2) => {
                                                                                                if found_2 {
                                                                                                    {
                                                                                                        (pending, seen) = (pending_8, seen_2);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
{
                                                                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result5) => {
                                                                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result5)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if (self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![117, 110, 105, 116]), self::ml_string::Text::from_units(vec![98, 111, 111, 108]), self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]), self::ml_string::Text::from_units(vec![105, 110, 116]), self::ml_string::Text::from_units(vec![110, 97, 116]), self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])], &kind.clone(), 0f64, false, |a, b| a == b)) {
                                                                        {
                                                                            (pending, seen) = (pending_2, seen_2);
                                                                            continue;
                                                                        }
                                                                    }
{
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
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
                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Result {
    MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Done,
    MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_record_field_write_operator_loop9(
    operation: &self::ml_string::Text,
    source_ml_source1_temporary_values8: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index8: self::ml::Big,
) -> self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Result {
    loop {
        return if source_ml_source1_temporary_index8
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values8.len() as u128))
        {
            {
                let operator = self::ml_array::at(
                    source_ml_source1_temporary_values8,
                    source_ml_source1_temporary_index8.to_index(),
                );
                if operation.starts_with(
                    &((self::ml_string::Text::from_units(vec![
                        119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
                    ]))
                    .concat(&operator))
                    .concat(&self::ml_string::Text::from_units(vec![58])),
                ) {
                    self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Return(operator.clone())
                } else {
                    {
                        let source_ml_source1_temporary_index8_2 =
                            source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index8 =
                                source_ml_source1_temporary_index8_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_record_field_write_suffix_loop10(
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Result {
    MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Done(self::ml_string::Text),
    MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_class_record_owner_loop11(
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    fields: &[self::ml_string::Text],
    mut owner: self::ml_string::Text,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Result {
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
                        self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Return(self::ml_string::Text::from_units(vec![35, 97, 109, 98, 105, 103, 117, 111, 117, 115]))
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
            self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Done(owner.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExactLoop13Result {
    MlSource1TemporaryExactLoop13Done,
    MlSource1TemporaryExactLoop13Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_exact_loop13(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    source_ml_source1_temporary_values12: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index12: self::ml::Big,
) -> self::MlSource1TemporaryExactLoop13Result {
    loop {
        return if source_ml_source1_temporary_index12
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values12.len() as u128))
        {
            {
                let name = self::ml_array::at(
                    source_ml_source1_temporary_values12,
                    source_ml_source1_temporary_index12.to_index(),
                );
                if (self::dependency_function3(starts, counts, labels, targets, node, &(name)))
                    < 0f64
                {
                    self::MlSource1TemporaryExactLoop13Result::MlSource1TemporaryExactLoop13Return(
                        false,
                    )
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
            self::MlSource1TemporaryExactLoop13Result::MlSource1TemporaryExactLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStringsLoop14Result {
    MlSource1TemporaryStringsLoop14Done(Vec<self::ml_string::Text>),
    MlSource1TemporaryStringsLoop14Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_strings_loop14(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut node: f64,
    mut result: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlSource1TemporaryStringsLoop14Result {
    loop {
        return if at < (self::dependency_function1(counts, node)) {
            {
                let value = self::dependency_function4(starts, counts, targets, node, at);
                if ((self::dependency_function2(forms, value))
                    != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
                    || ((self::dependency_function2(texts, value))
                        == (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryStringsLoop14Result::MlSource1TemporaryStringsLoop14Return(vec![self::ml_string::Text::from_units(vec![])])
                } else {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value47 = result;
                            {
                                let source_ml_source2_concat_value48 =
                                    vec![self::dependency_function2(texts, value)];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value47,
                                    ),
                                    source_ml_source2_concat_value48,
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
            self::MlSource1TemporaryStringsLoop14Result::MlSource1TemporaryStringsLoop14Done(result)
        };
    }
}
