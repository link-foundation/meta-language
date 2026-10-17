use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::surface::SProgram;
use serde_json::Value;
use std::process::Command;

#[test]
fn source_free_unit_product_fields_keep_the_established_source_representation() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/translation-product-unit.lino"
    ))
    .unwrap();
    for entry in fixture["cases"].as_array().unwrap() {
        let surface: SProgram = serde_json::from_value(entry["surface"].clone()).unwrap();
        let program = check_program(&surface).unwrap();
        let emitted = emit_javascript(&program).unwrap();
        assert_eq!(emitted.text, entry["javascript"].as_str().unwrap());
        let observation = format!(
            "{}\nconst value=make(); console.log(value[0] === {}, typeof value[0], value[1], Object.isFrozen(value));",
            emitted.text,
            entry["unitExpression"].as_str().unwrap()
        );
        let result = Command::new("node")
            .arg("-e")
            .arg(observation)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            String::from_utf8(result.stdout).unwrap().trim(),
            format!("true {} true true", entry["unitType"].as_str().unwrap())
        );
    }
}
