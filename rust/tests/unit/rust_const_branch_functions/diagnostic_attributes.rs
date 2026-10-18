//! Actual source prefixes and generated const predicates; native execution is CI-only.

use super::*;
use meta_language::translation::lexer::Source;

const DIAGNOSTIC_MODELS: [&str; 2] = [
    "closed pure Rust const predicates 1",
    "closed Boolean Rust const decision trees 1",
];
const DIAGNOSTIC_ATTRIBUTE: &str = "#[expect(clippy::fn_params_excessive_bools, reason = \"why\")]";
const DIAGNOSTIC_LEAF: &str = "const fn keep(value:bool)->bool{value}";
const DIAGNOSTIC_CAPTURE: &str = r#"#[must_use]
#[expect(
    clippy::fn_params_excessive_bools,
    reason = "the checked source interface retains five independent ordered Boolean facts"
)]
pub const fn accept_java_script_terminal_tag_guard(
    positive: bool,
    alternate: bool,
    terminal: bool,
    same_binding: bool,
    duplicate: bool,
) -> bool {
    (((positive && !alternate) && terminal) && same_binding) && !duplicate
}"#;

fn diagnostic_digest(source: &str) -> String {
    Sha256::digest(source.as_bytes())
        .iter()
        .fold(String::new(), |mut text, byte| {
            write!(text, "{byte:02x}").expect("writing into a String succeeds");
            text
        })
}

fn diagnostic_oracle(source: &str, model: &str, name: &str, inputs: &Value) -> Value {
    let script = r"
import vm from 'node:vm';
import {parseRust,parseRustTokenStream} from './js/src/translation/rust.js';
import {tokenize} from './js/src/translation/lexer.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from './js/src/typed-program-links.js';
import {recheckLinkedTypedProgram} from './js/src/typed-program-checked-projection.js';
import {LinkNetwork} from './js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from './js/src/link-network-binary.js';
const source=process.argv[1],options={constFunctionModel:process.argv[2]},surface=parseRust(source,options),program=checkProgram(surface);
const name=process.argv[3],inputs=JSON.parse(process.argv[4]);
const observe=program=>{const context={};vm.runInNewContext(emitJavaScript(program).text,context,{timeout:1000});return inputs.map(args=>context[name](...args));};
const emission=emitRust(program),packet=encodeTypedProgramLinks(program),archives=[];
for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
 const replay=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
 archives.push({rust:emitRust(replay).text,javascript:emitJavaScript(replay).text,values:observe(replay)});
}
let tokenOnly;
try{tokenOnly={surface:parseRustTokenStream(tokenize(source,'Rust').tokens,options)};}catch(error){tokenOnly={kind:error.kind,reason:error.reason};}
console.log(JSON.stringify({surface,rust:emission.text,definitions:emission.definitions,javascript:emitJavaScript(program).text,values:observe(program),archives,tokenOnly}));
";
    let output = run(Command::new("node")
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
            model,
            name,
            &inputs.to_string(),
        ]));
    assert!(output.ok, "{}", output.stderr);
    serde_json::from_str(&output.stdout).unwrap()
}

fn diagnostic_program(
    source: &str,
    model: &str,
    name: &str,
    inputs: &Value,
    values: &Value,
) -> (Program, Value) {
    let surface = parse_rust_with_const_function_model(source, model, "").unwrap();
    let program = check_program(&surface).unwrap();
    let oracle = diagnostic_oracle(source, model, name, inputs);
    assert_eq!(serde_json::to_value(&surface).unwrap(), oracle["surface"]);
    let rust = emit_rust(&program).unwrap();
    let javascript = emit_javascript(&program).unwrap().text;
    assert_eq!(rust.text, oracle["rust"].as_str().unwrap());
    assert_eq!(
        serde_json::to_value(&rust.definitions).unwrap(),
        oracle["definitions"]
    );
    assert_eq!(javascript, oracle["javascript"].as_str().unwrap());
    assert_eq!(&oracle["values"], values);
    for replay in oracle["archives"].as_array().unwrap() {
        assert_eq!(replay["rust"].as_str().unwrap(), rust.text);
        assert_eq!(replay["javascript"].as_str().unwrap(), javascript);
        assert_eq!(&replay["values"], values);
    }
    let packet = encode_typed_program_links(&program).unwrap();
    for network in archives(&packet) {
        let replay = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(
            replay, program,
            "every source owner and facet survives both codecs"
        );
        assert_eq!(emit_rust(&replay).unwrap().text, rust.text);
        assert_eq!(emit_javascript(&replay).unwrap().text, javascript);
    }
    (program, oracle)
}

fn diagnostic_refusal(source: &str, model: &str, checked: bool) -> Value {
    let parsed = parse_rust_with_const_function_model(source, model, "");
    let error = if checked {
        parsed
            .and_then(|surface| check_program(&surface).map(|_| ()))
            .unwrap_err()
    } else {
        parsed.map(|_| ()).unwrap_err()
    };
    let actual = json!({"kind":error.kind.as_str(),"reason":error.reason,"span":error.span});
    let script = r"
import {parseRust} from './js/src/translation/rust.js';
import {checkProgram} from './js/src/translation/check.js';
let error;try{const surface=parseRust(process.argv[1],{constFunctionModel:process.argv[2]});if(process.argv[3]==='checked')checkProgram(surface);}catch(caught){error=caught;}
if(!error)throw new Error('the complete unchanged source must refuse');
console.log(JSON.stringify({kind:error.kind,reason:error.reason,span:error.span??null}));
";
    let output = run(Command::new("node")
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
            model,
            if checked { "checked" } else { "parsed" },
        ]));
    assert!(output.ok, "{}", output.stderr);
    let oracle: Value = serde_json::from_str(&output.stdout).unwrap();
    assert_eq!(actual, oracle);
    actual
}

#[test]
fn complete_diagnostic_const_capture_retains_all_thirty_two_rows_and_both_source_free_archives() {
    let provider = include_str!("../../../src/translation/javascript/surface_rules.rs");
    assert_eq!(
        diagnostic_digest(provider),
        "d957b63185208b2f2ccc9b6641c2b69b7d6febe3c109abe577a7216b311c0ae7"
    );
    assert_eq!(Source::new(provider).slice(9570, 9971), DIAGNOSTIC_CAPTURE);
    assert_eq!(DIAGNOSTIC_CAPTURE.len(), 401);
    assert_eq!(
        diagnostic_digest(DIAGNOSTIC_CAPTURE),
        "df595bebe7953c30b2cc5ddad6f1a176050f38cd9184fecb9829bb706f6a16dd"
    );
    let inputs = (0..32)
        .map(|bits| {
            (0..5)
                .map(|at| Value::Bool(bits & (1 << at) != 0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let values = inputs
        .iter()
        .map(|row| {
            Value::Bool(
                row[0].as_bool().unwrap()
                    && !row[1].as_bool().unwrap()
                    && row[2].as_bool().unwrap()
                    && row[3].as_bool().unwrap()
                    && !row[4].as_bool().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let source = format!("{DIAGNOSTIC_CAPTURE}\nfn main(){{}}");
    for (index, model) in DIAGNOSTIC_MODELS.into_iter().enumerate() {
        let (program, oracle) = diagnostic_program(
            &source,
            model,
            "accept_java_script_terminal_tag_guard",
            &json!(inputs),
            &json!(values),
        );
        let Decl::Fn(function) = &program.declarations[0] else {
            panic!("the actual source declares a function")
        };
        let facet = function.source_diagnostic_attributes.as_ref().unwrap();
        assert_eq!(facet["model"], "Rust const diagnostic attributes 1");
        assert_eq!(facet["language"], "Rust");
        assert_eq!(facet["edition"], "2024");
        let proof = function.source_const_function.as_ref().unwrap();
        assert_eq!(
            facet["attachment"],
            json!({"sourceName":function.name,"modulePath":[],"constKeywordSpan":proof["keywordSpan"],"functionKeywordSpan":proof["functionKeywordSpan"]})
        );
        assert_eq!(facet["span"]["end"], proof["keywordSpan"]["start"]);
        assert!(
            oracle["definitions"][0]
                .as_str()
                .unwrap()
                .starts_with(facet["raw"].as_str().unwrap())
        );
        assert_eq!(
            proof
                .as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>(),
            [
                "model",
                "language",
                "sourceName",
                "keyword",
                "keywordSpan",
                "functionKeywordSpan",
                "headerSpan",
                "bodySpan",
                "interpretation"
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<std::collections::BTreeSet<_>>()
        );
        let directory = TemporaryDirectory::new(&format!("diagnostic-const-{index}"));
        let original = directory.0.join("original.rs");
        let translated = directory.0.join("translated.rs");
        fs::write(&original, &source).unwrap();
        fs::write(&translated, emit_rust(&program).unwrap().text).unwrap();
        let driver = directory.0.join("main.rs");
        fs::write(&driver, "#[path=\"original.rs\"]mod original;#[path=\"translated.rs\"]mod translated;fn main(){for bits in 0..32{let [positive,alternate,terminal,same_binding,duplicate]=std::array::from_fn(|at|bits&(1<<at)!=0);let expected=positive&&!alternate&&terminal&&same_binding&&!duplicate;assert_eq!(original::accept_java_script_terminal_tag_guard(positive,alternate,terminal,same_binding,duplicate),expected);assert_eq!(translated::accept_java_script_terminal_tag_guard(positive,alternate,terminal,same_binding,duplicate),expected);}}").unwrap();
        let executable = directory
            .0
            .join(format!("program{}", std::env::consts::EXE_SUFFIX));
        let compiled = run(Command::new("rustc")
            .arg("--edition=2024")
            .arg(&driver)
            .arg("-o")
            .arg(&executable));
        assert!(
            compiled.ok,
            "CI compilation of the complete original and emitted artifacts: {}",
            compiled.stderr
        );
        let actual = run(&mut Command::new(&executable));
        assert!(actual.ok, "{}", actual.stderr);
    }
}

#[test]
fn ordered_diagnostic_prefixes_keep_utf16_raw_reasons_visibility_and_following_owners() {
    let prefix = format!(
        "#[must_use]\n#[expect(clippy /* owner é */ :: fn_params_excessive_bools, reason=r###\"😀 ] custom!(x)\"###,)]\n/* between 😀 */#[must_use]\n{DIAGNOSTIC_ATTRIBUTE}\n/* before visibility */pub "
    );
    let source = format!(
        "// before 😀\n{prefix}{DIAGNOSTIC_LEAF}\n#[must_use]const fn other(value:bool)->bool{{!value}}\nfn main(){{}}"
    );
    for model in DIAGNOSTIC_MODELS {
        let (program, _) = diagnostic_program(
            &source,
            model,
            "keep",
            &json!([[false], [true]]),
            &json!([false, true]),
        );
        let Decl::Fn(first) = &program.declarations[0] else {
            unreachable!()
        };
        let Decl::Fn(second) = &program.declarations[1] else {
            unreachable!()
        };
        let facet = first.source_diagnostic_attributes.as_ref().unwrap();
        assert_eq!(facet["raw"], prefix);
        let start = "// before 😀\n".encode_utf16().count();
        assert_eq!(facet["span"]["start"], start);
        assert!("// before 😀\n".len() > start);
        assert_eq!(facet["span"]["end"], start + prefix.encode_utf16().count());
        assert_eq!(
            facet["tokens"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|token| token["kind"] == "string")
                .map(|token| token["value"].as_str().unwrap())
                .collect::<Vec<_>>(),
            ["😀 ] custom!(x)", "why"]
        );
        assert!(
            second.source_diagnostic_attributes.is_none(),
            "a prefix cannot drift to the next function"
        );
        let proof = first.source_const_function.as_ref().unwrap();
        let text = Source::new(&source);
        for (field, expected) in [("keywordSpan", "const"), ("functionKeywordSpan", "fn")] {
            let span = &proof[field];
            let start = usize::try_from(span["start"].as_u64().unwrap()).unwrap();
            let end = usize::try_from(span["end"].as_u64().unwrap()).unwrap();
            assert_eq!(text.slice(start, end), expected);
        }
        diagnostic_program(
            &source,
            model,
            "other",
            &json!([[false], [true]]),
            &json!([true, false]),
        );
    }
}

#[test]
fn legacy_must_use_and_source_free_token_entry_keep_their_original_contract() {
    let legacy = format!("#[must_use]pub {DIAGNOSTIC_LEAF}fn main(){{}}");
    let diagnostic = format!("{DIAGNOSTIC_ATTRIBUTE}{DIAGNOSTIC_LEAF}fn main(){{}}");
    for model in DIAGNOSTIC_MODELS {
        let (legacy_program, legacy_oracle) = diagnostic_program(
            &legacy,
            model,
            "keep",
            &json!([[false], [true]]),
            &json!([false, true]),
        );
        assert_eq!(
            legacy_oracle["tokenOnly"]["surface"],
            legacy_oracle["surface"]
        );
        assert!(legacy_program.declarations.iter().all(|item| matches!(item, Decl::Fn(function) if function.source_diagnostic_attributes.is_none())));
        let (_, diagnostic_oracle) = diagnostic_program(
            &diagnostic,
            model,
            "keep",
            &json!([[false], [true]]),
            &json!([false, true]),
        );
        assert_eq!(diagnostic_oracle["tokenOnly"]["kind"], "type");
        assert_eq!(
            diagnostic_oracle["tokenOnly"]["reason"],
            "Rust diagnostic source attributes: actual complete source and token positions are required"
        );
        // Rust's public API accepts source text, so it always retains the real
        // source. Its private missing-source guard adds no public token API.
    }
}

#[test]
fn diagnostic_admission_keeps_all_twenty_seven_old_refusals_and_exact_unknown_attribute_errors() {
    let old = read_structured_data_lino(include_str!(
        "../../../../experiments/rust-const-function-fixture.lino"
    ))
    .unwrap();
    let invalid = old["invalid"].as_array().unwrap();
    assert_eq!(invalid.len(), 27);
    for entry in invalid {
        let actual = diagnostic_refusal(
            entry["source"].as_str().unwrap(),
            old["model"].as_str().unwrap(),
            true,
        );
        assert_eq!(
            actual,
            json!({"kind":entry["kind"],"reason":entry["reason"],"span":entry["span"]}),
            "{}",
            entry["name"]
        );
    }
    let rejected = [
        DIAGNOSTIC_ATTRIBUTE.replace("clippy::", "other::"),
        DIAGNOSTIC_ATTRIBUTE.replace("fn_params_excessive_bools", "unknown"),
        DIAGNOSTIC_ATTRIBUTE.replace("\"why\"", "1"),
        DIAGNOSTIC_ATTRIBUTE.replace("\"why\"", "concat!(\"why\")"),
        DIAGNOSTIC_ATTRIBUTE.replace("\"why\"", "\"why\", reason=\"again\""),
        "#[cfg(any())]".to_owned(),
        "#[derive(Clone)]".to_owned(),
        "#[inline]".to_owned(),
        "#[no_mangle]".to_owned(),
        "#[custom::expect(\"why\")]".to_owned(),
    ];
    for model in DIAGNOSTIC_MODELS {
        for prefix in &rejected {
            let actual = diagnostic_refusal(
                &format!("{prefix}{DIAGNOSTIC_LEAF}fn main(){{}}"),
                model,
                false,
            );
            assert_eq!(
                actual,
                json!({"kind":"unsupported","reason":"Rust const function attribute: only the actual zero-argument must_use function attribute is admitted","span":null})
            );
        }
    }
}

#[test]
fn diagnostic_prefixes_retain_actual_const_attachment_and_interface_validation_order() {
    for model in DIAGNOSTIC_MODELS {
        for item in [
            "mod nested{}",
            "enum Other{One}",
            "fn ordinary(value:bool)->bool{value}",
        ] {
            let actual = diagnostic_refusal(
                &format!("{DIAGNOSTIC_ATTRIBUTE}{item}fn main(){{}}"),
                model,
                false,
            );
            assert_eq!(actual["kind"], "unsupported");
            assert_eq!(
                actual["reason"],
                "Rust const function context: only actual const predicate declarations and the ordinary main are admitted"
            );
        }
        let main = format!("{DIAGNOSTIC_ATTRIBUTE}\nfn main(){{}}");
        assert_eq!(
            diagnostic_refusal(&main, model, false),
            json!({"kind":"unsupported","reason":"Rust const diagnostic attribute attachment: the supported diagnostic prefix must belong to an actual const function","span":{"start":0,"end":DIAGNOSTIC_ATTRIBUTE.len()+1}})
        );
        let bad =
            format!("{DIAGNOSTIC_ATTRIBUTE}const fn bad(value:bool)->f64{{1.0}}fn main(){{}}");
        diagnostic_refusal(&bad, model, true);
        let inner = format!(
            "#![expect(clippy::fn_params_excessive_bools,reason=\"why\")]{DIAGNOSTIC_LEAF}fn main(){{}}"
        );
        assert_eq!(
            diagnostic_refusal(&inner, model, false),
            json!({"kind":"unsupported","reason":"Rust const function context: crate attributes require an independent compilation interpretation","span":null})
        );
    }
}
