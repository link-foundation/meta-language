// Generated from the checked JavaScript array value decisions. Do not edit.
/// Read the shared array value decision.
#[must_use]
pub fn select_optional_array_read(
    language: &str,
    optional: bool,
    proven: bool,
    expected_optional: bool,
) -> bool {
    (optional || ((language == "JavaScript") && !proven)) || expected_optional
}

/// Read the shared array value decision.
#[must_use]
pub fn select_optional_array_index_number(language: &str, kind: &str, element: &str) -> bool {
    ((language == "JavaScript") && (kind == "option"))
        && ((element == "float") || (element == "unit"))
}

/// Read the shared array value decision.
#[must_use]
pub fn read_array_iterable_form(kind: &str, reference: bool) -> String {
    if kind == "array" {
        String::from("array")
    } else if kind == "string" {
        String::from("string")
    } else if reference && ((kind == "set") || (kind == "map")) {
        String::from("mutable")
    } else if kind == "set" {
        String::from("set")
    } else if kind == "map" {
        String::from("map")
    } else {
        String::new()
    }
}

/// Read the shared array value decision.
#[must_use]
pub fn read_reference_mutation_diagnostic(field: &str) -> String {
    if field == "feature" {
        String::from("reference mutation")
    } else if field == "reason" {
        String::from(
            "this mutation requires its shared reference and slot operations to be represented",
        )
    } else {
        String::new()
    }
}

/// Read the shared array value decision.
#[must_use]
pub fn render_optional_array_read(
    target: &str,
    kind: &str,
    values: &str,
    position: &str,
) -> String {
    if target == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), values)),
                    (String::from(")["))
                )),
                position
            )),
            (String::from("]"))
        )
    } else if target == "Rust" {
        {
            let index = if kind == "float" {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("crate::ml_array::number_index(")),
                        position
                    )),
                    (String::from(")"))
                )
            } else {
                format!("{}{}", position, (String::from(".to_index()")))
            };
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("crate::ml_array::optional_at(")),
                            values
                        )),
                        (String::from(", "))
                    )),
                    index
                )),
                (String::from(")"))
            )
        }
    } else if target == "Lean" {
        if kind == "float" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(let ml_index := ")), position)),
                        (String::from(
                            "; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then "
                        ))
                    )),
                    values
                )),
                (String::from("[ml_index.toUInt64.toNat]? else none)"))
            )
        } else if kind == "nat" {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(")), values)),
                        (String::from("["))
                    )),
                    position
                )),
                (String::from("]?)"))
            )
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(let ml_index := ")), position)),
                        (String::from("; if ml_index < 0 then none else "))
                    )),
                    values
                )),
                (String::from("[ml_index.toNat]?)"))
            )
        }
    } else {
        {
            let index_2 = if kind == "float" {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(ml_float_index ")), position)),
                    (String::from(")"))
                )
            } else if kind == "nat" {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(Z.of_N ")), position)),
                    (String::from(")"))
                )
            } else {
                position.to_owned()
            };
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(let ml_index := ")), index_2)),
                        (String::from(" in if Z.ltb ml_index 0 then None else List.nth_error "))
                    )),
                    values
                )),
                (String::from(" (Z.to_nat ml_index))"))
            )
        }
    }
}

/// Read the shared array value decision.
#[must_use]
pub fn render_string_point_iteration(target: &str, utf16: bool, value: &str) -> String {
    if target == "JavaScript" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("[...")), value)),
            (String::from("]"))
        )
    } else if target == "Rust" {
        if utf16 {
            format!("{}{}", value, (String::from(".code_points()")))
        } else {
            format!(
                "{}{}",
                value,
                (String::from(
                    ".chars().map(|ml_character| ml_character.to_string()).collect::<Vec<String>>()"
                ))
            )
        }
    } else if target == "Lean" {
        format!(
            "{}{}",
            (format!("{}{}", (String::from("((")), value)),
            (String::from(
                ".toList.map (fun ml_character => String.singleton ml_character)).toArray)"
            ))
        )
    } else {
        String::new()
    }
}

/// Read the shared array value decision.
#[must_use]
pub fn render_collection_iteration(
    target: &str,
    map: bool,
    operation: &str,
    value: &str,
    key_type: &str,
    value_type: &str,
) -> String {
    if target == "JavaScript" {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("[...(")), value)),
                    (String::from(")."))
                )),
                operation
            )),
            (String::from("()]"))
        )
    } else if target == "Rust" {
        if operation == "entries" {
            if map {
                value.to_owned()
            } else {
                format!(
                    "{}{}",
                    value,
                    (String::from(
                        ".iter().map(|ml_value| (ml_value.to_owned(), ml_value.to_owned())).collect::<Vec<_>>()"
                    ))
                )
            }
        } else if map {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        value,
                        (String::from(".iter().map(|ml_pair| ml_pair."))
                    )),
                    (if operation == "keys" {
                        String::from("0")
                    } else {
                        String::from("1")
                    })
                )),
                (String::from(".clone()).collect::<Vec<_>>()"))
            )
        } else {
            value.to_owned()
        }
    } else if target == "Lean" {
        if operation == "entries" {
            if map {
                value.to_owned()
            } else {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), value)),
                    (String::from(".map (fun ml_value => (ml_value, ml_value)))"))
                )
            }
        } else if map {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(")), value)),
                        (String::from(".map (fun ml_pair => ml_pair."))
                    )),
                    (if operation == "keys" {
                        String::from("1")
                    } else {
                        String::from("2")
                    })
                )),
                (String::from("))"))
            )
        } else {
            value.to_owned()
        }
    } else if operation == "entries" {
        if map {
            value.to_owned()
        } else {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(List.map (fun ml_value => (ml_value, ml_value)) ")),
                    value
                )),
                (String::from(")"))
            )
        }
    } else if map {
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
                                        (String::from("(List.map (@")),
                                        (if operation == "keys" {
                                            String::from("fst")
                                        } else {
                                            String::from("snd")
                                        })
                                    )),
                                    (String::from(" "))
                                )),
                                key_type
                            )),
                            (String::from(" "))
                        )),
                        value_type
                    )),
                    (String::from(") "))
                )),
                value
            )),
            (String::from(")"))
        )
    } else {
        value.to_owned()
    }
}
