//! Complete future numeric controls. Execution and compilation belong to CI.
use super::translation_rust_records::assert_nominal_translation_product;

#[test]
fn javascript_numeric_conversion_retains_syntax_rounding_and_evaluation_order() {
    assert_nominal_translation_product(
        "../experiments/string-number/oracle.mjs",
        "exact JavaScript numeric conversions",
        meta_language::create_string_number_program_links,
    );
}

#[test]
fn native_numeric_kernel_retains_exact_utf16_and_rounding() {
    use serde_json::json;
    let version = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap();
    assert!(version.status.success());
    let version = String::from_utf8(version.stdout).unwrap();
    let active = version.split_whitespace().nth(1).unwrap();
    let parts = active
        .split('.')
        .take(2)
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {active}; >=1.90 <1.100"
    );
    let observed = std::process::Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=192",
            "../experiments/string-number/units-oracle.mjs",
        ])
        .output()
        .unwrap();
    assert!(
        observed.status.success(),
        "{}",
        String::from_utf8_lossy(&observed.stderr)
    );
    let fixture =
        meta_language::read_structured_data_lino(std::str::from_utf8(&observed.stdout).unwrap())
            .unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let units = row["units"]
            .as_array()
            .unwrap()
            .iter()
            .map(|unit| u16::try_from(unit.as_u64().unwrap()).unwrap())
            .collect::<Vec<_>>();
        let value = meta_language::convert_javascript_string_number_units(&units);
        let text = meta_language::translation::js_number::js_number(value);
        let actual = json!({"value":text,"negativeZero":value.to_bits() == (-0.0_f64).to_bits()});
        let expected = json!({"value":row["value"],"negativeZero":row["negativeZero"]});
        assert_eq!(row["original"], expected, "{}", row["name"]);
        assert_eq!(row["javascript"], expected, "{}", row["name"]);
        assert_eq!(actual, expected, "{}", row["name"]);
        if let Ok(text) = String::from_utf16(&units) {
            let from_text = meta_language::convert_javascript_string_number(&text);
            assert!(
                from_text.to_bits() == value.to_bits() || from_text.is_nan() && value.is_nan(),
                "{}",
                row["name"]
            );
        }
    }
}
