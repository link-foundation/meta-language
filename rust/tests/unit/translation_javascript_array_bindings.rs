//! Original JavaScript binding sources and source-free transport parity, CI only.
use std::path::Path;
use std::process::Command;

use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript, surface::SProgram,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use serde_json::{Value, json};

fn fixture() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/javascript-array-bindings.lino"
    ))
    .expect("the original binding sources are shared inspectable data")
}

fn observe(source: &str) -> String {
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

#[test]
fn original_array_binding_sources_keep_output_and_both_source_free_archives() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let script = "import {parseJavaScript} from './js/src/translation/javascript.js';import {checkProgram} from './js/src/translation/check.js';import {emitRust} from './js/src/translation/emit-rust.js';const surface=parseJavaScript(process.argv[1]);process.stdout.write(JSON.stringify({surface,rust:emitRust(checkProgram(surface)).text}));";
    let fixture = fixture();
    let accepted = fixture["accepted"].as_array().unwrap();
    assert_eq!(accepted.len(), 8);
    for entry in accepted {
        let source = entry["source"].as_str().unwrap();
        let expected = entry["expected"].as_str().unwrap();
        assert_eq!(
            observe(source),
            expected,
            "independent original source {}",
            entry["id"]
        );
        let output = Command::new("node")
            .current_dir(root)
            .args(["--input-type=module", "-e", script, source])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let javascript: Value = serde_json::from_slice(&output.stdout).unwrap();
        let surface = parse_javascript(source).expect("the complete actual source parses");
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            javascript["surface"],
            "actual parsed source {}",
            entry["id"]
        );
        let program = check_program(&surface).expect("the original source checks");
        assert_eq!(observe(&emit_javascript(&program).unwrap().text), expected);
        let rust = emit_rust(&program).unwrap().text;
        assert_eq!(
            rust,
            javascript["rust"].as_str().unwrap(),
            "complete target code {}",
            entry["id"]
        );
        let saved = encode_typed_program_links(&program).unwrap();
        let text = LinkNetwork::from_lino(&saved.network.to_lino()).unwrap();
        let binary = decode_link_network_binary(
            &encode_link_network_binary(&saved.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap();
        for network in [text, binary] {
            let restored = decode_typed_program_links(&network, saved.root).unwrap();
            assert_eq!(observe(&emit_javascript(&restored).unwrap().text), expected);
            assert_eq!(emit_rust(&restored).unwrap().text, rust);
        }
    }
}

#[test]
fn every_original_binding_refusal_remains_a_refusal() {
    let fixture = fixture();
    let refused = fixture["refused"].as_array().unwrap();
    assert_eq!(refused.len(), 13);
    for entry in refused {
        let source = entry[1].as_str().unwrap();
        if let Some(correction) = fixture["correctedBindingRefusals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["id"] == entry[0])
        {
            assert_eq!(
                correction["previousAssertion"],
                "checkProgram throws for the original input"
            );
            assert!(
                fixture["bindingAssertionCorrectionBasis"]["source"]
                    .as_str()
                    .unwrap()
                    .starts_with("https://tc39.es/")
            );
            let expected = correction["expected"].as_str().unwrap();
            assert_eq!(observe(source), expected);
            let program = check_program(&parse_javascript(source).unwrap()).unwrap();
            assert_eq!(observe(&emit_javascript(&program).unwrap().text), expected);
            let rust = emit_rust(&program).unwrap().text;
            let saved = encode_typed_program_links(&program).unwrap();
            for network in [
                LinkNetwork::from_lino(&saved.network.to_lino()).unwrap(),
                decode_link_network_binary(
                    &encode_link_network_binary(&saved.network, BinaryLinksOptions::default())
                        .unwrap(),
                    BinaryLinksOptions::default(),
                )
                .unwrap(),
            ] {
                let restored = decode_typed_program_links(&network, saved.root).unwrap();
                assert_eq!(observe(&emit_javascript(&restored).unwrap().text), expected);
                assert_eq!(emit_rust(&restored).unwrap().text, rust);
            }
            continue;
        }
        if entry[0].as_str() == Some("reserved-prefix") {
            let runnable = format!("{source}\nconsole.log(f([2,3]));");
            let checked = check_program(&parse_javascript(&runnable).unwrap()).unwrap();
            assert_eq!(observe(&runnable), "1\n");
            assert_eq!(
                observe(&emit_javascript(&checked).unwrap().text),
                observe(&runnable)
            );
            continue;
        }
        assert!(
            parse_javascript(source)
                .and_then(|program| check_program(&program))
                .is_err(),
            "original refusal {}",
            entry[0]
        );
    }
}

fn raw_program(start: &Value) -> Value {
    json!({"language":"JavaScript","items":[{"k":"fn","name":"suffix","params":[{"name":"values","type":{"kind":"array","element":{"kind":"float"}}}],"ret":{"kind":"array","element":{"kind":"float"}},"body":{"k":"arrayBindingRest","object":{"k":"name","path":["values"]},"start":start}}],"main":null})
}

#[test]
fn raw_binding_transport_preserves_presence_and_only_validates_the_selected_literal() {
    let valid = json!({"k":"num","value":"0"});
    for start in [
        json!({"k":"num","value":""}),
        json!({"k":"num","value":" \t\n"}),
        json!({"k":"num","value":"0x1"}),
        json!({"k":"num","value":"1e0"}),
        json!({"k":"num","value":"-0"}),
    ] {
        let raw = raw_program(&start);
        let program: SProgram = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(
            serde_json::to_value(&program).unwrap()["items"][0]["body"]["start"],
            raw["items"][0]["body"]["start"]
        );
        check_program(&program)
            .expect("the validated raw Number receives its internal Float annotation");
    }
    for start in [
        json!({"k":"num","value":"0","type":{"kind":"float"}}),
        json!({"k":"num","value":"0","unit":false}),
        json!({"k":"num","value":"0","negative":null}),
        json!({"k":"num","value":false}),
        json!({"k":"num","value":"1_0"}),
        json!({"k":"num","value":"-0x1"}),
        json!({"k":"num","value":"NaN"}),
        json!({"k":"num","value":"0.5"}),
        json!({"k":"num","value":"-1"}),
        json!({"k":"num","value":"9007199254740992"}),
    ] {
        let program: SProgram = serde_json::from_value(raw_program(&start)).unwrap();
        assert!(check_program(&program).is_err());
    }
    for (name, value) in [
        ("extra", json!(false)),
        ("block", json!(false)),
        ("tagTest", Value::Null),
        ("parenthesized", Value::Null),
    ] {
        let mut raw = raw_program(&valid);
        raw["items"][0]["body"][name] = value;
        let refused = serde_json::from_value::<SProgram>(raw)
            .map_or(true, |program| check_program(&program).is_err());
        assert!(
            refused,
            "the actual present field {name} must not be erased"
        );
    }
}

fn declaration_list_contract(source: &str, emitted: &str) -> Value {
    let script = concat!(
        r"
import {runInNewContext} from 'node:vm';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitRust} from './js/src/translation/emit-rust.js';
function observe(source){const lines=[];try{runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{",
        r"timeout:1000});return{lines,error:null};}catch(error){return{lines,error:{name:error.name,message:error.message}};}}
const source=process.argv[1];
try{const surface=parseJavaScript(source);process.stdout.write(JSON.stringify({surface,rust:emitRust(checkProgram(surface)).text,original:observe(source),emitted:observe(process.argv[2])}));}
catch(error){process.stdout.write(JSON.stringify({diagnostic:{kind:error.kind,reason:error.reason,span:error.span}}));}
"
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let output = Command::new("node")
        .current_dir(root)
        .args(["--input-type=module", "-e", script, source, emitted])
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
fn const_declaration_lists_keep_mixed_binding_order_and_complete_source_free_replays() {
    for (source, expected) in [
        (
            "function selected(first, second) { if(first) { if(second) return 3; } return 7; }\nfunction closed(first,second){const read=()=>{if(first){if(second)return 5;}return 9;};return read();}\nconsole.log(selected(true,true),selected(true,false),selected(false,true),closed(true,true),closed(true,false));",
            json!({"lines":["3 7 7 5 9"],"error":null}),
        ),
        (
            "function step(value) { console.log(value); return value + 1; }\nfunction selected() { const first = step(1), second = step(first), third = second + first; return third; }\nconsole.log(selected());",
            json!({"lines":["1","2","5"],"error":null}),
        ),
        (
            "const first = 2, second = 3;\nfunction selected() { return first + second; }\nconsole.log(selected());",
            json!({"lines":["5"],"error":null}),
        ),
        (
            "function values() { console.log('array'); return [2]; }\nfunction record() { console.log('record'); return {left: 3, right: 5}; }\nfunction selected() { const [first, next = first + 1] = values(), {left, right: last} = record(), total = first + next + left + last; return total; }\nconsole.log(selected());",
            json!({"lines":["array","record","13"],"error":null}),
        ),
        (
            "/** @returns {number} */ function stop() { console.log('stop'); throw new Error('halt'); }\nfunction later() { console.log('later'); return 9; }\nfunction selected() { const first = stop(), second = later(); return first + second; }\nconsole.log(selected());",
            json!({"lines":["stop"],"error":{"name":"Error","message":"halt"}}),
        ),
        (
            "function selected() { const first = [1, 2], text = ',const fake', {left, right: last} = {left: 3, right: 4}, total = first[0] + left + last; return total; }console.log(selected());",
            json!({"lines":["8"],"error":null}),
        ),
        (
            "function selected() {\nconst first = 2, second = first + 1\n{ const first = 7, second = first + 1; console.log(first, second); }\nconst third = second + 1, fourth = third + 1\nreturn fourth\n}\nconsole.log(selected());",
            json!({"lines":["7 8","5"],"error":null}),
        ),
        (
            "function selected(){const first=2, read=()=>first;return read();}console.log(selected());",
            json!({"lines":["2"],"error":null}),
        ),
        (
            "function selected(){const read=(later)=>later, later=3;return read(later);}console.log(selected());",
            json!({"lines":["3"],"error":null}),
        ),
    ] {
        require_declaration_list_equivalent(source, &expected);
    }
}

fn require_declaration_list_equivalent(source: &str, expected: &Value) {
    let surface =
        parse_javascript(source).unwrap_or_else(|error| panic!("{source}: {}", error.message()));
    let program = check_program(&surface).unwrap();
    let javascript = emit_javascript(&program).unwrap().text;
    let rust = emit_rust(&program).unwrap().text;
    let original = declaration_list_contract(source, &javascript);
    assert_eq!(
        serde_json::to_value(&surface).unwrap(),
        original["surface"],
        "{source}"
    );
    assert_eq!(rust, original["rust"].as_str().unwrap(), "{source}");
    assert_eq!(
        &original["original"], expected,
        "unchanged independent source: {source}"
    );
    assert_eq!(&original["emitted"], expected, "{source}");
    let saved = encode_typed_program_links(&program)
        .unwrap_or_else(|error| panic!("initial checked archive encoding for {source}: {error}"));
    for network in [
        LinkNetwork::from_lino(&saved.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&saved.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, saved.root)
            .unwrap_or_else(|error| panic!("source-free archive decoding for {source}: {error}"));
        encode_typed_program_links(&restored).unwrap_or_else(|error| {
            panic!("restored checked archive encoding for {source}: {error}")
        });
        assert_eq!(restored, program);
        assert_eq!(emit_javascript(&restored).unwrap().text, javascript);
        assert_eq!(emit_rust(&restored).unwrap().text, rust);
    }
}

#[test]
fn constructor_fields_follow_single_evaluation_aliases_and_lexical_shadows() {
    for (body, expected) in [
        ("const {left,right}=value;return left+right;", "5"),
        (
            "const {left,right}=value,total=left+right;return total;",
            "5",
        ),
        (
            "const original=value,again=original;const {left,right}=again;return left+right;",
            "5",
        ),
        (
            "const original=value;{const value=7;const {left,right}=original;return left+right+value;}",
            "12",
        ),
        (
            "const {left,right}=value;let total=left+right;total=total+1;return total;",
            "6",
        ),
    ] {
        let source = concat!(
            "/** @typedef {{ $: 'none' } | { $: 'pair', left: number, right: number }} Item */\n",
            "/** @param {Item} value @returns {number} */\n",
        )
        .to_owned()
            + &format!(
                "function selected(value){{switch(value.$){{case 'pair':{{{body}}}default:return 0;}}}}\nconsole.log(selected({{$:'pair',left:2,right:3}}));console.log(selected({{$:'none'}}));"
            );
        require_declaration_list_equivalent(&source, &json!({"lines":[expected,"0"],"error":null}));
    }
}

fn require_declaration_list_refusal(source: &str, reason: &str) {
    let error = parse_javascript(source)
        .and_then(|program| check_program(&program))
        .unwrap_err();
    assert!(
        error.reason.contains(reason),
        "{source}: {}",
        error.message()
    );
    let original = declaration_list_contract(source, "");
    assert_eq!(
        json!({"kind":error.kind,"reason":error.reason,"span":error.span}),
        original["diagnostic"],
        "{source}"
    );
}

#[test]
fn const_lists_preserve_temporal_dead_zones_duplicate_and_empty_pattern_refusals() {
    for declaration in ["const", "let"] {
        for body in [
            format!("{declaration} first = later, later = 2; return first;"),
            format!("{declaration} first = 1, later = later; return first;"),
            format!("console.log(later); {declaration} first = 1, later = 2; return first;"),
            format!("{declaration} first = 1, Math = Math.floor(2); return first;"),
        ] {
            let source = format!(
                "function selected() {{ const later = 7; {{ {body} }} }} console.log(selected());"
            );
            require_declaration_list_refusal(&source, "temporal dead zone");
        }
    }
    for source in [
        "function selected(){const read=()=>later,later=2;return read();}",
        "function selected(){const read=()=>read,next=2;return next;}",
        "function selected(){const [first=later,later=2]=[],next=first;return next;}",
    ] {
        require_declaration_list_refusal(source, "temporal dead zone");
    }
    for declaration in [
        "first=1,first=2",
        "first=1,[first]=[2]",
        concat!("first=1,{", "value:first}={", "value:2}"),
    ] {
        require_declaration_list_refusal(
            &format!("function selected(){{const {declaration};return first;}}"),
            "duplicate lexical binding",
        );
    }
    for (pattern, reason) in [
        ("[]", "empty array binding"),
        ("[,]", "empty array binding"),
        ("{}", "empty object binding"),
    ] {
        require_declaration_list_refusal(
            &format!(
                "function selected(){{const {pattern}=null,next=1;return next;}}console.log(selected());"
            ),
            reason,
        );
    }
}

#[test]
fn mutable_dense_literal_bindings_keep_order_scope_and_complete_archives() {
    for (source, expected) in [
        (
            "function selected(first, second) {\n  let [more, less] = [false, false];\n  if (first > 0) more = true;\n  if (second > 0) less = true;\n  return more === less ? 0 : more ? 1 : -1;\n}\nconsole.log(selected(1,0), selected(0,1), selected(1,1), selected(0,0));",
            json!({"lines":["1 -1 0 0"],"error":null}),
        ),
        (
            "function mark(value) { console.log(value); return value; }\nfunction selected(seed) {\n  let first = seed, [left, right] = [mark(first), mark(first + 1)], last = right;\n  left += last;\n  right = left + 1;\n  return left + right;\n}\nconsole.log(selected(2));",
            json!({"lines":["2","3","11"],"error":null}),
        ),
        (
            "function stop() { console.log('stop'); throw new Error('halt'); }\nfunction later() { console.log('later'); return 2; }\nfunction selected() { let [first, second] = [stop(), later()]; return first + second; }\nconsole.log(selected());",
            json!({"lines":["stop"],"error":{"name":"Error","message":"halt"}}),
        ),
        (
            "function selected() {\n  let total = 0;\n  for (let [index, limit] = [0, 4]; index < limit; index++) total += index;\n  let [left, right] = [2, 3];\n  { let [left, right] = [7, 8]; left++; console.log(left, right); }\n  right += total;\n  return left + right;\n}\nconsole.log(selected());",
            json!({"lines":["8 8","11"],"error":null}),
        ),
        (
            "function selected() { let [left, right] = [-0, Infinity]; left = 1 / left; return left + right; }\nconsole.log(selected());",
            json!({"lines":["NaN"],"error":null}),
        ),
    ] {
        require_declaration_list_equivalent(source, &expected);
    }
}

#[test]
fn mutable_array_bindings_keep_original_refusals_and_temporal_dead_zones() {
    let original = "function f(n) { let [a] = n; return a; }";
    assert_eq!(
        parse_javascript(original).unwrap_err().message(),
        "let destructuring: declare each binding with its own let at 16..20"
    );
    for binding in [
        "[first]=values",
        "[first,second]=[1]",
        "[first]=[1,2]",
        "[first=2]=[1]",
        "[...first]=[1]",
        "[[first]]=[[1]]",
        "[,first]=[1,2]",
        "[first]=[...values]",
        "{first}={first:1}",
    ] {
        let source = format!("function selected(values){{let {binding};return 0;}}");
        let error = parse_javascript(&source).unwrap_err();
        assert!(
            error.message().contains("let destructuring"),
            "{source}: {error:?}"
        );
    }
    for source in [
        "function selected(){let [first,second]=[second,1];return first;}",
        "function selected(){const first=9;{let [first,second]=[first,1];return second;}}",
        "function selected(){console.log(first);let [first,second]=[1,2];return second;}",
        "function selected(){let first=second,[second,last]=[1,2];return first;}",
    ] {
        let error = parse_javascript(source).unwrap_err();
        assert!(
            error.message().contains("temporal dead zone"),
            "{source}: {error:?}"
        );
    }
    for source in [
        "function selected(){let [first,first]=[1,2];return first;}",
        "function selected(){let first=1,[first]=[2];return first;}",
        "function selected(){let [first]=[1],first=2;return first;}",
    ] {
        let error = parse_javascript(source).unwrap_err();
        assert!(
            error
                .message()
                .contains("distinct admitted lexical identifier")
                || error.message().contains("duplicate lexical binding"),
            "{source}: {error:?}"
        );
    }
    let changed = "function selected(){let [left,right]=[1,2];return left+right;}Array.prototype[Symbol.iterator]=()=>({next:()=>({value:9,done:false})});console.log(selected());";
    assert!(
        parse_javascript(changed)
            .and_then(|surface| check_program(&surface))
            .is_err()
    );
}

#[test]
fn mutable_literal_binding_owners_preserve_shadows_and_refuse_redeclarations() {
    for (source, expected) in [
        (
            "function selected(left) {\n  for (let [left,right]=[1,2];left<right;left++) console.log(left,right);\n  return left;\n}\nconsole.log(selected(9));",
            json!({"lines":["1 2","9"],"error":null}),
        ),
        (
            "function selected() {\n  let [left,right]=[10,20];\n  for(let [left,right]=[0,2];left<right;left++) {\n    let [left,right]=[3,4];console.log(left,right);\n  }\n  return left+right;\n}\nconsole.log(selected());",
            json!({"lines":["3 4","3 4","30"],"error":null}),
        ),
        (
            "function selected(){const apply=left=>{{let [left,right]=[2,3];return left+right;}};return apply(9);}\nconsole.log(selected());",
            json!({"lines":["5"],"error":null}),
        ),
    ] {
        require_declaration_list_equivalent(source, &expected);
    }
    for source in [
        "function selected(){let left=9;let [left,right]=[1,2];return right;}",
        "function selected(){let [left,right]=[1,2];let left=9;return right;}",
        "function selected(){const left=9;let [left,right]=[1,2];return right;}",
        "function selected(){let [left,right]=[1,2];const left=9;return right;}",
        "function selected(){const [left]=[9];let [left,right]=[1,2];return right;}",
        "function selected(){let [left,right]=[1,2];const {value:left}={value:9};return right;}",
        "function selected(left){let [left,right]=[1,2];return right;}",
        "function selected(){const apply=left=>{let [left,right]=[1,2];return right;};return apply(9);}",
        "function selected(){const apply=function(left){let [left,right]=[1,2];return right;};return apply(9);}",
    ] {
        require_declaration_list_refusal(source, "duplicate lexical binding left");
        let output = Command::new("node").args([
            "--input-type=module", "-e",
            "import {Script} from 'node:vm';try{new Script(process.argv[1]);process.stdout.write('accepted');}catch(error){process.stdout.write(error.name);}", source,
        ]).output().unwrap();
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            "SyntaxError",
            "{source}"
        );
    }
    require_declaration_list_refusal(
        "let [left,right]=[1,2];console.log(left+right);",
        "mutable literal bindings require a function-owned lexical scope",
    );
}

#[test]
fn represented_defaults_keep_actual_absence_and_lazy_initializer_effects() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/binding-default-oracle.mjs",
        "JavaScript represented binding defaults",
        meta_language::create_array_binding_program_links,
    );
}
