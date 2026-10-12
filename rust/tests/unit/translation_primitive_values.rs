use meta_language::translate_program;
use meta_language::translation::primitive_value_rules::{
    read_absent_number_value, read_number_display_function, read_number_display_refusal,
    read_scalar_cell_operation, render_absent_number_expression,
};
use serde_json::Value;
use std::process::Command;

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../parity/fixtures/translation-primitive-values.json"
    ))
    .unwrap()
}

#[test]
fn generated_primitive_value_decisions_match_shared_independent_fixtures() {
    let data = fixture();
    for entry in data["absences"].as_array().unwrap() {
        let operation = entry["operation"].as_str().unwrap();
        let absence = entry["absence"].as_str().unwrap();
        assert_eq!(
            read_absent_number_value(operation.to_owned(), absence.to_owned()),
            entry["value"].as_str().unwrap()
        );
        for (target, expression) in entry["expressions"].as_object().unwrap() {
            assert_eq!(
                render_absent_number_expression(
                    target.to_owned(),
                    operation.to_owned(),
                    absence.to_owned()
                ),
                expression.as_str().unwrap()
            );
        }
    }
    for entry in data["cells"].as_array().unwrap() {
        assert_eq!(
            read_scalar_cell_operation(entry["expression"].as_str().unwrap().to_owned()),
            entry["operation"].as_str().unwrap()
        );
    }
    assert_eq!(
        read_number_display_refusal("Rust".to_owned(), "float".to_owned()),
        data["displayRefusal"].as_str().unwrap()
    );
    assert_eq!(
        read_number_display_function("float".to_owned(), false),
        "js_number"
    );
    assert_eq!(
        read_number_display_function("float".to_owned(), true),
        "js_console"
    );
}

#[test]
fn native_rust_source_f64_display_refuses_until_a_faithful_formatter_exists() {
    let data = fixture();
    let reason = format!(
        "Rust Float display: {}",
        data["displayRefusal"].as_str().unwrap()
    );
    for entry in data["rustFloatCases"].as_array().unwrap() {
        let expression = entry["expression"].as_str().unwrap();
        let printed = format!("fn main() {{ println!(\"{{}}\", {expression}); }}");
        for target in ["JavaScript", "Lean", "Rocq"] {
            let result = translate_program(&printed, "Rust", target).unwrap();
            let diagnostic = result.diagnostic().unwrap();
            assert_eq!(diagnostic.kind.as_str(), "unsupported");
            assert_eq!(diagnostic.reason, reason, "{}:{target}", entry["name"]);
        }
        let nonprinting = format!("pub fn value() -> f64 {{ {expression} }} fn main() {{}}");
        let translated = translate_program(&nonprinting, "Rust", "JavaScript").unwrap();
        assert_eq!(translated.diagnostic(), None, "{}", entry["name"]);
        let script = format!(
            "{}\nconst actual = value(); console.log(Object.is(actual, -0) ? '-0' : String(actual));",
            translated.code()
        );
        let output = Command::new("node")
            .args(["--input-type=module", "-e", &script])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            format!("{}\n", entry["expected"].as_str().unwrap()),
            "{}",
            entry["name"]
        );
    }
    let stringified = translate_program(
        "pub fn text(value: f64) -> String { value.to_string() } fn main() {}",
        "Rust",
        "JavaScript",
    )
    .unwrap();
    assert_eq!(stringified.diagnostic().unwrap().reason, reason);
}
