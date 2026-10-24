// Generated from complete js/src/translation/javascript-comparison-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_comparison_primitive(kind: &str) -> bool {
    (((((kind == "float") || (kind == "int")) || (kind == "nat")) || (kind == "bool"))
        || (kind == "string"))
        || (kind == "unit")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_primitive_comparison(left: &str, right: &str) -> bool {
    if (left == "option") || (right == "option") {
        true
    } else if (left == right) && (left != "unit") {
        false
    } else {
        !(((left == "int") || (left == "nat")) && ((right == "int") || (right == "nat")))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_comparison_absent_number(absence: &str) -> String {
    if absence == "null" {
        String::from("0")
    } else if absence == "undefined" {
        String::from("NaN")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_comparison_unit_absence(kind: &str) -> String {
    if kind == "unit" {
        String::from("undefined")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_comparison_inference_step(left: &str, right: &str, force: bool) -> String {
    if (left == "var") || (right == "var") {
        if force {
            String::from("unify")
        } else {
            String::from("defer")
        }
    } else if (self::accept_java_script_comparison_primitive(left) || (left == "option"))
        && (self::accept_java_script_comparison_primitive(right) || (right == "option"))
    {
        String::from("independent")
    } else {
        String::from("unify")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_comparison_form(
    operation: &str,
    left: &str,
    right: &str,
) -> String {
    if (operation == "eq") || (operation == "ne") {
        String::from("strict")
    } else {
        {
            let a = (left == "int") || (left == "nat");
            {
                let b = (right == "int") || (right == "nat");
                if (left == "string") && (right == "string") {
                    String::from("string")
                } else if (left == "string") && b {
                    String::from("string-integer")
                } else if a && (right == "string") {
                    String::from("integer-string")
                } else if a && b {
                    String::from("integer")
                } else if (left == "float") && (right == "float") {
                    String::from("number")
                } else if (left == "float") && b {
                    String::from("number-integer")
                } else if a && (right == "float") {
                    String::from("integer-number")
                } else {
                    String::from("coerce")
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_comparison_accepted_orders(operation: &str, reverse: bool) -> Vec<f64> {
    if operation == "eq" {
        vec![0f64]
    } else if operation == "ne" {
        vec![-1f64, 1f64, 2f64]
    } else if operation == "lt" {
        vec![if reverse { 1f64 } else { -1f64 }]
    } else if operation == "gt" {
        vec![if reverse { -1f64 } else { 1f64 }]
    } else if operation == "le" {
        vec![if reverse { 1f64 } else { -1f64 }, 0f64]
    } else if operation == "ge" {
        vec![if reverse { -1f64 } else { 1f64 }, 0f64]
    } else {
        Vec::<f64>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_loose_equality_operation(symbol: &str) -> String {
    if symbol == "==" {
        String::from("testJavaScriptPrimitiveLooseEquality")
    } else if symbol == "!=" {
        String::from("testJavaScriptPrimitiveLooseInequality")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_loose_equality_operation(operation: &str) -> bool {
    (operation == "testJavaScriptPrimitiveLooseEquality")
        || (operation == "testJavaScriptPrimitiveLooseInequality")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_loose_equality_negation(operation: &str) -> bool {
    operation == "testJavaScriptPrimitiveLooseInequality"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_loose_equality_form(left: &str, right: &str) -> String {
    if left == right {
        String::from("strict")
    } else {
        {
            let a = (left == "int") || (left == "nat");
            {
                let b = (right == "int") || (right == "nat");
                if a && b {
                    String::from("integer")
                } else if (left == "bool") || ((left == "string") && (right == "float")) {
                    String::from("left-number")
                } else if (right == "bool") || ((right == "string") && (left == "float")) {
                    String::from("right-number")
                } else if (left == "string") && b {
                    String::from("string-integer")
                } else if a && (right == "string") {
                    String::from("integer-string")
                } else if (left == "float") && b {
                    String::from("number-integer")
                } else if a && (right == "float") {
                    String::from("integer-number")
                } else {
                    String::from("different")
                }
            }
        }
    }
}
