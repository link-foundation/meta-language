//! Declared profile controls for the translated selector; CI executes Rust.
//! No synthetic profile is an attestation of the linked `serde_json` artifact.
#[path = "../../src/translation/owned_outcome_number_kind_rules.rs"]
mod rules;

#[test]
fn translated_number_kind_selection_preserves_every_declared_profile_control() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-outcome-number-kind-rules.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 160);
    assert_eq!(fixture["declarations"].as_array().unwrap().len(), 1);
    assert_eq!(fixture["model"]["versionRange"], ">=1.0.151 <1.0.152");
    assert_eq!(fixture["model"]["excludedFeature"], "arbitrary_precision");
    for row in cases {
        assert_eq!(
            rules::read_owned_outcome_native_number_kind(
                row["profileAvailable"].as_bool().unwrap(),
                row["version"].as_str().unwrap(),
                (
                    row["arbitraryPrecision"].as_bool().unwrap(),
                    row["signed"].as_bool().unwrap(),
                    row["unsigned"].as_bool().unwrap(),
                    row["floating"].as_bool().unwrap()
                ),
            ),
            row["expected"].as_str().unwrap(),
            "complete declared profile: {row}"
        );
    }
}
