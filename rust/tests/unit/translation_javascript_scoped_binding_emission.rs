//! Whole emitted modules, owned binding scopes and independent Node witnesses.

use std::fs;
use std::process::Command;

use meta_language::translation::emit_javascript::{
    JavaScriptEmissionOptions, emit_javascript, emit_javascript_with_options,
};
use meta_language::translation::ir::{Decl, Expr, Node};
use meta_language::translation::types::Type;
use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, TypedProgramLinksEmissionOptions,
    create_typed_program_links_with_javascript_scalar_loop_model, decode_typed_program_links,
    emit_typed_program_links, emit_typed_program_links_with_options, percent_encode_links_text,
};
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/javascript-scoped-binding-emission.lino"
    ))
    .unwrap()
}

fn options(data: &Value) -> TypedProgramLinksEmissionOptions {
    TypedProgramLinksEmissionOptions {
        java_script_scoped_binding_emission_model: Some(data["model"].as_str().unwrap().to_owned()),
        ..TypedProgramLinksEmissionOptions::default()
    }
}

fn reload(case: &Value) -> (LinkNetwork, LinkId) {
    (
        LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap(),
        LinkId::from_u64(case["root"].as_u64().unwrap()),
    )
}

fn raw_field(network: &LinkNetwork, parent: LinkId, label: &str) -> LinkId {
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, name, child]
                if *owner == parent
                    && link.metadata().link_type() == Some(LinkType::Field)
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some(label) =>
            {
                Some(*child)
            }
            _ => None,
        })
        .unwrap()
}

fn replace_raw_field(
    network: &LinkNetwork,
    parent: LinkId,
    label: &str,
    child: LinkId,
) -> LinkNetwork {
    let existing = network.links().find_map(|link| match link.references() {
        [owner, name, value]
            if *owner == parent
                && link.metadata().link_type() == Some(LinkType::Field)
                && network.link(*name).and_then(|name| name.metadata().term()) == Some(label) =>
        {
            Some((link.id(), *name, *value))
        }
        _ => None,
    });
    let Some((relation, name, previous)) = existing else {
        let mut changed = network.clone();
        changed.insert_field(parent, label, child);
        return changed;
    };
    let old = format!("({relation}: {parent} {name} {previous} ");
    let replacement = format!("({relation}: {parent} {name} {child} ");
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&old) {
                line.replacen(&old, &replacement, 1)
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let changed = LinkNetwork::from_lino(&text).unwrap();
    assert_eq!(raw_field(&changed, parent, label), child);
    changed
}

fn observe_complete_module(code: &str, case: &Value) {
    let directory = TemporaryDirectory::new("scoped-bindings");
    let script = directory.0.join("complete.js");
    fs::write(&script, code).unwrap();
    let checked = run(Command::new("node").arg("--check").arg(&script));
    assert!(checked.ok, "{}", checked.stderr);
    let observed = run(Command::new("node").arg(&script));
    assert!(observed.ok, "{}", observed.stderr);
    let expected = case["output"]
        .as_array()
        .unwrap()
        .iter()
        .map(|line| line.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let expected = if expected.is_empty() {
        expected
    } else {
        format!("{expected}\n")
    };
    assert_eq!(
        observed.stdout.replace("\r\n", "\n"),
        expected,
        "{}",
        case["id"]
    );
    let mut checks = Vec::new();
    for observations in std::iter::once(case).chain(
        case["additionalObservations"]
            .as_array()
            .into_iter()
            .flatten(),
    ) {
        let name = observations["entry"].as_str().unwrap();
        for (index, input) in observations["inputs"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            let arguments = input
                .as_array()
                .unwrap()
                .iter()
                .map(|value| {
                    if value.as_str() == Some("-0") {
                        "-0".to_owned()
                    } else {
                        value.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(",");
            let expected = &observations["expected"][index];
            let expected = if expected.as_str() == Some("-0") {
                "-0".to_owned()
            } else {
                expected.to_string()
            };
            checks.push(format!("if(!Object.is({name}({arguments}),{expected}))throw new Error('independent observation {index}');"));
        }
    }
    fs::write(&script, format!("{code}\n{}", checks.join("\n"))).unwrap();
    let witnessed = run(Command::new("node").arg(&script));
    assert!(witnessed.ok, "{}", witnessed.stderr);
}

fn edit_definition(network: &LinkNetwork, id: LinkId, value: &str) -> LinkNetwork {
    let old = network.link(id).unwrap().metadata().definition().unwrap();
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({id}: ")) {
                line.replacen(
                    &format!("(def: {})", percent_encode_links_text(old)),
                    &format!("(def: {})", percent_encode_links_text(value)),
                    1,
                )
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&text).unwrap()
}

#[test]
fn five_actual_complete_modules_replay_four_times_with_checked_execution_and_independent_observations()
 {
    let data = fixture();
    let options = options(&data);
    assert_eq!(data["cases"].as_array().unwrap().len(), 5);
    for case in data["cases"].as_array().unwrap() {
        let (mut network, mut root) = reload(case);
        for cycle in 0..4 {
            let before = network.to_lino();
            let program = decode_typed_program_links(&network, root).unwrap();
            let emitted =
                emit_typed_program_links_with_options(&network, root, "JavaScript", &options)
                    .unwrap();
            assert_eq!(network.to_lino(), before);
            assert_eq!(
                emitted.code,
                emit_javascript_with_options(
                    &program,
                    JavaScriptEmissionOptions {
                        scoped_binding_model: data["model"].as_str(),
                        identifier_model: None
                    }
                )
                .unwrap()
                .text
            );
            assert!(
                program
                    .declaration(case["entry"].as_str().unwrap())
                    .is_some()
            );
            if case["id"] == "declaration-loop" {
                assert!(emitted.code.contains("const ml_scalar_write0 ="), "{cycle}");
            }
            if case["id"] == "scalar-snapshot-declaration-collision" {
                assert!(emitted.code.contains("function ml_scalar_write0("));
                assert!(emitted.code.contains("const ml_scalar_write0 ="));
                assert!(!emitted.code.contains("const ml_scalar_write0_2 ="));
            }
            observe_complete_module(&emitted.code, case);
            let next = create_typed_program_links_with_javascript_scalar_loop_model(
                &emitted.code,
                "JavaScript",
                data["loopModel"].as_str().unwrap(),
                None,
            )
            .unwrap();
            network = LinkNetwork::from_lino(&next.network.to_lino()).unwrap();
            root = next.root;
        }
    }
}

#[test]
fn old_default_output_and_its_full_module_reserved_name_failure_remain_exact() {
    let data = fixture();
    let (network, root) = reload(&data["cases"][0]);
    let program = decode_typed_program_links(&network, root).unwrap();
    let first = emit_typed_program_links(&network, root, "JavaScript")
        .unwrap()
        .code;
    assert_eq!(first, emit_javascript(&program).unwrap().text);
    let second = create_typed_program_links_with_javascript_scalar_loop_model(
        &first,
        "JavaScript",
        data["loopModel"].as_str().unwrap(),
        None,
    )
    .unwrap();
    let code = emit_typed_program_links(&second.network, second.root, "JavaScript")
        .unwrap()
        .code;
    let failure = create_typed_program_links_with_javascript_scalar_loop_model(
        &code,
        "JavaScript",
        data["loopModel"].as_str().unwrap(),
        None,
    )
    .unwrap_err();
    assert_eq!(
        failure.message,
        "reserved identifier: ml_main_ignored1 uses the translator's reserved ml_ prefix at 390..406"
    );
    assert!(
        emit_typed_program_links_with_options(&network, root, "Rust", &options(&data))
            .unwrap_err()
            .message
            .contains("requires JavaScript target")
    );
    let unknown = TypedProgramLinksEmissionOptions {
        java_script_scoped_binding_emission_model: Some("unknown".to_owned()),
        ..TypedProgramLinksEmissionOptions::default()
    };
    assert!(
        emit_typed_program_links_with_options(&network, root, "JavaScript", &unknown)
            .unwrap_err()
            .message
            .contains("JavaScript scoped binding emission model")
    );
}

#[test]
fn actual_scalar_link_edits_survive_complete_edited_module_and_next_complete_module() {
    let data = fixture();
    let (network, mut root) = reload(&data["cases"][0]);
    let id = network
        .links()
        .find(|link| {
            link.metadata().term() == Some("string value")
                && link.metadata().definition() == Some("1")
        })
        .unwrap()
        .id();
    let mut network = edit_definition(&network, id, "2");
    let mut case = data["cases"][0].clone();
    case["inputs"] = serde_json::json!([[0, 3]]);
    case["expected"] = serde_json::json!([4]);
    for _ in 0..3 {
        let code =
            emit_typed_program_links_with_options(&network, root, "JavaScript", &options(&data))
                .unwrap()
                .code;
        observe_complete_module(&code, &case);
        let next = create_typed_program_links_with_javascript_scalar_loop_model(
            &code,
            "JavaScript",
            data["loopModel"].as_str().unwrap(),
            None,
        )
        .unwrap();
        network = LinkNetwork::from_lino(&next.network.to_lino()).unwrap();
        root = next.root;
    }
}

#[test]
fn counterfeit_loop_source_scope_or_slot_cannot_authorize_snapshot_preservation() {
    let data = fixture();
    let (network, root) = reload(&data["cases"][0]);
    for change in 0..3 {
        let mut program = decode_typed_program_links(&network, root).unwrap();
        let Decl::Fn(function) = &mut program.declarations[0] else {
            panic!("actual function")
        };
        let proof = function.source_loop.as_mut().unwrap();
        match change {
            0 => proof.source_name = "counterfeit".to_owned(),
            1 => proof.state[0].name = "foreign".to_owned(),
            _ => proof.continuation.as_mut().unwrap().path = vec!["other".to_owned()],
        }
        assert!(
            emit_javascript_with_options(
                &program,
                JavaScriptEmissionOptions {
                    scoped_binding_model: data["model"].as_str(),
                    identifier_model: None
                }
            )
            .is_err()
        );
    }
}

#[test]
fn genuine_javascript_archives_preserve_snapshot_flags_while_false_ownership_and_nonboolean_flags_refuse()
 {
    let data = fixture();
    assert_eq!(data["replayArchives"].as_array().unwrap().len(), 3);
    for case in data["replayArchives"].as_array().unwrap() {
        let (network, root) = reload(case);
        let code =
            emit_typed_program_links_with_options(&network, root, "JavaScript", &options(&data))
                .unwrap()
                .code;
        observe_complete_module(&code, &data["cases"][0]);
    }
    let (network, root) = reload(&data["cases"][2]);
    for (kind, definition) in [
        ("boolean value", "true"),
        ("number value", "0"),
        ("string value", ""),
    ] {
        let mut changed = network.clone();
        let body = changed.link(root).unwrap().references()[0];
        let flag = changed.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_language("meta-language typed program")
                .with_term(kind)
                .with_definition(definition),
        );
        changed.insert_field(body, "reference semantics", flag);
        let restored = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
        assert!(
            emit_typed_program_links_with_options(&restored, root, "JavaScript", &options(&data))
                .is_err()
        );
    }
}

#[test]
fn the_exact_empty_optional_shape_does_not_admit_other_elements_tags_or_some_values() {
    let data = fixture();
    let (mut network, mut root) = reload(&data["cases"][0]);
    for _ in 0..2 {
        let code =
            emit_typed_program_links_with_options(&network, root, "JavaScript", &options(&data))
                .unwrap()
                .code;
        let next = create_typed_program_links_with_javascript_scalar_loop_model(
            &code,
            "JavaScript",
            data["loopModel"].as_str().unwrap(),
            None,
        )
        .unwrap();
        network = next.network;
        root = next.root;
    }
    let original = decode_typed_program_links(&network, root).unwrap();
    let none = network
        .links()
        .find(|link| {
            link.metadata().term() == Some("string value")
                && link.metadata().definition() == Some("none")
        })
        .unwrap()
        .id();
    let some = edit_definition(&network, none, "some");
    assert!(
        emit_typed_program_links_with_options(&some, root, "JavaScript", &options(&data)).is_err()
    );
    let index=original.declarations.iter().position(|value|matches!(value,Decl::Fn(function)if matches!(function.body.node,Node::None{..}))).unwrap();
    let Decl::Fn(empty) = &original.declarations[index] else {
        panic!("empty function")
    };
    assert_eq!(
        empty.ret,
        Type::Option {
            element: Box::new(Type::Float),
            absence: "undefined".to_owned()
        }
    );
    for (element, absence) in [(Type::Bool, "undefined"), (Type::Float, "null")] {
        let mut program = original.clone();
        let Decl::Fn(function) = &mut program.declarations[index] else {
            panic!("empty function")
        };
        function.ret = Type::Option {
            element: Box::new(element),
            absence: absence.to_owned(),
        };
        function.body = Expr::new(
            Node::None {
                absence: absence.to_owned(),
            },
            function.ret.clone(),
        );
        assert!(
            emit_javascript_with_options(
                &program,
                JavaScriptEmissionOptions {
                    scoped_binding_model: data["model"].as_str(),
                    identifier_model: None
                }
            )
            .is_err()
        );
    }
}

#[test]
fn actual_raw_linked_records_refuse_opaque_ownership_before_native_normalization() {
    let data = fixture();
    let (network, root) = reload(&data["cases"][2]);
    let program = network.link(root).unwrap().references()[0];
    let directory = raw_field(&network, program, "declaration mapping");
    let pair = network.link(directory).unwrap().references()[0];
    let function = network.link(pair).unwrap().references()[1];
    let parameters = raw_field(&network, function, "parameters");
    let parameter = network.link(parameters).unwrap().references()[0];
    let body = raw_field(&network, function, "body");
    let main = raw_field(&network, program, "main");
    let effects = raw_field(&network, main, "effects");
    let effect = network.link(effects).unwrap().references()[0];
    for (role, owner) in [
        ("program", program),
        ("main", main),
        ("declaration", function),
        ("parameter", parameter),
        ("effect", effect),
        ("expression", body),
    ] {
        for field in ["borrow escape", "opaque binding", "source empty option"] {
            let mut changed = network.clone();
            let child = if field == "opaque binding" {
                if owner == parameter { body } else { parameter }
            } else {
                changed.insert_link(
                    [],
                    LinkMetadata::new()
                        .with_link_type(LinkType::Semantic)
                        .with_named(true)
                        .with_language("meta-language typed program")
                        .with_term("boolean value")
                        .with_definition("true"),
                )
            };
            changed.insert_field(owner, field, child);
            let restored = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
            let failure = emit_typed_program_links_with_options(
                &restored,
                root,
                "JavaScript",
                &options(&data),
            )
            .unwrap_err();
            assert_ne!(failure.message, "");
            if field != "source empty option" {
                assert!(
                    failure
                        .message
                        .contains(&format!("a checked {role} has unrepresented own fields")),
                    "{field}:{}",
                    failure.message
                );
            }
        }
    }
    for field in [
        "source function signature",
        "source loop proof",
        "source primitive owners",
    ] {
        let mut changed = network.clone();
        let child = changed.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_language("meta-language typed program")
                .with_term("null value")
                .with_definition(""),
        );
        changed.insert_field(function, field, child);
        let restored = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
        let failure =
            emit_typed_program_links_with_options(&restored, root, "JavaScript", &options(&data))
                .unwrap_err();
        assert!(
            failure
                .message
                .contains("an owned source facet must retain its actual checked record"),
            "{field}:{}",
            failure.message
        );
    }
}

#[test]
fn actual_qualified_primitive_owner_and_signature_records_compose_after_strict_raw_admission() {
    let owners = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-primitive-owners.lino"
    ))
    .unwrap();
    let case = owners["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "qualified-standard-floor")
        .unwrap();
    let (network, root) = reload(case);
    let before = network.to_lino();
    let program = decode_typed_program_links(&network, root).unwrap();
    let Decl::Fn(function) = program.declaration("apply").unwrap() else {
        panic!("owned function")
    };
    assert!(function.source_signature.is_some());
    assert!(function.source_primitive_owners.is_some());
    let code =
        emit_typed_program_links_with_options(&network, root, "JavaScript", &options(&fixture()))
            .unwrap()
            .code;
    observe_complete_module(
        &code,
        &serde_json::json!({"id":"qualified-standard-floor","entry":"apply","inputs":[["-0"],[1.75]],"expected":["-0",1],"output":[]}),
    );
    assert_eq!(network.to_lino(), before);
}

#[test]
fn actual_raw_known_slots_refuse_malformed_sequences_booleans_and_explicit_undefined() {
    let data = fixture();
    let (network, root) = reload(&data["cases"][2]);
    let program = network.link(root).unwrap().references()[0];
    let directory = raw_field(&network, program, "declaration mapping");
    let pair = network.link(directory).unwrap().references()[0];
    let function = network.link(pair).unwrap().references()[1];
    let parameters = raw_field(&network, function, "parameters");
    let parameter = network.link(parameters).unwrap().references()[0];
    let body = raw_field(&network, function, "body");
    let main = raw_field(&network, program, "main");
    let effects = raw_field(&network, main, "effects");
    let effect = network.link(effects).unwrap().references()[0];
    for (owner, label, term, definition) in [
        (program, "imports", "null value", ""),
        (program, "externals", "null value", ""),
        (main, "sequential async", "null value", ""),
        (main, "sequential async", "number value", "0"),
        (main, "sequential async", "string value", ""),
        (function, "recursive definition", "null value", ""),
        (function, "mutual", "null value", ""),
        (function, "recursive definition", "boolean value", "true"),
        (function, "mutual", "boolean value", "true"),
        (function, "decreasing", "number value", "0"),
        (effect, "constant", "null value", ""),
        (parameter, "guard", "undefined value", ""),
        (parameter, "default", "undefined value", ""),
        (body, "source span", "undefined value", ""),
    ] {
        let mut changed = network.clone();
        let child = changed.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_language("meta-language typed program")
                .with_term(term)
                .with_definition(definition),
        );
        let changed = replace_raw_field(&changed, owner, label, child);
        let restored = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
        assert!(
            emit_typed_program_links_with_options(&restored, root, "JavaScript", &options(&data))
                .is_err(),
            "{label}:{term}"
        );
    }
}
