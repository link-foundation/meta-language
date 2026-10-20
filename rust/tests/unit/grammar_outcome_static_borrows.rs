//! Existing caller references retain their real compiler-enforced lifetime in CI.
#[path = "../../../experiments/grammar-outcome-network/native-transport/inverse/static-borrows.rs"]
mod storage;
use storage::rules;

#[test]
fn generated_static_reference_selection_preserves_all_independent_cases_and_resource_boundaries() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-outcome-static-borrows.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 256);
    for row in cases {
        let labels = row["registry"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let text = row["text"].as_str().unwrap();
        let bytes = labels.iter().map(String::len).sum::<usize>() + text.len();
        assert_eq!(
            row["bytes"].as_f64().unwrap().to_string(),
            bytes.to_string()
        );
        let expected = if labels.len() > 128 {
            "-3".to_owned()
        } else if labels
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != labels.len()
        {
            "-2".to_owned()
        } else {
            labels
                .iter()
                .position(|value| value == text)
                .map_or_else(|| "-1".to_owned(), |index| index.to_string())
        };
        assert_eq!(row["expected"].as_f64().unwrap().to_string(), expected);
        assert_eq!(
            rules::find_owned_outcome_static_borrow_index(
                text,
                &labels,
                row["bytes"].as_f64().unwrap()
            )
            .to_string(),
            expected
        );
    }
    for (entries, bytes, expected) in [
        (0.0, 0.0, true),
        (128.0, 1_048_576.0, true),
        (129.0, 0.0, false),
        (0.0, 1_048_577.0, false),
        (-1.0, 0.0, false),
        (0.5, 0.0, false),
        (0.0, 0.5, false),
        (f64::NAN, 0.0, false),
        (0.0, f64::INFINITY, false),
    ] {
        assert_eq!(
            rules::accept_owned_outcome_static_borrow_resources(entries, bytes),
            expected
        );
    }
}

#[test]
fn native_storage_returns_the_actual_existing_static_reference_without_allocating_a_new_lifetime() {
    let registry: &[&'static str] = &[
        "",
        "syntax",
        "ambiguity",
        "nestingDepth",
        "stepLimit",
        "😀",
        "é",
        "e\u{301}",
        "\0",
        "\r\n",
    ];
    for &value in registry {
        let reason = storage::resolve_owned_outcome_static_borrow(value, registry).unwrap();
        assert!(
            std::ptr::eq(reason, value),
            "the selected existing reference itself is retained"
        );
        let actual = meta_language::ParseRejection {
            diagnostics: None,
            offset_semantics: None,
            reason,
            offset: None,
            line: None,
            column: None,
            expected: None,
            limit: None,
        };
        assert_eq!(actual.reason, value);
        assert!(std::ptr::eq(actual.reason, value));
    }
    assert!(
        storage::resolve_owned_outcome_static_borrow("missing", registry)
            .unwrap_err()
            .contains("no existing")
    );
    assert!(
        storage::resolve_owned_outcome_static_borrow("syntax", &["syntax", "syntax"])
            .unwrap_err()
            .contains("ambiguous")
    );
    assert!(
        storage::resolve_owned_outcome_static_borrow("syntax", &["missing", "missing", "syntax"])
            .unwrap_err()
            .contains("ambiguous")
    );
    static LARGE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    let oversized: &'static str = LARGE.get_or_init(|| "x".repeat(1_048_577)).as_str();
    assert!(
        storage::resolve_owned_outcome_static_borrow("syntax", &[oversized])
            .unwrap_err()
            .contains("resource")
    );
}
