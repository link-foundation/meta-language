//! Deferred native original-source/full-definition/codec/four-product parity.
use super::translation_rust_records::assert_nominal_translation_product;
use std::process::Command;
#[test]
fn array_formal_parameters_preserve_actual_bindings_iterables_and_fresh_rest_values() {
    let output = Command::new("rustc").arg("--version").output().unwrap();
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
        "../../../parity/fixtures/array-formal-parameters.lino"
    ))
    .unwrap();
    for row in fixture["correctedInputs"].as_array().unwrap() {
        let surface = meta_language::translation::javascript::parse_javascript(
            row["source"].as_str().unwrap(),
        )
        .unwrap();
        meta_language::translation::check::check_program(&surface).unwrap();
    }
    assert_nominal_translation_product(
        "../experiments/javascript-program/array-formal-parameters.mjs",
        "JavaScript ordinary array formal parameter contexts",
        meta_language::create_java_script_array_formal_parameter_program_links,
    );
}
