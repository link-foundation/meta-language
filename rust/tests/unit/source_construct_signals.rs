//! Deferred direct generated-policy controls; never compile locally.
#[path = "../../src/translation/source_construct_signal_rules.rs"]
mod rules;
#[test]
fn source_candidate_counts_keep_exact_utf16_offsets_and_lf_coordinates() {
    super::unsigned_source_documents::require_versions();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/source-construct-signals.lino"
    ))
    .unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let units = |value: &serde_json::Value| {
            rules::ml_string::Text(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|unit| u16::try_from(unit.as_u64().unwrap()).unwrap())
                    .collect(),
            )
        };
        let source = units(&row["sourceUnits"]);
        let needles = row["needleUnits"]
            .as_array()
            .unwrap()
            .iter()
            .map(units)
            .collect::<Vec<_>>();
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::read_source_construct_signals(&source, &needles),
            expected,
            "{}",
            row["name"]
        );
    }
}
