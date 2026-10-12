use meta_language::translation::Language;
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::ir::{Decl, Node, Program};
use meta_language::translation::javascript::parse_javascript;
use serde_json::Value;
use std::process::Command;

fn rename_tag(value: &mut Value, tag: &str) {
    match value {
        Value::Array(children) => children.iter_mut().for_each(|child| rename_tag(child, tag)),
        Value::Object(fields) => {
            if let Some(Value::Array(ctors)) = fields.get_mut("ctors") {
                for ctor in ctors {
                    if ctor["name"] == "one" {
                        ctor["name"] = Value::String(tag.to_owned());
                    }
                }
            }
            if fields.get("ctor").is_some_and(|name| name == "one") {
                fields.insert("ctor".to_owned(), Value::String(tag.to_owned()));
            }
            for child in fields.values_mut() {
                rename_tag(child, tag);
            }
        }
        _ => {}
    }
}

fn rename_field(value: &mut Value, name: &str) {
    match value {
        Value::Array(children) => children
            .iter_mut()
            .for_each(|child| rename_field(child, name)),
        Value::Object(fields) => {
            if let Some(Value::Array(constructors)) = fields.get_mut("ctors") {
                for ctor in constructors {
                    for field in ctor["fields"].as_array_mut().unwrap() {
                        if field["name"] == "payload" {
                            field["name"] = Value::String(name.to_owned());
                        }
                    }
                }
            }
            for child in fields.values_mut() {
                rename_field(child, name);
            }
        }
        _ => {}
    }
}

#[test]
fn constructor_tag_strings_keep_their_data_value_in_javascript_output() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/translation-constructor-tags.json"
    ))
    .unwrap();
    let source = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one' }; }\n/** @param {Tag} value @returns {number} */ function read(value) { if (value.$ === 'one') return 1; return 2; }";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    for tag in fixture["tags"].as_array().unwrap() {
        let tag = tag.as_str().unwrap();
        let mut graph = serde_json::to_value(&checked).unwrap();
        rename_tag(&mut graph, tag);
        let program: Program = serde_json::from_value(graph).unwrap();
        let emitted = emit_javascript(&program).unwrap();
        let script = format!(
            "globalThis.injected = false;\n{}\nconsole.log(JSON.stringify({{ tag: make().$, selected: read(make()), other: read({{ $: 'two' }}), injected: globalThis.injected }}));",
            emitted.text
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
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            actual,
            serde_json::json!({ "tag": tag, "selected": 1, "other": 2, "injected": false })
        );
    }
}

#[test]
fn constructor_fields_preserve_exact_own_property_names_in_javascript_output() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/translation-constructor-tags.json"
    ))
    .unwrap();
    let source = "/** @typedef {{ $: 'one', payload: number } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one', payload: 7 }; }\n/** @param {Tag} value @returns {number} */ function read(value) { if (value.$ === 'one') return value.payload; return 2; }";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    for name in fixture["fields"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        let mut graph = serde_json::to_value(&checked).unwrap();
        rename_field(&mut graph, name);
        let program: Program = serde_json::from_value(graph).unwrap();
        let emitted = emit_javascript(&program).unwrap();
        let script = format!(
            "{}\nconst field = {}; const item = make(); const supplied = {{ $: 'one', [field]: 9 }}; console.log(JSON.stringify([Object.hasOwn(item, field), item[field], read(item), read(supplied), Object.getPrototypeOf(item) === Object.prototype]));",
            emitted.text,
            serde_json::to_string(name).unwrap()
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
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(actual, serde_json::json!([true, 7, 7, 9, true]), "{name}");
    }
}

#[test]
fn constructor_immutability_preserves_its_source_model_and_checked_marker() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/translation-constructor-tags.json"
    ))
    .unwrap();
    let source = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one' }; }";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    for model in fixture["models"].as_array().unwrap() {
        let mut program = checked.clone();
        program.source_language = Language::parse(model["language"].as_str().unwrap()).unwrap();
        let make = program
            .declarations
            .iter_mut()
            .find_map(|declaration| match declaration {
                Decl::Fn(function) if function.name == "make" => Some(function),
                _ => None,
            })
            .unwrap();
        let Node::Ctor { frozen, .. } = &mut make.body.node else {
            panic!("the source returns a constructor");
        };
        *frozen = model["frozen"].as_bool().unwrap();
        let serialized = serde_json::to_value(&program).unwrap();
        let restored: Program = serde_json::from_value(serialized).unwrap();
        let emitted = emit_javascript(&restored).unwrap();
        let script = format!(
            "{}\nconst item = make(); const descriptor = Object.getOwnPropertyDescriptor(item, '$'); console.log(JSON.stringify([Object.isFrozen(item), Object.isExtensible(item), descriptor.writable, descriptor.configurable, Reflect.set(item, '$', 'two')]));",
            emitted.text
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
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            actual, model["descriptors"],
            "{}:{}",
            model["language"], model["frozen"]
        );
    }
    let explicit = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return Object.freeze({ $: 'one' }); }";
    let checked = check_program(&parse_javascript(explicit).unwrap()).unwrap();
    let make = checked
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Decl::Fn(function) if function.name == "make" => Some(function),
            _ => None,
        })
        .unwrap();
    assert!(matches!(make.body.node, Node::Ctor { frozen: true, .. }));
    for target in ["Rust", "Lean", "Rocq"] {
        let result = meta_language::translate_program(explicit, "JavaScript", target).unwrap();
        assert_eq!(
            result.diagnostic().unwrap().construct.as_deref(),
            Some("explicit data object freeze")
        );
    }
}
