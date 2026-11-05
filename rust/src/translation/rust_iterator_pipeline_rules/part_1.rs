// Generated from complete js/src/translation/rust-iterator-pipeline-rules.js. Do not edit by hand.
/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_iterator_arithmetic_consumer(consumer: &str) -> bool {
    (consumer == "sum") || (consumer == "product")
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_iterator_arithmetic_error(
    kind: &str,
    known: bool,
    owner: bool,
    layers: f64,
    result_known: bool,
    result_layers: f64,
    same_type: bool,
    same_owner: bool,
) -> String {
    if ((((((((kind == "fixed") || (kind == "float")) && known) && owner)
        && ((layers == 0f64) || (layers == 1f64)))
        && result_known)
        && (result_layers == 0f64))
        && same_type)
        && same_owner
    {
        String::new()
    } else {
        String::from(
            "sum and product need one standard numeric item owner and the matching direct result type",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_arithmetic_seed(consumer: &str, kind: &str) -> Vec<String> {
    if consumer == "sum" {
        if kind == "float" {
            vec![String::from("0"), String::from("negative")]
        } else {
            vec![String::from("0"), String::from("positive")]
        }
    } else if consumer == "product" {
        vec![String::from("1"), String::from("positive")]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_arithmetic_terminal(consumer: &str) -> Vec<String> {
    if consumer == "sum" {
        vec![
            String::from("accumulator"),
            String::from("item"),
            String::from("add"),
        ]
    } else if consumer == "product" {
        vec![
            String::from("accumulator"),
            String::from("item"),
            String::from("multiply"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_iterator_pipeline_stage(operation: &str) -> bool {
    ((operation == "map") || (operation == "filter")) || (operation == "inspect")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_pipeline_parameter_layers(operation: &str, layers: f64) -> f64 {
    if (operation == "filter") || (operation == "inspect") {
        layers + 1f64
    } else if operation == "map" {
        layers
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_pipeline_result_context(operation: &str) -> String {
    if operation == "inspect" {
        String::from("unit")
    } else if operation == "filter" {
        String::from("bool")
    } else if operation == "map" {
        String::from("infer")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_pipeline_result_state(operation: &str, state: &str) -> String {
    if operation == "map" {
        String::from("owned")
    } else if (operation == "filter") || (operation == "inspect") {
        state.to_owned()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_pipeline_callable_error(
    kind: &str,
    parameters: f64,
    wrappers: f64,
    parameter: bool,
    result: bool,
    result_copy: bool,
    owner: bool,
) -> String {
    if ((((((kind == "function") && (parameters == 1f64)) && (wrappers == 0f64)) && parameter)
        && result)
        && result_copy)
        && owner
    {
        String::new()
    } else {
        String::from(
            "a pipeline stage needs its complete immutable scalar callable parameter and result owners",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_pipeline_stage(operation: &str) -> Vec<String> {
    if (operation == "map") || (operation == "inspect") {
        vec![
            String::from("application"),
            String::from("continuation"),
            String::from("bind"),
        ]
    } else if operation == "filter" {
        vec![
            String::from("application"),
            String::from("continuation"),
            String::from("inactive"),
            String::from("if"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_pipeline_terminal(consumer: &str) -> Vec<String> {
    if self::accept_rust_iterator_arithmetic_consumer(consumer) {
        self::plan_rust_iterator_arithmetic_terminal(consumer)
    } else if ((consumer == "all") || (consumer == "any")) || (consumer == "for_each") {
        vec![
            String::from("callback"),
            String::from("item"),
            String::from("call one"),
        ]
    } else if (consumer == "fold") || (consumer == "rfold") {
        vec![
            String::from("callback"),
            String::from("accumulator"),
            String::from("item"),
            String::from("call two"),
        ]
    } else if consumer == "count" {
        vec![
            String::from("accumulator"),
            String::from("one"),
            String::from("add"),
        ]
    } else if consumer == "collect" {
        vec![
            String::from("accumulator"),
            String::from("item"),
            String::from("append"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_pipeline_inactive(consumer: &str) -> String {
    if consumer == "all" {
        String::from("true")
    } else if consumer == "any" {
        String::from("false")
    } else if (((((consumer == "fold") || (consumer == "rfold")) || (consumer == "for_each"))
        || (consumer == "count"))
        || (consumer == "collect"))
        || self::accept_rust_iterator_arithmetic_consumer(consumer)
    {
        String::from("accumulator")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_pipeline_consumer_operation(consumer: &str) -> String {
    if consumer == "all" {
        String::from("every")
    } else if consumer == "any" {
        String::from("some")
    } else if (((((consumer == "fold") || (consumer == "rfold")) || (consumer == "for_each"))
        || (consumer == "count"))
        || (consumer == "collect"))
        || self::accept_rust_iterator_arithmetic_consumer(consumer)
    {
        String::from("reduce")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_pipeline_captures(consumer: &str, count: f64) -> Vec<String> {
    {
        let roles = vec![String::from("receiver")];
        {
            let index = 0f64;
            {
                let roles_2 = self::source_ml_temporary_plan_rust_iterator_pipeline_captures_loop1(
                    count, roles, index,
                );
                {
                    let roles_4 = if (consumer == "fold") || (consumer == "rfold") {
                        {
                            {
                                let (ml_append_right, ml_append_left) =
                                    (vec![String::from("initial")], roles_2);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    } else {
                        roles_2
                    };
                    {
                        if ((((consumer == "all") || (consumer == "any"))
                            || (consumer == "for_each"))
                            || (consumer == "fold"))
                            || (consumer == "rfold")
                        {
                            {
                                {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![String::from("callback")], roles_4);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                }
                            }
                        } else {
                            roles_4
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_pipeline_continuation_item(operation: &str) -> String {
    if operation == "map" {
        String::from("mapped")
    } else if (operation == "filter") || (operation == "inspect") {
        String::from("original")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_pipeline_seed(consumer: &str) -> String {
    if self::accept_rust_iterator_arithmetic_consumer(consumer) {
        String::from("numeric identity")
    } else if consumer == "for_each" {
        String::from("unit")
    } else if consumer == "count" {
        String::from("zero")
    } else if consumer == "collect" {
        String::from("empty array")
    } else if (consumer == "fold") || (consumer == "rfold") {
        String::from("initial")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_iterator_pipeline_captures_loop1(
    mut count: f64,
    mut roles: Vec<String>,
    mut index: f64,
) -> Vec<String> {
    loop {
        if index < count {
            {
                let roles_2 = {
                    let (ml_append_right, ml_append_left) = (vec![String::from("stage")], roles);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (roles, index) = (roles_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { roles };
    }
}
