// Generated from the complete checked optional kernel and its recorded source dependencies.
// Do not edit by hand.
use super::read_optional_equality_absence;
use super::render_optional_equality_absent;
use super::render_optional_equality_present;

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
            "({}{}{})",
            left.to_owned(),
            if negate {
                String::from(" !== ")
            } else {
                String::from(" === ")
            },
            right.to_owned()
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
                        String::new()
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
                                "{prefix}({left_present}, {right_present}) => {}",
                                compare.to_owned()
                            );
                            {
                                let left_count = if left_kind == "option" {
                                    if left_policy == "both" { 2f64 } else { 1f64 }
                                } else {
                                    0f64
                                };
                                {
                                    let right_count = if right_kind == "option" {
                                        if right_policy == "both" { 2f64 } else { 1f64 }
                                    } else {
                                        0f64
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
                                                    "{cases_2}{separator}{prefix}_ => false"
                                                );
                                                if format == "Rust" {
                                                    {
                                                        let result = format!(
                                                            "match ({}, {}) {{ {cases_3} }}",
                                                            left.to_owned(),
                                                            right.to_owned()
                                                        );
                                                        if negate {
                                                            format!("!({result})")
                                                        } else {
                                                            result
                                                        }
                                                    }
                                                } else if format == "Lean" {
                                                    {
                                                        let result_2 = format!(
                                                            "(match ({}, {}) with {cases_3})",
                                                            left.to_owned(),
                                                            right.to_owned()
                                                        );
                                                        if negate {
                                                            format!("(!{result_2})")
                                                        } else {
                                                            result_2
                                                        }
                                                    }
                                                } else if format == "Rocq" {
                                                    {
                                                        let result_3 = format!(
                                                            "(match ({}, {}) with {cases_3} end)",
                                                            left.to_owned(),
                                                            right.to_owned()
                                                        );
                                                        if negate {
                                                            format!("(negb {result_3})")
                                                        } else {
                                                            result_3
                                                        }
                                                    }
                                                } else {
                                                    String::new()
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
                "match {} {{ crate::MlOptional::Value(ml_present) => ml_present, _ => unreachable!(\"a proven present value\") }}",
                value.to_owned()
            )
        } else {
            format!(
                "{}.clone().expect(\"a proven present value\")",
                receiver.to_owned()
            )
        }
    } else if format == "Lean" {
        if policy == "both" {
            format!(
                "(match {} with | some (some ml_present) => ml_present | _ => default)",
                value.to_owned()
            )
        } else {
            format!("({}.get!)", value.to_owned())
        }
    } else if format == "Rocq" {
        format!(
            "(match {} with | {} => ml_present | _ => {} end)",
            value.to_owned(),
            if policy == "both" {
                String::from("Some (Some ml_present)")
            } else {
                String::from("Some ml_present")
            },
            fallback.to_owned()
        )
    } else {
        String::new()
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
        format!("Number({})", value.to_owned())
    } else if operation == "optionalIndexNumber" {
        value.to_owned()
    } else if operation == "optionalText" {
        format!(
            "{}{})",
            if console {
                String::from("ml_showOptional(")
            } else {
                String::from("String(")
            },
            value.to_owned()
        )
    } else if operation == "optionalTypeOf" {
        format!("(typeof {})", value.to_owned())
    } else if operation == "optionalPredicate" {
        format!("Number.{}({})", name.to_owned(), value.to_owned())
    } else {
        String::new()
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
        format!("{{ let _ = ml_present; {} }}", literal.to_owned())
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
                "({}).concat(&crate::ml_string::Text::from_text(\"n\"))",
                present.to_owned()
            )
        } else {
            format!("format!(\"{{}}n\", {})", present.to_owned())
        }
    } else if format == "Lean" {
        format!("({} ++ \"n\")", present.to_owned())
    } else if format == "Rocq" {
        format!("(String.append {} \"n\"%string)", present.to_owned())
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
    } else if language == "JavaScript" {
        if (left == "option") && left_element {
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
    } else {
        String::from("language-error")
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
        String::new()
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_render_optional_strict_equality_loop2(
    format: &str,
    left_policy: &str,
    right_policy: &str,
    prefix: &str,
    separator: &str,
    mut cases: String,
    mut right_count: f64,
    left_absence: &str,
    mut second: f64,
) -> String {
    loop {
        if second < right_count {
            {
                let right_absence = self::read_optional_equality_absence(right_policy, second);
                {
                    let same = if left_absence == right_absence {
                        String::from("true")
                    } else {
                        String::from("false")
                    };
                    {
                        let cases_2 = format!(
                            "{cases}{}{}({}, {}) => {same}",
                            separator.to_owned(),
                            prefix.to_owned(),
                            self::render_optional_equality_absent(
                                format,
                                left_policy,
                                left_absence
                            ),
                            self::render_optional_equality_absent(
                                format,
                                right_policy,
                                &(right_absence)
                            )
                        );
                        {
                            let second_2 = second + 1f64;
                            {
                                (cases, second) = (cases_2, second_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { cases };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_render_optional_strict_equality_loop1(
    format: &str,
    left_policy: &str,
    right_policy: &str,
    prefix: &str,
    separator: &str,
    mut cases: String,
    mut left_count: f64,
    mut right_count: f64,
    mut first: f64,
) -> String {
    loop {
        if first < left_count {
            {
                let left_absence = self::read_optional_equality_absence(left_policy, first);
                {
                    let second = 0f64;
                    {
                        let cases_2 = self::ml_render_optional_strict_equality_loop2(
                            format,
                            left_policy,
                            right_policy,
                            prefix,
                            separator,
                            cases,
                            right_count,
                            &(left_absence),
                            second,
                        );
                        {
                            let first_2 = first + 1f64;
                            {
                                (cases, first) = (cases_2, first_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { cases };
    }
}
