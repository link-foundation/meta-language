// Generated from complete js/src/translation/rust-array-lookup-rules.js. Do not edit by hand.
use super::accept_rust_primitive_from_source;
use super::read_rust_panic_message_source_error;
use super::read_rust_scalar_array_index_receiver_error;
use super::test_rust_array_lookup_lazy_fallback;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_lazy_fallback_error(
    kind: &str,
    parameters: f64,
    wrappers: f64,
    known_result: bool,
) -> String {
    if (((kind == "function") && (parameters == 0f64)) && (wrappers == 0f64)) && known_result {
        String::new()
    } else {
        String::from("a lazy lookup fallback needs a known direct zero-argument callable result")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_array_lookup_predicate_consumer(consumer: &str) -> bool {
    (consumer == "is_some_and") || (consumer == "is_none_or")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_predicate_error(
    kind: &str,
    parameters: f64,
    wrappers: f64,
    known_parameter: bool,
    result: &str,
    known_result: bool,
) -> String {
    if (((((kind == "function") && (parameters == 1f64)) && (wrappers == 0f64)) && known_parameter)
        && (result == "bool"))
        && known_result
    {
        String::new()
    } else {
        String::from(
            "a lookup predicate needs a known direct one-argument callable with a Boolean result",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_mapper_error(
    kind: &str,
    parameters: f64,
    wrappers: f64,
    known_parameter: bool,
    known_result: bool,
) -> String {
    if ((((kind == "function") && (parameters == 1f64)) && (wrappers == 0f64)) && known_parameter)
        && known_result
    {
        String::new()
    } else {
        String::from(
            "a lookup mapper needs a known direct one-argument callable with a complete result",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_callable_context_arity(
    consumer: &str,
    receiver: bool,
    element: bool,
) -> f64 {
    if !receiver || !element {
        -1f64
    } else if self::test_rust_array_lookup_lazy_fallback(consumer) {
        0f64
    } else if self::test_rust_array_lookup_predicate_consumer(consumer) || (consumer == "map_or") {
        1f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_receiver_error(
    kind: &str,
    element: &str,
    model: &str,
    proven: bool,
    immutable: bool,
    copy: bool,
    copied: bool,
) -> String {
    if copied {
        self::read_rust_scalar_array_index_receiver_error(
            kind, element, model, proven, immutable, copy,
        )
    } else if ((proven && immutable) && ((model == "owned vector") || (model == "immutable slice")))
        && ((kind == "array") || (((kind).is_empty()) && copy))
    {
        String::new()
    } else {
        String::from("the receiver needs a proven standard vector or immutable slice")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_element_owner_error(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
) -> String {
    if (kind == "unit")
        || self::accept_rust_primitive_from_source(
            kind,
            bits,
            signed,
            "primitive",
            owner,
            0f64,
            true,
            0f64,
        )
    {
        String::new()
    } else {
        String::from("a copied element needs its complete standard scalar source owner")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_cloned_element_error(
    kind: &str,
    bits: &str,
    signed: bool,
    owner_kind: &str,
    owner: &str,
    children: f64,
) -> String {
    if ((((kind == "string") && (owner_kind == "library")) && (owner == "std::string::String"))
        && (children == 0f64))
        || ((kind == "unit")
            || (((owner_kind == "primitive") && (children == 0f64))
                && ((self::read_rust_array_lookup_element_owner_error(kind, bits, signed, owner))
                    .is_empty())))
    {
        String::new()
    } else {
        String::from(
            "a cloned element needs its complete standard scalar or owned String source meaning",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_default_term(kind: &str, standard_string: bool) -> String {
    if kind == "unit" {
        String::from("unit")
    } else if (kind == "string") && standard_string {
        String::from("empty string")
    } else {
        String::from("primitive default")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_array_lookup_condition(method: &str) -> Vec<String> {
    if method == "get" {
        vec![
            String::from("index"),
            String::from("length"),
            String::from("lt"),
        ]
    } else if (method == "first") || (method == "last") {
        vec![
            String::from("length"),
            String::from("zero"),
            String::from("gt"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_array_lookup_index(method: &str) -> Vec<String> {
    if method == "get" {
        vec![String::from("index")]
    } else if method == "first" {
        vec![String::from("zero")]
    } else if method == "last" {
        vec![
            String::from("length"),
            String::from("one"),
            String::from("sub"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_array_lookup_consumer(consumer: &str) -> Vec<String> {
    if consumer == "is_some" {
        vec![String::from("condition")]
    } else if consumer == "is_none" {
        vec![String::from("condition"), String::from("not")]
    } else if consumer == "is_some_and" {
        vec![
            String::from("condition"),
            String::from("predicate"),
            String::from("false"),
            String::from("if"),
        ]
    } else if consumer == "is_none_or" {
        vec![
            String::from("condition"),
            String::from("predicate"),
            String::from("true"),
            String::from("if"),
        ]
    } else if ((((((consumer == "unwrap") || (consumer == "expect"))
        || (consumer == "unwrap_or"))
        || (consumer == "unwrap_or_else"))
        || (consumer == "unwrap_or_default"))
        || (consumer == "map_or"))
        || (consumer == "map_or_else")
    {
        vec![
            String::from("condition"),
            String::from("selected"),
            String::from("fallback"),
            String::from("if"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_array_lookup_panic_consumer(consumer: &str) -> bool {
    (consumer == "unwrap") || (consumer == "expect")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_panic_message_error(consumer: &str, kind: &str) -> String {
    if (consumer == "expect") && (kind != "str") {
        String::from(
            "expect needs its complete literal str panic message; computed messages need a dynamic panic payload",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_panic_message(consumer: &str, message: &str) -> String {
    if consumer == "unwrap" {
        String::from("called `Option::unwrap()` on a `None` value")
    } else if consumer == "expect" {
        message.to_owned()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_array_lookup_panic_value_error(
    consumer: &str,
    kind: &str,
    storage: &str,
    layers: f64,
    known: bool,
    references: bool,
) -> String {
    self::read_rust_panic_message_source_error(consumer, kind, storage, layers, known, references)
}
