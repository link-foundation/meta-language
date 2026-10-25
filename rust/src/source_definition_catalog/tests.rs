//! Shared metadata controls, compiled only in the later CI validation phase.
use super::{rules, texts};
use crate::read_structured_data_lino;
#[test]
fn exact_catalog_metadata_policy_uses_every_shared_control() {
    let fixture = read_structured_data_lino(include_str!(
        "../data/source-definition-catalog-controls.lino"
    ))
    .unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let strings = |name: &str| {
            texts(
                case[name]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned()),
            )
        };
        let counts = case["bytes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::validate_source_definition_catalog_entries(
                &strings("identifiers"),
                &strings("assets"),
                &strings("digests"),
                &counts,
                &strings("roots")
            ),
            case["result"].as_f64().unwrap(),
            "{}",
            case["name"]
        );
    }
    assert!(!rules::accept_source_definition_catalog_limits(
        f64::NAN,
        1.0,
        1.0
    ));
    assert!(rules::accept_source_definition_catalog_limits(
        1.0, 1.0, 1.0
    ));
}
