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

pub fn read_array_method_form(method: &str) -> String {
    if method == "concat" {
        String::from("concatenate")
    } else {
        String::new()
    }
}

pub fn accept_root_syntax_item(term: &str, has_children: bool, has_content: bool) -> bool {
    (((term == "ERROR") && !has_children) && has_content)
}

pub const fn accept_source_prefix_restoration(
    source_matches: bool,
    body_matches: bool,
    layout_only: bool,
) -> bool {
    ((source_matches && body_matches) && layout_only)
}

pub const fn accept_declaration_signature(constant: bool, literal: bool) -> bool {
    (!constant || literal)
}

pub fn read_string_test_operation(language: &str, method: &str) -> String {
    if (language == "JavaScript") {
        if (((method == "startsWith") || (method == "endsWith")) || (method == "includes")) {
            method.to_owned()
        } else {
            String::new()
        }
    } else {
        if (language == "Rust") {
            if (method == "starts_with") {
                String::from("startsWith")
            } else if (method == "ends_with") {
                String::from("endsWith")
            } else if (method == "contains") {
                String::from("includes")
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}

pub fn render_string_test_expression(
    target: &str,
    operation: &str,
    object: &str,
    search: &str,
) -> String {
    if (target == "JavaScript") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", object, (String::from(".")))),
                        operation
                    )),
                    (String::from("("))
                )),
                search
            )),
            (String::from(")"))
        )
    } else if (target == "Rust") {
        {
            let method = String::new();
            {
                let method_3 = if (operation == "startsWith") {
                    { String::from("starts_with") }
                } else {
                    method
                };
                {
                    let method_5 = if (operation == "endsWith") {
                        { String::from("ends_with") }
                    } else {
                        method_3
                    };
                    {
                        let method_7 = if (operation == "includes") {
                            { String::from("contains") }
                        } else {
                            method_5
                        };
                        format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!("{}{}", object, (String::from(".")))),
                                        method_7
                                    )),
                                    (String::from("("))
                                )),
                                search
                            )),
                            (String::from(")"))
                        )
                    }
                }
            }
        }
    } else if (target == "Lean") {
        if (operation == "includes") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(ml_string_includes ")), object)),
                        (String::from(" "))
                    )),
                    search
                )),
                (String::from(")"))
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
                                (format!("{}{}", (String::from("(String.")), operation)),
                                (String::from(" "))
                            )),
                            object
                        )),
                        (String::from(" "))
                    )),
                    search
                )),
                (String::from(")"))
            )
        }
    } else {
        {
            let helper = String::new();
            {
                let helper_3 = if (operation == "startsWith") {
                    { String::from("ml_string_starts_with") }
                } else {
                    helper
                };
                {
                    let helper_5 = if (operation == "endsWith") {
                        { String::from("ml_string_ends_with") }
                    } else {
                        helper_3
                    };
                    {
                        let helper_7 = if (operation == "includes") {
                            { String::from("ml_string_includes") }
                        } else {
                            helper_5
                        };
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
                                            (format!("{}{}", (String::from("(")), helper_7)),
                                            (String::from(" "))
                                        )),
                                        object
                                    )),
                                    (String::from(" "))
                                )),
                                search
                            )),
                            (String::from(")"))
                        )
                    }
                }
            }
        }
    }
}

pub fn read_string_test_helper(target: &str, operation: &str) -> String {
    if (target == "Lean") {
        if (operation == "includes") {
            String::from("stringIncludes")
        } else {
            String::new()
        }
    } else {
        if (target == "Rocq") {
            if (operation == "startsWith") {
                String::from("stringStartsWith")
            } else if (operation == "endsWith") {
                String::from("stringEndsWith")
            } else if (operation == "includes") {
                String::from("stringIncludes")
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}

pub fn read_string_test_support(target: &str, operation: &str) -> String {
    if ((target == "Lean") && (operation == "includes")) {
        String::from(
            "/-- String.prototype.includes: the search occurs at some position of the string. -/\ndef ml_string_includes (string search : String) : Bool :=\n  (List.range (string.length + 1)).any fun index => (string.drop index).startsWith search",
        )
    } else if ((target == "Rocq") && (operation == "startsWith")) {
        String::from(
            "(* String.prototype.startsWith: the string begins with the search. *)\nDefinition ml_string_starts_with (string search : string) : bool := String.prefix search string.",
        )
    } else if ((target == "Rocq") && (operation == "endsWith")) {
        String::from(
            "(* String.prototype.endsWith: the string ends with the search. *)\nDefinition ml_string_ends_with (string search : string) : bool :=\n  Nat.leb (String.length search) (String.length string) &&\n  String.eqb (String.substring (String.length string - String.length search) (String.length search) string) search.",
        )
    } else if ((target == "Rocq") && (operation == "includes")) {
        String::from(
            "(* String.prototype.includes: the search occurs at some position of the string. *)\nDefinition ml_string_includes (string search : string) : bool :=\n  match String.index 0 search string with Some _ => true | None => false end.",
        )
    } else {
        String::new()
    }
}

pub fn read_string_map_operation(language: &str, method: &str) -> String {
    if (language == "JavaScript") {
        if (((((method == "toLowerCase") || (method == "toUpperCase")) || (method == "trim"))
            || (method == "trimStart"))
            || (method == "trimEnd"))
        {
            method.to_owned()
        } else {
            String::new()
        }
    } else {
        if (language == "Rust") {
            if (method == "to_lowercase") {
                String::from("toLowerCase")
            } else if (method == "to_uppercase") {
                String::from("toUpperCase")
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}

pub fn render_string_map_expression(target: &str, operation: &str, object: &str) -> String {
    if (target == "JavaScript") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!("{}{}", object, (String::from(".")))),
                operation
            )),
            (String::from("()"))
        )
    } else if (operation == "toLowerCase") {
        format!("{}{}", object, (String::from(".to_lowercase()")))
    } else if (operation == "toUpperCase") {
        format!("{}{}", object, (String::from(".to_uppercase()")))
    } else {
        {
            let method = String::new();
            {
                let method_3 = if (operation == "trim") {
                    { String::from("trim_matches") }
                } else {
                    method
                };
                {
                    let method_5 = if (operation == "trimStart") {
                        { String::from("trim_start_matches") }
                    } else {
                        method_3
                    };
                    {
                        let method_7 = if (operation == "trimEnd") {
                            { String::from("trim_end_matches") }
                        } else {
                            method_5
                        };
                        format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!("{}{}", object, (String::from(".")))),
                                method_7
                            )),
                            (String::from(
                                "(|c: char| (c.is_whitespace() && c != '\\u{85}') || c == '\\u{feff}').to_string()"
                            ))
                        )
                    }
                }
            }
        }
    }
}

pub fn read_string_map_refusal(target: &str, operation: &str) -> String {
    if (((operation == "trim") || (operation == "trimStart")) || (operation == "trimEnd")) {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("JavaScript whitespace trimming has no ")),
                        target
                    )),
                    (String::from(" library counterpart; "))
                )),
                target
            )),
            (String::from(" trims ASCII whitespace only"))
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
                        (String::from("Unicode case mapping has no ")),
                        target
                    )),
                    (String::from(" library counterpart; "))
                )),
                target
            )),
            (String::from(" maps ASCII letters only"))
        )
    }
}

mod continuations;
pub use continuations::*;
