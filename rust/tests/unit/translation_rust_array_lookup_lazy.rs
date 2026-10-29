//! Deferred complete original/generated products and shared rule controls.
#[path = "../../src/translation/rust_array_lookup_rules.rs"]
mod rules;
#[test]
fn independent_lazy_lookup_sources_and_native_products() {
    super::callable_profile_products::assert_products(
        "rust-array-lookup-lazy",
        "array-lookup-lazy",
    );
}
#[test]
fn explicit_lazy_callable_signatures_and_branch_plan() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-array-lookup-lazy.lino"
    ))
    .unwrap();
    assert_eq!(
        rules::read_rust_array_lookup_consumer_arity("unwrap_or_else", true),
        1.0
    );
    assert_eq!(
        rules::read_rust_array_lookup_consumer_arity("unwrap_or_else", false),
        -1.0
    );
    assert!(rules::test_rust_array_lookup_lazy_fallback(
        "unwrap_or_else"
    ));
    assert_eq!(
        rules::plan_rust_array_lookup_consumer("unwrap_or_else"),
        ["condition", "selected", "fallback", "if"]
    );
    for row in fixture["lazySignatures"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_array_lookup_lazy_fallback_error(
                row["kind"].as_str().unwrap(),
                row["parameters"].as_f64().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["knownResult"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for consumer in ["is_some_and", "is_none_or"] {
        assert_eq!(
            rules::read_rust_array_lookup_consumer_arity(consumer, true),
            1.0
        );
        assert_eq!(
            rules::read_rust_array_lookup_consumer_arity(consumer, false),
            -1.0
        );
        assert!(rules::test_rust_array_lookup_predicate_consumer(consumer));
        assert!(!rules::test_rust_array_lookup_lazy_fallback(consumer));
        assert_eq!(
            rules::plan_rust_array_lookup_consumer(consumer),
            [
                "condition",
                "predicate",
                if consumer == "is_some_and" {
                    "false"
                } else {
                    "true"
                },
                "if"
            ]
        );
    }
    for row in fixture["predicateSignatures"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_array_lookup_predicate_error(
                row["kind"].as_str().unwrap(),
                row["parameters"].as_f64().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["knownParameter"].as_bool().unwrap(),
                row["result"].as_str().unwrap(),
                row["knownResult"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
}
