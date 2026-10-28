//! Deferred complete native rule cases; no compilation during drafting.
#[path = "../../src/translation/boolean_constructor_match_rules.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use meta_language::read_structured_data_lino;
use std::process::Command;

#[test]
fn boolean_constructor_match_retains_every_plan_and_refusal() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let data = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/boolean-constructor-match.lino"
    ))
    .unwrap();
    for row in data["rows"].as_array().unwrap() {
        let strings = |name: &str| {
            row[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let literals = row["literals"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            native::plan_boolean_constructor_match_sharing(
                row["owner"].as_str().unwrap(),
                &strings("declared"),
                &strings("owners"),
                &strings("constructors"),
                &literals
            ),
            expected,
            "{}",
            row["name"]
        );
    }
}
