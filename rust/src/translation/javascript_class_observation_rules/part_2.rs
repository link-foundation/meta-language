// Generated from complete js/src/translation/javascript-class-observation-rules.js. Do not edit by hand.
use super::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result;
use super::dependency_function1;
use super::dependency_function2;
use super::ml;
use super::ml_array;
use super::read_collection_construction_binding;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2;
use super::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_construction(
    target: &str,
    kind: &str,
    key_kind: &str,
    values: &str,
    collection_type: &str,
    element_type: &str,
    comparison: &str,
) -> String {
    {
        let set = kind == "set";
        if target == "JavaScript" {
            format!(
                "new {}({})",
                if set {
                    String::from("Set")
                } else {
                    String::from("Map")
                },
                values.to_owned()
            )
        } else if target == "Rust" {
            {
                let comparator = if key_kind == "float" {
                    String::from("|a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())")
                } else {
                    String::from("|a, b| a == b")
                };
                {
                    let values_3 = if key_kind == "float" {
                        {
                            format!(
                                "({}).into_iter().map(|ml_value| {}).collect::<Vec<_>>()",
                                values.to_owned(),
                                if set {
                                    String::from("if ml_value == 0f64 { 0f64 } else { ml_value }")
                                } else {
                                    String::from(
                                        "(if ml_value.0 == 0f64 { 0f64 } else { ml_value.0 }, ml_value.1)",
                                    )
                                }
                            )
                        }
                    } else {
                        values.to_owned()
                    };
                    format!(
                        "crate::ml_array::{}({values_3}, {comparator})",
                        kind.to_owned()
                    )
                }
            }
        } else {
            {
                let value = self::read_collection_construction_binding(target, kind, "value");
                {
                    let previous =
                        self::read_collection_construction_binding(target, kind, "previous");
                    if target == "Lean" {
                        {
                            let equal = if key_kind == "float" {
                                format!(
                                    "({previous} == {value} || ({previous}.isNaN && {value}.isNaN))"
                                )
                            } else {
                                format!("({previous} == {value})")
                            };
                            {
                                let normalized = if key_kind == "float" {
                                    if set {
                                        String::from(
                                            "(if ml_original == (0 : Float) then (0 : Float) else ml_original)",
                                        )
                                    } else {
                                        String::from(
                                            "((if ml_original.1 == (0 : Float) then (0 : Float) else ml_original.1), ml_original.2)",
                                        )
                                    }
                                } else {
                                    String::from("ml_original")
                                };
                                {
                                    let existing = if set {
                                        String::from("ml_accumulator")
                                    } else {
                                        format!(
                                            "(ml_accumulator.map (fun ml_previous => if {equal} then (ml_previous.1, ml_value.2) else ml_previous))"
                                        )
                                    };
                                    format!(
                                        "({}.foldl (fun ml_accumulator ml_original => let ml_value := {normalized}; if ml_accumulator.any (fun ml_previous => {equal}) then {existing} else ml_accumulator.push ml_value) (#[] : {}))",
                                        values.to_owned(),
                                        collection_type.to_owned()
                                    )
                                }
                            }
                        }
                    } else {
                        {
                            let equal_2 = comparison.to_owned();
                            {
                                let equal_4 = if key_kind == "float" {
                                    {
                                        format!(
                                            "(orb {equal_2} (andb (PrimFloat.is_nan {previous}) (PrimFloat.is_nan {value})))"
                                        )
                                    }
                                } else {
                                    equal_2
                                };
                                {
                                    let normalized_2 = if key_kind == "float" {
                                        if set {
                                            String::from(
                                                "(if PrimFloat.eqb ml_original 0%float then 0%float else ml_original)",
                                            )
                                        } else {
                                            String::from(
                                                "((if PrimFloat.eqb (fst ml_original) 0%float then 0%float else fst ml_original), snd ml_original)",
                                            )
                                        }
                                    } else {
                                        String::from("ml_original")
                                    };
                                    {
                                        let existing_2 = if set {
                                            String::from("ml_accumulator")
                                        } else {
                                            format!(
                                                "(List.map (fun ml_previous => if {equal_4} then (fst ml_previous, snd ml_value) else ml_previous) ml_accumulator)"
                                            )
                                        };
                                        format!(
                                            "(List.fold_left (fun ml_accumulator ml_original => let ml_value := {normalized_2} in if List.existsb (fun ml_previous => {equal_4}) ml_accumulator then {existing_2} else List.app ml_accumulator (ml_value :: nil)) {} (@nil {}))",
                                            values.to_owned(),
                                            element_type.to_owned()
                                        )
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_observation_operands(kind: &str, proof: &str) -> String {
    if self::ml_array::includes(
        &vec![
            String::from("eq"),
            String::from("ne"),
            String::from("lt"),
            String::from("le"),
            String::from("gt"),
            String::from("ge"),
        ],
        &proof.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("left right")
    } else if proof == "forall" {
        String::from("binders")
    } else if (kind == "toString") || (kind == "optionalText") {
        String::from("arg")
    } else if kind == "strictEqual" {
        String::from("left right")
    } else if (kind == "print") || (kind == "output") {
        String::from("expr")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_observation_operation_operands(
    kind: &str,
    proof: &str,
    operation: &str,
) -> String {
    if (kind == "binary")
        && self::ml_array::includes(
            &vec![String::from("eq"), String::from("ne")],
            &operation.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from("left right")
    } else {
        self::read_java_script_closed_class_observation_operands(kind, proof)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn contains_java_script_closed_class_brand(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    root: f64,
    declarations: f64,
    brands: &[String],
) -> bool {
    if (brands.len() as f64) == 0f64 {
        false
    } else {
        {
            let pending = vec![root];
            {
                let seen = Vec::<f64>::new();
                {
                    let source_ml_source2_s2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(texts, starts, counts, labels, targets, declarations, brands, pending, seen);
                    match source_ml_source2_s2 {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done => {
                            false
                        }
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn has_java_script_closed_class_exported_brand(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    declarations: f64,
    exports: &[String],
    brands: &[String],
) -> bool {
    if (exports.len() as f64) == 0f64 {
        false
    } else if declarations < 0f64 {
        true
    } else {
        {
            let source_ml_source1_temporary_values8 = exports.to_vec();
            {
                let source_ml_source1_temporary_index8 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s3 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(texts, starts, counts, labels, targets, declarations, brands, &(source_ml_source1_temporary_values8), source_ml_source1_temporary_index8);
                    match source_ml_source2_s3 {
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done => {
                                false
                            }
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result9) => {
                                source_ml_source1_temporary_result9
                            }
                        }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction1Loop1Result {
    MlSource1TemporaryDependencyFunction1Loop1Done,
    MlSource1TemporaryDependencyFunction1Loop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function1_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryDependencyFunction1Loop1Result {
    loop {
        return if at < (start + count) {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Return(self::read_required_number_fact(targets, at))
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
            self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Done
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
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    brands: &[String],
    mut pending: Vec<f64>,
    mut fields: f64,
    mut at: f64,
) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    loop {
        return if at < (self::read_required_number_fact(counts, fields)) {
            {
                let row = self::dependency_function2(starts, counts, targets, fields, at);
                {
                    let name =
                        self::dependency_function1(starts, counts, labels, targets, row, "name");
                    if (name < 0f64)
                        || (self::ml_array::includes(
                            brands,
                            &self::read_required_text_fact(texts, name),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))
                    {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                    } else {
                        {
                            let pending_2 = {
                                let source_ml_source2_concat_value4 = pending;
                                {
                                    let source_ml_source2_concat_value5 =
                                        vec![self::dependency_function1(
                                            starts, counts, labels, targets, row, "type",
                                        )];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value4,
                                        ),
                                        source_ml_source2_concat_value5,
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
        if at < (self::read_required_number_fact(counts, children)) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value6 = pending;
                    {
                        let source_ml_source2_concat_value7 = vec![self::dependency_function2(
                            starts, counts, targets, children, at,
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value6,
                            ),
                            source_ml_source2_concat_value7,
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
                    let source_ml_source2_concat_value8 = pending;
                    {
                        let source_ml_source2_concat_value9 = vec![self::dependency_function1(
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
                                source_ml_source2_concat_value8,
                            ),
                            source_ml_source2_concat_value9,
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
                                            let source_ml_source2_s10 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts, counts, labels, targets, pending, constructors, ctor);
                                            match source_ml_source2_s10 {
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
