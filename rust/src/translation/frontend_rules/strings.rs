// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
use super::*;

pub fn read_string_map_operation(language: &str, method: &str) -> String {
    match (language, method) {
        ("JavaScript", "toLowerCase" | "toUpperCase" | "trim" | "trimStart" | "trimEnd") => {
            method.to_owned()
        }
        ("Rust", "to_lowercase") => "toLowerCase".to_owned(),
        ("Rust", "to_uppercase") => "toUpperCase".to_owned(),
        ("Rust", "trim") => "rustTrim".to_owned(),
        ("Rust", "trim_start") => "rustTrimStart".to_owned(),
        ("Rust", "trim_end") => "rustTrimEnd".to_owned(),
        _ => String::new(),
    }
}

pub fn render_string_map_expression(target: &str, operation: &str, object: &str) -> String {
    let method = match operation {
        "rustTrim" => {
            if target == "JavaScript" {
                ".replace(/^\\p{White_Space}+|\\p{White_Space}+$/gu, '')"
            } else {
                ".trim().to_string()"
            }
        }
        "rustTrimStart" => {
            if target == "JavaScript" {
                ".replace(/^\\p{White_Space}+/gu, '')"
            } else {
                ".trim_start().to_string()"
            }
        }
        "rustTrimEnd" => {
            if target == "JavaScript" {
                ".replace(/\\p{White_Space}+$/gu, '')"
            } else {
                ".trim_end().to_string()"
            }
        }
        "toLowerCase" if target != "JavaScript" => ".to_lowercase()",
        "toUpperCase" if target != "JavaScript" => ".to_uppercase()",
        _ if target == "JavaScript" => return format!("{object}.{operation}()"),
        _ => {
            let method = match operation {
                "trim" => "trim_matches",
                "trimStart" => "trim_start_matches",
                "trimEnd" => "trim_end_matches",
                _ => "",
            };
            return format!(
                "{object}.{method}(|c: char| (c.is_whitespace() && c != '\\u{{85}}') || c == '\\u{{feff}}').to_string()"
            );
        }
    };
    format!("{object}{method}")
}

pub fn read_string_map_refusal(target: &str, operation: &str) -> String {
    if matches!(operation, "trim" | "trimStart" | "trimEnd") {
        format!(
            "JavaScript whitespace trimming has no {target} library counterpart; {target} trims ASCII whitespace only"
        )
    } else {
        format!(
            "Unicode case mapping has no {target} library counterpart; {target} maps ASCII letters only"
        )
    }
}

pub fn accept_checked_type_query_operand(kind: &str, constant_reference: bool) -> bool {
    (crate::translation::frontend_rules::accept_type_query_operand(kind)
        || ((kind == "call") && constant_reference))
}

pub const fn read_scanner_continuation_action(
    widthless: bool,
    starts_after: bool,
    ends_after: bool,
) -> f64 {
    if widthless {
        0f64
    } else if starts_after && ends_after {
        1f64
    } else {
        -1f64
    }
}

pub const fn accept_scanner_preemption_boundary(virtual_token: bool, at_boundary: bool) -> bool {
    (!virtual_token || !at_boundary)
}

pub const fn accept_source_envelope_restoration(
    source_matches: bool,
    body_matches: bool,
    original_matches: bool,
) -> bool {
    source_matches && body_matches && original_matches
}

pub fn accept_array_method_arguments(
    form: &str,
    has_arguments: bool,
    has_single_argument: bool,
) -> bool {
    form == "concatenate"
        || form == "construct"
        || (form == "copy" && !has_arguments)
        || (form == "copy-from" && has_single_argument)
}

pub fn render_string_length_expression(target: &str, object: &str) -> String {
    match target {
        "JavaScript" => format!("{object}.length"),
        "Rust" => format!("({object}.encode_utf16().count() as f64)"),
        "Lean" => format!(
            "(Float.ofNat (String.foldl (fun (count : Nat) c => count + (if c.toNat > 65535 then 2 else 1)) 0 {object}))"
        ),
        _ => format!(
            "(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat ((fix ml_utf16_length (s : string) : nat := match s with EmptyString => 0%nat | String c rest => ((if Nat.ltb (Ascii.nat_of_ascii c) 128 then 1 else if Nat.ltb (Ascii.nat_of_ascii c) 192 then 0 else if Nat.ltb (Ascii.nat_of_ascii c) 240 then 1 else 2) + ml_utf16_length rest)%nat end) {object}))))"
        ),
    }
}

pub fn read_rust_number_bounds(units: &[f64], start: f64) -> Vec<f64> {
    {
        let at = start;
        {
            let at_2 =
                crate::translation::frontend_rules::ml_read_rust_number_bounds_loop22(units, at);
            {
                let floating = false;
                {
                    let ml_s15 = if ((at_2 < (units.len() as f64))
                        && match (
                            crate::translation::frontend_rules::ml_array::optional_at(
                                units,
                                crate::translation::frontend_rules::ml_array::number_index(at_2),
                            ),
                            46f64,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) {
                        {
                            let member = false;
                            {
                                let member_3 = if (at_2 + 1f64 < (units.len() as f64)) {
                                    {
                                        let next = crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(at_2 + 1f64));
                                        {
                                            (((match (next, 46f64) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            } || match (next, 95f64) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }) || ((next.unwrap_or(f64::NAN) >= (65f64))
                                                && (next.unwrap_or(f64::NAN) <= (90f64))))
                                                || ((next.unwrap_or(f64::NAN) >= (97f64))
                                                    && (next.unwrap_or(f64::NAN) <= (122f64))))
                                        }
                                    }
                                } else {
                                    member
                                };
                                if member_3 {
                                    crate::translation::frontend_rules::MlReadRustNumberBoundsJoin23::MlReadRustNumberBoundsJoin23Next(at_2, floating)
                                } else {
                                    {
                                        let floating_2 = true;
                                        {
                                            let at_3 = at_2 + 1f64;
                                            {
                                                let at_4 = crate::translation::frontend_rules::ml_read_rust_number_bounds_loop27(units, at_3);
                                                crate::translation::frontend_rules::MlReadRustNumberBoundsJoin23::MlReadRustNumberBoundsJoin23Next(at_4, floating_2)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        crate::translation::frontend_rules::MlReadRustNumberBoundsJoin23::MlReadRustNumberBoundsJoin23Next(at_2, floating)
                    };
                    match ml_s15 {
                        crate::translation::frontend_rules::MlReadRustNumberBoundsJoin23::MlReadRustNumberBoundsJoin23Next(at_5, floating_3) => {
                            {
                                let ml_s16 = if ((at_5 < (units.len() as f64)) && (match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(at_5)), 69f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } || match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(at_5)), 101f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })) {
                                    {
                                        let exponent = (at_5 + 1f64);
                                        {
                                            let exponent_3 = if ((exponent < (units.len() as f64)) && (match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(exponent)), 43f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } || match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(exponent)), 45f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })) {
                                                {

                                                    (exponent + 1f64)
                                                }
                                            } else {
                                                exponent
                                            };
                                            if (((exponent_3 < (units.len() as f64)) && (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(exponent_3)).unwrap_or(f64::NAN) >= (48f64))) && (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(exponent_3)).unwrap_or(f64::NAN) <= (57f64))) {
                                                {
                                                    let floating_4 = true;
                                                    {
                                                        let at_6 = (exponent_3 + 1f64);
                                                        {
                                                            let at_7 = crate::translation::frontend_rules::ml_read_rust_number_bounds_loop26(units, at_6);
                                                            crate::translation::frontend_rules::MlReadRustNumberBoundsJoin24::MlReadRustNumberBoundsJoin24Next(at_7, floating_4)
                                                        }
                                                    }
                                                }
                                            } else {
                                                crate::translation::frontend_rules::MlReadRustNumberBoundsJoin24::MlReadRustNumberBoundsJoin24Next(at_5, floating_3)
                                            }
                                        }
                                    }
                                } else {
                                    crate::translation::frontend_rules::MlReadRustNumberBoundsJoin24::MlReadRustNumberBoundsJoin24Next(at_5, floating_3)
                                };
                                match ml_s16 {
                                    crate::translation::frontend_rules::MlReadRustNumberBoundsJoin24::MlReadRustNumberBoundsJoin24Next(at_8, floating_5) => {
                                        {
                                            let suffix = at_8;
                                            {
                                                let at_11 = if (((at_8 < (units.len() as f64)) && (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(at_8)).unwrap_or(f64::NAN) >= (97f64))) && (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(at_8)).unwrap_or(f64::NAN) <= (122f64))) {
                                                    {
                                                        let at_9 = (at_8 + 1f64);
                                                        {

                                                            crate::translation::frontend_rules::ml_read_rust_number_bounds_loop25(units, at_9)
                                                        }
                                                    }
                                                } else {
                                                    at_8
                                                };
                                                {
                                                    let floating_7 = if ((((at_11 - suffix) == (3f64)) && match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(suffix)), 102f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) && ((match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(suffix + 1f64)), 51f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } && match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(suffix + 2f64)), 50f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) || (match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(suffix + 1f64)), 54f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } && match (crate::translation::frontend_rules::ml_array::optional_at(units, crate::translation::frontend_rules::ml_array::number_index(suffix + 2f64)), 52f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }))) {
                                                        {

                                                            true
                                                        }
                                                    } else {
                                                        floating_5
                                                    };
                                                    vec![at_11, suffix, if floating_7 {
                                                        1f64
                                                    } else {
                                                        0f64
                                                    }]
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

pub fn read_lexical_cache_expression_form(kind: &str) -> f64 {
    if ((((((((kind == "empty") || (kind == "literal")) || (kind == "literalInsensitive"))
        || (kind == "charRange"))
        || (kind == "charClass"))
        || (kind == "byteClass"))
        || (kind == "any"))
        || (kind == "regex"))
    {
        1f64
    } else if (((kind == "seq") || (kind == "choice")) || (kind == "longest")) {
        2f64
    } else if ((((((((((((((kind == "optional") || (kind == "repeat0"))
        || (kind == "repeat1"))
        || (kind == "repeat"))
        || (kind == "and"))
        || (kind == "not"))
        || (kind == "capture"))
        || (kind == "alias"))
        || (kind == "precedence"))
        || (kind == "namedPrecedence"))
        || (kind == "dynamicPrecedence"))
        || (kind == "lexicalPrecedence"))
        || (kind == "token"))
        || (kind == "immediateToken"))
    {
        3f64
    } else {
        0f64
    }
}

pub fn read_unit_alternative_expression_form(kind: &str) -> f64 {
    if (kind == "ref") {
        1f64
    } else if (kind == "choice") {
        2f64
    } else if ((((((kind == "alias") || (kind == "capture")) || (kind == "precedence"))
        || (kind == "namedPrecedence"))
        || (kind == "dynamicPrecedence"))
        || (kind == "optional"))
    {
        3f64
    } else {
        0f64
    }
}

pub fn accept_precedence_operand_context(
    owner_contexts: Vec<bool>,
    child_contexts: Vec<bool>,
) -> bool {
    {
        let index = 0f64;
        {
            let ml_s17 =
                crate::translation::frontend_rules::ml_accept_precedence_operand_context_loop28(
                    owner_contexts,
                    child_contexts,
                    index,
                );
            match ml_s17 {
                crate::translation::frontend_rules::MlAcceptPrecedenceOperandContextLoop28Result::MlAcceptPrecedenceOperandContextLoop28Done => {
                    false
                }
                crate::translation::frontend_rules::MlAcceptPrecedenceOperandContextLoop28Result::MlAcceptPrecedenceOperandContextLoop28Return(ml_result28) => {
                    ml_result28
                }
            }
        }
    }
}

pub const fn accept_lexical_cache_context(in_token: bool, quiet: bool, repairing: bool) -> bool {
    ((in_token && quiet) && !repairing)
}

pub const fn accept_lexical_cache_reservation(cells: f64, count: f64, limit: f64) -> bool {
    (count <= (limit - cells))
}

#[must_use]
pub const fn accept_precedence_operand_head(
    source_rule_matches: bool,
    source_reduction_matches: bool,
) -> bool {
    source_rule_matches || source_reduction_matches
}
