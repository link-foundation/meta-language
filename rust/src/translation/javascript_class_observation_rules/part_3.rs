// Generated from complete js/src/translation/javascript-class-observation-rules.js. Do not edit by hand.
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result;
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result;
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result;
use super::contains_java_script_closed_class_brand;
use super::dependency_function1;
use super::dependency_function2;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;
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
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut declarations: f64,
    brands: &[String],
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    loop {
        return {
            let ml_library_condition_1 = {
                let ml_value = pending.len() as f64;
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_1 {
                if ((seen.len() as f64) >= 100_000f64) || ((pending.len() as f64) > 100_000f64) {
                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                } else {
                    {
                        let node = self::read_required_number_fact(
                            &(pending),
                            ((pending.len() as f64) - 1f64),
                        );
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
                                            let source_ml_source2_concat_value11 = seen;
                                            {
                                                let source_ml_source2_concat_value12 = vec![node];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value11,
                                                    ),
                                                    source_ml_source2_concat_value12,
                                                )
                                            }
                                        };
                                        {
                                            let owner = self::dependency_function1(
                                                starts, counts, labels, targets, node, "kind",
                                            );
                                            if owner < 0f64 {
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                            } else {
                                                {
                                                    let kind =
                                                        self::read_required_text_fact(texts, owner);
                                                    if kind == "record" {
                                                        {
                                                            let fields = self::dependency_function1(
                                                                starts, counts, labels, targets,
                                                                node, "fields",
                                                            );
                                                            if fields < 0f64 {
                                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                            } else {
                                                                {
                                                                    let at = 0f64;
                                                                    {
                                                                        let source_ml_source2_s13 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts, starts, counts, labels, targets, brands, pending_2, fields, at);
                                                                        match source_ml_source2_s13 {
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
                                                                String::from("array"),
                                                                String::from("option"),
                                                                String::from("set"),
                                                                String::from("cell"),
                                                            ],
                                                            &kind.clone(),
                                                            0f64,
                                                            false,
                                                            |a, b| a == b,
                                                        ) {
                                                            {
                                                                let pending_4 = {
                                                                    let source_ml_source2_concat_value14 =
                                                                        pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value15 = vec![self::dependency_function1(starts, counts, labels, targets, node, "element")];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value14), source_ml_source2_concat_value15)
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
                                                            if kind == "map" {
                                                                {
                                                                    let pending_5 = {
                                                                        let source_ml_source2_concat_value16 =
                                                                            pending_2;
                                                                        {
                                                                            let source_ml_source2_concat_value17 = vec![self::dependency_function1(starts, counts, labels, targets, node, "key"), self::dependency_function1(starts, counts, labels, targets, node, "value")];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value16), source_ml_source2_concat_value17)
                                                                        }
                                                                    };
                                                                    {
                                                                        (pending, seen) =
                                                                            (pending_5, seen_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            {
                                                                if ((kind == "tuple")
                                                                    || (kind == "union"))
                                                                    || (kind == "function")
                                                                {
                                                                    {
                                                                        let children = self::dependency_function1(starts, counts, labels, targets, node, &(if kind == "tuple" {
                                                                        String::from("elements")
                                                                    } else if kind == "union" {
                                                                            String::from("alternatives")
                                                                        } else {
                                                                            String::from("params")
                                                                        }));
                                                                        if children < 0f64 {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                        } else {
                                                                            {
                                                                                let at_2 = 0f64;
                                                                                {
                                                                                    let pending_6 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(starts, counts, targets, pending_2, children, at_2);
                                                                                    if kind
                                                                                        == "function"
                                                                                    {
                                                                                        {
                                                                                            let pending_7 = {
                                                                                                let source_ml_source2_concat_value18 = pending_6;
                                                                                                {
                                                                                                    let source_ml_source2_concat_value19 = vec![self::dependency_function1(starts, counts, labels, targets, node, "ret")];
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value18), source_ml_source2_concat_value19)
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
                                                                } else if kind == "data" {
                                                                    {
                                                                        let name = self::dependency_function1(starts, counts, labels, targets, node, "name");
                                                                        if (name < 0f64)
                                                                            || (declarations < 0f64)
                                                                        {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                        } else {
                                                                            {
                                                                                let found = false;
                                                                                {
                                                                                    let at_3 = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s20 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts, starts, counts, labels, targets, declarations, pending_2, name, found, at_3);
                                                                                        match source_ml_source2_s20 {
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
                                                                    if (self::ml_array::includes(
                                                                        &vec![
                                                                            String::from("unit"),
                                                                            String::from("bool"),
                                                                            String::from("float"),
                                                                            String::from("int"),
                                                                            String::from("nat"),
                                                                            String::from("fixed"),
                                                                            String::from("string"),
                                                                        ],
                                                                        &kind.clone(),
                                                                        0f64,
                                                                        false,
                                                                        |a, b| a == b,
                                                                    )) {
                                                                        {
                                                                            (pending, seen) =
                                                                                (pending_2, seen_2);
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
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut declarations: f64,
    brands: &[String],
    mut params: f64,
    mut at: f64,
) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    loop {
        return if at < (self::read_required_number_fact(counts, params)) {
            {
                let parameter = self::dependency_function2(starts, counts, targets, params, at);
                {
                    let type_ = self::dependency_function1(
                        starts, counts, labels, targets, parameter, "type",
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
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut declarations: f64,
    brands: &[String],
    exported: &str,
    mut found: bool,
    mut index: f64,
) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    loop {
        return if index < (self::read_required_number_fact(counts, declarations)) {
            {
                let declaration =
                    self::dependency_function2(starts, counts, targets, declarations, index);
                {
                    let name = self::dependency_function1(
                        starts,
                        counts,
                        labels,
                        targets,
                        declaration,
                        "name",
                    );
                    if (name < 0f64) || ((self::read_required_text_fact(texts, name)) != exported) {
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
                            let kind = self::dependency_function1(
                                starts,
                                counts,
                                labels,
                                targets,
                                declaration,
                                "k",
                            );
                            if (kind < 0f64)
                                || ((self::read_required_text_fact(texts, kind)) != "fn")
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
                                    let params = self::dependency_function1(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        declaration,
                                        "params",
                                    );
                                    {
                                        let ret = self::dependency_function1(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            declaration,
                                            "ret",
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
                                                        let source_ml_source2_s21 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(texts, starts, counts, labels, targets, declarations, brands, params, at);
                                                        match source_ml_source2_s21 {
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
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut declarations: f64,
    brands: &[String],
    source_ml_source1_temporary_values8: &[String],
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
                            let source_ml_source2_s22 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(texts, starts, counts, labels, targets, declarations, brands, &(exported), found, index);
                            match source_ml_source2_s22 {
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
