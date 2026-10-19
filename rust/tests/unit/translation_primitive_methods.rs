//! Primitive method source, checked nodes and actual target observations.

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SExpr, SItem, SNode};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_typed_program_links, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_link_network_binary,
};
use std::fmt::Write as _;
use std::fs;
use std::process::Command;

fn javascript(source: &str) -> String {
    let output = Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn rust(source: &str, name: &str) -> String {
    let directory = std::env::temp_dir().join(format!(
        "meta-language-primitive-methods-{}-{name}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).unwrap();
    let file = directory.join("program.rs");
    let executable = directory.join("program");
    fs::write(&file, source).unwrap();
    let compiled = Command::new("rustc")
        .arg("--edition=2024")
        .arg(&file)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}; source retained at {}",
        String::from_utf8_lossy(&compiled.stderr),
        file.display()
    );
    let output = Command::new(&executable).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = String::from_utf8(output.stdout).unwrap();
    fs::remove_dir_all(directory).unwrap();
    output
}

#[test]
fn primitive_methods_run_with_utf16_defaults_slice_and_same_value_zero() {
    let source = "/** @param {string} text @returns {string} */function crop(text){return text.slice(1,3);}\n/** @param {number[]} values @returns {number[]} */function middle(values){return values.slice(1,-1);}\n/** @param {number[]} values @param {number} sought @returns {boolean} */function has(values,sought){return values.includes(sought);}\nconsole.log(`${crop('A😀B')}`);console.log(middle([1,2,3,4]).length);console.log('abc'.split().length);console.log(`${'x'.padStart(3)}`);console.log(`${'aba'.replaceAll('a','x')}`);console.log(has([NaN],NaN),has([-0],0),has([0],-0),has([1],1),has([1],2));";
    let expected = "😀\n2\n1\n  x\nxbx\ntrue true true true false\n";
    assert_eq!(javascript(source), expected);
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(
        javascript(&emit_javascript(&checked).unwrap().text),
        expected
    );
    let emitted = emit_rust(&checked).unwrap();
    assert_eq!(rust(&emitted.text, "scalar"), expected);
}

#[test]
fn nullable_membership_retains_nan_and_separate_absences() {
    let source = "/** @param {(number|null|undefined)[]} values @param {number|null|undefined} sought @returns {boolean} */function has(values,sought){return values.includes(sought);}\nconsole.log(has([NaN,null,undefined,-0],NaN),has([null],undefined),has([undefined],null),has([-0],0));";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(javascript(source), "true false false true\n");
    assert_eq!(
        javascript(&emit_javascript(&checked).unwrap().text),
        "true false false true\n"
    );
    let emitted = emit_rust(&checked).unwrap();
    assert!(
        emitted
            .text
            .contains("(crate::MlOptional::Null, crate::MlOptional::Null)")
    );
    assert!(emitted.text.contains("a.is_nan() && b.is_nan()"));
    assert_eq!(rust(&emitted.text, "nullable"), "true false false true\n");
}

#[test]
fn slice_argument_errors_keep_the_actual_method_span() {
    let source =
        "/** @param {string} text @returns {string} */function crop(text){return text.slice(1n);}";
    let error = parse_javascript(source).unwrap_err();
    assert_eq!(error.kind.as_str(), "type");
    assert_eq!(
        error.reason,
        "a BigInt is used as a number; JavaScript does not mix BigInt with other types"
    );
    let span = error.span.unwrap();
    assert_eq!(span.start, source.find("text.slice").unwrap());
    assert_eq!(span.end, source.find(';').unwrap());
}

#[test]
fn utf16_method_self_translation_matches_javascript_and_restores_the_actual_source() {
    let source = "/** @param {string} text @returns {string} */\nexport function crop(text){return text.trim().slice(1,3);}\n";
    let script = "import{selfTranslate}from'./js/src/self-translation.js';process.stdout.write(selfTranslate(process.argv[1],'JavaScript','Rust').code);";
    let output = Command::new("node")
        .current_dir(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap(),
        )
        .args(["--input-type=module", "-e", script, source])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let translated = meta_language::self_translate(source, "JavaScript", "Rust").unwrap();
    assert!(
        translated
            .items
            .iter()
            .all(|item| item.status == "translated")
    );
    assert_eq!(translated.code, String::from_utf8(output.stdout).unwrap());
    assert_eq!(
        meta_language::self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    let program = translated.code
        + "\nfn main(){println!(\"{}\",crop(crate::ml_string::Text::from_text(\" A😀B \")));}\n";
    assert_eq!(rust(&program, "self-translation"), "😀\n");
}

fn char_code_unit_oracle(source: &str, self_translation: bool) -> serde_json::Value {
    let script = r"
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
if(process.argv[2]==='true'){
  const {selfTranslate}=await import('./js/src/self-translation.js');
  process.stdout.write(JSON.stringify({rust:selfTranslate(process.argv[1],'JavaScript','Rust').code}));
}else{
  const checked=checkProgram(parseJavaScript(process.argv[1]));
  process.stdout.write(JSON.stringify({javascript:emitJavaScript(checked).text,rust:emitRust(checked).text}));
}
";
    let output = Command::new("node")
        .current_dir(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap(),
        )
        .args([
            "--input-type=module",
            "-e",
            script,
            source,
            if self_translation { "true" } else { "false" },
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn code_unit_template_keeps_numeric_defaults_absences_order_and_both_semantic_archives() {
    let source = "/** @param {string} text @param {number} position @returns {number} */function unit(text,position){return text.charCodeAt(position);}\n/** @param {string} text @param {number|null|undefined} position @returns {number} */function nullableUnit(text,position){return text.charCodeAt(position);}\nconsole.log('ABC'.charCodeAt(),'ABC'.charCodeAt(undefined),'ABC'.charCodeAt(null));console.log(unit('A😀B',1),unit('A😀B',2),unit('A😀B',3));console.log(unit('ABC',-0.9),unit('ABC',1.9),unit('ABC',NaN));console.log(unit('ABC',-1),unit('ABC',3),unit('',0));console.log(unit('ABC',-Infinity),unit('ABC',Infinity));console.log(nullableUnit('ABC',null),nullableUnit('ABC',undefined),nullableUnit('ABC',2));\n/** @returns {string} */function receiver(){console.log('receiver');return 'A😀B';}/** @returns {number} */function index(){console.log('index');return 1.9;}function absent(){console.log('absent');return undefined;}console.log(receiver().charCodeAt(index()));console.log(receiver().charCodeAt(absent()));console.log(receiver().charCodeAt());\n/** @param {string} text @param {number} position @param {number} index @param {number} Math @param {number} Number @returns {number} */function collision(text,position,index,Math,Number){const readStringCodeUnit=999;return text.charCodeAt(position)+index+readStringCodeUnit+Math+Number;}console.log(collision('A😀B',1,7,4,5));";
    let expected = "65 65 65\n55357 56832 66\n65 66 65\nNaN NaN NaN\nNaN NaN\n65 65 67\nreceiver\nindex\n55357\nreceiver\nabsent\n65\nreceiver\n65\n56372\n";
    assert_eq!(javascript(source), expected);
    let oracle = char_code_unit_oracle(source, false);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let javascript_code = emit_javascript(&program).unwrap().text;
    let rust_code = emit_rust(&program).unwrap().text;
    assert_eq!(javascript_code, oracle["javascript"].as_str().unwrap());
    assert_eq!(rust_code, oracle["rust"].as_str().unwrap());
    assert_eq!(javascript(&javascript_code), expected);
    assert_eq!(rust(&rust_code, "code-unit-semantics"), expected);
    let packet = create_typed_program_links(source, "JavaScript").unwrap();
    let networks = [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ];
    for network in networks {
        let rechecked = decode_typed_program_links(&network, packet.root).unwrap();
        let encoded = serde_json::to_value(&rechecked).unwrap();
        assert!(!encoded.to_string().contains("\"op\":\"charCodeAt\""));
        let linked_javascript = emit_typed_program_links(&network, packet.root, "JavaScript")
            .unwrap()
            .code;
        let linked_rust = emit_typed_program_links(&network, packet.root, "Rust")
            .unwrap()
            .code;
        assert_eq!(linked_javascript, javascript_code);
        assert_eq!(linked_rust, rust_code);
        assert_eq!(javascript(&linked_javascript), expected);
    }
}

#[test]
fn code_unit_parameters_keep_lone_surrogates_pair_halves_and_unusual_positions_in_native_rust() {
    let source = "/** @param {string} text @param {number} position @returns {number} */function unit(text,position){return text.charCodeAt(position);}";
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let emitted = emit_rust(&program).unwrap();
    let oracle = char_code_unit_oracle(source, false);
    assert_eq!(emitted.text, oracle["rust"].as_str().unwrap());
    let units: [Vec<u16>; 7] = [
        vec![],
        vec![65, 66, 67],
        vec![0, 65535],
        vec![0xd800, 120, 0xdfff],
        vec![0xd800],
        vec![0xdfff],
        vec![65, 0xd83d, 0xde00, 66],
    ];
    let javascript_calls = format!(
        "for(const units of {}){{const text=String.fromCharCode(...units);for(const position of [-Infinity,-0.9,-0,0,1.9,2,Infinity,NaN])console.log(unit(text,position));}}",
        serde_json::to_string(&units).unwrap()
    );
    let expected = javascript(&(source.to_owned() + &javascript_calls));
    let emitted_javascript = emit_javascript(&program).unwrap().text;
    assert_eq!(
        javascript(&(emitted_javascript + &javascript_calls)),
        expected
    );
    let positions = [
        "f64::NEG_INFINITY",
        "-0.9f64",
        "-0f64",
        "0f64",
        "1.9f64",
        "2f64",
        "f64::INFINITY",
        "f64::NAN",
    ];
    let mut calls = String::new();
    for value in &units {
        let value = value
            .iter()
            .map(u16::to_string)
            .collect::<Vec<_>>()
            .join(",");
        for position in positions {
            writeln!(calls, "println!(\"{{}}\",unit(crate::ml_string::Text::from_units(vec![{value}]),{position}));").unwrap();
        }
    }
    let rust_code = emitted.preludes.join("\n\n")
        + "\n\n"
        + &emitted.definitions.join("\n\n")
        + "\nfn main(){\n"
        + &calls
        + "}\n";
    assert_eq!(rust(&rust_code, "code-unit-surrogate-parameters"), expected);
    assert!(expected.contains("55296\n"));
    assert!(expected.contains("57343\n"));
    assert!(expected.contains("55357\n"));
    assert!(expected.contains("56832\n"));
    let sibling = "/** @param {string} text @param {number} position @returns {number} */\nexport function codeUnit(text,position){return text.charCodeAt(position);}\n/** @returns {number} */\nexport function choose(){return codeUnit(\"A😀B\",1);}\n";
    let oracle = char_code_unit_oracle(sibling, true);
    let translated = meta_language::self_translate(sibling, "JavaScript", "Rust").unwrap();
    assert_eq!(translated.code, oracle["rust"].as_str().unwrap());
    assert!(translated.items.iter().all(|item| item.status != "carried"));
    assert_eq!(
        meta_language::self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        sibling
    );
    assert_eq!(
        javascript(&(sibling.to_owned() + "console.log(choose());")),
        "55357\n"
    );
    assert!(
        !translated.code.contains("fn main("),
        "self-translation emits definitions and preludes without a program entry point"
    );
    assert_eq!(
        rust(
            &(translated.code + "\nfn main(){println!(\"{}\",choose());}\n"),
            "code-unit-sibling-call"
        ),
        "55357\n"
    );
}

#[test]
fn code_unit_conversion_keeps_unrepresented_source_and_internal_domains_explicit() {
    for index in ["true", "false", "'1'", "{}", "[1]"] {
        let source = format!("console.log('ABC'.charCodeAt({index}));");
        assert!(
            parse_javascript(&source)
                .and_then(|surface| check_program(&surface))
                .is_err()
        );
    }
    let extra = "console.log('ABC'.charCodeAt(0,1));";
    let failure = parse_javascript(extra)
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
    assert!(failure.reason.contains("0 to 1 arguments"));
    let escaped = r"console.log('\ud800'.charCodeAt(0));";
    assert_eq!(javascript(escaped), "55296\n");
    assert!(
        parse_javascript(escaped)
            .unwrap_err()
            .reason
            .contains("not a scalar value")
    );
    let annotated = "/** @returns {string} */function receiver(){console.log('receiver');return 'A😀B';}\n/** @returns {number} */function index(){console.log('index');return 1.9;}\n/** @returns {undefined} */function absent(){console.log('absent');return undefined;}\nconsole.log(receiver().charCodeAt(index()));\nconsole.log(receiver().charCodeAt(absent()));\nconsole.log(receiver().charCodeAt());";
    assert_eq!(
        javascript(annotated),
        "receiver\nindex\n55357\nreceiver\nabsent\n65\nreceiver\n65\n"
    );
    let failure = parse_javascript(annotated)
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
    assert!(failure.reason.contains("absence"));
    let original =
        parse_javascript("/** @returns {number} */function converted(){return 0;}").unwrap();
    for arg in [
        serde_json::json!({"k":"str","value":"1"}),
        serde_json::json!({"k":"bool","value":true}),
        serde_json::json!({"k":"unit"}),
        serde_json::json!({"k":"some","value":{"k":"str","value":"1"},"type":{"kind":"option","element":{"kind":"string"},"absence":"undefined"}}),
    ] {
        let mut surface = original.clone();
        let SItem::Fn(function) = &mut surface.items[0] else {
            panic!("the original converted function exists");
        };
        function.body = SExpr::new(
            SNode::NumberValue {
                arg: Box::new(serde_json::from_value(arg).unwrap()),
            },
            None,
        );
        assert!(check_program(&surface).is_err());
    }
    assert!(serde_json::from_value::<SExpr>(serde_json::json!({"k":"numberValue"})).is_err());
}
