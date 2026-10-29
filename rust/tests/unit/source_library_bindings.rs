//! Deferred source-derived binding/export policy parity controls.
#[path = "../../src/translation/source_library_binding_rules.rs"]
mod bindings;
#[path = "../../src/translation/source_library_export_rules.rs"]
mod exports;
#[test]
fn actual_binding_and_export_policy_retains_literal_source_roles() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/source-library-constant-functions.lino"
    ))
    .unwrap();
    for row in fixture["policies"].as_array().unwrap() {
        let keyword = row["keyword"].as_str().unwrap();
        let kind = row["kind"].as_str().unwrap();
        let name = row["name"].as_str().unwrap();
        let callable = row["callable"].as_bool().unwrap();
        assert_eq!(
            bindings::read_source_library_declaration_binding_kind(keyword, kind, name, callable),
            row["binding"].as_str().unwrap()
        );
        assert_eq!(
            exports::accept_source_library_function_export(keyword, kind, name, callable),
            row["exported"].as_bool().unwrap()
        );
    }
}
