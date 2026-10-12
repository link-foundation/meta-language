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
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
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
    fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
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
            if (((kind == "array") || (kind == "set")) || (kind == "cell")) {
                format!(
                    "{}{}",
                    (format!("{}{}", kind, (String::from(":")))),
                    (match crate::translation::structured_type_rules::ml_array::optional_at(
                        &children,
                        crate::translation::structured_type_rules::ml_array::number_index(0f64)
                    ) {
                        Some(ml_present) => ml_present.clone(),
                        None => String::from("undefined"),
                    })
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
                        (match crate::translation::structured_type_rules::ml_array::optional_at(
                            &children,
                            crate::translation::structured_type_rules::ml_array::number_index(0f64)
                        ) {
                            Some(ml_present) => ml_present.clone(),
                            None => String::from("undefined"),
                        })
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
                                format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("map:")), (match crate::translation::structured_type_rules::ml_array::optional_at(&children, crate::translation::structured_type_rules::ml_array::number_index(0f64)) { Some(ml_present) => ml_present.clone(), None => String::from("undefined") }))), (String::from(":")))), (match crate::translation::structured_type_rules::ml_array::optional_at(&children, crate::translation::structured_type_rules::ml_array::number_index(1f64)) { Some(ml_present) => ml_present.clone(), None => String::from("undefined") }))
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
    if (kind == "cell") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("std::rc::Rc<std::cell::RefCell<")),
                (match crate::translation::structured_type_rules::ml_array::optional_at(
                    &children,
                    crate::translation::structured_type_rules::ml_array::number_index(0f64)
                ) {
                    Some(ml_present) => ml_present.clone(),
                    None => String::from("undefined"),
                })
            )),
            (String::from(">>"))
        )
    } else {
        if ((kind == "array") || (kind == "set")) {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("Vec<")),
                    (match crate::translation::structured_type_rules::ml_array::optional_at(
                        &children,
                        crate::translation::structured_type_rules::ml_array::number_index(0f64)
                    ) {
                        Some(ml_present) => ml_present.clone(),
                        None => String::from("undefined"),
                    })
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
                        (match crate::translation::structured_type_rules::ml_array::optional_at(
                            &children,
                            crate::translation::structured_type_rules::ml_array::number_index(0f64)
                        ) {
                            Some(ml_present) => ml_present.clone(),
                            None => String::from("undefined"),
                        })
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
                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("Vec<(")), (match crate::translation::structured_type_rules::ml_array::optional_at(&children, crate::translation::structured_type_rules::ml_array::number_index(0f64)) { Some(ml_present) => ml_present.clone(), None => String::from("undefined") }))), (String::from(", ")))), (match crate::translation::structured_type_rules::ml_array::optional_at(&children, crate::translation::structured_type_rules::ml_array::number_index(1f64)) { Some(ml_present) => ml_present.clone(), None => String::from("undefined") }))), (String::from(")>")))
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
}

pub fn read_rust_optional_prelude() -> String {
    String::from(
        "/// A nullable value retains the distinct JavaScript null and undefined cases.\n#[derive(Clone, Debug, PartialEq, Eq)]\npub enum MlOptional<T> { Value(T), Null, Undefined }",
    )
}

pub fn wrap_portable_reference_identity(identity: String, reference: bool) -> String {
    if reference {
        format!("{}{}", (String::from("reference:")), identity)
    } else {
        identity.clone()
    }
}

pub fn render_rust_reference_type(type_: String, reference: bool) -> String {
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
        type_.clone()
    }
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
                        (match crate::translation::structured_type_rules::ml_array::optional_at(
                            &children,
                            crate::translation::structured_type_rules::ml_array::number_index(at)
                        ) {
                            Some(ml_present) => ml_present.clone(),
                            None => String::from("undefined"),
                        })
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
