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

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
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

pub fn join_type_names(children: Vec<String>, separator: String) -> String {
    {
        let text = String::from("");
        {
            let at = 0f64;
            {
                let text_2 = crate::translation::structured_type_rules::ml_join_type_names_loop1(
                    children.clone(),
                    separator.clone(),
                    text.clone(),
                    at,
                );
                text_2.clone()
            }
        }
    }
}

pub fn merge_optional_absence(inner: String, outer: String) -> String {
    if (inner == outer) {
        inner.clone()
    } else {
        String::from("both")
    }
}

pub fn accept_type_identity(left: String, right: String) -> bool {
    (left == right)
}

pub fn accept_optional_absence(expected: String, actual: String) -> bool {
    ((expected == "both") || (expected == actual))
}

pub fn render_portable_type_identity(
    kind: String,
    detail: String,
    children: Vec<String>,
) -> String {
    if (kind == "fixed") {
        detail.clone()
    } else {
        if (kind == "data") {
            format!("{}{}", (String::from("data:")), detail)
        } else {
            if ((kind == "array") || (kind == "set")) {
                format!(
                    "{}{}",
                    (format!("{}{}", kind, (String::from(":")))),
                    (crate::translation::structured_type_rules::ml_array::at(
                        &children,
                        crate::translation::structured_type_rules::ml_array::number_index(0f64)
                    ))
                )
            } else {
                if (kind == "option") {
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!("{}{}", (String::from("option:")), detail)),
                            (String::from(":"))
                        )),
                        (crate::translation::structured_type_rules::ml_array::at(
                            &children,
                            crate::translation::structured_type_rules::ml_array::number_index(0f64)
                        ))
                    )
                } else {
                    if (kind == "tuple") {
                        format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (String::from("tuple:[")),
                                (crate::translation::structured_type_rules::join_type_names(
                                    children.clone(),
                                    String::from(",")
                                ))
                            )),
                            (String::from("]"))
                        )
                    } else {
                        if (kind == "record") {
                            format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!("{}{}", (String::from("record:")), detail)),
                                        (String::from("["))
                                    )),
                                    (crate::translation::structured_type_rules::join_type_names(
                                        children.clone(),
                                        String::from(",")
                                    ))
                                )),
                                (String::from("]"))
                            )
                        } else {
                            if (kind == "map") {
                                format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("map:")), (crate::translation::structured_type_rules::ml_array::at(&children, crate::translation::structured_type_rules::ml_array::number_index(0f64))))), (String::from(":")))), (crate::translation::structured_type_rules::ml_array::at(&children, crate::translation::structured_type_rules::ml_array::number_index(1f64))))
                            } else {
                                if (kind == "function") {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("function:(")), (crate::translation::structured_type_rules::join_type_names(children.clone(), String::from(","))))), (String::from("):")))), detail)
                                } else {
                                    kind.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_rust_compound_type(kind: String, detail: String, children: Vec<String>) -> String {
    if ((kind == "array") || (kind == "set")) {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("Vec<")),
                (crate::translation::structured_type_rules::ml_array::at(
                    &children,
                    crate::translation::structured_type_rules::ml_array::number_index(0f64)
                ))
            )),
            (String::from(">"))
        )
    } else {
        if (kind == "option") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (if (detail == "both") {
                        String::from("crate::MlOptional<")
                    } else {
                        String::from("Option<")
                    }),
                    (crate::translation::structured_type_rules::ml_array::at(
                        &children,
                        crate::translation::structured_type_rules::ml_array::number_index(0f64)
                    ))
                )),
                (String::from(">"))
            )
        } else {
            if ((kind == "tuple") || (kind == "record")) {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("(")),
                            (crate::translation::structured_type_rules::join_type_names(
                                children.clone(),
                                String::from(", ")
                            ))
                        )),
                        (if ((children.len() as f64) == (1f64)) {
                            String::from(",")
                        } else {
                            String::from("")
                        })
                    )),
                    (String::from(")"))
                )
            } else {
                if (kind == "map") {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("Vec<(")), (crate::translation::structured_type_rules::ml_array::at(&children, crate::translation::structured_type_rules::ml_array::number_index(0f64))))), (String::from(", ")))), (crate::translation::structured_type_rules::ml_array::at(&children, crate::translation::structured_type_rules::ml_array::number_index(1f64))))), (String::from(")>")))
                } else {
                    if (kind == "function") {
                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("std::rc::Rc<dyn Fn(")), (crate::translation::structured_type_rules::join_type_names(children.clone(), String::from(", "))))), (String::from(") -> ")))), detail)), (String::from(">")))
                    } else {
                        String::from("")
                    }
                }
            }
        }
    }
}

pub fn read_rust_optional_prelude() -> String {
    String::from(
        "/// A nullable value retains the distinct JavaScript null and undefined cases.\n#[derive(Clone, Debug, PartialEq)]\npub enum MlOptional<T> { Value(T), Null, Undefined }",
    )
}

pub fn ml_join_type_names_loop1(
    mut children: Vec<String>,
    mut separator: String,
    mut text: String,
    mut at: f64,
) -> String {
    loop {
        return if (at < (children.len() as f64)) {
            {
                let text_3 = if (at > (0f64)) {
                    {
                        let text_2 = format!("{}{}", text, separator);
                        text_2.clone()
                    }
                } else {
                    text.clone()
                };
                {
                    let text_4 = format!(
                        "{}{}",
                        text_3,
                        (crate::translation::structured_type_rules::ml_array::at(
                            &children,
                            crate::translation::structured_type_rules::ml_array::number_index(at)
                        ))
                    );
                    {
                        let at_2 = (at + 1f64);
                        {
                            (text, at) = (text_4.clone(), at_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

fn ml_main() {}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || ml_main())
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}
