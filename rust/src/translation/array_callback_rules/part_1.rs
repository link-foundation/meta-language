// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_result_form(operation: &str) -> String {
    if operation == "map" {
        String::from("mapped array")
    } else if operation == "filter" {
        String::from("receiver")
    } else if operation == "find" {
        String::from("optional element")
    } else if operation == "findIndex" {
        String::from("float")
    } else if (operation == "some") || (operation == "every") {
        String::from("bool")
    } else if operation == "flatMap" {
        String::from("callback array")
    } else if operation == "forEach" {
        String::from("unit")
    } else if operation == "reduce" {
        String::from("accumulator")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_expected_result_form(operation: &str) -> String {
    if ((((operation == "filter") || (operation == "find")) || (operation == "findIndex"))
        || (operation == "some"))
        || (operation == "every")
    {
        String::from("bool")
    } else if operation == "reduce" {
        String::from("accumulator")
    } else if operation == "flatMap" {
        String::from("array")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_maximum_arguments(operation: &str) -> f64 {
    if operation == "reduce" {
        2f64
    } else if (self::read_array_callback_result_form(operation)).is_empty() {
        -1f64
    } else {
        1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_signature_error(
    operation: &str,
    kind: &str,
    declared_count: f64,
    provided_count: f64,
    parameters_match: bool,
    result_matches: bool,
    expected_result: &str,
) -> String {
    if kind == "function" {
        if (declared_count > provided_count) || !parameters_match {
            format!(".{}() callback parameters differ", operation.to_owned())
        } else if (!(expected_result).is_empty()) && !result_matches {
            format!(
                ".{}() callback result must be {}",
                operation.to_owned(),
                expected_result.to_owned()
            )
        } else {
            String::new()
        }
    } else {
        format!(".{}() requires a function", operation.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_parameter_role(operation: &str, position: f64) -> String {
    if operation == "reduce" {
        if position == 0f64 {
            String::from("accumulator")
        } else if position == 1f64 {
            String::from("element")
        } else if position == 2f64 {
            String::from("index")
        } else if position == 3f64 {
            String::from("receiver")
        } else {
            String::new()
        }
    } else if position == 0f64 {
        String::from("element")
    } else if position == 1f64 {
        String::from("index")
    } else if position == 2f64 {
        String::from("receiver")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_argument(
    operation: &str,
    position: f64,
    suffix: &str,
    clone_element: bool,
) -> String {
    {
        let role = self::read_array_callback_parameter_role(operation, position);
        if role == "accumulator" {
            format!("ml_accumulator{}", suffix.to_owned())
        } else if role == "element" {
            format!(
                "ml_value{}{}",
                suffix.to_owned(),
                if clone_element {
                    String::from(".clone()")
                } else {
                    String::new()
                }
            )
        } else if role == "index" {
            format!("(ml_index{} as f64)", suffix.to_owned())
        } else if role == "receiver" {
            format!("ml_values{}.clone()", suffix.to_owned())
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_call(
    operation: &str,
    parameter_count: f64,
    suffix: &str,
    clone_element: bool,
) -> String {
    {
        let arguments_text = String::new();
        {
            let position = 0f64;
            {
                let maximum = if operation == "reduce" { 4f64 } else { 3f64 };
                {
                    let arguments_text_2 = self::ml_render_array_callback_call_loop1(
                        operation,
                        parameter_count,
                        suffix,
                        clone_element,
                        arguments_text,
                        position,
                        maximum,
                    );
                    format!("ml_callback{}({arguments_text_2})", suffix.to_owned())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_expression(
    operation: &str,
    object: &str,
    callback: &str,
    parameter_count: f64,
    suffix: &str,
    initial: &str,
    has_initial: bool,
) -> String {
    {
        let values = format!("ml_values{}", suffix.to_owned());
        {
            let index = format!("ml_index{}", suffix.to_owned());
            {
                let value = format!("ml_value{}", suffix.to_owned());
                {
                    let call =
                        self::render_array_callback_call(operation, parameter_count, suffix, false);
                    {
                        let head = format!(
                            "let {values} = {}; let ml_callback{} = {};",
                            object.to_owned(),
                            suffix.to_owned(),
                            callback.to_owned()
                        );
                        {
                            let iterator = format!("{values}.iter().cloned().enumerate()");
                            if (operation == "some") || (operation == "every") {
                                format!(
                                    "{{ {head} {iterator}.{}(|({index}, {value})| {call}) }}",
                                    if operation == "some" {
                                        String::from("any")
                                    } else {
                                        String::from("all")
                                    }
                                )
                            } else if operation == "findIndex" {
                                format!(
                                    "{{ {head} {iterator}.position(|({index}, {value})| {call}).map_or(-1f64, |ml_position| ml_position as f64) }}"
                                )
                            } else if operation == "flatMap" {
                                format!(
                                    "{{ {head} {iterator}.flat_map(|({index}, {value})| {call}).collect::<Vec<_>>() }}"
                                )
                            } else if (operation == "filter") || (operation == "find") {
                                {
                                    let predicate = self::render_array_callback_call(
                                        operation,
                                        parameter_count,
                                        suffix,
                                        true,
                                    );
                                    if operation == "filter" {
                                        format!(
                                            "{{ {head} {iterator}.filter_map(|({index}, {value})| if {predicate} {{ Some({value}) }} else {{ None }}).collect::<Vec<_>>() }}"
                                        )
                                    } else {
                                        format!(
                                            "{{ {head} {iterator}.find_map(|({index}, {value})| if {predicate} {{ Some({value}) }} else {{ None }}) }}"
                                        )
                                    }
                                }
                            } else if operation == "forEach" {
                                format!(
                                    "{{ {head} for ({index}, {value}) in {iterator} {{ {call}; }} () }}"
                                )
                            } else if operation == "reduce" {
                                if has_initial {
                                    format!(
                                        "{{ {head} {iterator}.fold({}, |ml_accumulator{}, ({index}, {value})| {call}) }}",
                                        initial.to_owned(),
                                        suffix.to_owned()
                                    )
                                } else {
                                    format!(
                                        "{{ {head} let mut ml_iterator{} = {iterator}; let ml_initial{} = ml_iterator{}.next().map(|(_, ml_first)| ml_first).expect(\"Reduce of empty array with no initial value\"); ml_iterator{}.fold(ml_initial{}, |ml_accumulator{}, ({index}, {value})| {call}) }}",
                                        suffix.to_owned(),
                                        suffix.to_owned(),
                                        suffix.to_owned(),
                                        suffix.to_owned(),
                                        suffix.to_owned(),
                                        suffix.to_owned()
                                    )
                                }
                            } else {
                                String::new()
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_argument_count_error(operation: &str, count: f64) -> String {
    {
        let maximum = self::read_array_callback_maximum_arguments(operation);
        if (maximum < 0f64) || ((count >= 1f64) && (count <= maximum)) {
            String::new()
        } else {
            {
                let expected = if maximum == 1f64 {
                    String::from("1")
                } else {
                    String::from("1 to 2")
                };
                format!(".{}() expects {expected} arguments", operation.to_owned())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_checked_result_context(operation: &str) -> String {
    {
        let expected = self::read_array_callback_expected_result_form(operation);
        if expected == "array" {
            String::new()
        } else {
            expected
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_result_error(operation: &str, result_kind: &str) -> String {
    if (operation == "flatMap") && (result_kind != "array") {
        String::from("flatMap callback must return an array")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_callback_checked_argument_position(
    operation: &str,
    has_initial: bool,
    position: f64,
) -> f64 {
    if (operation == "reduce") && has_initial {
        if position == 0f64 {
            1f64
        } else if position == 1f64 {
            0f64
        } else {
            -1f64
        }
    } else if position == 0f64 {
        0f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_array_callback_reference_expression(
    operation: &str,
    object: &str,
    callback: &str,
    parameter_count: f64,
    suffix: &str,
    reference_result: bool,
) -> String {
    if (operation != "flatMap") || !reference_result {
        String::new()
    } else {
        {
            let values = format!("ml_values{}", suffix.to_owned());
            {
                let index = format!("ml_index{}", suffix.to_owned());
                {
                    let value = format!("ml_value{}", suffix.to_owned());
                    {
                        let result = format!("ml_callback_values{}", suffix.to_owned());
                        {
                            let call = self::render_array_callback_call(
                                operation,
                                parameter_count,
                                suffix,
                                false,
                            );
                            {
                                let head = format!(
                                    "let {values} = {}; let ml_callback{} = {};",
                                    object.to_owned(),
                                    suffix.to_owned(),
                                    callback.to_owned()
                                );
                                {
                                    let iterator = format!("{values}.iter().cloned().enumerate()");
                                    format!(
                                        "{{ {head} std::rc::Rc::new(std::cell::RefCell::new({iterator}.flat_map(|({index}, {value})| {{ let {result} = {call}.borrow().clone(); {result} }}).collect::<Vec<_>>())) }}"
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_render_array_callback_call_loop1(
    operation: &str,
    mut parameter_count: f64,
    suffix: &str,
    mut clone_element: bool,
    mut arguments_text: String,
    mut position: f64,
    mut maximum: f64,
) -> String {
    loop {
        if (position < parameter_count) && (position < maximum) {
            {
                let arguments_text_3 = if position > 0f64 {
                    { format!("{arguments_text}, ") }
                } else {
                    arguments_text
                };
                {
                    let arguments_text_4 = format!(
                        "{arguments_text_3}{}",
                        self::render_array_callback_argument(
                            operation,
                            position,
                            suffix,
                            clone_element
                        )
                    );
                    {
                        let position_2 = position + 1f64;
                        {
                            (arguments_text, position) = (arguments_text_4, position_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { arguments_text };
    }
}
