// Generated from the complete checked optional kernel and its recorded source dependencies.
// Do not edit by hand.
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
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_source_literal(token: &str, declared: bool) -> String {
    if (token == "null") || ((token == "undefined") && !declared) {
        token.to_owned()
    } else {
        String::new()
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
#[expect(
    clippy::fn_params_excessive_bools,
    reason = "the checked source interface retains independent Boolean and text inputs in their original evaluation order"
)]
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
        String::new()
    } else if current == "option" {
        if (current_absence != actual_absence) && (current_absence != "both") {
            String::from("merge")
        } else {
            String::from("elements")
        }
    } else if recursive {
        String::new()
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
    if expected == "option" {
        if expression == "none" {
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
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_optional_field_operation(kind: &str, element: &str, field: &str) -> String {
    if kind == "option" {
        if ((field == "length") && ((element == "string") || (element == "array")))
            || ((field == "size") && ((element == "set") || (element == "map")))
        {
            String::from("project")
        } else if element == "record" {
            String::from("optional-field")
        } else {
            String::from("unsupported")
        }
    } else {
        String::from("field")
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
                "crate::MlOptional::{}",
                if absence == "null" {
                    String::from("Null")
                } else {
                    String::from("Undefined")
                }
            )
        } else {
            format!("None::<{}>", element.to_owned())
        }
    } else if format == "Lean" {
        if (policy == "both") && (absence == "undefined") {
            String::from("(some none)")
        } else {
            String::from("none")
        }
    } else if format == "Rocq" {
        if (policy == "both") && (absence == "undefined") {
            format!("(Some (@None {}))", element.to_owned())
        } else {
            format!(
                "(@None {})",
                if policy == "both" {
                    format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(option ")), element.to_owned())),
                        (String::from(")"))
                    )
                } else {
                    element.to_owned()
                }
            )
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_present_value(format: &str, policy: &str, value: &str) -> String {
    if format == "JavaScript" {
        value.to_owned()
    } else if format == "Rust" {
        if policy == "both" {
            format!("crate::MlOptional::Value({})", value.to_owned())
        } else {
            format!("Some({})", value.to_owned())
        }
    } else if format == "Lean" {
        if policy == "both" {
            format!("(some (some {}))", value.to_owned())
        } else {
            format!("(some {})", value.to_owned())
        }
    } else if format == "Rocq" {
        if policy == "both" {
            format!("(Some (Some {}))", value.to_owned())
        } else {
            format!("(Some {})", value.to_owned())
        }
    } else {
        String::new()
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
                "match {} {{ crate::MlOptional::Value({}) => {}, crate::MlOptional::Null => {}, crate::MlOptional::Undefined => {} }}",
                value.to_owned(),
                binder.to_owned(),
                present.to_owned(),
                null_case.to_owned(),
                undefined_case.to_owned()
            )
        } else {
            format!(
                "match {} {{ Some({}) => {}, None => {} }}",
                value.to_owned(),
                binder.to_owned(),
                present.to_owned(),
                if policy == "null" {
                    null_case.to_owned()
                } else {
                    undefined_case.to_owned()
                }
            )
        }
    } else if format == "Lean" {
        format!(
            "(match {} with {})",
            value.to_owned(),
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
            }
        )
    } else if format == "Rocq" {
        format!(
            "(match {} with {} end)",
            value.to_owned(),
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
            }
        )
    } else {
        String::new()
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
        format!("({} ?? {})", value.to_owned(), fallback.to_owned())
    } else if format == "Rust" {
        if policy == "both" {
            format!(
                "match {} {{ crate::MlOptional::Value(ml_value) => ml_value, _ => {} }}",
                value.to_owned(),
                fallback.to_owned()
            )
        } else {
            format!(
                "{}.clone().unwrap_or_else(|| {})",
                receiver.to_owned(),
                fallback.to_owned()
            )
        }
    } else if format == "Lean" {
        format!(
            "(match {} with | {} => ml_value | _ => {})",
            value.to_owned(),
            if policy == "both" {
                String::from("some (some ml_value)")
            } else {
                String::from("some ml_value")
            },
            fallback.to_owned()
        )
    } else if format == "Rocq" {
        format!(
            "(match {} with | {} => ml_value | _ => {} end)",
            value.to_owned(),
            if policy == "both" {
                String::from("Some (Some ml_value)")
            } else {
                String::from("Some ml_value")
            },
            fallback.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_flatten_absence(format: &str, policy: &str, absence: &str) -> String {
    if format == "Rust" {
        if policy == "both" {
            format!(
                "crate::MlOptional::{}",
                if absence == "null" {
                    String::from("Null")
                } else {
                    String::from("Undefined")
                }
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
        String::new()
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
        format!("({})?.[{}]", value.to_owned(), field.to_owned())
    } else if format == "Rust" {
        if policy == "both" {
            format!(
                "match {} {{ crate::MlOptional::Value({}) => Some({}), _ => None }}",
                value.to_owned(),
                binder.to_owned(),
                present.to_owned()
            )
        } else {
            format!(
                "{}.clone().map(|{}| {})",
                receiver.to_owned(),
                binder.to_owned(),
                present.to_owned()
            )
        }
    } else if format == "Lean" {
        format!(
            "(match {} with | {} => some {} | _ => none)",
            value.to_owned(),
            if policy == "both" {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("some (some ")), binder.to_owned())),
                    (String::from(")"))
                )
            } else {
                format!("{}{}", (String::from("some ")), binder.to_owned())
            },
            present.to_owned()
        )
    } else if format == "Rocq" {
        format!(
            "(match {} with | {} => Some {} | _ => None end)",
            value.to_owned(),
            if policy == "both" {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("Some (Some ")), binder.to_owned())),
                    (String::from(")"))
                )
            } else {
                format!("{}{}", (String::from("Some ")), binder.to_owned())
            },
            present.to_owned()
        )
    } else {
        String::new()
    }
}

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
        format!("!!({})", value.to_owned())
    } else if kind == "bool" {
        value.to_owned()
    } else if format == "Rust" {
        if kind == "float" {
            format!(
                "{{ let ml_value = {}; ml_value != 0f64 && !ml_value.is_nan() }}",
                value.to_owned()
            )
        } else if (kind == "int") || (kind == "nat") {
            format!("!{}.is_zero()", receiver.to_owned())
        } else if kind == "fixed" {
            format!("({} != 0)", receiver.to_owned())
        } else if kind == "string" {
            format!(
                "!{}{}",
                receiver.to_owned(),
                if utf16 {
                    String::from(".0.is_empty()")
                } else {
                    String::from(".is_empty()")
                }
            )
        } else {
            format!(
                "{{ let _ = {}; {} }}",
                value.to_owned(),
                if kind == "unit" {
                    String::from("false")
                } else {
                    String::from("true")
                }
            )
        }
    } else if format == "Lean" {
        if kind == "float" {
            format!(
                "(let ml_value := {}; ml_value != (0 : Float) && !ml_value.isNaN)",
                value.to_owned()
            )
        } else if ((kind == "int") || (kind == "nat")) || (kind == "fixed") {
            format!("({} != 0)", value.to_owned())
        } else if kind == "string" {
            format!("(!{}.isEmpty)", value.to_owned())
        } else {
            format!(
                "(let ml_ignored := {}; {})",
                value.to_owned(),
                if kind == "unit" {
                    String::from("false")
                } else {
                    String::from("true")
                }
            )
        }
    } else if format == "Rocq" {
        if kind == "float" {
            format!(
                "(let ml_value := {} in andb (negb (PrimFloat.eqb ml_value 0%float)) (negb (PrimFloat.is_nan ml_value)))",
                value.to_owned()
            )
        } else if (kind == "int") || ((kind == "fixed") && signed) {
            format!("(negb (Z.eqb {} 0))", value.to_owned())
        } else if (kind == "nat") || (kind == "fixed") {
            format!("(negb (N.eqb {} 0))", value.to_owned())
        } else if kind == "string" {
            format!("(negb (String.eqb {} EmptyString))", value.to_owned())
        } else {
            format!(
                "(let ml_ignored := {} in {})",
                value.to_owned(),
                if kind == "unit" {
                    String::from("false")
                } else {
                    String::from("true")
                }
            )
        }
    } else {
        String::new()
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
    if kind == "option" {
        if format == "Rust" {
            if policy == "both" {
                format!("crate::MlOptional::Value({})", name.to_owned())
            } else {
                format!("Some({})", name.to_owned())
            }
        } else if format == "Lean" {
            if policy == "both" {
                format!("some (some {})", name.to_owned())
            } else {
                format!("some {}", name.to_owned())
            }
        } else if policy == "both" {
            format!("Some (Some {})", name.to_owned())
        } else {
            format!("Some {}", name.to_owned())
        }
    } else {
        name.to_owned()
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
                "crate::MlOptional::{}",
                if absence == "null" {
                    String::from("Null")
                } else {
                    String::from("Undefined")
                }
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
