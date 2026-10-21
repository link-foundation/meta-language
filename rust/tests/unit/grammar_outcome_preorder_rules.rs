//! The complete shared worklist algorithm runs natively only in CI.
#[path = "../../src/translation/owned_outcome_preorder_rules.rs"]
mod rules;

#[test]
fn translated_owned_preorder_checks_every_independent_shape_and_deep_worklist() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-outcome-preorder-rules.lino"
    ))
    .unwrap();
    let numbers = |value: &serde_json::Value| {
        value
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>()
    };
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 23);
    for row in fixture["cases"].as_array().unwrap() {
        let offsets = numbers(&row["offsets"]);
        let children = numbers(&row["children"]);
        let limits = numbers(&row["limits"]);
        assert_eq!(
            rules::check_owned_outcome_preorder(
                &offsets, &children, limits[0], limits[1], limits[2]
            ),
            numbers(&row["expected"]),
            "{}",
            row["name"]
        );
    }
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            rules::check_owned_outcome_preorder(&[0.0, 0.0], &[], value, 1.0, 1.0),
            [-1.0, 0.0, 0.0, 0.0]
        );
        assert_eq!(
            rules::check_owned_outcome_preorder(&[0.0, value, 1.0], &[2.0], 2.0, 1.0, 2.0),
            [-2.0, 0.0, 0.0, 0.0]
        );
        assert_eq!(
            rules::check_owned_outcome_preorder(&[0.0, 1.0, 1.0], &[value], 2.0, 1.0, 2.0),
            [-4.0, 0.0, 0.0, 0.0]
        );
    }
    let depth = 2048_u32;
    let offsets = (0..=depth)
        .map(|index| f64::from(index.min(depth - 1)))
        .collect::<Vec<_>>();
    let children = (2..=depth).map(f64::from).collect::<Vec<_>>();
    let count = f64::from(depth);
    assert_eq!(
        rules::check_owned_outcome_preorder(&offsets, &children, count, count - 1.0, count),
        [0.0, count, count - 1.0, count]
    );
    assert_eq!(
        rules::check_owned_outcome_preorder(&offsets, &children, count - 1.0, count - 1.0, count),
        [-2.0, 0.0, 0.0, 0.0]
    );
    assert_eq!(
        rules::check_owned_outcome_preorder(&offsets, &children, count, count - 2.0, count),
        [-2.0, 0.0, 0.0, 0.0]
    );
    assert_eq!(
        rules::check_owned_outcome_preorder(&offsets, &children, count, count - 1.0, count - 1.0),
        [-5.0, 0.0, 0.0, 0.0]
    );
}

#[test]
fn complete_owned_preorder_source_translates_without_carried_items_and_restores_exactly() {
    let source = include_str!(
        "../../../experiments/grammar-outcome-network/native-transport/inverse/preorder-rules.mjs"
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
