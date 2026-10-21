//! Shared source fixtures; compilation and native observation run in CI only.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn policy() -> Value {
    read_structured_data_lino(include_str!(
        "../../src/data/array-classification-rules.lino"
    ))
    .unwrap()
}
fn versions() {
    let node = run(Command::new("node").args(["-p", "process.versions.node"]));
    assert!(node.ok, "{}", node.stderr);
    let parts = node
        .stdout
        .trim()
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 22 && parts[1] >= 15 || parts[0] == 24,
        "uncovered Node {}; supported >=22.15.0 <23 || >=24 <25",
        node.stdout.trim()
    );
    let rust = run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let release = rust.stdout.split_whitespace().nth(1).unwrap();
    let parts = release
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {release}; supported >=1.90.0 <1.100.0"
    );
}
fn observe(source: &str) -> String {
    let output = run(Command::new("node").args(["--input-type=module", "-e", source]));
    assert!(output.ok, "{}", output.stderr);
    output.stdout
}
fn javascript_contract(source: &str) -> Value {
    let script = r"
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
const surface=parseJavaScript(process.argv[1]),program=checkProgram(surface);
console.log(JSON.stringify({surface,javascript:emitJavaScript(program).text,rust:emitRust(program).text}));
";
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let output = run(Command::new("node").current_dir(root).args([
        "--input-type=module",
        "-e",
        script,
        source,
    ]));
    assert!(output.ok, "{}", output.stderr);
    serde_json::from_str(&output.stdout).unwrap()
}
fn archives(program: &meta_language::translation::ir::Program) -> Vec<meta_language::LinkNetwork> {
    let packet = encode_typed_program_links(program).unwrap();
    vec![
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}
fn compiled(source: &str) -> (bool, String, String) {
    let directory = TemporaryDirectory::new("array-classification");
    let input = directory.0.join("array-classification.rs");
    let output = directory.0.join(if cfg!(windows) {
        "array-classification.exe"
    } else {
        "array-classification"
    });
    fs::write(&input, source).unwrap();
    let built = run(Command::new("rustc")
        .args(["--edition", "2024"])
        .arg(&input)
        .arg("-o")
        .arg(&output));
    assert!(built.ok, "{}", built.stderr);
    let result = run(&mut Command::new(output));
    (result.ok, result.stdout, result.stderr)
}

#[test]
fn array_classification_keeps_actual_raw_shape_optional_presence_effects_and_both_codecs() {
    versions();
    let policy = policy();
    for row in policy["cases"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let expected = row["expected"].as_str().unwrap();
        assert_eq!(
            observe(source),
            expected,
            "independent Node: {}",
            row["name"]
        );
        let contract = javascript_contract(source);
        let surface = parse_javascript(source).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), contract["surface"]);
        let program = check_program(&surface).unwrap();
        assert_eq!(observe(&emit_javascript(&program).unwrap().text), expected);
        let rust = emit_rust(&program).unwrap().text;
        assert_eq!(rust, contract["rust"].as_str().unwrap());
        let direct = compiled(&rust);
        assert!(direct.0, "{}", direct.2);
        assert_eq!(direct.1, expected);
        let packet = encode_typed_program_links(&program).unwrap();
        for network in archives(&program) {
            let restored = decode_typed_program_links(&network, packet.root).unwrap();
            encode_typed_program_links(&restored).unwrap();
            assert_eq!(restored, program);
            assert_eq!(observe(&emit_javascript(&restored).unwrap().text), expected);
            let replay = emit_rust(&restored).unwrap().text;
            assert_eq!(replay, rust);
            let observed = compiled(&replay);
            assert!(observed.0, "{}", observed.2);
            assert_eq!(observed.1, expected);
        }
    }
}

#[test]
fn array_classification_never_discards_the_actual_failing_argument() {
    versions();
    let policy = policy();
    for row in policy["throwing"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let message = row["message"].as_str().unwrap();
        let original = run(Command::new("node").args(["--input-type=module", "-e", source]));
        assert!(!original.ok);
        assert!(original.stderr.contains(message));
        let program = check_program(&parse_javascript(source).unwrap()).unwrap();
        let packet = encode_typed_program_links(&program).unwrap();
        for network in archives(&program) {
            let restored = decode_typed_program_links(&network, packet.root).unwrap();
            let emitted = emit_javascript(&restored).unwrap().text;
            let observed = run(Command::new("node").args(["--input-type=module", "-e", &emitted]));
            assert!(!observed.ok);
            assert!(observed.stderr.contains(message));
            let native = compiled(&emit_rust(&restored).unwrap().text);
            assert!(!native.0);
            assert!(native.2.contains(message));
        }
    }
}

#[test]
fn array_classification_refusals_preserve_every_original_independent_oracle() {
    versions();
    let policy = policy();
    for row in policy["refusals"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let original = run(Command::new("node").args(["--input-type=module", "-e", source]));
        if let Some(error) = row["errorName"].as_str() {
            assert!(!original.ok);
            assert!(original.stderr.contains(error));
        } else {
            assert!(original.ok, "{}", original.stderr);
            assert_eq!(original.stdout, row["expected"].as_str().unwrap());
        }
        assert!(
            parse_javascript(source)
                .and_then(|surface| check_program(&surface))
                .is_err(),
            "{}",
            row["name"]
        );
    }
}

#[path = "../../src/translation/array_classification_rules.rs"]
mod generated_array_classification;

fn actual_array_classification_plan(kind: &str, element: &str) -> Vec<bool> {
    generated_array_classification::read_java_script_array_classification_plan(kind, element)
}

#[test]
fn actual_generated_array_decision_matches_every_shared_kind_and_optional_vector() {
    versions();
    let policy = policy();
    for row in policy["decisionVectors"].as_array().unwrap() {
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_bool().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            actual_array_classification_plan(
                row["kind"].as_str().unwrap(),
                row["element"].as_str().unwrap()
            ),
            expected,
            "{row}"
        );
    }
}

#[test]
fn actual_supplied_array_owner_is_not_replaced_with_the_intrinsic() {
    use meta_language::translation::{
        javascript::{ModuleContext, parse_javascript_in},
        surface::External,
        types::FLOAT,
    };
    let source = "function probe(){return Array.isArray([1]);}console.log(probe());";
    assert_eq!(observe(source), "true\n");
    let context = ModuleContext {
        externals: vec![External::Constant {
            name: "Array".into(),
            ty: FLOAT,
            literal: false,
        }],
        ..ModuleContext::default()
    };
    assert!(
        parse_javascript_in(source, &context)
            .and_then(|surface| check_program(&surface))
            .is_err()
    );
}

#[test]
fn complete_original_and_emitted_programs_keep_lossless_cst_archives_and_reverse_javascript_oracles()
 {
    use meta_language::{ParseConfiguration, create_typed_program_links, emit_typed_program_links};
    versions();
    let policy = policy();
    for row in policy["cases"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let expected = row["expected"].as_str().unwrap();
        let program = check_program(&parse_javascript(source).unwrap()).unwrap();
        let model = identifier_model();
        let default_javascript = emit_javascript(&program).unwrap().text;
        assert_eq!(observe(&default_javascript), expected);
        assert!(
            parse_javascript(&default_javascript)
                .unwrap_err()
                .reason
                .starts_with("reserved identifier: ")
        );
        let contract = identifier_contract(source, &model);
        let javascript = identifier_emission(&program, &model);
        assert_eq!(javascript, contract["javascript"].as_str().unwrap());
        let rust = emit_rust(&program).unwrap().text;
        for (text, language) in [(source, "JavaScript"), (rust.as_str(), "Rust")] {
            let network = LinkNetwork::parse(text, language, ParseConfiguration::default());
            assert!(
                network
                    .links()
                    .all(|link| !link.metadata().flags().has_error()
                        && !link.metadata().flags().is_missing())
            );
            let reloaded = [
                LinkNetwork::from_lino(&network.to_lino()).unwrap(),
                decode_link_network_binary(
                    &encode_link_network_binary(&network, BinaryLinksOptions::default()).unwrap(),
                    BinaryLinksOptions::default(),
                )
                .unwrap(),
            ];
            for graph in reloaded {
                assert_eq!(graph.reconstruct_text().as_bytes(), text.as_bytes());
            }
        }
        let reverse = create_typed_program_links(&javascript, "JavaScript").unwrap();
        let reverse_program = decode_typed_program_links(&reverse.network, reverse.root).unwrap();
        let target = emit_typed_program_links(&reverse.network, reverse.root, "Rust")
            .unwrap()
            .code;
        assert_eq!(target, contract["reverseRust"].as_str().unwrap());
        let compiled_target = compiled(&target);
        assert!(compiled_target.0, "{}", compiled_target.2);
        assert_eq!(compiled_target.1, expected);
        let packet = encode_typed_program_links(&reverse_program).unwrap();
        for graph in archives(&reverse_program) {
            let restored = decode_typed_program_links(&graph, packet.root).unwrap();
            assert_eq!(restored, reverse_program);
            assert_eq!(observe(&identifier_emission(&restored, &model)), expected);
            assert_eq!(
                emit_typed_program_links(&graph, packet.root, "Rust")
                    .unwrap()
                    .code,
                target
            );
        }
    }
}

fn identifier_model() -> String {
    read_structured_data_lino(include_str!(
        "../../src/data/javascript-identifier-models.lino"
    ))
    .unwrap()["model"]
        .as_str()
        .unwrap()
        .to_owned()
}

fn identifier_contract(source: &str, model: &str) -> Value {
    let script = r"
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
const program=checkProgram(parseJavaScript(process.argv[1]));
const javascript=emitJavaScript(program,{identifierModel:process.argv[2]}).text;
let reverseRust=null,reverseError=null;
try{reverseRust=emitRust(checkProgram(parseJavaScript(javascript))).text;}
catch(error){reverseError={reason:error.reason,span:error.span};}
console.log(JSON.stringify({javascript,defaultJavaScript:emitJavaScript(program).text,reverseRust,reverseError}));
";
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let output = run(Command::new("node").current_dir(root).args([
        "--input-type=module",
        "-e",
        script,
        source,
        model,
    ]));
    assert!(output.ok, "{}", output.stderr);
    serde_json::from_str(&output.stdout).unwrap()
}

fn identifier_emission(program: &meta_language::translation::ir::Program, model: &str) -> String {
    use meta_language::translation::emit_javascript::{
        JavaScriptEmissionOptions, emit_javascript_with_options,
    };
    emit_javascript_with_options(
        program,
        JavaScriptEmissionOptions {
            identifier_model: Some(model),
            ..JavaScriptEmissionOptions::default()
        },
    )
    .unwrap()
    .text
}

fn assert_identifier_reverse(javascript: &str, expected: &str, model: &str, javascript_rust: &str) {
    let reverse = check_program(&parse_javascript(javascript).unwrap()).unwrap();
    let rust = emit_rust(&reverse).unwrap().text;
    assert_eq!(
        rust, javascript_rust,
        "the actual JavaScript and native reverse frontends retain complete target parity"
    );
    let result = compiled(&rust);
    assert!(result.0, "{}", result.2);
    assert_eq!(result.1, expected);
    let packet = encode_typed_program_links(&reverse).unwrap();
    for graph in archives(&reverse) {
        let restored = decode_typed_program_links(&graph, packet.root).unwrap();
        assert_eq!(restored, reverse);
        assert_eq!(observe(&identifier_emission(&restored, model)), expected);
        assert_eq!(emit_rust(&restored).unwrap().text, rust);
    }
}

#[test]
fn ordinary_identifier_models_retain_both_collision_sources_oracles_and_source_free_ownership() {
    versions();
    let model = identifier_model();
    let cases = [
        (
            "function binding_ml_array_test1(){return 19;}function probe(binding_ml_array_test1){const binding_ml_array_test1_2=17;console.log(binding_ml_array_test1);console.log(binding_ml_array_test1_2);return Array.isArray([binding_ml_array_test1]);}console.log(binding_ml_array_test1());console.log(probe(7));",
            "19\n7\n17\ntrue\n",
            true,
        ),
        (
            "function probe(binding_ml_array_test1){return [binding_ml_array_test1].map(binding_ml_array_test1_2=>Array.isArray([binding_ml_array_test1_2])).join('|');}console.log(probe(3));",
            "true\n",
            false,
        ),
    ];
    for (source, expected, fixed_number_prelude) in cases {
        assert_eq!(
            observe(source),
            expected,
            "independent original Node oracle"
        );
        let contract = identifier_contract(source, &model);
        let program = check_program(&parse_javascript(source).unwrap()).unwrap();
        let original_rust = emit_rust(&program).unwrap().text;
        let observed = compiled(&original_rust);
        assert!(observed.0, "{}", observed.2);
        assert_eq!(observed.1, expected);
        let packet = encode_typed_program_links(&program).unwrap();
        for graph in archives(&program) {
            let restored = decode_typed_program_links(&graph, packet.root).unwrap();
            assert_eq!(restored, program);
            let javascript = identifier_emission(&restored, &model);
            assert_eq!(javascript, contract["javascript"].as_str().unwrap());
            assert_eq!(observe(&javascript), expected);
            assert_eq!(
                emit_javascript(&restored).unwrap().text,
                contract["defaultJavaScript"].as_str().unwrap()
            );
            assert_eq!(emit_rust(&restored).unwrap().text, original_rust);
            if fixed_number_prelude {
                assert_eq!(
                    env!("CARGO_PKG_VERSION"),
                    "1.0.0",
                    "uncovered package version; >=1.0.0 <1.0.1 required for this exact fixed Number prelude refusal"
                );
                let failure = parse_javascript(&javascript).unwrap_err();
                assert_eq!(
                    failure.reason,
                    "Object.is: the JavaScript standard library is outside the portable core"
                );
                assert_eq!(
                    failure.span.unwrap(),
                    meta_language::translation::Span {
                        start: 195,
                        end: 216
                    }
                );
                assert_eq!(contract["reverseError"]["reason"], failure.reason);
            } else {
                assert_identifier_reverse(
                    &javascript,
                    expected,
                    &model,
                    contract["reverseRust"].as_str().unwrap(),
                );
            }
        }
    }
    let variants = [
        (
            0,
            "true\ntrue\ntrue\ntrue\n",
            vec![
                (
                    "function binding_ml_array_test1(){",
                    "/** @returns {number} */function binding_ml_array_test1(){",
                ),
                (
                    "function probe(binding_ml_array_test1){",
                    "/** @param {number} binding_ml_array_test1 @returns {boolean} */function probe(binding_ml_array_test1){",
                ),
                (
                    "console.log(binding_ml_array_test1);",
                    "console.log(binding_ml_array_test1===7);",
                ),
                (
                    "console.log(binding_ml_array_test1_2);",
                    "console.log(binding_ml_array_test1_2===17);",
                ),
                (
                    "console.log(binding_ml_array_test1());",
                    "console.log(binding_ml_array_test1()===19);",
                ),
            ],
        ),
        (
            1,
            "true\n",
            vec![(
                "function probe(binding_ml_array_test1){",
                "/** @param {number} binding_ml_array_test1 @returns {string} */function probe(binding_ml_array_test1){",
            )],
        ),
    ];
    for (index, expected, edits) in variants {
        let original = cases[index].0;
        let mut source = original.to_owned();
        for (before, after) in &edits {
            assert_eq!(source.matches(*before).count(), 1);
            source = source.replacen(*before, after, 1);
        }
        let mut inverse = source.clone();
        for (before, after) in edits.iter().rev() {
            assert_eq!(inverse.matches(*after).count(), 1);
            inverse = inverse.replacen(*after, before, 1);
        }
        assert_eq!(inverse, original);
        assert_eq!(
            observe(&source),
            expected,
            "independent annotated numeric collision oracle"
        );
        let contract = identifier_contract(&source, &model);
        let program = check_program(&parse_javascript(&source).unwrap()).unwrap();
        let javascript = identifier_emission(&program, &model);
        assert_eq!(javascript, contract["javascript"].as_str().unwrap());
        assert_eq!(observe(&javascript), expected);
        assert_identifier_reverse(
            &javascript,
            expected,
            &model,
            contract["reverseRust"].as_str().unwrap(),
        );
    }
}

#[test]
fn ordinary_identifier_selection_preserves_default_and_old_model_and_refuses_unknowns() {
    use meta_language::translation::emit_javascript::{
        JavaScriptEmissionOptions, emit_javascript_with_options,
    };
    versions();
    let model = identifier_model();
    let policy = policy();
    let source = policy["cases"][0]["source"].as_str().unwrap();
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let before = encode_typed_program_links(&program)
        .unwrap()
        .network
        .to_lino();
    assert_eq!(
        emit_javascript(&program).unwrap(),
        emit_javascript_with_options(&program, JavaScriptEmissionOptions::default()).unwrap()
    );
    identifier_emission(&program, &model);
    assert_eq!(
        encode_typed_program_links(&program)
            .unwrap()
            .network
            .to_lino(),
        before
    );
    for unknown in ["", "unregistered identifier model"] {
        let error = emit_javascript_with_options(
            &program,
            JavaScriptEmissionOptions {
                identifier_model: Some(unknown),
                ..JavaScriptEmissionOptions::default()
            },
        )
        .unwrap_err();
        assert_eq!(
            error.construct.as_deref(),
            Some("JavaScript identifier model")
        );
    }
    assert!(
        emit_javascript_with_options(
            &program,
            JavaScriptEmissionOptions {
                scoped_binding_model: Some("owned JavaScript scoped bindings 1"),
                ..JavaScriptEmissionOptions::default()
            }
        )
        .is_err()
    );
    let error = emit_javascript_with_options(
        &program,
        JavaScriptEmissionOptions {
            identifier_model: Some(&model),
            scoped_binding_model: Some("owned JavaScript scoped bindings 1"),
        },
    )
    .unwrap_err();
    assert_eq!(
        error.construct.as_deref(),
        Some("JavaScript identifier model composition")
    );
    assert!(
        parse_javascript("function probe(ml_source_name){return ml_source_name;}")
            .unwrap_err()
            .reason
            .starts_with("reserved identifier: ml_source_name")
    );
}
