use meta_language::translate_program;
use meta_language::translation::primitive_value_rules::{
    accept_object_freeze_operand, accept_object_property_definition, read_absent_number_value,
    read_number_display_function, read_number_display_refusal, read_scalar_cell_operation,
    render_absent_number_expression, require_data_object_freeze,
};
use serde_json::Value;
use std::process::Command;

fn fixture() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/translation-primitive-values.lino"
    ))
    .unwrap()
}

#[test]
fn generated_primitive_value_decisions_match_shared_independent_fixtures() {
    let data = fixture();
    for entry in data["objectProperties"].as_array().unwrap() {
        assert_eq!(
            accept_object_property_definition(
                entry["name"].as_str().unwrap(),
                entry["computed"].as_bool().unwrap(),
                entry["colon"].as_bool().unwrap()
            ),
            entry["accepted"].as_bool().unwrap(),
            "{entry}"
        );
    }
    for entry in data["freezeOperands"].as_array().unwrap() {
        assert_eq!(
            accept_object_freeze_operand(
                entry["expression"].as_str().unwrap(),
                entry["readonlyLiteral"].as_bool().unwrap()
            ),
            entry["accepted"].as_bool().unwrap(),
            "{entry}"
        );
    }
    for entry in data["dataObjectFreeze"].as_array().unwrap() {
        assert_eq!(
            require_data_object_freeze(
                entry["language"].as_str().unwrap(),
                entry["explicitFrozen"].as_bool().unwrap()
            ),
            entry["frozen"].as_bool().unwrap(),
            "{entry}"
        );
    }
    for entry in data["absences"].as_array().unwrap() {
        let operation = entry["operation"].as_str().unwrap();
        let absence = entry["absence"].as_str().unwrap();
        assert_eq!(
            read_absent_number_value(operation, absence),
            entry["value"].as_str().unwrap()
        );
        for (target, expression) in entry["expressions"].as_object().unwrap() {
            assert_eq!(
                render_absent_number_expression(target, operation, absence),
                expression.as_str().unwrap()
            );
        }
    }
    for entry in data["cells"].as_array().unwrap() {
        assert_eq!(
            read_scalar_cell_operation(entry["expression"].as_str().unwrap()),
            entry["operation"].as_str().unwrap()
        );
    }
    assert_eq!(
        read_number_display_refusal("Rust", "float"),
        data["displayRefusal"].as_str().unwrap()
    );
    assert_eq!(read_number_display_function("float", false), "js_number");
    assert_eq!(read_number_display_function("float", true), "js_console");
}

#[test]
fn native_javascript_source_refuses_prototype_semantics_without_creating_own_fields() {
    let data = fixture();
    for source in data["prototypeSources"].as_array().unwrap() {
        let source = source.as_str().unwrap();
        let error = match meta_language::translation::javascript::parse_javascript(source) {
            Ok(surface) => meta_language::translation::check::check_program(&surface).unwrap_err(),
            Err(error) => error,
        };
        assert_eq!(error.kind.as_str(), "unsupported", "{source}: {error}");
        assert_eq!(
            error.construct.as_deref(),
            Some("object prototype definition"),
            "{source}: {error}"
        );
        assert!(
            error.reason.contains("prototype-definition syntax"),
            "{source}: {error}"
        );
    }
}

#[test]
fn native_javascript_source_preserves_tagged_proto_data_and_explicit_freezing() {
    use meta_language::translation::check::check_program;
    use meta_language::translation::emit_javascript::emit_javascript;
    use meta_language::translation::javascript::parse_javascript;

    let data = fixture();
    for entry in data["objectDataSources"].as_array().unwrap() {
        let source = entry["source"].as_str().unwrap();
        let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
        let emitted = emit_javascript(&checked).unwrap();
        let inspect = "\nconst item = make(7); console.log(JSON.stringify([Object.hasOwn(item, '__proto__'), item.__proto__, Object.getPrototypeOf(item) === Object.prototype, Object.getOwnPropertyDescriptor(item, '__proto__')]));";
        let observe = |program: &str| {
            let output = Command::new("node")
                .args(["--input-type=module", "-e", &format!("{program}{inspect}")])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            serde_json::from_slice::<Value>(&output.stdout).unwrap()
        };
        let original = observe(source);
        let frozen = entry["frozen"].as_bool().unwrap();
        assert_eq!(
            original,
            serde_json::json!([true, 7, true, {"value": 7, "writable": !frozen, "enumerable": true, "configurable": !frozen}])
        );
        assert_eq!(observe(&emitted.text), original, "{source}");
    }
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

#[test]
fn native_javascript_freeze_gap_preserves_source_and_readonly_module_constants() {
    use meta_language::self_translate;
    use meta_language::translation::javascript::parse_javascript;
    for source in fixture()["freezeSources"].as_array().unwrap() {
        let source = source.as_str().unwrap();
        let error = parse_javascript(source).unwrap_err();
        assert_eq!(error.kind.as_str(), "unsupported", "{source}: {error}");
        assert!(
            matches!(
                error.construct.as_deref(),
                Some("Object.freeze operand" | "object without a $ tag")
            ),
            "{source}: {error}"
        );
        let carried = self_translate(source, "JavaScript", "Rust").unwrap();
        assert!(
            carried.items.iter().any(|item| item.status == "carried"
                && source[item.start..item.end].contains("Object.freeze")),
            "{source}: {}",
            carried.code
        );
        assert_eq!(
            self_translate(&carried.code, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
    }
    let source = "const MAX_LENGTH = 2048;\nexport const TERM = 'identifier:trigram';\nexport const PAIRS = Object.freeze([['en', 'Hawaii']]);\n";
    let translated = self_translate(source, "JavaScript", "Rust").unwrap();
    assert_eq!(
        translated
            .items
            .iter()
            .map(|item| item.status.as_str())
            .collect::<Vec<_>>(),
        vec!["translated", "translated", "translated"]
    );
    assert!(
        translated
            .code
            .contains("pub static PAIRS: std::sync::LazyLock<Vec<Vec<String>>>"),
        "{}",
        translated.code
    );
    assert_eq!(
        self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
}
