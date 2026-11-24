// Generated from complete js/src/translation/rust-iterator-arithmetic-rules.js. Do not edit by hand.
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
