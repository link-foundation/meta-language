// Generated from js/src/translation/primitive-value-rules.js by
// js/scripts/generate-primitive-value-rules.mjs. Do not edit by hand.
/// Apply the shared primitive value decision.
#[must_use]
pub fn accept_object_property_definition(name: &str, computed: bool, colon: bool) -> bool {
    name != "__proto__" || computed || !colon
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn read_absent_number_value(operation: &str, absence: &str) -> String {
    if absence != "null" && absence != "undefined" {
        String::new()
    } else if operation == "arithmetic" {
        if absence == "null" {
            String::from("zero")
        } else {
            String::from("not-a-number")
        }
    } else if operation == "array-index" {
        String::from("not-a-number")
    } else {
        String::new()
    }
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn render_absent_number_expression(target: &str, operation: &str, absence: &str) -> String {
    {
        let value =
            crate::translation::primitive_value_rules::read_absent_number_value(operation, absence);
        if value.is_empty() {
            String::new()
        } else if target == "JavaScript" {
            if value == "zero" {
                String::from("0")
            } else {
                String::from("NaN")
            }
        } else if target == "Rust" {
            if value == "zero" {
                String::from("0f64")
            } else {
                String::from("f64::NAN")
            }
        } else if target == "Lean" {
            if value == "zero" {
                String::from("(0 : Float)")
            } else {
                String::from("(0 / 0 : Float)")
            }
        } else if target == "Rocq" {
            if value == "zero" {
                String::from("0%float")
            } else {
                String::from("PrimFloat.nan")
            }
        } else {
            String::new()
        }
    }
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn read_number_display_refusal(language: &str, kind: &str) -> String {
    if language == "Rust" && kind == "float" {
        String::from(
            "Rust formatting retains negative zero, spells infinities inf and expands exponents; it needs its own operational formatter",
        )
    } else {
        String::new()
    }
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn read_number_display_function(kind: &str, console_style: bool) -> String {
    if kind == "float" {
        if console_style {
            String::from("js_console")
        } else {
            String::from("js_number")
        }
    } else {
        String::new()
    }
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn accept_linked_text_conversion_kind(kind: &str) -> bool {
    kind == "bool"
        || kind == "float"
        || kind == "int"
        || kind == "nat"
        || kind == "fixed"
        || kind == "string"
        || kind == "record"
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn read_scalar_cell_operation(expression: &str) -> String {
    if expression == "cellAllocate" {
        String::from("allocate-reference-cell")
    } else if expression == "cellRead" {
        String::from("read-reference-cell")
    } else if expression == "cellWrite" {
        String::from("write-reference-cell")
    } else {
        String::new()
    }
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn require_data_object_freeze(language: &str, explicit_frozen: bool) -> bool {
    explicit_frozen || language != "JavaScript"
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn accept_object_freeze_operand(expression: &str, readonly_literal: bool) -> bool {
    expression == "ctorObject" || expression == "array" && readonly_literal
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn accept_immutable_product_kind(kind: &str) -> bool {
    kind == "bool" || kind == "float" || kind == "fixed" || kind == "unit" || kind == "tuple"
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn accept_tuple_index(index: f64, length: f64) -> bool {
    (0.0_f64..=9_007_199_254_740_991_f64).contains(&index)
        && (index % 1f64) == 0f64
        && index < length
}

/// Apply the shared primitive value decision.
#[must_use]
pub fn read_tuple_position(index: f64) -> f64 {
    if index == 0f64 { 0f64 } else { index }
}
