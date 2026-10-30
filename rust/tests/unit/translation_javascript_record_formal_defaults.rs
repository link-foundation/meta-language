//! Future original-source, native, source-definition, codec and emission parity.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn record_formal_defaults_keep_actual_source_initializers_and_undefined_only_choice() {
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
        "../../../parity/fixtures/record-formal-defaults.lino"
    ))
    .unwrap();
    for row in fixture["correctedInputs"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let surface = meta_language::translation::javascript::parse_javascript(source).unwrap();
        meta_language::translation::check::check_program(&surface).unwrap();
    }
    assert_nominal_translation_product(
        "../experiments/javascript-program/record-formal-defaults.mjs",
        "JavaScript actual record formal defaults",
        meta_language::create_java_script_record_parameter_program_links,
    );
}
