// Generated from complete js/src/translation/javascript-class-observation-rules.js. Do not edit by hand.
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result;
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result;
use super::dependency_function1;
use super::dependency_function2;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3;

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
        if at < (self::read_required_number_fact(counts, children)) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value5 = pending;
                    {
                        let source_ml_source2_concat_value6 = vec![self::dependency_function2(
                            starts, counts, targets, children, at,
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value5,
                            ),
                            source_ml_source2_concat_value6,
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
    labels: &[String],
    targets: &[f64],
    mut pending: Vec<f64>,
    mut fields: f64,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (self::read_required_number_fact(counts, fields)) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value7 = pending;
                    {
                        let source_ml_source2_concat_value8 = vec![self::dependency_function1(
                            starts,
                            counts,
                            labels,
                            targets,
                            self::dependency_function2(starts, counts, targets, fields, index),
                            "type",
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value7,
                            ),
                            source_ml_source2_concat_value8,
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
    labels: &[String],
    targets: &[f64],
    mut pending: Vec<f64>,
    mut constructors: f64,
    mut ctor: f64,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    loop {
        return if ctor < (self::read_required_number_fact(counts, constructors)) {
            {
                let fields = self::dependency_function1(
                    starts,
                    counts,
                    labels,
                    targets,
                    self::dependency_function2(starts, counts, targets, constructors, ctor),
                    "fields",
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
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut declarations: f64,
    mut pending: Vec<f64>,
    mut name: f64,
    mut found: bool,
    mut at: f64,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    loop {
        return if at < (self::read_required_number_fact(counts, declarations)) {
            {
                let row = self::dependency_function2(starts, counts, targets, declarations, at);
                {
                    let full_name = self::dependency_function1(
                        starts, counts, labels, targets, row, "fullName",
                    );
                    if (full_name < 0f64)
                        || ((self::read_required_text_fact(texts, full_name))
                            != (self::read_required_text_fact(texts, name)))
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
                            let constructors = self::dependency_function1(
                                starts, counts, labels, targets, row, "ctors",
                            );
                            if constructors < 0f64 {
                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(true)
                            } else {
                                {
                                    let found_2 = true;
                                    {
                                        let ctor = 0f64;
                                        {
                                            let source_ml_source2_s9 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts, counts, labels, targets, pending, constructors, ctor);
                                            match source_ml_source2_s9 {
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
                                            let source_ml_source2_concat_value10 = seen;
                                            {
                                                let source_ml_source2_concat_value11 = vec![node];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value10,
                                                    ),
                                                    source_ml_source2_concat_value11,
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
                                                                        let source_ml_source2_s12 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts, starts, counts, labels, targets, brands, pending_2, fields, at);
                                                                        match source_ml_source2_s12 {
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
                                                                    let source_ml_source2_concat_value13 =
                                                                        pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value14 = vec![self::dependency_function1(starts, counts, labels, targets, node, "element")];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value13), source_ml_source2_concat_value14)
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
                                                                        let source_ml_source2_concat_value15 =
                                                                            pending_2;
                                                                        {
                                                                            let source_ml_source2_concat_value16 = vec![self::dependency_function1(starts, counts, labels, targets, node, "key"), self::dependency_function1(starts, counts, labels, targets, node, "value")];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value15), source_ml_source2_concat_value16)
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
                                                                                                let source_ml_source2_concat_value17 = pending_6;
                                                                                                {
                                                                                                    let source_ml_source2_concat_value18 = vec![self::dependency_function1(starts, counts, labels, targets, node, "ret")];
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value17), source_ml_source2_concat_value18)
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
                                                                                        let source_ml_source2_s19 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts, starts, counts, labels, targets, declarations, pending_2, name, found, at_3);
                                                                                        match source_ml_source2_s19 {
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
