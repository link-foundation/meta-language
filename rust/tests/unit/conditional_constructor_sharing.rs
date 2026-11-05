//! Deferred native controls for the source-derived finite selection rules.
#[path = "../../src/translation/conditional_constructor_sharing_rules.rs"]
mod native;
use meta_language::read_structured_data_lino;

#[test]
fn actual_constructor_ownership_and_finite_selection_match_literal_controls() {
    super::translation_javascript_object_binding_declarations::require_supported_rust_version();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/conditional-constructor-sharing.lino"
    ))
    .unwrap();
    let text =
        |value: &serde_json::Value| native::ml_string::Text::from_text(value.as_str().unwrap());
    for row in fixture["owners"].as_array().unwrap() {
        let owners = row["owners"]
            .as_array()
            .unwrap()
            .iter()
            .map(text)
            .collect::<Vec<_>>();
        let constructors = row["constructors"]
            .as_array()
            .unwrap()
            .iter()
            .map(text)
            .collect::<Vec<_>>();
        let arities = row["arities"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            native::test_conditional_constructor_owners(
                &text(&row["owner"]),
                &text(&row["constructor"]),
                row["fields"].as_f64().unwrap(),
                &owners,
                &constructors,
                &arities
            ),
            row["expected"].as_bool().unwrap(),
            "{}",
            row["name"]
        );
    }
    for row in fixture["selections"].as_array().unwrap() {
        let numbers = |name: &str| {
            row[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            native::plan_conditional_constructor_field_selection(&numbers("classes")),
            numbers("expected")
        );
    }
}
