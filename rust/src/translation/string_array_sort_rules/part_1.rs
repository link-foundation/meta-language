// Generated from complete js/src/translation/string-array-sort-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_string_array_copy_sort(
    kind: &str,
    operation: &str,
    receiver: &str,
    element: &str,
    reference: bool,
    count: f64,
) -> bool {
    (((((kind == "method") && (operation == "toSorted")) && (receiver == "array"))
        && (element == "string"))
        && !reference)
        && (count == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_string_array_copy_sort_units(kind: &str, operation: &str) -> bool {
    (kind == "method") && (operation == "toSorted")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_string_array_copy_sort(target: &str, object: &str, units: bool) -> String {
    if target == "JavaScript" {
        format!("({}).toSorted()", object.to_owned())
    } else if target == "Rust" {
        {
            let comparison = if units {
                String::from("ml_sort_left.0.cmp(&ml_sort_right.0)")
            } else {
                String::from("ml_sort_left.encode_utf16().cmp(ml_sort_right.encode_utf16())")
            };
            format!(
                "{{ let mut ml_sorted_values = {}; ml_sorted_values.sort_by(|ml_sort_left, ml_sort_right| {comparison}); ml_sorted_values }}",
                object.to_owned()
            )
        }
    } else {
        String::new()
    }
}
