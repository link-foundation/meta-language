// Generated from the complete checked optional kernel and its recorded source dependencies.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_primitive_truth(
    format: &str,
    kind: &str,
    value: &str,
    receiver: &str,
    utf16: bool,
    signed: bool,
) -> String {
    if format == "JavaScript" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("!!(")), value.to_owned())),
            (String::from(")"))
        )
    } else if kind == "bool" {
        value.to_owned()
    } else if format == "Rust" {
        if kind == "float" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("{ let ml_value = ")),
                    value.to_owned()
                )),
                (String::from("; ml_value != 0f64 && !ml_value.is_nan() }"))
            )
        } else if (kind == "int") || (kind == "nat") {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("!")), receiver.to_owned())),
                (String::from(".is_zero()"))
            )
        } else if kind == "fixed" {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(")), receiver.to_owned())),
                (String::from(" != 0)"))
            )
        } else if kind == "string" {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("!")), receiver.to_owned())),
                (if utf16 {
                    String::from(".0.is_empty()")
                } else {
                    String::from(".is_empty()")
                })
            )
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("{ let _ = ")), value.to_owned())),
                        (String::from("; "))
                    )),
                    (if kind != "unit" {
                        String::from("true")
                    } else {
                        String::from("false")
                    })
                )),
                (String::from(" }"))
            )
        }
    } else if format == "Lean" {
        if kind == "float" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(let ml_value := ")),
                    value.to_owned()
                )),
                (String::from("; ml_value != (0 : Float) && !ml_value.isNaN)"))
            )
        } else if ((kind == "int") || (kind == "nat")) || (kind == "fixed") {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(")), value.to_owned())),
                (String::from(" != 0)"))
            )
        } else if kind == "string" {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(!")), value.to_owned())),
                (String::from(".isEmpty)"))
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
                            (String::from("(let ml_ignored := ")),
                            value.to_owned()
                        )),
                        (String::from("; "))
                    )),
                    (if kind != "unit" {
                        String::from("true")
                    } else {
                        String::from("false")
                    })
                )),
                (String::from(")"))
            )
        }
    } else if format == "Rocq" {
        if kind == "float" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(let ml_value := ")),
                    value.to_owned()
                )),
                (String::from(
                    " in andb (negb (PrimFloat.eqb ml_value 0%float)) (negb (PrimFloat.is_nan ml_value)))"
                ))
            )
        } else if (kind == "int") || ((kind == "fixed") && signed) {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(negb (Z.eqb ")), value.to_owned())),
                (String::from(" 0))"))
            )
        } else if (kind == "nat") || (kind == "fixed") {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(negb (N.eqb ")), value.to_owned())),
                (String::from(" 0))"))
            )
        } else if kind == "string" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(negb (String.eqb ")),
                    value.to_owned()
                )),
                (String::from(" EmptyString))"))
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
                            (String::from("(let ml_ignored := ")),
                            value.to_owned()
                        )),
                        (String::from(" in "))
                    )),
                    (if kind != "unit" {
                        String::from("true")
                    } else {
                        String::from("false")
                    })
                )),
                (String::from(")"))
            )
        }
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_equality_present(
    format: &str,
    kind: &str,
    policy: &str,
    name: &str,
) -> String {
    if kind != "option" {
        name.to_owned()
    } else if format == "Rust" {
        if policy == "both" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("crate::MlOptional::Value(")),
                    name.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("Some(")), name.to_owned())),
                (String::from(")"))
            )
        }
    } else if format == "Lean" {
        if policy == "both" {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("some (some ")), name.to_owned())),
                (String::from(")"))
            )
        } else {
            format!("{}{}", (String::from("some ")), name.to_owned())
        }
    } else if policy == "both" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("Some (Some ")), name.to_owned())),
            (String::from(")"))
        )
    } else {
        format!("{}{}", (String::from("Some ")), name.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_equality_absence(policy: &str, index: f64) -> String {
    if policy == "both" {
        if index == 0f64 {
            String::from("null")
        } else {
            String::from("undefined")
        }
    } else {
        policy.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_equality_absent(format: &str, policy: &str, absence: &str) -> String {
    if format == "Rust" {
        if policy == "both" {
            format!(
                "{}{}",
                (String::from("crate::MlOptional::")),
                (if absence == "null" {
                    String::from("Null")
                } else {
                    String::from("Undefined")
                })
            )
        } else {
            String::from("None")
        }
    } else if format == "Lean" {
        if (policy == "both") && (absence == "undefined") {
            String::from("some none")
        } else {
            String::from("none")
        }
    } else if (policy == "both") && (absence == "undefined") {
        String::from("Some None")
    } else {
        String::from("None")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn render_optional_strict_equality(
    format: &str,
    left_kind: &str,
    left_policy: &str,
    right_kind: &str,
    right_policy: &str,
    compare: &str,
    left: &str,
    right: &str,
    negate: bool,
) -> String {
    if format == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), left.to_owned())),
                    (if negate {
                        String::from(" !== ")
                    } else {
                        String::from(" === ")
                    })
                )),
                right.to_owned()
            )),
            (String::from(")"))
        )
    } else {
        {
            let left_present = self::render_optional_equality_present(
                format,
                left_kind,
                left_policy,
                &(if compare == "false" {
                    String::from("_ml_left")
                } else {
                    String::from("ml_left")
                }),
            );
            {
                let right_present = self::render_optional_equality_present(
                    format,
                    right_kind,
                    right_policy,
                    &(if compare == "false" {
                        String::from("_ml_right")
                    } else {
                        String::from("ml_right")
                    }),
                );
                {
                    let prefix = if format == "Rust" {
                        String::from("")
                    } else {
                        String::from("| ")
                    };
                    {
                        let separator = if format == "Rust" {
                            String::from(", ")
                        } else {
                            String::from(" ")
                        };
                        {
                            let cases = format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (format!("{}{}", prefix, (String::from("(")))),
                                                left_present
                                            )),
                                            (String::from(", "))
                                        )),
                                        right_present
                                    )),
                                    (String::from(") => "))
                                )),
                                compare.to_owned()
                            );
                            {
                                let left_count = if left_kind != "option" {
                                    0f64
                                } else if left_policy == "both" {
                                    2f64
                                } else {
                                    1f64
                                };
                                {
                                    let right_count = if right_kind != "option" {
                                        0f64
                                    } else if right_policy == "both" {
                                        2f64
                                    } else {
                                        1f64
                                    };
                                    {
                                        let first = 0f64;
                                        {
                                            let cases_2 =
                                                self::ml_render_optional_strict_equality_loop1(
                                                    format,
                                                    left_policy,
                                                    right_policy,
                                                    &(prefix),
                                                    &(separator),
                                                    cases,
                                                    left_count,
                                                    right_count,
                                                    first,
                                                );
                                            {
                                                let cases_3 = format!(
                                                    "{}{}",
                                                    cases_2,
                                                    (format!(
                                                        "{}{}",
                                                        (format!("{}{}", separator, prefix)),
                                                        (String::from("_ => false"))
                                                    ))
                                                );
                                                if format == "Rust" {
                                                    {
                                                        let result = format!(
                                                            "{}{}",
                                                            (format!(
                                                                "{}{}",
                                                                (format!(
                                                                    "{}{}",
                                                                    (format!(
                                                                        "{}{}",
                                                                        (format!(
                                                                            "{}{}",
                                                                            (format!(
                                                                                "{}{}",
                                                                                (String::from(
                                                                                    "match ("
                                                                                )),
                                                                                left.to_owned()
                                                                            )),
                                                                            (String::from(", "))
                                                                        )),
                                                                        right.to_owned()
                                                                    )),
                                                                    (String::from(") { "))
                                                                )),
                                                                cases_3
                                                            )),
                                                            (String::from(" }"))
                                                        );
                                                        if negate {
                                                            format!(
                                                                "{}{}",
                                                                (format!(
                                                                    "{}{}",
                                                                    (String::from("!(")),
                                                                    result
                                                                )),
                                                                (String::from(")"))
                                                            )
                                                        } else {
                                                            result.clone()
                                                        }
                                                    }
                                                } else if format == "Lean" {
                                                    {
                                                        let result_2 = format!(
                                                            "{}{}",
                                                            (format!(
                                                                "{}{}",
                                                                (format!(
                                                                    "{}{}",
                                                                    (format!(
                                                                        "{}{}",
                                                                        (format!(
                                                                            "{}{}",
                                                                            (format!(
                                                                                "{}{}",
                                                                                (String::from(
                                                                                    "(match ("
                                                                                )),
                                                                                left.to_owned()
                                                                            )),
                                                                            (String::from(", "))
                                                                        )),
                                                                        right.to_owned()
                                                                    )),
                                                                    (String::from(") with "))
                                                                )),
                                                                cases_3
                                                            )),
                                                            (String::from(")"))
                                                        );
                                                        if negate {
                                                            format!(
                                                                "{}{}",
                                                                (format!(
                                                                    "{}{}",
                                                                    (String::from("(!")),
                                                                    result_2
                                                                )),
                                                                (String::from(")"))
                                                            )
                                                        } else {
                                                            result_2.clone()
                                                        }
                                                    }
                                                } else if format == "Rocq" {
                                                    {
                                                        let result_3 = format!(
                                                            "{}{}",
                                                            (format!(
                                                                "{}{}",
                                                                (format!(
                                                                    "{}{}",
                                                                    (format!(
                                                                        "{}{}",
                                                                        (format!(
                                                                            "{}{}",
                                                                            (format!(
                                                                                "{}{}",
                                                                                (String::from(
                                                                                    "(match ("
                                                                                )),
                                                                                left.to_owned()
                                                                            )),
                                                                            (String::from(", "))
                                                                        )),
                                                                        right.to_owned()
                                                                    )),
                                                                    (String::from(") with "))
                                                                )),
                                                                cases_3
                                                            )),
                                                            (String::from(" end)"))
                                                        );
                                                        if negate {
                                                            format!(
                                                                "{}{}",
                                                                (format!(
                                                                    "{}{}",
                                                                    (String::from("(negb ")),
                                                                    result_3
                                                                )),
                                                                (String::from(")"))
                                                            )
                                                        } else {
                                                            result_3.clone()
                                                        }
                                                    }
                                                } else {
                                                    String::from("")
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_unwrapping(
    format: &str,
    policy: &str,
    value: &str,
    receiver: &str,
    fallback: &str,
) -> String {
    if format == "JavaScript" {
        value.to_owned()
    } else if format == "Rust" {
        if policy == "both" {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("match ")), value.to_owned())),
                (String::from(
                    " { crate::MlOptional::Value(ml_present) => ml_present, _ => unreachable!(\"a proven present value\") }"
                ))
            )
        } else {
            format!(
                "{}{}",
                receiver.to_owned(),
                (String::from(".clone().expect(\"a proven present value\")"))
            )
        }
    } else if format == "Lean" {
        if policy == "both" {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(match ")), value.to_owned())),
                (String::from(" with | some (some ml_present) => ml_present | _ => default)"))
            )
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(")), value.to_owned())),
                (String::from(".get!)"))
            )
        }
    } else if format == "Rocq" {
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
                            (format!("{}{}", (String::from("(match ")), value.to_owned())),
                            (String::from(" with | "))
                        )),
                        (if policy == "both" {
                            String::from("Some (Some ml_present)")
                        } else {
                            String::from("Some ml_present")
                        })
                    )),
                    (String::from(" => ml_present | _ => "))
                )),
                fallback.to_owned()
            )),
            (String::from(" end)"))
        )
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_java_script_operation(
    operation: &str,
    value: &str,
    name: &str,
    console: bool,
) -> String {
    if operation == "nullishNumber" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("Number(")), value.to_owned())),
            (String::from(")"))
        )
    } else if operation == "optionalIndexNumber" {
        value.to_owned()
    } else if operation == "optionalText" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (if console {
                    String::from("ml_showOptional(")
                } else {
                    String::from("String(")
                }),
                value.to_owned()
            )),
            (String::from(")"))
        )
    } else if operation == "optionalTypeOf" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("(typeof ")), value.to_owned())),
            (String::from(")"))
        )
    } else if operation == "optionalPredicate" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("Number.")), name.to_owned())),
                    (String::from("("))
                )),
                value.to_owned()
            )),
            (String::from(")"))
        )
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_numeric_present(format: &str, kind: &str, operation: &str) -> String {
    if kind == "unit" {
        if format == "Rust" {
            String::from("f64::NAN")
        } else if format == "Lean" {
            String::from("(0 / 0 : Float)")
        } else {
            String::from("PrimFloat.nan")
        }
    } else if (kind == "bool") && (operation == "arithmetic") {
        if format == "Rust" {
            String::from("if ml_present { 1f64 } else { 0f64 }")
        } else if format == "Lean" {
            String::from("(if ml_present then (1 : Float) else (0 : Float))")
        } else {
            String::from("(if ml_present then 1%float else 0%float)")
        }
    } else {
        String::from("ml_present")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_type_query_present(format: &str, literal: &str) -> String {
    if format == "Rust" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("{ let _ = ml_present; ")),
                literal.to_owned()
            )),
            (String::from(" }"))
        )
    } else {
        literal.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_optional_numeric_predicate(kind: &str) -> bool {
    kind == "float"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_console_text(
    format: &str,
    present: &str,
    bigint: bool,
    console: bool,
    utf16: bool,
) -> String {
    if !bigint || !console {
        present.to_owned()
    } else if format == "Rust" {
        if utf16 {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(")), present.to_owned())),
                (String::from(").concat(&crate::ml_string::Text::from_text(\"n\"))"))
            )
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("format!(\"{}n\", ")),
                    present.to_owned()
                )),
                (String::from(")"))
            )
        }
    } else if format == "Lean" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("(")), present.to_owned())),
            (String::from(" ++ \"n\")"))
        )
    } else if format == "Rocq" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("(String.append ")),
                present.to_owned()
            )),
            (String::from(" \"n\"%string)"))
        )
    } else {
        present.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_java_script_text_prelude() -> String {
    String::from(
        "function ml_showOptional(value) {\n  if (value === null) return 'null';\n  if (value === undefined) return 'undefined';\n  if (typeof value === 'bigint') return String(value) + 'n';\n  return Object.is(value, -0) ? '-0' : String(value);\n}",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_optional_primitive_kind(kind: &str) -> bool {
    ((((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "int"))
        || (kind == "nat"))
        || (kind == "string"))
        || (kind == "unit")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_logical_operation(
    language: &str,
    left: &str,
    right: &str,
    left_element: bool,
    right_element: bool,
    same: bool,
    operation: &str,
) -> String {
    if (left == "bool") && (right == "bool") {
        String::from("boolean")
    } else if language != "JavaScript" {
        String::from("language-error")
    } else if (left == "option") && left_element {
        if operation == "or" {
            String::from("unwrap-left")
        } else {
            String::from("wrap-right")
        }
    } else if (right == "option") && right_element {
        String::from("wrap-left")
    } else if same {
        String::from("identity")
    } else {
        String::from("union-error")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_comparison_operation(
    language: &str,
    left: &str,
    right: &str,
    operation: &str,
    same: bool,
    primitive: bool,
) -> String {
    if (language != "JavaScript") || ((left != "option") && (right != "option")) {
        String::from("")
    } else if (operation == "eq") || (operation == "ne") {
        if same && !primitive {
            String::from("object-error")
        } else {
            String::from("strict")
        }
    } else {
        String::from("number")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_output_absences(policy: &str) -> Vec<String> {
    if policy == "both" {
        vec![String::from("null"), String::from("undefined")]
    } else {
        vec![policy.to_owned()]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_inference_join_alias(kind: &str, alias: f64, original: f64) -> bool {
    (kind == "var") && (alias != original)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_optional_literal_absence(absence: &str) -> bool {
    (absence == "null") || (absence == "undefined")
}
