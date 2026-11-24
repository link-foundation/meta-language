//! Deferred complete originals and independently compiled native products.
#[test]
fn canonical_discovery_preserves_complete_source_and_native_products() {
    super::callable_profile_products::assert_products(
        "rust-canonical-discovery",
        "canonical-discovery",
    );
}
