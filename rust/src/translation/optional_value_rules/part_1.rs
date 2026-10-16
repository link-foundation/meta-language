// Generated from the complete checked optional kernel and its recorded source dependencies.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_optional_flattening(kind: &str, element: &str) -> bool {
    (kind == "option") && (element == "option")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_optional_coalescing(kind: &str) -> bool {
    kind == "option"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_number_operation(language: &str, kind: &str, element: &str) -> String {
    if (language != "JavaScript") || (kind != "option") {
        String::from("identity")
    } else if ((element == "float") || (element == "unit")) || (element == "bool") {
        String::from("number")
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_boolean_operation(language: &str, kind: &str) -> String {
    if kind == "bool" {
        String::from("identity")
    } else if language == "JavaScript" {
        String::from("truthy")
    } else {
        String::from("coerce")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_logical_mixing_error(
    position: &str,
    coalesced: bool,
    parenthesized: bool,
    kind: &str,
    operation: &str,
) -> String {
    if (((position == "left") && !parenthesized) && (kind == "binary"))
        && ((operation == "and") || (operation == "or"))
    {
        String::from("mixing ?? with && or || needs parentheses")
    } else if (((position == "right") && !parenthesized) && (kind == "binary"))
        && (operation == "and")
    {
        String::from("mixing ?? with && needs parentheses")
    } else if (position == "or") && coalesced {
        String::from("mixing ?? with || needs parentheses")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_source_literal(token: &str, declared: bool) -> String {
    if (token == "null") || ((token == "undefined") && !declared) {
        token.to_owned()
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_big_int_constraint(kind: &str, element: &str, integer: bool) -> String {
    if (kind == "option") && (element == "var") {
        String::from("mark")
    } else if (kind == "option") && integer {
        String::from("accept")
    } else {
        String::from("error")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_optional_result_binding(
    declared: bool,
    additional: bool,
    actual: &str,
    result: &str,
    same: bool,
    recursive: bool,
) -> bool {
    ((((!declared || additional) && (actual != "var")) && (result == "var")) && !same) && !recursive
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_call_binding(
    parameter: &str,
    actual: &str,
    current: &str,
    current_absence: &str,
    actual_absence: &str,
    recursive: bool,
) -> String {
    if (parameter != "var") || (actual != "option") {
        String::from("")
    } else if current == "option" {
        if (current_absence != actual_absence) && (current_absence != "both") {
            String::from("merge")
        } else {
            String::from("elements")
        }
    } else if recursive {
        String::from("")
    } else {
        String::from("bind")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_join_binding(original: bool, same: bool, recursive: bool) -> String {
    if original {
        String::from("identity")
    } else if same {
        String::from("changes")
    } else if recursive {
        String::from("identity")
    } else {
        String::from("bind")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_numeric_term(kind: &str) -> String {
    if kind == "option" {
        String::from("element")
    } else if kind == "unit" {
        String::from("float")
    } else {
        String::from("identity")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_coercion_operation(
    language: &str,
    expected: &str,
    actual: &str,
    expression: &str,
    accepts_actual: bool,
    accepts_undefined: bool,
    same_element: bool,
) -> String {
    if expected != "option" {
        String::from("")
    } else if expression == "none" {
        if accepts_actual {
            String::from("none")
        } else {
            String::from("absence-error")
        }
    } else if ((language == "JavaScript") && (actual == "unit")) && accepts_undefined {
        if expression == "unit" {
            String::from("unit")
        } else {
            String::from("discard-unit")
        }
    } else if same_element {
        String::from("some")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_field_operation(kind: &str, element: &str, field: &str) -> String {
    if kind != "option" {
        String::from("field")
    } else if (field == "length") && ((element == "string") || (element == "array")) {
        String::from("project")
    } else if (field == "size") && ((element == "set") || (element == "map")) {
        String::from("project")
    } else if element == "record" {
        String::from("optional-field")
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_absent_value(
    format: &str,
    policy: &str,
    absence: &str,
    element: &str,
) -> String {
    if format == "JavaScript" {
        if absence == "null" {
            String::from("null")
        } else {
            String::from("undefined")
        }
    } else if format == "Rust" {
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
            format!(
                "{}{}",
                (format!("{}{}", (String::from("None::<")), element.to_owned())),
                (String::from(">"))
            )
        }
    } else if format == "Lean" {
        if (policy == "both") && (absence == "undefined") {
            String::from("(some none)")
        } else {
            String::from("none")
        }
    } else if format == "Rocq" {
        if (policy == "both") && (absence == "undefined") {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(Some (@None ")), element.to_owned())),
                (String::from("))"))
            )
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(@None ")),
                    (if policy == "both" {
                        format!(
                            "{}{}",
                            (format!("{}{}", (String::from("(option ")), element.to_owned())),
                            (String::from(")"))
                        )
                    } else {
                        element.to_owned()
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
pub fn render_optional_present_value(format: &str, policy: &str, value: &str) -> String {
    if format == "JavaScript" {
        value.to_owned()
    } else if format == "Rust" {
        if policy == "both" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("crate::MlOptional::Value(")),
                    value.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("Some(")), value.to_owned())),
                (String::from(")"))
            )
        }
    } else if format == "Lean" {
        if policy == "both" {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(some (some ")), value.to_owned())),
                (String::from("))"))
            )
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(some ")), value.to_owned())),
                (String::from(")"))
            )
        }
    } else if format == "Rocq" {
        if policy == "both" {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(Some (Some ")), value.to_owned())),
                (String::from("))"))
            )
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("(Some ")), value.to_owned())),
                (String::from(")"))
            )
        }
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_match(
    format: &str,
    policy: &str,
    value: &str,
    binder: &str,
    present: &str,
    null_case: &str,
    undefined_case: &str,
) -> String {
    if format == "Rust" {
        if policy == "both" {
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
                                                    (String::from("match ")),
                                                    value.to_owned()
                                                )),
                                                (String::from(" { crate::MlOptional::Value("))
                                            )),
                                            binder.to_owned()
                                        )),
                                        (String::from(") => "))
                                    )),
                                    present.to_owned()
                                )),
                                (String::from(", crate::MlOptional::Null => "))
                            )),
                            null_case.to_owned()
                        )),
                        (String::from(", crate::MlOptional::Undefined => "))
                    )),
                    undefined_case.to_owned()
                )),
                (String::from(" }"))
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
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!(
                                            "{}{}",
                                            (String::from("match ")),
                                            value.to_owned()
                                        )),
                                        (String::from(" { Some("))
                                    )),
                                    binder.to_owned()
                                )),
                                (String::from(") => "))
                            )),
                            present.to_owned()
                        )),
                        (String::from(", None => "))
                    )),
                    (if policy == "null" {
                        null_case.to_owned()
                    } else {
                        undefined_case.to_owned()
                    })
                )),
                (String::from(" }"))
            )
        }
    } else if format == "Lean" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(match ")), value.to_owned())),
                    (String::from(" with "))
                )),
                (if policy == "both" {
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
                                        (format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (String::from("| some (some ")),
                                                binder.to_owned()
                                            )),
                                            (String::from(") => "))
                                        )),
                                        present.to_owned()
                                    )),
                                    (String::from(" | none => "))
                                )),
                                null_case.to_owned()
                            )),
                            (String::from(" | some none => "))
                        )),
                        undefined_case.to_owned()
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
                                    (format!("{}{}", (String::from("| some ")), binder.to_owned())),
                                    (String::from(" => "))
                                )),
                                present.to_owned()
                            )),
                            (String::from(" | none => "))
                        )),
                        (if policy == "null" {
                            null_case.to_owned()
                        } else {
                            undefined_case.to_owned()
                        })
                    )
                })
            )),
            (String::from(")"))
        )
    } else if format == "Rocq" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(match ")), value.to_owned())),
                    (String::from(" with "))
                )),
                (if policy == "both" {
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
                                        (format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (String::from("| Some (Some ")),
                                                binder.to_owned()
                                            )),
                                            (String::from(") => "))
                                        )),
                                        present.to_owned()
                                    )),
                                    (String::from(" | None => "))
                                )),
                                null_case.to_owned()
                            )),
                            (String::from(" | Some None => "))
                        )),
                        undefined_case.to_owned()
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
                                    (format!("{}{}", (String::from("| Some ")), binder.to_owned())),
                                    (String::from(" => "))
                                )),
                                present.to_owned()
                            )),
                            (String::from(" | None => "))
                        )),
                        (if policy == "null" {
                            null_case.to_owned()
                        } else {
                            undefined_case.to_owned()
                        })
                    )
                })
            )),
            (String::from(" end)"))
        )
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_coalescing(
    format: &str,
    policy: &str,
    value: &str,
    receiver: &str,
    fallback: &str,
) -> String {
    if format == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), value.to_owned())),
                    (String::from(" ?? "))
                )),
                fallback.to_owned()
            )),
            (String::from(")"))
        )
    } else if format == "Rust" {
        if policy == "both" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("match ")), value.to_owned())),
                        (String::from(" { crate::MlOptional::Value(ml_value) => ml_value, _ => "))
                    )),
                    fallback.to_owned()
                )),
                (String::from(" }"))
            )
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        receiver.to_owned(),
                        (String::from(".clone().unwrap_or_else(|| "))
                    )),
                    fallback.to_owned()
                )),
                (String::from(")"))
            )
        }
    } else if format == "Lean" {
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
                            String::from("some (some ml_value)")
                        } else {
                            String::from("some ml_value")
                        })
                    )),
                    (String::from(" => ml_value | _ => "))
                )),
                fallback.to_owned()
            )),
            (String::from(")"))
        )
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
                            String::from("Some (Some ml_value)")
                        } else {
                            String::from("Some ml_value")
                        })
                    )),
                    (String::from(" => ml_value | _ => "))
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
pub fn render_optional_flatten_absence(format: &str, policy: &str, absence: &str) -> String {
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
            String::from("(some none)")
        } else {
            String::from("none")
        }
    } else if format == "Rocq" {
        if (policy == "both") && (absence == "undefined") {
            String::from("(Some None)")
        } else {
            String::from("None")
        }
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_flattening(
    format: &str,
    outer: &str,
    inner: &str,
    result: &str,
    value: &str,
) -> String {
    if format == "JavaScript" {
        value.to_owned()
    } else {
        {
            let null_case = self::render_optional_flatten_absence(format, result, "null");
            {
                let undefined_case =
                    self::render_optional_flatten_absence(format, result, "undefined");
                {
                    let present = self::render_optional_present_value(format, result, "ml_present");
                    {
                        let nested = self::render_optional_match(
                            format,
                            inner,
                            "ml_nested",
                            "ml_present",
                            &(present),
                            &(null_case),
                            &(undefined_case),
                        );
                        self::render_optional_match(
                            format,
                            outer,
                            value,
                            "ml_nested",
                            &(nested),
                            &(null_case),
                            &(undefined_case),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_projection(
    format: &str,
    policy: &str,
    value: &str,
    receiver: &str,
    binder: &str,
    present: &str,
    field: &str,
) -> String {
    if format == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), value.to_owned())),
                    (String::from(")?.["))
                )),
                field.to_owned()
            )),
            (String::from("]"))
        )
    } else if format == "Rust" {
        if policy == "both" {
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
                                (format!("{}{}", (String::from("match ")), value.to_owned())),
                                (String::from(" { crate::MlOptional::Value("))
                            )),
                            binder.to_owned()
                        )),
                        (String::from(") => Some("))
                    )),
                    present.to_owned()
                )),
                (String::from("), _ => None }"))
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
                                receiver.to_owned(),
                                (String::from(".clone().map(|"))
                            )),
                            binder.to_owned()
                        )),
                        (String::from("| "))
                    )),
                    present.to_owned()
                )),
                (String::from(")"))
            )
        }
    } else if format == "Lean" {
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
                            format!(
                                "{}{}",
                                (format!("{}{}", (String::from("some (some ")), binder.to_owned())),
                                (String::from(")"))
                            )
                        } else {
                            format!("{}{}", (String::from("some ")), binder.to_owned())
                        })
                    )),
                    (String::from(" => some "))
                )),
                present.to_owned()
            )),
            (String::from(" | _ => none)"))
        )
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
                            format!(
                                "{}{}",
                                (format!("{}{}", (String::from("Some (Some ")), binder.to_owned())),
                                (String::from(")"))
                            )
                        } else {
                            format!("{}{}", (String::from("Some ")), binder.to_owned())
                        })
                    )),
                    (String::from(" => Some "))
                )),
                present.to_owned()
            )),
            (String::from(" | _ => None end)"))
        )
    } else {
        String::from("")
    }
}
