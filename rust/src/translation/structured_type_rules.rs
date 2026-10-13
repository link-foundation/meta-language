// Generated from js/src/translation/structured-type-rules.js by
// js/scripts/generate-structured-type-rules.mjs. Do not edit by hand.
// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons
)]

pub use crate::translation::frontend_rules::ml_array;

/// Execute the checked shared structural type decision.
#[must_use]
pub fn join_type_names(children: &[String], separator: &str) -> String {
    {
        let text = String::new();
        {
            let at = 0f64;
            {
                crate::translation::structured_type_rules::ml_join_type_names_loop1(
                    children, separator, text, at,
                )
            }
        }
    }
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn merge_optional_absence(inner: &str, outer: &str) -> String {
    if (inner == outer) {
        inner.to_owned()
    } else {
        String::from("both")
    }
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn accept_type_identity(left: &str, right: &str) -> bool {
    (left == right)
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn accept_optional_absence(expected: &str, actual: &str) -> bool {
    ((expected == "both") || (expected == actual))
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn render_portable_type_identity(kind: &str, detail: &str, children: &[String]) -> String {
    if (kind == "var") {
        format!("{}{}", (String::from("var:")), detail)
    } else if (kind == "fixed") {
        detail.to_owned()
    } else if (kind == "data") {
        format!("{}{}", (String::from("data:")), detail)
    } else if (((kind == "array") || (kind == "set")) || (kind == "cell")) {
        format!(
            "{}{}",
            (format!("{}{}", kind, (String::from(":")))),
            (crate::translation::structured_type_rules::ml_array::optional_at(
                children,
                crate::translation::structured_type_rules::ml_array::number_index(0f64)
            )
            .unwrap_or_else(|| String::from("undefined")))
        )
    } else if (kind == "option") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!("{}{}", (String::from("option:")), detail)),
                (String::from(":"))
            )),
            (crate::translation::structured_type_rules::ml_array::optional_at(
                children,
                crate::translation::structured_type_rules::ml_array::number_index(0f64)
            )
            .unwrap_or_else(|| String::from("undefined")))
        )
    } else if (kind == "tuple") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("tuple:[")),
                (crate::translation::structured_type_rules::join_type_names(children, ","))
            )),
            (String::from("]"))
        )
    } else if (kind == "record") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("record:")), detail)),
                    (String::from("["))
                )),
                (crate::translation::structured_type_rules::join_type_names(children, ","))
            )),
            (String::from("]"))
        )
    } else if (kind == "map") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("map:")),
                    (crate::translation::structured_type_rules::ml_array::optional_at(
                        children,
                        crate::translation::structured_type_rules::ml_array::number_index(0f64)
                    )
                    .unwrap_or_else(|| String::from("undefined")))
                )),
                (String::from(":"))
            )),
            (crate::translation::structured_type_rules::ml_array::optional_at(
                children,
                crate::translation::structured_type_rules::ml_array::number_index(1f64)
            )
            .unwrap_or_else(|| String::from("undefined")))
        )
    } else if (kind == "function") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("function:(")),
                    (crate::translation::structured_type_rules::join_type_names(children, ","))
                )),
                (String::from("):"))
            )),
            detail
        )
    } else {
        kind.to_owned()
    }
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn render_rust_compound_type(kind: &str, detail: &str, children: &[String]) -> String {
    if (kind == "cell") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("std::rc::Rc<std::cell::RefCell<")),
                (crate::translation::structured_type_rules::ml_array::optional_at(
                    children,
                    crate::translation::structured_type_rules::ml_array::number_index(0f64)
                )
                .unwrap_or_else(|| String::from("undefined")))
            )),
            (String::from(">>"))
        )
    } else if ((kind == "array") || (kind == "set")) {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("Vec<")),
                (crate::translation::structured_type_rules::ml_array::optional_at(
                    children,
                    crate::translation::structured_type_rules::ml_array::number_index(0f64)
                )
                .unwrap_or_else(|| String::from("undefined")))
            )),
            (String::from(">"))
        )
    } else if (kind == "option") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (if (detail == "both") {
                    String::from("crate::MlOptional<")
                } else {
                    String::from("Option<")
                }),
                (crate::translation::structured_type_rules::ml_array::optional_at(
                    children,
                    crate::translation::structured_type_rules::ml_array::number_index(0f64)
                )
                .unwrap_or_else(|| String::from("undefined")))
            )),
            (String::from(">"))
        )
    } else if ((kind == "tuple") || (kind == "record")) {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(")),
                    (crate::translation::structured_type_rules::join_type_names(children, ", "))
                )),
                (if (children.len() == 1) {
                    String::from(",")
                } else {
                    String::new()
                })
            )),
            (String::from(")"))
        )
    } else if (kind == "map") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("Vec<(")),
                        (crate::translation::structured_type_rules::ml_array::optional_at(
                            children,
                            crate::translation::structured_type_rules::ml_array::number_index(0f64)
                        )
                        .unwrap_or_else(|| String::from("undefined")))
                    )),
                    (String::from(", "))
                )),
                (crate::translation::structured_type_rules::ml_array::optional_at(
                    children,
                    crate::translation::structured_type_rules::ml_array::number_index(1f64)
                )
                .unwrap_or_else(|| String::from("undefined")))
            )),
            (String::from(")>"))
        )
    } else if (kind == "function") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("std::rc::Rc<dyn Fn(")),
                        (crate::translation::structured_type_rules::join_type_names(
                            children, ", "
                        ))
                    )),
                    (String::from(") -> "))
                )),
                detail
            )),
            (String::from(">"))
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn read_rust_optional_prelude() -> String {
    String::from(
        "/// A nullable value retains the distinct JavaScript null and undefined cases.\n#[derive(Clone, Debug, PartialEq, Eq)]\npub enum MlOptional<T> { Value(T), Null, Undefined }",
    )
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn wrap_portable_reference_identity(identity: &str, reference: bool) -> String {
    if reference {
        format!("{}{}", (String::from("reference:")), identity)
    } else {
        identity.to_owned()
    }
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn render_rust_reference_type(type_: &str, reference: bool) -> String {
    if reference {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("std::rc::Rc<std::cell::RefCell<")),
                type_
            )),
            (String::from(">>"))
        )
    } else {
        type_.to_owned()
    }
}

/// Execute the checked shared structural type decision.
#[must_use]
pub fn ml_join_type_names_loop1(
    children: &[String],
    separator: &str,
    mut text: String,
    mut at: f64,
) -> String {
    loop {
        return if (at
            < (children
                .len()
                .to_string()
                .parse::<f64>()
                .expect("a usize length is a finite binary64 number")))
        {
            {
                let text_3 = if (at > (0f64)) {
                    { format!("{text}{separator}") }
                } else {
                    text
                };
                {
                    let text_4 = format!(
                        "{}{}",
                        text_3,
                        (crate::translation::structured_type_rules::ml_array::optional_at(
                            children,
                            crate::translation::structured_type_rules::ml_array::number_index(at)
                        )
                        .unwrap_or_else(|| String::from("undefined")))
                    );
                    {
                        let at_2 = (at + 1f64);
                        {
                            (text, at) = (text_4, at_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            text
        };
    }
}
