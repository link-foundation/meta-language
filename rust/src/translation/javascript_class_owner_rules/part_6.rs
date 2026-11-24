// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result;
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result;
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result;
use super::contains_java_script_closed_class_brand;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3;
use super::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4;
use super::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5;

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
                                            let source_ml_source2_concat_value38 = seen;
                                            {
                                                let source_ml_source2_concat_value39 = vec![node];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value38,
                                                    ),
                                                    source_ml_source2_concat_value39,
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
                                                                        let source_ml_source2_s40 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts, starts, counts, labels, targets, brands, pending_2, fields, at);
                                                                        match source_ml_source2_s40 {
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
                                                                    let source_ml_source2_concat_value41 =
                                                                        pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value42 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value41), source_ml_source2_concat_value42)
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
                                                                    let source_ml_source2_concat_value43 = pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value44 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![107, 101, 121]))), self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value43), source_ml_source2_concat_value44)
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
                                                                                            let source_ml_source2_concat_value45 = pending_6;
                                                                                            {
                                                                                                let source_ml_source2_concat_value46 = vec![self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![114, 101, 116])))];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value45), source_ml_source2_concat_value46)
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
                                                                                        let source_ml_source2_s47 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts, starts, counts, labels, targets, declarations, pending_2, name, found, at_3);
                                                                                        match source_ml_source2_s47 {
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
                                                        let source_ml_source2_s48 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(texts, starts, counts, labels, targets, declarations, brands, params, at);
                                                        match source_ml_source2_s48 {
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
                            let source_ml_source2_s49 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(texts, starts, counts, labels, targets, declarations, brands, &(exported), found, index);
                            match source_ml_source2_s49 {
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
                            let source_ml_source2_concat_value50 = result;
                            {
                                let source_ml_source2_concat_value51 =
                                    vec![self::dependency_function2(texts, value)];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value50,
                                    ),
                                    source_ml_source2_concat_value51,
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
