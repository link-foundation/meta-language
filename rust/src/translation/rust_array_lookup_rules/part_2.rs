// Generated from complete js/src/translation/rust-array-lookup-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_array_lookup_consumer(consumer: &str) -> Vec<String> {
    if consumer == "is_some" {
        vec![String::from("condition")]
    } else if consumer == "is_none" {
        vec![String::from("condition"), String::from("not")]
    } else if (consumer == "unwrap_or") || (consumer == "unwrap_or_default") {
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
