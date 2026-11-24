//! Future actual-source early errors and complete paired translation products.
use super::translation_rust_records::assert_nominal_translation_product;
use meta_language::translation::javascript::parse_javascript;
#[test]
fn actual_lexical_declarations_preserve_scope_and_reject_early_errors() {
    let output = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
    let version = String::from_utf8(output.stdout).unwrap();
    let version = version.split_whitespace().nth(1).unwrap();
    let mut parts = version.split('.');
    assert_eq!(parts.next(), Some("1"), "uncovered Rust {version}");
    let minor: u32 = parts.next().unwrap().parse().unwrap();
    assert!(
        (90..100).contains(&minor),
        "uncovered Rust {version}; >=1.90 <1.100; edition 2024"
    );
    let fixture = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/lexical-declarations.lino"
    ))
    .unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        parse_javascript(row["source"].as_str().unwrap()).unwrap();
    }
    for row in fixture["refusals"].as_array().unwrap() {
        let error = parse_javascript(row["source"].as_str().unwrap()).unwrap_err();
        assert_eq!(
            error.kind.as_str(),
            row["kind"].as_str().unwrap(),
            "{}",
            row["name"]
        );
        assert_eq!(
            error.reason,
            row["reason"].as_str().unwrap(),
            "{}",
            row["name"]
        );
        let span = error.span.unwrap();
        assert!(span.end > span.start);
    }
    assert_nominal_translation_product(
        "../experiments/javascript-program/lexical-declarations.mjs",
        "JavaScript actual lexical declaration scopes",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/lexical-declaration-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
