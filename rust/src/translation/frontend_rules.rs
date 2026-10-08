// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
// Translated from JavaScript by meta-language: portable core, Rust target.
// JavaScript Number equality and separate arithmetic rounding are preserved.
// Array index casts are guarded by the generated runtime.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::suboptimal_flops,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnicodeEscape {
    Scalar(f64, f64),
    Malformed,
    Unsupported(f64, f64),
}

pub fn decode_unicode_escape(
    units: Vec<f64>,
    allow_fixed: bool,
) -> crate::translation::frontend_rules::UnicodeEscape {
    {
        let index = 2f64;
        {
            let code = 0f64;
            {
                let digits = 0f64;
                {
                    let braced = false;
                    {
                        let ml_s1 = if (((units.len() as f64) > (2f64))
                            && ((crate::translation::frontend_rules::ml_array::at(
                                &units,
                                crate::translation::frontend_rules::ml_array::number_index(2f64),
                            )) == (123f64)))
                        {
                            {
                                let braced_2 = true;
                                {
                                    let index_2 = 3f64;
                                    crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin1::MlDecodeUnicodeEscapeJoin1Next(index_2, braced_2)
                                }
                            }
                        } else if allow_fixed {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin1::MlDecodeUnicodeEscapeJoin1Next(index, braced)
                        } else {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin1::MlDecodeUnicodeEscapeJoin1Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
                        };
                        match ml_s1 {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin1::MlDecodeUnicodeEscapeJoin1Next(index_3, braced_3) => {
                                {
                                    let ml_s2 = crate::translation::frontend_rules::ml_decode_unicode_escape_loop2(units.clone(), index_3, code, digits, braced_3);
                                    match ml_s2 {
                                        crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop2Result::MlDecodeUnicodeEscapeLoop2Done(index_4, code_2, digits_2) => {
                                            {
                                                let ml_s3 = if braced_3 {
                                                    if (((digits_2 == (0f64)) || (index_4 >= (units.len() as f64))) || ((crate::translation::frontend_rules::ml_array::at(&units, crate::translation::frontend_rules::ml_array::number_index(index_4))) != (125f64))) {
                                                        crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin3::MlDecodeUnicodeEscapeJoin3Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
                                                    } else {
                                                        {
                                                            let index_5 = (index_4 + 1f64);
                                                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin3::MlDecodeUnicodeEscapeJoin3Next(index_5)
                                                        }
                                                    }
                                                } else {
                                                    if digits_2 == (4f64) {
                                                        crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin3::MlDecodeUnicodeEscapeJoin3Next(index_4)
                                                    } else {
                                                        crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin3::MlDecodeUnicodeEscapeJoin3Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
                                                    }
                                                };
                                                match ml_s3 {
                                                    crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin3::MlDecodeUnicodeEscapeJoin3Next(index_6) => {
                                                        {
                                                            let escape_length = index_6;
                                                            {
                                                                let ml_s5 = if (((((!braced_3 && (code_2 >= (55296f64))) && (code_2 <= (56319f64))) && ((units.len() as f64) >= (index_6 + 6f64))) && ((crate::translation::frontend_rules::ml_array::at(&units, crate::translation::frontend_rules::ml_array::number_index(index_6))) == (92f64))) && ((crate::translation::frontend_rules::ml_array::at(&units, crate::translation::frontend_rules::ml_array::number_index(index_6 + 1f64))) == (117f64))) {
                                                                    {
                                                                        let low = 0f64;
                                                                        {
                                                                            let offset = 2f64;
                                                                            {
                                                                                let ml_s6 = crate::translation::frontend_rules::ml_decode_unicode_escape_loop5(units, index_6, low, offset);
                                                                                match ml_s6 {
                                                                                    crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop5Result::MlDecodeUnicodeEscapeLoop5Done(low_2, offset_2) => {
                                                                                        if (((offset_2 == (6f64)) && (low_2 >= (56320f64))) && (low_2 <= (57343f64))) {
                                                                                            {
                                                                                                let code_3 = (((65536f64 + ((code_2 - 55296f64) * 1024f64)) + low_2) - 56320f64);
                                                                                                {
                                                                                                    let index_7 = (index_6 + 6f64);
                                                                                                    crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin4::MlDecodeUnicodeEscapeJoin4Next(index_7, code_3)
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin4::MlDecodeUnicodeEscapeJoin4Next(index_6, code_2)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin4::MlDecodeUnicodeEscapeJoin4Next(index_6, code_2)
                                                                };
                                                                match ml_s5 {
                                                                    crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin4::MlDecodeUnicodeEscapeJoin4Next(index_8, code_4) => {
                                                                        if ((code_4 > (1_114_111_f64)) || (55296f64..=57343f64).contains(&code_4)) {
                                                                            crate::translation::frontend_rules::UnicodeEscape::Unsupported(index_8, escape_length)
                                                                        } else {
                                                                            crate::translation::frontend_rules::UnicodeEscape::Scalar(code_4, index_8)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin3::MlDecodeUnicodeEscapeJoin3Return(ml_result5) => {

                                                        *ml_result5
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeJoin1::MlDecodeUnicodeEscapeJoin1Return(ml_result5_2) => {

                                *ml_result5_2
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_argument_count(defaults: Vec<bool>, count: f64) -> bool {
    if (count > (defaults.len() as f64)) {
        false
    } else {
        {
            let index = count;
            {
                let ml_s7 = crate::translation::frontend_rules::ml_accept_argument_count_loop6(
                    defaults, index,
                );
                match ml_s7 {
                    crate::translation::frontend_rules::MlAcceptArgumentCountLoop6Result::MlAcceptArgumentCountLoop6Done => {
                        true
                    }
                    crate::translation::frontend_rules::MlAcceptArgumentCountLoop6Result::MlAcceptArgumentCountLoop6Return(ml_result6) => {
                        ml_result6
                    }
                }
            }
        }
    }
}

pub fn find_default_parameter_reference(
    kinds: Vec<String>,
    values: Vec<String>,
    parameters: Vec<String>,
) -> f64 {
    {
        let at = 0f64;
        {
            let depth = 0f64;
            {
                let ml_s8 =
                    crate::translation::frontend_rules::ml_find_default_parameter_reference_loop7(
                        kinds, values, parameters, at, depth,
                    );
                match ml_s8 {
                    crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop7Result::MlFindDefaultParameterReferenceLoop7Done => {
                        (-1f64)
                    }
                    crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop7Result::MlFindDefaultParameterReferenceLoop7Return(ml_result7) => {
                        ml_result7
                    }
                }
            }
        }
    }
}

pub fn start_regular_expression(kind: &str, value: &str) -> bool {
    if (kind.is_empty()) {
        true
    } else if (kind == "identifier") {
        ((((((((((value == "return") || (value == "throw")) || (value == "case"))
            || (value == "delete"))
            || (value == "void"))
            || (value == "typeof"))
            || (value == "yield"))
            || (value == "await"))
            || (value == "in"))
            || (value == "of"))
    } else if kind == "punct" {
        (((((((value != ")") && (value != "]")) && (value != "}")) && (value != "."))
            && (value != "?."))
            && (value != "++"))
            && (value != "--"))
    } else {
        false
    }
}

pub fn regular_expression_end(units: &[f64], start: f64) -> f64 {
    {
        let index = (start + 1f64);
        {
            let escaped = false;
            {
                let character_class = false;
                {
                    let ml_s9 =
                        crate::translation::frontend_rules::ml_regular_expression_end_loop10(
                            units.to_vec(),
                            index,
                            escaped,
                            character_class,
                        );
                    match ml_s9 {
                        crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result::MlRegularExpressionEndLoop10Done => {
                            (-1f64)
                        }
                        crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result::MlRegularExpressionEndLoop10Return(ml_result10) => {
                            ml_result10
                        }
                    }
                }
            }
        }
    }
}

pub fn find_binding_run_end(terms: &[String], start: f64) -> f64 {
    {
        let end = start;
        {
            let end_2 = crate::translation::frontend_rules::ml_find_binding_run_end_loop13(
                terms.to_vec(),
                end,
            );
            if (end_2 == start) {
                (start + 1f64)
            } else {
                end_2
            }
        }
    }
}

pub fn accept_binding_scope(statuses: &[String], reasons: &[String]) -> bool {
    if ((statuses.len() as f64) < (2f64)) {
        false
    } else {
        {
            let index = 0f64;
            {
                let ml_s10 = crate::translation::frontend_rules::ml_accept_binding_scope_loop14(
                    statuses.to_vec(),
                    reasons.to_vec(),
                    index,
                );
                match ml_s10 {
                    crate::translation::frontend_rules::MlAcceptBindingScopeLoop14Result::MlAcceptBindingScopeLoop14Done => {
                        false
                    }
                    crate::translation::frontend_rules::MlAcceptBindingScopeLoop14Result::MlAcceptBindingScopeLoop14Return(ml_result14) => {
                        ml_result14
                    }
                }
            }
        }
    }
}

pub fn accept_literal_binding(
    value_kind: &str,
    constant: bool,
    effect_count: f64,
    declaration_count: f64,
) -> bool {
    (((constant && (effect_count == (1f64))) && (declaration_count == (0f64)))
        && (((value_kind == "num") || (value_kind == "bool")) || (value_kind == "str")))
}

pub fn constant_binding_form(value_kind: &str, type_kind: &str) -> String {
    if ((value_kind == "lit")
        && (((type_kind == "float") || (type_kind == "bool")) || (type_kind == "fixed")))
    {
        String::from("scalar")
    } else if ((value_kind == "lit") && (type_kind == "string")) {
        String::from("string")
    } else {
        String::from("lazy")
    }
}

pub fn accept_constant_emission(
    constant: bool,
    effects: f64,
    declarations: f64,
    legal_name: bool,
) -> bool {
    (((constant && (effects == (1f64))) && (declarations == (0f64))) && legal_name)
}

pub fn render_constant_binding(form: &str, name: &str, type_: &str, value: &str) -> String {
    if (form == "lazy") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!("{}{}", (String::from("pub static ")), name)),
                            (String::from(": std::sync::LazyLock<"))
                        )),
                        type_
                    )),
                    (String::from("> = std::sync::LazyLock::new(|| "))
                )),
                value
            )),
            (String::from(");"))
        )
    } else {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!("{}{}", (String::from("pub const ")), name)),
                            (String::from(": "))
                        )),
                        type_
                    )),
                    (String::from(" = "))
                )),
                value
            )),
            (String::from(";"))
        )
    }
}

pub fn accept_module_binding_scope(terms: &[String], carried: bool) -> bool {
    if carried {
        {
            let declarations = 0f64;
            {
                let index = 0f64;
                {
                    let ml_s11 =
                        crate::translation::frontend_rules::ml_accept_module_binding_scope_loop15(
                            terms.to_vec(),
                            declarations,
                            index,
                        );
                    match ml_s11 {
                        crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop15Result::MlAcceptModuleBindingScopeLoop15Done(declarations_2) => {
                            (declarations_2 > (0f64))
                        }
                        crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop15Result::MlAcceptModuleBindingScopeLoop15Return(ml_result15) => {
                            ml_result15
                        }
                    }
                }
            }
        }
    } else {
        false
    }
}

pub fn find_documentation_parameter_range(units: &[f64]) -> Vec<f64> {
    {
        let start = 0f64;
        {
            let start_2 =
                crate::translation::frontend_rules::ml_find_documentation_parameter_range_loop16(
                    units.to_vec(),
                    start,
                );
            {
                let optional = false;
                {
                    let ml_s12 = if ((start_2 < (units.len() as f64))
                        && ((crate::translation::frontend_rules::ml_array::at(
                            units,
                            crate::translation::frontend_rules::ml_array::number_index(start_2),
                        )) == (91f64)))
                    {
                        {
                            let optional_2 = true;
                            {
                                let start_3 = (start_2 + 1f64);
                                crate::translation::frontend_rules::MlFindDocumentationParameterRangeJoin17::MlFindDocumentationParameterRangeJoin17Next(start_3, optional_2)
                            }
                        }
                    } else {
                        crate::translation::frontend_rules::MlFindDocumentationParameterRangeJoin17::MlFindDocumentationParameterRangeJoin17Next(start_2, optional)
                    };
                    match ml_s12 {
                        crate::translation::frontend_rules::MlFindDocumentationParameterRangeJoin17::MlFindDocumentationParameterRangeJoin17Next(start_4, optional_3) => {
                            {
                                let end = start_4;
                                {
                                    let end_2 = crate::translation::frontend_rules::ml_find_documentation_parameter_range_loop18(units.to_vec(), start_4, end);
                                    if (end_2 == start_4) {
                                        Vec::<f64>::new()
                                    } else {
                                        {
                                            let ml_s13 = if optional_3 {
                                                {
                                                    let after = end_2;
                                                    {
                                                        let after_2 = crate::translation::frontend_rules::ml_find_documentation_parameter_range_loop20(units.to_vec(), after);
                                                        if ((after_2 >= (units.len() as f64)) || (((crate::translation::frontend_rules::ml_array::at(units, crate::translation::frontend_rules::ml_array::number_index(after_2))) != (93f64)) && ((crate::translation::frontend_rules::ml_array::at(units, crate::translation::frontend_rules::ml_array::number_index(after_2))) != (61f64)))) {
                                                            crate::translation::frontend_rules::MlFindDocumentationParameterRangeJoin19::MlFindDocumentationParameterRangeJoin19Return(Vec::<f64>::new())
                                                        } else {
                                                            crate::translation::frontend_rules::MlFindDocumentationParameterRangeJoin19::MlFindDocumentationParameterRangeJoin19Next
                                                        }
                                                    }
                                                }
                                            } else {
                                                crate::translation::frontend_rules::MlFindDocumentationParameterRangeJoin19::MlFindDocumentationParameterRangeJoin19Next
                                            };
                                            match ml_s13 {
                                                crate::translation::frontend_rules::MlFindDocumentationParameterRangeJoin19::MlFindDocumentationParameterRangeJoin19Next => {
                                                    vec![start_4, end_2]
                                                }
                                                crate::translation::frontend_rules::MlFindDocumentationParameterRangeJoin19::MlFindDocumentationParameterRangeJoin19Return(ml_result19) => {
                                                    ml_result19
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

pub fn accept_type_query_operand(kind: &str) -> bool {
    ((((((kind == "var") || (kind == "lit")) || (kind == "name")) || (kind == "num"))
        || (kind == "bool"))
        || (kind == "str"))
}

pub fn read_type_query_result(kind: &str) -> String {
    if kind == "float" {
        String::from("number")
    } else if kind == "nat" || kind == "int" {
        String::from("bigint")
    } else if kind == "bool" {
        String::from("boolean")
    } else if kind == "string" {
        String::from("string")
    } else if kind == "array" || kind == "data" {
        String::from("object")
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin4 {
    MlDecodeUnicodeEscapeJoin4Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeLoop5Result {
    MlDecodeUnicodeEscapeLoop5Done(f64, f64),
}

pub fn ml_decode_unicode_escape_loop5(
    mut units: Vec<f64>,
    mut index: f64,
    mut low: f64,
    mut offset: f64,
) -> crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop5Result {
    loop {
        return if (offset < (6f64)) {
            {
                let unit = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index + offset),
                );
                {
                    let digit = (-1f64);
                    {
                        let digit_5 = if (48f64..=57f64).contains(&unit) {
                            { (unit - 48f64) }
                        } else if (65f64..=70f64).contains(&unit) {
                            { (unit - 55f64) }
                        } else if (97f64..=102f64).contains(&unit) {
                            { (unit - 87f64) }
                        } else {
                            digit
                        };
                        if (digit_5 < (0f64)) {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop5Result::MlDecodeUnicodeEscapeLoop5Done(low, offset)
                        } else {
                            {
                                let low_2 = ((low * 16f64) + digit_5);
                                {
                                    let offset_2 = (offset + 1f64);
                                    {
                                        (low, offset) = (low_2, offset_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop5Result::MlDecodeUnicodeEscapeLoop5Done(low, offset)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin3 {
    MlDecodeUnicodeEscapeJoin3Next(f64),
    MlDecodeUnicodeEscapeJoin3Return(Box<crate::translation::frontend_rules::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeLoop2Result {
    MlDecodeUnicodeEscapeLoop2Done(f64, f64, f64),
}

pub fn ml_decode_unicode_escape_loop2(
    mut units: Vec<f64>,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
    mut braced: bool,
) -> crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop2Result {
    loop {
        return if ((index < (units.len() as f64)) && (digits < (if braced { 6f64 } else { 4f64 })))
        {
            {
                let unit = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                {
                    let digit = (-1f64);
                    {
                        let digit_5 = if (48f64..=57f64).contains(&unit) {
                            { (unit - 48f64) }
                        } else if (65f64..=70f64).contains(&unit) {
                            { (unit - 55f64) }
                        } else if (97f64..=102f64).contains(&unit) {
                            { (unit - 87f64) }
                        } else {
                            digit
                        };
                        if (digit_5 < (0f64)) {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop2Result::MlDecodeUnicodeEscapeLoop2Done(index, code, digits)
                        } else {
                            {
                                let code_2 = ((code * 16f64) + digit_5);
                                {
                                    let digits_2 = (digits + 1f64);
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            (index, code, digits) = (index_2, code_2, digits_2);
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
            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop2Result::MlDecodeUnicodeEscapeLoop2Done(index, code, digits)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin1 {
    MlDecodeUnicodeEscapeJoin1Next(f64, bool),
    MlDecodeUnicodeEscapeJoin1Return(Box<crate::translation::frontend_rules::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptArgumentCountLoop6Result {
    MlAcceptArgumentCountLoop6Done,
    MlAcceptArgumentCountLoop6Return(bool),
}

pub fn ml_accept_argument_count_loop6(
    mut defaults: Vec<bool>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptArgumentCountLoop6Result {
    loop {
        return if (index < (defaults.len() as f64)) {
            if (crate::translation::frontend_rules::ml_array::at(
                &defaults,
                crate::translation::frontend_rules::ml_array::number_index(index),
            )) {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            crate::translation::frontend_rules::MlAcceptArgumentCountLoop6Result::MlAcceptArgumentCountLoop6Return(false)
        } else {
            crate::translation::frontend_rules::MlAcceptArgumentCountLoop6Result::MlAcceptArgumentCountLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceLoop7Result {
    MlFindDefaultParameterReferenceLoop7Done,
    MlFindDefaultParameterReferenceLoop7Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceJoin8 {
    MlFindDefaultParameterReferenceJoin8Next,
    MlFindDefaultParameterReferenceJoin8Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceLoop9Result {
    MlFindDefaultParameterReferenceLoop9Done,
    MlFindDefaultParameterReferenceLoop9Return(f64),
}

pub fn ml_find_default_parameter_reference_loop9(
    mut parameters: Vec<String>,
    mut at: f64,
    mut value: String,
    mut parameter_index: f64,
) -> crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result {
    loop {
        return if (parameter_index < (parameters.len() as f64)) {
            if ((crate::translation::frontend_rules::ml_array::at(
                &parameters,
                crate::translation::frontend_rules::ml_array::number_index(parameter_index),
            )) == value)
            {
                crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result::MlFindDefaultParameterReferenceLoop9Return(at)
            } else {
                {
                    let parameter_index_2 = (parameter_index + 1f64);
                    {
                        parameter_index = parameter_index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result::MlFindDefaultParameterReferenceLoop9Done
        };
    }
}

pub fn ml_find_default_parameter_reference_loop7(
    mut kinds: Vec<String>,
    mut values: Vec<String>,
    mut parameters: Vec<String>,
    mut at: f64,
    mut depth: f64,
) -> crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop7Result {
    loop {
        return if (((at < (values.len() as f64))
            && ((crate::translation::frontend_rules::ml_array::at(
                &kinds,
                crate::translation::frontend_rules::ml_array::number_index(at),
            )) != "eof"))
            && ((depth > (0f64))
                || (((crate::translation::frontend_rules::ml_array::at(
                    &values,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                )) != ",")
                    && ((crate::translation::frontend_rules::ml_array::at(
                        &values,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    )) != ")"))))
        {
            {
                let value = crate::translation::frontend_rules::ml_array::at(
                    &values,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                );
                {
                    let depth_3 = if (((value == "(") || (value == "[")) || (value == "{")) {
                        { (depth + 1f64) }
                    } else {
                        depth
                    };
                    {
                        let depth_5 = if (((value == ")") || (value == "]")) || (value == "}")) {
                            { (depth_3 - 1f64) }
                        } else {
                            depth_3
                        };
                        {
                            let ml_s15 = if (((crate::translation::frontend_rules::ml_array::at(
                                &kinds,
                                crate::translation::frontend_rules::ml_array::number_index(at),
                            )) == "identifier")
                                && !((at > (0f64))
                                    && ((crate::translation::frontend_rules::ml_array::at(
                                        &values,
                                        crate::translation::frontend_rules::ml_array::number_index(
                                            (at - 1f64),
                                        ),
                                    )) == ".")))
                            {
                                {
                                    let parameter_index = 0f64;
                                    {
                                        let ml_s16 = crate::translation::frontend_rules::ml_find_default_parameter_reference_loop9(parameters.clone(), at, value.clone(), parameter_index);
                                        match ml_s16.clone() {
                                            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result::MlFindDefaultParameterReferenceLoop9Done => {
                                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Next
                                            }
                                            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result::MlFindDefaultParameterReferenceLoop9Return(ml_result9) => {
                                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Return(ml_result9)
                                            }
                                        }
                                    }
                                }
                            } else {
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Next
                            };
                            match ml_s15.clone() {
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Next => {
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            (at, depth) = (at_2, depth_5);
                                            continue;
                                        }
                                    }
                                }
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Return(ml_result8) => {
                                    crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop7Result::MlFindDefaultParameterReferenceLoop7Return(ml_result8)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop7Result::MlFindDefaultParameterReferenceLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlRegularExpressionEndLoop10Result {
    MlRegularExpressionEndLoop10Done,
    MlRegularExpressionEndLoop10Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlRegularExpressionEndJoin11 {
    MlRegularExpressionEndJoin11Next(f64, bool, bool),
    MlRegularExpressionEndJoin11Return(f64),
}

pub fn ml_regular_expression_end_loop12(mut units: Vec<f64>, mut index: f64) -> f64 {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let flag = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if ((65f64..=90f64).contains(&flag) || (97f64..=122f64).contains(&flag)) {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                index
            }
        } else {
            index
        };
    }
}

pub fn ml_regular_expression_end_loop10(
    mut units: Vec<f64>,
    mut index: f64,
    mut escaped: bool,
    mut character_class: bool,
) -> crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let unit = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if ((((unit == (10f64)) || (unit == (13f64))) || (unit == (8232f64)))
                    || (unit == (8233f64)))
                {
                    crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result::MlRegularExpressionEndLoop10Return(-1f64)
                } else {
                    {
                        let ml_s17 = if escaped {
                            {
                                let escaped_2 = false;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped_2, character_class)
                            }
                        } else if (unit == (92f64)) {
                            {
                                let escaped_3 = true;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped_3, character_class)
                            }
                        } else if (unit == (91f64)) {
                            {
                                let character_class_2 = true;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped, character_class_2)
                            }
                        } else if (unit == (93f64)) {
                            {
                                let character_class_3 = false;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped, character_class_3)
                            }
                        } else if ((unit == (47f64)) && !character_class) {
                            {
                                let index_2 = (index + 1f64);
                                {
                                    let index_3 = crate::translation::frontend_rules::ml_regular_expression_end_loop12(units.clone(), index_2);
                                    crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Return(index_3)
                                }
                            }
                        } else {
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped, character_class)
                        };
                        match ml_s17.clone() {
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index_4, escaped_4, character_class_4) => {
                                {
                                    let index_5 = (index_4 + 1f64);
                                    {
                                        (index, escaped, character_class) = (index_5, escaped_4, character_class_4);
                                        continue;
                                    }
                                }
                            }
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Return(ml_result11) => {
                                crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result::MlRegularExpressionEndLoop10Return(ml_result11)
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result::MlRegularExpressionEndLoop10Done
        };
    }
}

pub fn ml_find_binding_run_end_loop13(mut terms: Vec<String>, mut end: f64) -> f64 {
    loop {
        return if (end < (terms.len() as f64)) {
            {
                let term = crate::translation::frontend_rules::ml_array::at(
                    &terms,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                );
                if (((term != "function_declaration") && (term != "lexical_declaration"))
                    && (term != "export_statement"))
                {
                    end
                } else {
                    {
                        let end_2 = (end + 1f64);
                        {
                            end = end_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptBindingScopeLoop14Result {
    MlAcceptBindingScopeLoop14Done,
    MlAcceptBindingScopeLoop14Return(bool),
}

pub fn ml_accept_binding_scope_loop14(
    mut statuses: Vec<String>,
    mut reasons: Vec<String>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptBindingScopeLoop14Result {
    loop {
        return if (index < (statuses.len() as f64)) {
            if (((crate::translation::frontend_rules::ml_array::at(
                &statuses,
                crate::translation::frontend_rules::ml_array::number_index(index),
            )) == "carried")
                && ((crate::translation::frontend_rules::ml_array::at(
                    &reasons,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                )) == "type"))
            {
                crate::translation::frontend_rules::MlAcceptBindingScopeLoop14Result::MlAcceptBindingScopeLoop14Return(true)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlAcceptBindingScopeLoop14Result::MlAcceptBindingScopeLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptModuleBindingScopeLoop15Result {
    MlAcceptModuleBindingScopeLoop15Done(f64),
    MlAcceptModuleBindingScopeLoop15Return(bool),
}

pub fn ml_accept_module_binding_scope_loop15(
    mut terms: Vec<String>,
    mut declarations: f64,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop15Result {
    loop {
        return if (index < (terms.len() as f64)) {
            {
                let term = crate::translation::frontend_rules::ml_array::at(
                    &terms,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if term.is_empty() {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
                if (((term != "function_declaration") && (term != "lexical_declaration"))
                    && (term != "export_statement"))
                {
                    crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop15Result::MlAcceptModuleBindingScopeLoop15Return(false)
                } else {
                    {
                        let declarations_2 = (declarations + 1f64);
                        {
                            let index_2 = (index + 1f64);
                            {
                                (declarations, index) = (declarations_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop15Result::MlAcceptModuleBindingScopeLoop15Done(declarations)
        };
    }
}

pub fn ml_find_documentation_parameter_range_loop16(mut units: Vec<f64>, mut start: f64) -> f64 {
    loop {
        return if ((start < (units.len() as f64))
            && (((crate::translation::frontend_rules::ml_array::at(
                &units,
                crate::translation::frontend_rules::ml_array::number_index(start),
            )) == (32f64))
                || ((crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(start),
                )) == (9f64))))
        {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

pub fn ml_find_documentation_parameter_range_loop18(
    mut units: Vec<f64>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        return if (end < (units.len() as f64)) {
            {
                let code = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                );
                if (((((65f64..=90f64).contains(&code) || (97f64..=122f64).contains(&code))
                    || (code == (95f64)))
                    || (code == (36f64)))
                    || (((end > start) && (code >= (48f64))) && (code <= (57f64))))
                {
                    {
                        let end_2 = (end + 1f64);
                        {
                            end = end_2;
                            continue;
                        }
                    }
                }
                end
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDocumentationParameterRangeJoin19 {
    MlFindDocumentationParameterRangeJoin19Next,
    MlFindDocumentationParameterRangeJoin19Return(Vec<f64>),
}

pub fn ml_find_documentation_parameter_range_loop20(mut units: Vec<f64>, mut after: f64) -> f64 {
    loop {
        return if ((after < (units.len() as f64))
            && (((crate::translation::frontend_rules::ml_array::at(
                &units,
                crate::translation::frontend_rules::ml_array::number_index(after),
            )) == (32f64))
                || ((crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(after),
                )) == (9f64))))
        {
            {
                let after_2 = (after + 1f64);
                {
                    after = after_2;
                    continue;
                }
            }
        } else {
            after
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDocumentationParameterRangeJoin17 {
    MlFindDocumentationParameterRangeJoin17Next(f64, bool),
}

const fn ml_main() {}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(ml_main)
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}
