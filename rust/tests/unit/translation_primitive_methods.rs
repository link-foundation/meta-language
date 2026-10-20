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

#[test]
fn character_template_preserves_defaults_order_and_complete_semantic_archives() {
    let source = "/** @param {string} text @param {number} position @returns {string} */function character(text,position){return text.charAt(position);}\n/** @param {string} text @param {number|null|undefined} position @returns {string} */function nullableCharacter(text,position){return text.charAt(position);}\nconsole.log('%s %s %s','ABC'.charAt(),'ABC'.charAt(undefined),'ABC'.charAt(null));console.log('%s %s %s',character('ABC',-0.9),character('ABC',1.9),character('ABC',NaN));console.log(character('ABC',-1).length,character('ABC',3).length,character('',0).length);console.log('%s %s',nullableCharacter('ABC',null),nullableCharacter('ABC',undefined));\n/** @returns {string} */function receiver(){console.log('receiver');return 'ABC';}/** @returns {number} */function position(){console.log('position');return 1.9;}console.log(receiver().charAt(position()));console.log(receiver().charAt());";
    let expected = "A A A\nA B A\n0 0 0\nA A\nreceiver\nposition\nB\nreceiver\nA\n";
    assert_eq!(javascript(source), expected);
    let oracle = char_code_unit_oracle(source, false);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let javascript_code = emit_javascript(&program).unwrap().text;
    let rust_code = emit_rust(&program).unwrap().text;
    assert_eq!(javascript_code, oracle["javascript"].as_str().unwrap());
    assert_eq!(rust_code, oracle["rust"].as_str().unwrap());
    assert_eq!(javascript(&javascript_code), expected);
    assert_eq!(rust(&rust_code, "character-template-semantics"), expected);
    let packet = create_typed_program_links(source, "JavaScript").unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert!(
            !serde_json::to_string(&restored)
                .unwrap()
                .contains("\"op\":\"charAt\"")
        );
        assert_eq!(
            emit_typed_program_links(&network, packet.root, "JavaScript")
                .unwrap()
                .code,
            javascript_code
        );
        assert_eq!(
            emit_typed_program_links(&network, packet.root, "Rust")
                .unwrap()
                .code,
            rust_code
        );
    }
}

#[test]
fn character_parameters_retain_every_surrogate_unit_and_empty_out_of_range_result() {
    let source = "/** @param {string} text @param {number} position @returns {string} */function character(text,position){return text.charAt(position);}";
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
    let driver = format!(
        "for(const units of {}){{const text=String.fromCharCode(...units);for(const position of [-Infinity,-0.9,-0,0,1.9,2,Infinity,NaN]){{const value=character(text,position);console.log('['+Array.from({{length:value.length}},(_,index)=>value.charCodeAt(index)).join(', ')+']');}}}}",
        serde_json::to_string(&units).unwrap()
    );
    let expected = javascript(&(source.to_owned() + &driver));
    assert_eq!(
        javascript(&(emit_javascript(&program).unwrap().text + &driver)),
        expected
    );
    let mut calls = String::new();
    for value in &units {
        let value = value
            .iter()
            .map(u16::to_string)
            .collect::<Vec<_>>()
            .join(",");
        for position in [
            "f64::NEG_INFINITY",
            "-0.9f64",
            "-0f64",
            "0f64",
            "1.9f64",
            "2f64",
            "f64::INFINITY",
            "f64::NAN",
        ] {
            writeln!(calls,"println!(\"{{:?}}\",character(crate::ml_string::Text::from_units(vec![{value}]),{position}).0);").unwrap();
        }
    }
    let native = emitted.preludes.join("\n\n")
        + "\n\n"
        + &emitted.definitions.join("\n\n")
        + "\nfn main(){\n"
        + &calls
        + "}\n";
    assert_eq!(
        rust(&native, "character-template-surrogate-parameters"),
        expected
    );
    assert!(expected.contains("[55296]\n"));
    assert!(expected.contains("[57343]\n"));
    assert!(expected.contains("[]\n"));
    let translated = meta_language::self_translate(source, "JavaScript", "Rust").unwrap();
    assert_eq!(
        translated
            .items
            .iter()
            .filter(|item| item.status == "carried")
            .count(),
        0
    );
    assert_eq!(
        meta_language::self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
}

#[test]
fn relative_character_preserves_all_independent_utf16_positions_and_undefined_results() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/string-relative-character-observations.lino"
    ))
    .unwrap();
    let source = fixture["source"].as_str().unwrap();
    let units = fixture["units"].as_array().unwrap();
    let positions = fixture["positions"].as_array().unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), units.len() * positions.len());
    let mut expected = String::new();
    for (index, row) in cases.iter().enumerate() {
        assert_eq!(
            row["text"].as_u64().unwrap(),
            u64::try_from(index / positions.len()).unwrap()
        );
        assert_eq!(
            row["position"].as_u64().unwrap(),
            u64::try_from(index % positions.len()).unwrap()
        );
        if row["undefined"].as_bool().unwrap() {
            assert!(row["units"].as_array().unwrap().is_empty());
            expected.push_str("undefined\n");
        } else {
            let result = row["units"].as_array().unwrap();
            assert_eq!(result.len(), 1);
            writeln!(expected, "[{}]", result[0].as_u64().unwrap()).unwrap();
        }
    }
    let driver = format!(
        "for(const units of {}){{const text=String.fromCharCode(...units);for(const spelling of {}){{const value=relativeCharacter(text,Number(spelling));console.log(value===undefined?'undefined':'['+Array.from({{length:value.length}},(_,index)=>value.charCodeAt(index)).join(', ')+']');}}}}",
        serde_json::to_string(units).unwrap(),
        serde_json::to_string(positions).unwrap()
    );
    assert_eq!(javascript(&(source.to_owned() + &driver)), expected);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let emitted = emit_rust(&program).unwrap();
    let oracle = char_code_unit_oracle(source, false);
    assert_eq!(emitted.text, oracle["rust"].as_str().unwrap());
    assert_eq!(
        javascript(&(emit_javascript(&program).unwrap().text + &driver)),
        expected
    );
    let mut calls = String::new();
    for input in units {
        let input = input
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_u64().unwrap().to_string())
            .collect::<Vec<_>>()
            .join(",");
        for spelling in positions {
            let spelling = spelling.as_str().unwrap();
            let position = match spelling {
                "Infinity" => "f64::INFINITY".to_owned(),
                "-Infinity" => "f64::NEG_INFINITY".to_owned(),
                "NaN" => "f64::NAN".to_owned(),
                value => format!("{value}f64"),
            };
            writeln!(calls,"match relative_character(crate::ml_string::Text::from_units(vec![{input}]),{position}){{Some(value)=>println!(\"{{:?}}\",value.0),None=>println!(\"undefined\")}}").unwrap();
        }
    }
    let native = emitted.preludes.join("\n\n")
        + "\n\n"
        + &emitted.definitions.join("\n\n")
        + "\nfn main(){\n"
        + &calls
        + "}\n";
    assert_eq!(
        rust(&native, "relative-character-independent-positions"),
        expected
    );
    assert!(expected.contains("[55296]\n"));
    assert!(expected.contains("[57343]\n"));
    assert!(expected.contains("undefined\n"));
}

#[test]
fn relative_character_keeps_default_order_complete_archives_and_lossless_source() {
    let source = "/** @returns {string} */function receiver(){console.log('receiver');return 'ABC';}/** @returns {number} */function position(){console.log('position');return -1.9;}console.log(receiver().at(position()));console.log(receiver().at());console.log('%s %s %s','ABC'.at(undefined),'ABC'.at(null),'ABC'.at(NaN));console.log(''.at(0)===undefined,'ABC'.at(-4)===undefined);";
    let expected = "receiver\nposition\nC\nreceiver\nA\nA A A\ntrue true\n";
    assert_eq!(javascript(source), expected);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let javascript_code = emit_javascript(&program).unwrap().text;
    let rust_code = emit_rust(&program).unwrap().text;
    let oracle = char_code_unit_oracle(source, false);
    assert_eq!(javascript_code, oracle["javascript"].as_str().unwrap());
    assert_eq!(rust_code, oracle["rust"].as_str().unwrap());
    assert_eq!(javascript(&javascript_code), expected);
    assert_eq!(
        rust(&rust_code, "relative-character-defaults-and-order"),
        expected
    );
    let packet = create_typed_program_links(source, "JavaScript").unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert!(
            !serde_json::to_string(&restored)
                .unwrap()
                .contains("\"op\":\"at\"")
        );
        for (language, code) in [("JavaScript", &javascript_code), ("Rust", &rust_code)] {
            assert_eq!(
                emit_typed_program_links(&network, packet.root, language)
                    .unwrap()
                    .code,
                *code
            );
        }
    }
    let translated = meta_language::self_translate(source, "JavaScript", "Rust").unwrap();
    assert!(translated.items.iter().all(|item| item.status != "carried"));
    assert_eq!(
        meta_language::self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    for position in ["true", "false", "'1'", "{}", "[1]", "1n"] {
        assert!(
            parse_javascript(&format!("console.log('ABC'.at({position}));"))
                .and_then(|surface| check_program(&surface))
                .is_err()
        );
    }
    assert!(check_program(&parse_javascript("console.log('ABC'.at(0,1));").unwrap()).is_err());
}

fn assert_unicode_order_fixture_observations(cases: &[serde_json::Value], observations: &str) {
    assert_eq!(cases.len(), 2916);
    let observed = observations.split_whitespace().collect::<Vec<_>>();
    assert_eq!(observed.len(), cases.len());
    for (index, (actual, row)) in observed.iter().zip(cases).enumerate() {
        assert_eq!(
            row["left"].as_f64().unwrap().to_string(),
            (index / (27 * 4)).to_string()
        );
        assert_eq!(
            row["right"].as_f64().unwrap().to_string(),
            ((index / 4) % 27).to_string()
        );
        assert_eq!(
            row["operator"].as_str().unwrap(),
            ["lt", "le", "gt", "ge"][index % 4]
        );
        assert_eq!(
            *actual,
            if row["expected"].as_bool().unwrap() {
                "true"
            } else {
                "false"
            }
        );
    }
}

#[test]
fn explicit_unicode_order_matches_every_shared_case_and_every_single_code_unit() {
    let source = "/** @param {string} left @param {string} right @returns {boolean} */function before(left,right){return left<right;}\n/** @param {string} left @param {string} right @returns {boolean} */function atMost(left,right){return left<=right;}\n/** @param {string} left @param {string} right @returns {boolean} */function after(left,right){return left>right;}\n/** @param {string} left @param {string} right @returns {boolean} */function atLeast(left,right){return left>=right;}";
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/unicode-string-order.lino"
    ))
    .unwrap();
    let units = fixture["units"].as_array().unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(units.len(), 27);
    assert_eq!(cases.len(), 2916);
    let driver = format!(
        "for(const left of {})for(const right of {}){{const a=String.fromCharCode(...left),b=String.fromCharCode(...right);console.log(before(a,b),atMost(a,b),after(a,b),atLeast(a,b));}}",
        serde_json::to_string(units).unwrap(),
        serde_json::to_string(units).unwrap()
    );
    let expected = javascript(&(source.to_owned() + &driver));
    assert_unicode_order_fixture_observations(cases, &expected);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert!(
        meta_language::translation::emit_lean::emit_lean(&program)
            .unwrap_err()
            .reason
            .contains("UTF-16 operational string encoding for lexical comparison")
    );
    assert!(
        meta_language::translation::emit_rocq::emit_rocq(&program)
            .unwrap_err()
            .reason
            .contains("UTF-16 operational string encoding for lexical comparison")
    );
    let native_source = "fn before(left: String,right: String)->bool {left<right} fn main() {}";
    let refusal = meta_language::translation::rust::parse_rust(native_source)
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
    assert!(refusal.reason.contains("ordering on string"));
    let emitted = emit_rust(&program).unwrap();
    let javascript_code = emit_javascript(&program).unwrap().text;
    let oracle = char_code_unit_oracle(source, false);
    assert_eq!(emitted.text, oracle["rust"].as_str().unwrap());
    assert_eq!(javascript_code, oracle["javascript"].as_str().unwrap());
    assert_eq!(javascript(&(javascript_code + &driver)), expected);
    let mut native = String::from("let inputs:Vec<Vec<u16>>=vec![\n");
    for row in units {
        let values = row
            .as_array()
            .unwrap()
            .iter()
            .map(|value| {
                let text = value.as_f64().unwrap().to_string();
                assert!(
                    text.parse::<u16>().is_ok(),
                    "a source unit is a canonical unsigned sixteen bit integer"
                );
                text
            })
            .collect::<Vec<_>>()
            .join(",");
        writeln!(native, "vec![{values}],").unwrap();
    }
    native.push_str("];for left in &inputs{for right in &inputs{println!(\"{} {} {} {}\",before(crate::ml_string::Text::from_units(left.clone()),crate::ml_string::Text::from_units(right.clone())),at_most(crate::ml_string::Text::from_units(left.clone()),crate::ml_string::Text::from_units(right.clone())),after(crate::ml_string::Text::from_units(left.clone()),crate::ml_string::Text::from_units(right.clone())),at_least(crate::ml_string::Text::from_units(left.clone()),crate::ml_string::Text::from_units(right.clone())));}}\n");
    native.push_str("for unit in 0u16..=u16::MAX{let left=crate::ml_string::Text::from_units(vec![unit]);let right=crate::ml_string::Text::from_units(vec![0xd800]);assert_eq!(before(left.clone(),right.clone()),unit<0xd800);assert_eq!(at_most(left.clone(),right.clone()),unit<=0xd800);assert_eq!(after(left.clone(),right.clone()),unit>0xd800);assert_eq!(at_least(left,right),unit>=0xd800);}\n");
    let code = emitted.preludes.join("\n\n")
        + "\n\n"
        + &emitted.definitions.join("\n\n")
        + "\nfn main(){\n"
        + &native
        + "}\n";
    assert_eq!(
        rust(&code, "unicode-order-all-units-and-shared-cases"),
        expected
    );
    let packet = create_typed_program_links(source, "JavaScript").unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        assert_eq!(
            emit_typed_program_links(&network, packet.root, "Rust")
                .unwrap()
                .code,
            emitted.text
        );
        let linked = emit_typed_program_links(&network, packet.root, "JavaScript")
            .unwrap()
            .code;
        assert_eq!(javascript(&(linked + &driver)), expected);
    }
    let translated = meta_language::self_translate(source, "JavaScript", "Rust").unwrap();
    assert!(translated.items.iter().all(|item| item.status != "carried"));
    assert_eq!(
        meta_language::self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
}

#[test]
fn number_predicates_preserve_all_shared_values_ordered_effects_and_complete_archives() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/number-predicate-observations.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 80);
    let mut source = String::new();
    let mut expected = String::new();
    for row in cases {
        writeln!(
            source,
            "console.log(Number.{}({}));",
            row["operation"].as_str().unwrap(),
            row["expression"].as_str().unwrap()
        )
        .unwrap();
        writeln!(expected, "{}", row["expected"].as_bool().unwrap()).unwrap();
    }
    source.push_str("/** @param {string | null | undefined} value @returns {boolean} */function nullable(value){return Number.isNaN(value);}console.log(nullable('NaN'),nullable(null),nullable(undefined));\n/** @returns {number} */function number(){console.log('number');return 2;}/** @returns {string} */function text(){console.log('text');return 'NaN';}/** @returns {boolean} */function extra(){console.log('extra');return true;}console.log(Number.isInteger(number(),text(),extra()));console.log(Number.isNaN(text(),number(),extra()));");
    expected.push_str("false false false\nnumber\ntext\nextra\ntrue\ntext\nnumber\nextra\nfalse\n");
    assert_eq!(javascript(&source), expected);
    let program = check_program(&parse_javascript(&source).unwrap()).unwrap();
    let emitted = emit_rust(&program).unwrap();
    let actual_javascript = emit_javascript(&program).unwrap().text;
    let oracle = char_code_unit_oracle(&source, false);
    assert_eq!(emitted.text, oracle["rust"].as_str().unwrap());
    assert_eq!(actual_javascript, oracle["javascript"].as_str().unwrap());
    assert_eq!(javascript(&actual_javascript), expected);
    assert_eq!(
        rust(&emitted.text, "number-predicate-arguments-and-all-types"),
        expected
    );
    let packet = create_typed_program_links(&source, "JavaScript").unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        assert_eq!(
            emit_typed_program_links(&network, packet.root, "Rust")
                .unwrap()
                .code,
            emitted.text
        );
        assert_eq!(
            javascript(
                &emit_typed_program_links(&network, packet.root, "JavaScript")
                    .unwrap()
                    .code
            ),
            expected
        );
    }
}

#[test]
fn complete_source_layout_envelopes_retain_bytes_and_refuse_obsolete_target_restoration() {
    for source in [
        "const a=1;const b=2;",
        "  const a=1;\tconst b=2; \n  ",
        "\n\nconst a=1;\n \t\nconst b=2;\t",
        "/*😀*/const a=1;/*é*/const b=2;",
        " \t\n",
        "",
        "/** @returns {number} */function value(){return 1;}console.log(value());",
    ] {
        let translated = meta_language::self_translate(source, "JavaScript", "Rust").unwrap();
        assert_eq!(
            translated.code,
            char_code_unit_oracle(source, true)["rust"]
                .as_str()
                .unwrap()
        );
        assert_eq!(
            meta_language::self_translate(&translated.code, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
    }
    let source = "  /** @returns {number} */function value(){return 1;}\t";
    let mut translated = meta_language::self_translate(source, "JavaScript", "Rust")
        .unwrap()
        .code;
    assert!(translated.contains(" envelope-sha256="));
    let index = translated.rfind("1f64").unwrap();
    translated.replace_range(index..index + 4, "2f64");
    let restored = meta_language::self_translate(&translated, "Rust", "JavaScript")
        .unwrap()
        .code;
    assert_ne!(restored, source);
    assert_eq!(javascript(&(restored + "\nconsole.log(value());")), "2\n");
}
