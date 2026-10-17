//! Direct Surface contexts are checked without parser-populated callback types.
//! Native execution belongs to CI; the fixtures retain the original JS oracle.

use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::ir::{Effect, Expr, Node, Program};
use meta_language::translation::surface::SProgram;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use serde_json::{Value, json};

fn fixtures() -> &'static [Value] {
    static FIXTURES: OnceLock<Vec<Value>> = OnceLock::new();
    FIXTURES.get_or_init(|| {
        let script = r#"
import {callableExpectedContextCases,callableExpectedContextSummary} from './js/tests/fixtures/callable-expected-context.mjs';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import vm from 'node:vm';
import {format} from 'node:util';
const observed=callableExpectedContextCases().map(entry=>{
  const program=checkProgram(entry.surface),lines=[];
  vm.runInNewContext(emitJavaScript(program).text,{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});
  return {...entry,summary:callableExpectedContextSummary(program),observed:lines};
});
console.log(JSON.stringify(observed));
"#;
        let output = Command::new("node")
            .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
            .args(["--input-type=module", "-e", script])
            .output()
            .expect("CI executes the original direct Surface JavaScript oracle");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).unwrap()
    })
}

fn summary(program: &Program) -> Value {
    fn visit(expression: &Expr, callables: &mut Vec<Value>) {
        match &expression.node {
            Node::Lambda { params, .. } => callables.push(json!({
                "k":"lambda", "type":expression.ty, "params":params,
            })),
            Node::FunctionRef { func } => callables.push(json!({
                "k":"functionRef", "type":expression.ty, "fn":func,
            })),
            _ => {}
        }
        for child in expression.children() {
            visit(child, callables);
        }
    }
    let Effect::Let { value, .. } = &program.main.as_ref().unwrap().effects[0] else {
        panic!("the actual mapped value keeps its main binding")
    };
    let mut callables = Vec::new();
    visit(value, &mut callables);
    json!({"type":value.ty,"callables":callables})
}

fn observe(program: &Program) -> Vec<String> {
    let emitted = emit_javascript(program).unwrap();
    let output = Command::new("node")
        .args(["--input-type=module", "-e", &emitted.text])
        .output()
        .expect("CI executes the complete emitted JavaScript artifact");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}

#[test]
fn direct_surface_partial_callable_contexts_keep_actual_parameters_returns_and_effect_order() {
    assert_eq!(fixtures().len(), 8);
    for entry in fixtures() {
        let source: SProgram = serde_json::from_value(entry["surface"].clone()).unwrap();
        let program = check_program(&source).unwrap();
        assert_eq!(summary(&program), entry["summary"], "{}", entry["id"]);
        assert_eq!(
            entry["summary"]["type"],
            json!({"kind":"array", "element":{"kind":"int"}}),
            "the absent callback return is not replaced with Float"
        );
        assert_eq!(entry["observed"], entry["lines"]);
        assert_eq!(
            json!(observe(&program)),
            entry["observed"],
            "{}",
            entry["id"]
        );
    }
}

#[test]
fn original_match1_partial_return_counterexample_rechecks_both_source_free_archives() {
    let entry = fixtures()
        .iter()
        .find(|entry| entry["id"].as_str() == Some("match1"))
        .unwrap();
    let source: SProgram = serde_json::from_value(entry["surface"].clone()).unwrap();
    let program = check_program(&source).unwrap();
    let packet = encode_typed_program_links(&program).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let decoded = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(summary(&decoded), entry["summary"]);
        assert_eq!(json!(observe(&decoded)), entry["observed"]);
    }
}
