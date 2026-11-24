// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result;
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result;
use super::contains_java_script_closed_class_brand;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::ml;
use super::ml_array;
use super::ml_string;

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
                                let source_ml_source2_concat_value47 = pending;
                                {
                                    let source_ml_source2_concat_value48 =
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
                                            source_ml_source2_concat_value47,
                                        ),
                                        source_ml_source2_concat_value48,
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
                    let source_ml_source2_concat_value49 = pending;
                    {
                        let source_ml_source2_concat_value50 = vec![self::dependency_function4(
                            starts, counts, targets, children, at,
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value49,
                            ),
                            source_ml_source2_concat_value50,
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
                    let source_ml_source2_concat_value51 = pending;
                    {
                        let source_ml_source2_concat_value52 = vec![self::dependency_function3(
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
                                source_ml_source2_concat_value51,
                            ),
                            source_ml_source2_concat_value52,
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
                                            let source_ml_source2_s53 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts, counts, labels, targets, pending, constructors, ctor);
                                            match source_ml_source2_s53 {
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
                                            let source_ml_source2_concat_value54 = seen;
                                            {
                                                let source_ml_source2_concat_value55 = vec![node];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value54,
                                                    ),
                                                    source_ml_source2_concat_value55,
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
                                                                        let source_ml_source2_s56 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts, starts, counts, labels, targets, brands, pending_2, fields, at);
                                                                        match source_ml_source2_s56 {
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
                                                                    let source_ml_source2_concat_value57 =
                                                                        pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value58 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value57), source_ml_source2_concat_value58)
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
                                                                    let source_ml_source2_concat_value59 = pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value60 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![107, 101, 121]))), self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value59), source_ml_source2_concat_value60)
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
                                                                                            let source_ml_source2_concat_value61 = pending_6;
                                                                                            {
                                                                                                let source_ml_source2_concat_value62 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![114, 101, 116])))];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value61), source_ml_source2_concat_value62)
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
                                                                                        let source_ml_source2_s63 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts, starts, counts, labels, targets, declarations, pending_2, name, found, at_3);
                                                                                        match source_ml_source2_s63 {
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
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(bool),
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    brands: &[self::ml_string::Text],
    mut params: f64,
    mut at: f64,
) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    loop {
        return if at < (self::dependency_function1(counts, params)) {
            {
                let parameter = self::dependency_function4(starts, counts, targets, params, at);
                {
                    let type_ = self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        parameter,
                        &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                    );
                    if self::contains_java_script_closed_class_brand(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        type_,
                        declarations,
                        brands,
                    ) {
                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(true)
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
            }
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done
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
pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    brands: &[self::ml_string::Text],
    exported: &self::ml_string::Text,
    mut found: bool,
    mut index: f64,
) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    loop {
        return if index < (self::dependency_function1(counts, declarations)) {
            {
                let declaration =
                    self::dependency_function4(starts, counts, targets, declarations, index);
                {
                    let name = self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        declaration,
                        &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                    );
                    if (name < 0f64) || ((self::dependency_function2(texts, name)) != (*exported)) {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let kind = self::dependency_function3(
                                starts,
                                counts,
                                labels,
                                targets,
                                declaration,
                                &(self::ml_string::Text::from_units(vec![107])),
                            );
                            if (kind < 0f64)
                                || ((self::dependency_function2(texts, kind))
                                    != (self::ml_string::Text::from_units(vec![102, 110])))
                            {
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        index = index_3;
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
                                        declaration,
                                        &(self::ml_string::Text::from_units(vec![
                                            112, 97, 114, 97, 109, 115,
                                        ])),
                                    );
                                    {
                                        let ret = self::dependency_function3(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            declaration,
                                            &(self::ml_string::Text::from_units(vec![
                                                114, 101, 116,
                                            ])),
                                        );
                                        if (found || (params < 0f64))
                                            || self::contains_java_script_closed_class_brand(
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                ret,
                                                declarations,
                                                brands,
                                            )
                                        {
                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(true)
                                        } else {
                                            {
                                                let found_2 = true;
                                                {
                                                    let at = 0f64;
                                                    {
                                                        let source_ml_source2_s64 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(texts, starts, counts, labels, targets, declarations, brands, params, at);
                                                        match source_ml_source2_s64 {
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done => {
                                                                {
                                                                    let index_4 = index + 1f64;
                                                                    {
                                                                        (found, index) = (found_2, index_4);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(source_ml_source1_temporary_result11) => {
                                                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result11)
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
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found)
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
pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    brands: &[self::ml_string::Text],
    source_ml_source1_temporary_values8: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index8: self::ml::Big,
) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    loop {
        return if source_ml_source1_temporary_index8
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values8.len() as u128))
        {
            {
                let exported = self::ml_array::at(
                    source_ml_source1_temporary_values8,
                    source_ml_source1_temporary_index8.to_index(),
                );
                {
                    let found = false;
                    {
                        let index = 0f64;
                        {
                            let source_ml_source2_s65 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(texts, starts, counts, labels, targets, declarations, brands, &(exported), found, index);
                            match source_ml_source2_s65 {
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found_2) => {
                                    if found_2 {
                                        {
                                            let source_ml_source1_temporary_index8_2 = source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index8 = source_ml_source1_temporary_index8_2;
                                                continue;
                                            }
                                        }
                                    }
{
                                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(true)
                                    }
                                }
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result10) => {
                                    self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result10)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result {
    MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done,
    MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(self::ml_string::Text),
}
