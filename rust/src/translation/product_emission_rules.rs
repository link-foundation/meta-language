// Generated from js/src/translation/product-emission-rules.js by
// js/scripts/generate-product-emission-rules.mjs. Do not edit by hand.
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

pub fn join_product_items(items: Vec<String>) -> String {
    {
        let text = String::from("");
        {
            let index = 0f64;
            {
                let text_2 =
                    crate::translation::product_emission_rules::ml_join_product_items_loop1(
                        items.clone(),
                        text.clone(),
                        index,
                    );
                text_2.clone()
            }
        }
    }
}

pub fn render_tuple_value(target: String, items: Vec<String>, frozen: bool) -> String {
    if (target == "JavaScript") {
        {
            let text = format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("[")),
                    (crate::translation::product_emission_rules::join_product_items(items.clone()))
                )),
                (String::from("]"))
            );
            if frozen {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("Object.freeze(")), text)),
                    (String::from(")"))
                )
            } else {
                text.clone()
            }
        }
    } else {
        if (target == "Rust") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("(")),
                        (crate::translation::product_emission_rules::join_product_items(
                            items.clone()
                        ))
                    )),
                    (if ((items.len() as f64) == (1f64)) {
                        String::from(",")
                    } else {
                        String::from("")
                    })
                )),
                (String::from(")"))
            )
        } else {
            String::from("")
        }
    }
}

pub fn render_tuple_projection(target: String, object: String, index: String) -> String {
    if (target == "JavaScript") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), object)),
                    (String::from(")["))
                )),
                index
            )),
            (String::from("]"))
        )
    } else {
        if (target == "Rust") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(")), object)),
                    (String::from(")."))
                )),
                index
            )
        } else {
            String::from("")
        }
    }
}

pub fn accept_tuple_copy_type(kind: String, fields: Vec<bool>) -> bool {
    if ((((kind == "bool") || (kind == "fixed")) || (kind == "float")) || (kind == "unit")) {
        true
    } else {
        if (kind != "tuple") {
            false
        } else {
            {
                let index = 0f64;
                {
                    let ml_s1 =
                        crate::translation::product_emission_rules::ml_accept_tuple_copy_type_loop2(
                            fields.clone(),
                            index,
                        );
                    match ml_s1.clone() {
                        crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result::MlAcceptTupleCopyTypeLoop2Done => {
                            true
                        }
                        crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result::MlAcceptTupleCopyTypeLoop2Return(ml_result2) => {
                            ml_result2
                        }
                    }
                }
            }
        }
    }
}

pub fn render_java_script_unit(language: String) -> String {
    if (language == "JavaScript") {
        String::from("undefined")
    } else {
        String::from("null")
    }
}

pub fn ml_join_product_items_loop1(
    mut items: Vec<String>,
    mut text: String,
    mut index: f64,
) -> String {
    loop {
        return if (index < (items.len() as f64)) {
            {
                let text_3 = if (index > (0f64)) {
                    {
                        let text_2 = format!("{}{}", text, (String::from(", ")));
                        text_2.clone()
                    }
                } else {
                    text.clone()
                };
                {
                    let text_4 = format!(
                        "{}{}",
                        text_3,
                        (match crate::translation::product_emission_rules::ml_array::optional_at(
                            &items,
                            crate::translation::product_emission_rules::ml_array::number_index(
                                index
                            )
                        ) {
                            Some(ml_present) => ml_present.clone(),
                            None => String::from("undefined"),
                        })
                    );
                    {
                        let index_2 = (index + 1f64);
                        {
                            (text, index) = (text_4.clone(), index_2);
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptTupleCopyTypeLoop2Result {
    MlAcceptTupleCopyTypeLoop2Done,
    MlAcceptTupleCopyTypeLoop2Return(bool),
}

pub fn ml_accept_tuple_copy_type_loop2(
    mut fields: Vec<bool>,
    mut index: f64,
) -> crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result {
    loop {
        return if (index < (fields.len() as f64)) {
            if !(match crate::translation::product_emission_rules::ml_array::optional_at(
                &fields,
                crate::translation::product_emission_rules::ml_array::number_index(index),
            ) {
                Some(ml_present) => ml_present,
                None => false,
            }) {
                crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result::MlAcceptTupleCopyTypeLoop2Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result::MlAcceptTupleCopyTypeLoop2Done
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
