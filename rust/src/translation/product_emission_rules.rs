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
pub use crate::translation::frontend_rules::ml_array;

#[must_use]
pub fn join_product_items(items: &[String]) -> String {
    {
        let text = String::new();
        {
            let index = 0f64;
            {
                crate::translation::product_emission_rules::ml_join_product_items_loop1(
                    items, text, index,
                )
            }
        }
    }
}

#[must_use]
pub fn render_tuple_value(target: &str, items: &[String], frozen: bool) -> String {
    if (target == "JavaScript") {
        {
            let text = format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("[")),
                    (crate::translation::product_emission_rules::join_product_items(items))
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
                text
            }
        }
    } else if (target == "Rust") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("(")),
                    (crate::translation::product_emission_rules::join_product_items(items))
                )),
                (if items.len() == 1 {
                    String::from(",")
                } else {
                    String::new()
                })
            )),
            (String::from(")"))
        )
    } else {
        String::new()
    }
}

#[must_use]
pub fn render_tuple_projection(target: &str, object: &str, index: &str) -> String {
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
    } else if (target == "Rust") {
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
        String::new()
    }
}

#[must_use]
pub fn accept_tuple_copy_type(kind: &str, fields: &[bool]) -> bool {
    if ((((kind == "bool") || (kind == "fixed")) || (kind == "float")) || (kind == "unit")) {
        true
    } else if kind == "tuple" {
        {
            let index = 0f64;
            {
                let ml_s1 =
                    crate::translation::product_emission_rules::ml_accept_tuple_copy_type_loop2(
                        fields, index,
                    );
                match ml_s1 {
                        crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result::MlAcceptTupleCopyTypeLoop2Done => {
                            true
                        }
                        crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result::MlAcceptTupleCopyTypeLoop2Return(ml_result2) => {
                            ml_result2
                        }
                    }
            }
        }
    } else {
        false
    }
}

#[must_use]
pub fn render_java_script_unit(language: &str) -> String {
    if (language == "JavaScript") {
        String::from("undefined")
    } else {
        String::from("null")
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "the checked JavaScript array-length domain is at most 2^32 - 1, and its loop index starts at zero, advances by one, and is bounded by that length"
)]
#[must_use]
pub fn ml_join_product_items_loop1(items: &[String], mut text: String, mut index: f64) -> String {
    loop {
        if (index < (items.len() as f64)) {
            {
                let text_3 = if (index > (0f64)) {
                    { format!("{}{}", text, (String::from(", "))) }
                } else {
                    text
                };
                {
                    let text_4 = format!(
                        "{}{}",
                        text_3,
                        (crate::translation::product_emission_rules::ml_array::optional_at(
                            items,
                            crate::translation::product_emission_rules::ml_array::number_index(
                                index
                            )
                        )
                        .unwrap_or_else(|| String::from("undefined")))
                    );
                    {
                        let index_2 = (index + 1f64);
                        {
                            (text, index) = (text_4, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { text };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptTupleCopyTypeLoop2Result {
    MlAcceptTupleCopyTypeLoop2Done,
    MlAcceptTupleCopyTypeLoop2Return(bool),
}

#[expect(
    clippy::cast_precision_loss,
    reason = "the checked JavaScript array-length domain is at most 2^32 - 1, and its loop index starts at zero, advances by one, and is bounded by that length"
)]
#[must_use]
pub fn ml_accept_tuple_copy_type_loop2(
    fields: &[bool],
    mut index: f64,
) -> crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result {
    loop {
        return if (index < (fields.len() as f64)) {
            if (crate::translation::product_emission_rules::ml_array::optional_at(
                fields,
                crate::translation::product_emission_rules::ml_array::number_index(index),
            )
            .unwrap_or_default())
            {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result::MlAcceptTupleCopyTypeLoop2Return(false)
            }
        } else {
            crate::translation::product_emission_rules::MlAcceptTupleCopyTypeLoop2Result::MlAcceptTupleCopyTypeLoop2Done
        };
    }
}

const fn ml_main() {}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(ml_main)
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}
