//! Deferred shared native CST controls; no initializer is executed.
use meta_language::{
    accept_intrinsic_module_initializer_scope, read_inert_module_binding,
    read_intrinsic_module_initializer_binding, read_structured_data_lino, self_translate,
};
#[test]
fn actual_initializer_sources_have_matching_owners_and_refusals() {
    let fixture = read_structured_data_lino(include_str!(
        "../../parity/fixtures/intrinsic-module-initializers.lino"
    ))
    .unwrap();
    for row in fixture["positives"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        assert_eq!(
            read_intrinsic_module_initializer_binding(source),
            Some((
                row["name"].as_str().unwrap().to_owned(),
                u8::try_from(row["owner"].as_u64().unwrap()).unwrap()
            ))
        );
        assert!(read_inert_module_binding(source).is_none());
    }
    for source in fixture["negatives"].as_array().unwrap() {
        assert!(
            read_intrinsic_module_initializer_binding(source.as_str().unwrap()).is_none(),
            "{source}"
        );
    }
    for row in fixture["scope"].as_array().unwrap() {
        let texts = |key: &str| {
            row[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let owners = row["owners"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            accept_intrinsic_module_initializer_scope(&texts("forms"), &owners, &texts("bindings")),
            row["accepted"].as_bool().unwrap()
        );
    }
    let output = self_translate(fixture["module"].as_str().unwrap(), "JavaScript", "Rust").unwrap();
    assert!(output.code.contains("fn keep"));
    assert!(!output.code.contains("const TABLE:"));
    assert!(output.items.iter().any(|item| item.status == "carried"));
    assert!(output.items.iter().any(|item| item.status == "translated"));
}
