// Generated from js/src/translation/primitive-value-rules.js by
// js/scripts/generate-primitive-value-rules.mjs. Do not edit by hand.
// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons
)]

pub fn read_absent_number_value(operation: String, absence: String) -> String {
    if ((absence != "null") && (absence != "undefined")) {
        String::from("")
    } else {
        if (operation == "arithmetic") {
            if (absence == "null") {
                String::from("zero")
            } else {
                String::from("not-a-number")
            }
        } else {
            if (operation == "array-index") {
                String::from("not-a-number")
            } else {
                String::from("")
            }
        }
    }
}

pub fn render_absent_number_expression(
    target: String,
    operation: String,
    absence: String,
) -> String {
    {
        let value = crate::translation::primitive_value_rules::read_absent_number_value(
            operation.clone(),
            absence.clone(),
        );
        if (value == "") {
            String::from("")
        } else {
            if (target == "JavaScript") {
                if (value == "zero") {
                    String::from("0")
                } else {
                    String::from("NaN")
                }
            } else {
                if (target == "Rust") {
                    if (value == "zero") {
                        String::from("0f64")
                    } else {
                        String::from("f64::NAN")
                    }
                } else {
                    if (target == "Lean") {
                        if (value == "zero") {
                            String::from("(0 : Float)")
                        } else {
                            String::from("(0 / 0 : Float)")
                        }
                    } else {
                        if (target == "Rocq") {
                            if (value == "zero") {
                                String::from("0%float")
                            } else {
                                String::from("PrimFloat.nan")
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

pub fn read_number_display_refusal(language: String, kind: String) -> String {
    if ((language == "Rust") && (kind == "float")) {
        String::from(
            "Rust formatting retains negative zero, spells infinities inf and expands exponents; it needs its own operational formatter",
        )
    } else {
        String::from("")
    }
}

pub fn read_number_display_function(kind: String, console_style: bool) -> String {
    if (kind == "float") {
        if console_style {
            String::from("js_console")
        } else {
            String::from("js_number")
        }
    } else {
        String::from("")
    }
}

pub fn read_scalar_cell_operation(expression: String) -> String {
    if (expression == "cellAllocate") {
        String::from("allocate-reference-cell")
    } else {
        if (expression == "cellRead") {
            String::from("read-reference-cell")
        } else {
            if (expression == "cellWrite") {
                String::from("write-reference-cell")
            } else {
                String::from("")
            }
        }
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
