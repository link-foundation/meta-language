// Generated from complete js/src/translation/collection-access-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_access_result(kind: &str, operation: &str) -> String {
    if (kind != "map") && (kind != "set") {
        String::new()
    } else if operation == "has" {
        String::from("boolean")
    } else if (kind == "map") && (operation == "get") {
        String::from("optional value")
    } else if operation == "keys" {
        String::from("keys")
    } else if operation == "values" {
        String::from("values")
    } else if operation == "entries" {
        String::from("entries")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_access_argument_count(operation: &str) -> f64 {
    if (operation == "get") || (operation == "has") {
        1f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_size_expression(target: &str, object: &str) -> String {
    if target == "JavaScript" {
        format!("({}).size", object.to_owned())
    } else if target == "Rust" {
        format!("({}.len() as f64)", object.to_owned())
    } else if target == "Lean" {
        format!("(Float.ofNat {}.size)", object.to_owned())
    } else if target == "Rocq" {
        format!(
            "(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length {}))))",
            object.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn render_collection_read_expression(
    target: &str,
    kind: &str,
    operation: &str,
    object: &str,
    search: &str,
    float_key: bool,
    comparison: &str,
    key_type: &str,
    value_type: &str,
) -> String {
    if target == "JavaScript" {
        format!(
            "({}).{}({})",
            object.to_owned(),
            operation.to_owned(),
            search.to_owned()
        )
    } else if target == "Rust" {
        {
            let equal = if float_key {
                String::from("(ml_key == &ml_search || (ml_key.is_nan() && ml_search.is_nan()))")
            } else {
                String::from("ml_key == &ml_search")
            };
            {
                let bindings = if kind == "map" {
                    String::from("(ml_key, ml_value)")
                } else {
                    String::from("ml_key")
                };
                {
                    let action = if operation == "get" {
                        format!(
                            "find_map(|{bindings}| if {equal} {{ Some(ml_value.clone()) }} else {{ None }})"
                        )
                    } else {
                        format!("any(|{bindings}| {equal})")
                    };
                    format!(
                        "{{ let ml_collection = {}; let ml_search = {}; ml_collection.iter().{action} }}",
                        object.to_owned(),
                        search.to_owned()
                    )
                }
            }
        }
    } else if target == "Lean" {
        {
            let key = if kind == "map" {
                String::from("ml_value.1")
            } else {
                String::from("ml_value")
            };
            {
                let equal_2 = if float_key {
                    format!("({key} == ml_search || ({key}.isNaN && ml_search.isNaN))")
                } else {
                    format!("({key} == ml_search)")
                };
                {
                    let result = if operation == "get" {
                        format!(
                            "((ml_values.toList.find? (fun ml_value => {equal_2})).map (fun ml_pair => ml_pair.2))"
                        )
                    } else {
                        format!("(ml_values.any (fun ml_value => {equal_2}))")
                    };
                    format!(
                        "(let ml_values := {}; let ml_search := {}; {result})",
                        object.to_owned(),
                        search.to_owned()
                    )
                }
            }
        }
    } else if target == "Rocq" {
        {
            let key_2 = if kind == "map" {
                String::from("(fst ml_value)")
            } else {
                String::from("ml_value")
            };
            {
                let equal_3 = if float_key {
                    format!(
                        "(orb {} (andb (PrimFloat.is_nan {key_2}) (PrimFloat.is_nan ml_search)))",
                        comparison.to_owned()
                    )
                } else {
                    comparison.to_owned()
                };
                {
                    let result_2 = if operation == "get" {
                        format!(
                            "(option_map (@snd {} {}) (List.find (fun ml_value => {equal_3}) ml_values))",
                            key_type.to_owned(),
                            value_type.to_owned()
                        )
                    } else {
                        format!("(List.existsb (fun ml_value => {equal_3}) ml_values)")
                    };
                    format!(
                        "(let ml_values := {} in let ml_search := {} in {result_2})",
                        object.to_owned(),
                        search.to_owned()
                    )
                }
            }
        }
    } else {
        String::new()
    }
}
