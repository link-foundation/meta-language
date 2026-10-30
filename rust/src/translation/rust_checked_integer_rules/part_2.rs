// Generated from complete js/src/translation/rust-checked-integer-rules.js. Do not edit by hand.
use super::test_rust_option_panic_consumer;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_result_owner(
    consumer: &str,
    receiver: &str,
    mapper: &str,
) -> String {
    if (consumer == "is_some") || (consumer == "is_none") {
        String::from("bool")
    } else if ((consumer == "unwrap_or") || (consumer == "unwrap_or_default"))
        || self::test_rust_option_panic_consumer(consumer)
    {
        receiver.to_owned()
    } else if consumer == "map_or" {
        mapper.to_owned()
    } else {
        String::new()
    }
}
