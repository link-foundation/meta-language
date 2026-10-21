//! The complete shared source-type algorithm runs natively only in CI.
#[path = "../../src/translation/owned_outcome_source_type_rules.rs"]
mod rules;

fn number_column(value: &serde_json::Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_f64().unwrap())
        .collect()
}

fn check_columns(schema: &serde_json::Value, row: &serde_json::Value) -> f64 {
    rules::check_owned_outcome_source_types(
        &number_column(&row["types"]),
        &number_column(&row["selectors"]),
        &number_column(&row["offsets"]),
        &number_column(&row["children"]),
        &number_column(&schema["modes"]),
        &number_column(&schema["patterns"]),
        &number_column(&schema["patternOffsets"]),
        &number_column(&schema["patternTypes"]),
        &number_column(&schema["variantOwners"]),
    )
}

#[test]
fn translated_source_types_check_every_shape_and_original_projection() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-outcome-source-type-rules.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    let projections = fixture["projections"].as_array().unwrap();
    assert_eq!(cases.len(), 33);
    assert_eq!(projections.len(), 14);
    assert_eq!(fixture["declarations"].as_array().unwrap().len(), 16);
    assert_eq!(
        fixture["sourceSchema"]["sourceRoots"]
            .as_array()
            .unwrap()
            .len(),
        9
    );
    assert_eq!(
        fixture["sourceSchema"]["sourcePins"]
            .as_array()
            .unwrap()
            .len(),
        13
    );
    for row in cases {
        assert_eq!(
            check_columns(&fixture["schema"], row),
            row["expected"].as_f64().unwrap(),
            "{}",
            row["name"]
        );
    }
    for row in projections {
        assert_eq!(
            check_columns(&fixture["sourceSchema"], row),
            0.0,
            "{}",
            row["name"]
        );
    }
    let row = cases
        .iter()
        .find(|row| row["name"] == "fixed fields")
        .unwrap();
    let schema = &fixture["schema"];
    let columns = [
        number_column(&row["types"]),
        number_column(&row["selectors"]),
        number_column(&row["offsets"]),
        number_column(&row["children"]),
        number_column(&schema["modes"]),
        number_column(&schema["patterns"]),
        number_column(&schema["patternOffsets"]),
        number_column(&schema["patternTypes"]),
        number_column(&schema["variantOwners"]),
    ];
    let expected = [-3.0, -4.0, -2.0, -6.0, -1.0, -1.0, -1.0, -1.0, -1.0];
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        for (index, result) in expected.into_iter().enumerate() {
            let mut altered = columns.clone();
            altered[index][0] = value;
            assert_eq!(
                rules::check_owned_outcome_source_types(
                    &altered[0],
                    &altered[1],
                    &altered[2],
                    &altered[3],
                    &altered[4],
                    &altered[5],
                    &altered[6],
                    &altered[7],
                    &altered[8],
                ),
                result,
                "nonfinite channel {index}"
            );
        }
    }
    let depth = 8192_u32;
    let offsets = (0..=depth)
        .map(|index| f64::from(index.min(depth - 1)))
        .collect::<Vec<_>>();
    let children = (2..=depth).map(f64::from).collect::<Vec<_>>();
    let mut types = vec![7.0; depth as usize];
    let mut selectors = vec![4.0; depth as usize];
    *selectors.last_mut().unwrap() = 0.0;
    assert_eq!(
        rules::check_owned_outcome_source_types(
            &types,
            &selectors,
            &offsets,
            &children,
            &columns[4],
            &columns[5],
            &columns[6],
            &columns[7],
            &columns[8]
        ),
        0.0
    );
    *types.last_mut().unwrap() = 0.0;
    assert_eq!(
        rules::check_owned_outcome_source_types(
            &types,
            &selectors,
            &offsets,
            &children,
            &columns[4],
            &columns[5],
            &columns[6],
            &columns[7],
            &columns[8]
        ),
        -6.0
    );
}

#[test]
fn complete_source_type_algorithm_translates_without_carried_items_and_restores_exactly() {
    let source = include_str!(
        "../../../experiments/grammar-outcome-network/native-transport/inverse/source-type-rules.mjs"
    );
    let translated = meta_language::self_translate(source, "JavaScript", "Rust").unwrap();
    assert!(!translated.items.iter().any(|item| item.status == "carried"));
    assert!(
        translated
            .items
            .iter()
            .any(|item| item.status == "translated")
    );
    assert_eq!(
        meta_language::self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
}
