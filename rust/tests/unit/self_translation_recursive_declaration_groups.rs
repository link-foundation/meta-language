//! Unexecuted private draft. Native compilation and observation run only in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, ParseConfiguration, SelfTranslationOptions,
    create_typed_program_links, decode_link_network_binary, decode_typed_program_links,
    emit_typed_program_links, encode_link_network_binary, encode_typed_program_links,
    read_structured_data_lino, self_translate, self_translate_with, self_translation_signatures,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command, sync::OnceLock};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}
fn policy() -> Value {
    read_structured_data_lino(include_str!(
        "../../src/data/declaration-dependency-rules.lino"
    ))
    .unwrap()
}
fn versions() {
    static CHECKED: OnceLock<()> = OnceLock::new();
    CHECKED.get_or_init(|| {
        let value = policy();
        assert_eq!(value["assertionVersions"]["Rust"], ">=1.90.0 <1.100.0");
        assert_eq!(
            value["assertionVersions"]["Node"],
            ">=22.15.0 <23.0.0 || >=24.0.0 <25.0.0"
        );
        for (tool, rust) in [("rustc", true), ("node", false)] {
            let result = run(Command::new(tool).arg("--version"));
            assert!(result.ok, "{}", result.stderr);
            let version = if rust {
                result.stdout.split_whitespace().nth(1).unwrap()
            } else {
                result.stdout.trim().trim_start_matches('v')
            };
            let mut parts = version.split('.');
            let major = parts.next().unwrap().parse::<u32>().unwrap();
            let minor = parts.next().unwrap().parse::<u32>().unwrap();
            assert!(
                if rust {
                    major == 1 && (90..100).contains(&minor)
                } else {
                    major == 22 && minor >= 15 || major == 24
                },
                "uncovered {tool} version {version}"
            );
        }
    });
}
fn archives(network: &LinkNetwork) -> [LinkNetwork; 2] {
    [
        LinkNetwork::from_lino(&network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}
fn javascript_bridge(source: &str, language: &str, script_mode: bool) -> Value {
    let script = r#"
import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';
import {structuredDataToLino} from './js/src/structured-data-links.js';
import {runInNewContext} from 'node:vm';
const source=process.argv[1],language=process.argv[2],scriptMode=process.argv[3]==='Script';
const rawOwner=scriptMode?runInNewContext(source+'\n({value:first,error:(()=>{try{second(0);return null;}catch(error){return error.name;}})()});',{}, {timeout:1000}):null;
process.stdout.write(structuredDataToLino({translation:selfTranslate(source,language,'Rust'),signatures:selfTranslationSignatures(source,language),rawOwner}));
"#;
    let output = run(Command::new("node").current_dir(root()).args([
        "--input-type=module",
        "-e",
        script,
        source,
        language,
        if script_mode { "Script" } else { "Module" },
    ]));
    assert!(output.ok, "{}", output.stderr);
    read_structured_data_lino(&output.stdout).unwrap()
}
fn javascript_fixture_program(source: &str, calls: &Value) -> String {
    let suffix = calls
        .as_array()
        .unwrap()
        .iter()
        .map(|call| format!("console.log({});", call.as_str().unwrap()))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{source}\n{suffix}")
}
fn observe_javascript(source: &str, calls: &Value, expected: &str, script_mode: bool) {
    let source = javascript_fixture_program(source, calls);
    if script_mode {
        let directory = TemporaryDirectory::new("recursive-script-fixture");
        let file = directory.0.join("original-script.txt");
        fs::write(&file, source).unwrap();
        let runner = "import {readFileSync} from 'node:fs';import {runInNewContext} from 'node:vm';runInNewContext(readFileSync(process.argv[1],'utf8'),{console},{timeout:1000});";
        let output = run(Command::new("node")
            .args(["--input-type=module", "-e", runner])
            .arg(file));
        assert!(output.ok, "{}", output.stderr);
        assert_eq!(output.stdout, expected);
    } else {
        observe_complete_javascript(&source, expected, "recursive-javascript-fixture");
    }
}
fn observe_complete_javascript(source: &str, expected: &str, name: &str) {
    let directory = TemporaryDirectory::new(name);
    let file = directory.0.join("program.mjs");
    fs::write(&file, source).unwrap();
    let output = run(Command::new("node").arg(file));
    assert!(output.ok, "{}", output.stderr);
    assert_eq!(output.stdout, expected);
}
fn compile_and_observe(source: &str, calls: &Value, expected: &str, name: &str) {
    let directory = TemporaryDirectory::new(name);
    let source_file = directory.0.join("program.rs");
    let binary = directory
        .0
        .join(format!("program{}", std::env::consts::EXE_SUFFIX));
    let calls = calls
        .as_array()
        .unwrap()
        .iter()
        .map(|call| format!("println!(\"{{}}\",{});", call.as_str().unwrap()))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        &source_file,
        format!("{source}\nfn main(){{\n{calls}\n}}\n"),
    )
    .unwrap();
    let mut command = Command::new("rustc");
    command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        command.arg("-C").arg(format!("linker={linker}"));
    }
    let output = run(command.arg(&source_file).arg("-o").arg(&binary));
    assert!(output.ok, "{}\n{}", output.stdout, output.stderr);
    let output = run(&mut Command::new(binary));
    assert!(output.ok, "{}", output.stderr);
    assert_eq!(output.stdout, expected);
}
fn compile_complete_and_observe(source: &str, expected: &str, name: &str) {
    let directory = TemporaryDirectory::new(name);
    let file = directory.0.join("program.rs");
    let binary = directory
        .0
        .join(format!("program{}", std::env::consts::EXE_SUFFIX));
    fs::write(&file, source).unwrap();
    let mut command = Command::new("rustc");
    command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        command.arg("-C").arg(format!("linker={linker}"));
    }
    let output = run(command.arg(&file).arg("-o").arg(&binary));
    assert!(output.ok, "{}", output.stderr);
    let output = run(&mut Command::new(binary));
    assert!(output.ok, "{}", output.stderr);
    assert_eq!(output.stdout, expected);
}

#[test]
fn every_shared_complete_module_fixture_keeps_atomic_results_actual_signatures_spans_and_ci_observations()
 {
    versions();
    let value = policy();
    for fixture in value["moduleFixtures"].as_array().unwrap() {
        let source = fixture["source"].as_str().unwrap();
        let language = fixture["language"].as_str().unwrap();
        let script_mode = fixture["sourceMode"] == "Script";
        let expected = fixture["expectedLines"].as_str().unwrap();
        let actual = self_translate(source, language, "Rust").unwrap();
        let javascript = javascript_bridge(source, language, script_mode);
        if script_mode {
            assert_eq!(javascript["rawOwner"]["value"], fixture["sourceOwnerValue"]);
            assert_eq!(
                javascript["rawOwner"]["error"],
                fixture["sourceOwnerCallError"]
            );
        }
        assert_eq!(
            actual.code,
            javascript["translation"]["code"].as_str().unwrap(),
            "{}",
            fixture["id"]
        );
        assert_eq!(
            serde_json::to_value(
                self_translation_signatures(source, language, &Default::default()).unwrap()
            )
            .unwrap(),
            javascript["signatures"]
        );
        let items=actual.items.iter().map(|item|json!({"term":item.term,"start":item.start,"end":item.end,"status":item.status,"reason":item.reason})).collect::<Vec<_>>();
        assert_eq!(
            serde_json::to_value(items).unwrap(),
            javascript["translation"]["items"],
            "original noncontiguous source groups retain their actual byte spans"
        );
        let statuses = actual
            .items
            .iter()
            .filter(|item| item.term == "function_declaration" || item.term == "export_statement")
            .map(|item| item.status)
            .collect::<Vec<_>>();
        assert_eq!(
            serde_json::to_value(statuses).unwrap(),
            fixture["functionStatuses"]
        );
        let signatures =
            self_translation_signatures(source, language, &Default::default()).unwrap();
        assert_eq!(
            serde_json::to_value(
                signatures
                    .iter()
                    .map(|signature| signature.name())
                    .collect::<Vec<_>>()
            )
            .unwrap(),
            fixture["signatures"]
        );
        assert_eq!(
            self_translate(&actual.code, "Rust", language).unwrap().code,
            source
        );
        observe_javascript(source, &fixture["calls"], expected, script_mode);
        compile_and_observe(
            &actual.code,
            &fixture["rustCalls"],
            expected,
            "recursive-module-fixture",
        );
        let concrete = LinkNetwork::parse(source, language, ParseConfiguration::default());
        for restored in archives(&concrete) {
            assert_eq!(restored.reconstruct_text().as_bytes(), source.as_bytes());
        }
        if fixture["semanticCodecs"] == true {
            let complete = javascript_fixture_program(source, &fixture["calls"]);
            assert!(
                complete.starts_with(source),
                "the original whole fixture remains the exact source prefix"
            );
            let packet = create_typed_program_links(&complete, language).unwrap();
            let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
            for restored in archives(&packet.network) {
                assert_eq!(
                    decode_typed_program_links(&restored, packet.root).unwrap(),
                    program
                );
                let native = emit_typed_program_links(&restored, packet.root, "Rust")
                    .unwrap()
                    .code;
                compile_complete_and_observe(&native, expected, "recursive-typed-codec");
                observe_javascript(
                    &emit_typed_program_links(&restored, packet.root, "JavaScript")
                        .unwrap()
                        .code,
                    &json!([]),
                    expected,
                    false,
                );
            }
            let mut changed = program.clone();
            let declaration = changed
                .declarations
                .iter_mut()
                .find(|declaration| declaration.name() == "isOdd")
                .unwrap();
            let meta_language::translation::ir::Decl::Fn(function) = declaration else {
                panic!("the actual original isOdd owner is a checked function")
            };
            function.body.ty = meta_language::translation::types::Type::String;
            assert!(
                encode_typed_program_links(&changed).is_err(),
                "altered recursive return semantics cannot publish as checked Links"
            );
        }
    }
}

#[test]
fn whole_module_writes_and_eval_cannot_leak_a_recursive_owner_through_either_retry_path() {
    versions();
    let first = "function first(value) { if(value <= 0) return true; return second(value - 1); }\n";
    let second = "function second(value) { return first(value); }\n";
    for effect in ["first = 0;\n", "eval(\"first = 0\");\n"] {
        for between in [true, false] {
            let source = format!(
                "{first}{}{second}{}function independent() {{ return 31; }}\n",
                if between { effect } else { "" },
                if between { "" } else { effect }
            );
            let javascript = javascript_bridge(&source, "JavaScript", true);
            assert_eq!(javascript["rawOwner"]["value"], 0);
            assert_eq!(javascript["rawOwner"]["error"], "TypeError");
            let actual = self_translate(&source, "JavaScript", "Rust").unwrap();
            assert_eq!(
                actual.code,
                javascript["translation"]["code"].as_str().unwrap()
            );
            assert!(!actual.code.contains("pub fn first("));
            assert!(!actual.code.contains("pub fn second("));
            assert!(actual.code.contains("pub fn independent("));
            assert!(
                self_translation_signatures(&source, "JavaScript", &Default::default())
                    .unwrap()
                    .is_empty()
            );
            assert_eq!(
                self_translate(&actual.code, "Rust", "JavaScript")
                    .unwrap()
                    .code,
                source
            );
            observe_javascript(&source, &json!(["independent()"]), "31\n", true);
            compile_and_observe(
                &actual.code,
                &json!(["independent()"]),
                "31\n",
                "recursive-module-effect",
            );
        }
    }
}

#[test]
fn actual_named_import_signatures_and_complete_wider_module_context_match_javascript() {
    versions();
    let value = policy();
    let pair = value["moduleFixtures"][0]["source"].as_str().unwrap();
    let complete = format!("{pair}console.log(choose(6));\n");
    let actual = self_translate(&complete, "JavaScript", "Rust").unwrap();
    let javascript = javascript_bridge(&complete, "JavaScript", false);
    assert_eq!(
        actual.code,
        javascript["translation"]["code"].as_str().unwrap()
    );
    let statuses = actual
        .items
        .iter()
        .filter(|item| item.term == "function_declaration" || item.term == "export_statement")
        .map(|item| item.status)
        .collect::<Vec<_>>();
    assert_eq!(statuses, vec!["translated", "translated", "translated"]);
    assert_eq!(
        self_translate(&actual.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        complete
    );
    observe_complete_javascript(&complete, "true\n", "recursive-wider-module");

    let dependency = "/** @param {number} value @returns {number} */\nexport function lower(value) { return value - 1; }\n";
    let source = "import { lower as step } from \"./dependency.js\";\nexport function left(value) { if(value <= 0) return true; return right(step(value)); }\nfunction unavailable() { return new Date(); }\nfunction right(value) { if(value <= 0) return false; return left(step(value)); }\n";
    let signatures =
        self_translation_signatures(dependency, "JavaScript", &Default::default()).unwrap();
    let options = SelfTranslationOptions {
        imports: BTreeMap::from([("./dependency.js".to_owned(), signatures)]),
        ..Default::default()
    };
    let actual = self_translate_with(source, "JavaScript", "Rust", &options).unwrap();
    let script = r#"
import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';
import {structuredDataToLino} from './js/src/structured-data-links.js';
const source=process.argv[1],dependency=process.argv[2];
const options={imports:{'./dependency.js':selfTranslationSignatures(dependency,'JavaScript')}};
process.stdout.write(structuredDataToLino({translation:selfTranslate(source,'JavaScript','Rust',options),signatures:selfTranslationSignatures(source,'JavaScript',options)}));
"#;
    let output = run(Command::new("node").current_dir(root()).args([
        "--input-type=module",
        "-e",
        script,
        source,
        dependency,
    ]));
    assert!(output.ok, "{}", output.stderr);
    let javascript = read_structured_data_lino(&output.stdout).unwrap();
    assert_eq!(
        actual.code,
        javascript["translation"]["code"].as_str().unwrap()
    );
    let signatures = self_translation_signatures(source, "JavaScript", &options).unwrap();
    assert_eq!(
        serde_json::to_value(&signatures).unwrap(),
        javascript["signatures"]
    );
    assert_eq!(
        signatures
            .iter()
            .map(meta_language::SelfTranslationSignature::name)
            .collect::<Vec<_>>(),
        vec!["left"]
    );
    let statuses = actual
        .items
        .iter()
        .filter(|item| item.term == "function_declaration" || item.term == "export_statement")
        .map(|item| item.status)
        .collect::<Vec<_>>();
    assert_eq!(statuses, vec!["translated", "carried", "translated"]);
    assert!(
        actual
            .code
            .contains("use crate::dependency::lower as step;")
    );
    assert_eq!(
        self_translate(&actual.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    let dependency = self_translate(dependency, "JavaScript", "Rust").unwrap();
    compile_and_observe(
        &format!(
            "{}\nmod dependency {{\n{}\n}}",
            actual.code, dependency.code
        ),
        &json!(["left(0f64)", "left(1f64)", "left(4f64)"]),
        "true\nfalse\ntrue\n",
        "recursive-import-context",
    );
}

#[test]
fn ordinary_and_public_module_signatures_use_actual_owner_counts_and_preserve_every_safe_fixture_in_ci()
 {
    versions();
    let value = policy();
    let original = value["ordinaryModuleOwnerSource"].as_str().unwrap();
    assert_eq!(
        original,
        "function first(value) { return value; }\nvar first = 0;\nfunction second(value) { return first(value); }\n"
    );
    let javascript = javascript_bridge(original, "JavaScript", true);
    assert_eq!(javascript["rawOwner"]["value"], 0);
    assert_eq!(javascript["rawOwner"]["error"], "TypeError");
    let native = self_translate(original, "JavaScript", "Rust").unwrap();
    assert_eq!(
        native.code,
        javascript["translation"]["code"].as_str().unwrap()
    );
    assert!(!native.code.contains("pub fn second("));
    assert!(
        self_translation_signatures(original, "JavaScript", &Default::default())
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        self_translate(&native.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        original
    );
    let script = r#"
import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';
import {structuredDataToLino,readStructuredDataLino} from './js/src/structured-data-links.js';
import {runInNewContext} from 'node:vm';
const fixture=readStructuredDataLino(process.argv[1]);
const raw=fixture.sourceMode==='Script'?runInNewContext(fixture.source+'\n({first,second,independent});',{}, {timeout:1000})
  :await import('data:text/javascript;base64,'+Buffer.from(fixture.source).toString('base64'));
const owner=Object.hasOwn(fixture,'sourceOwnerValue')?{value:raw.first,error:(()=>{try{raw.second(0);return null;}catch(error){return error.name;}})()}:null;
const lines=fixture.calls.map(call=>{if(call==='limit')return String(raw.limit);const match=/^([A-Za-z_][A-Za-z_0-9]*)\((\d*)\)$/u.exec(call);if(!match)throw new Error('unsupported fixture call');return String(match[2]?raw[match[1]](Number(match[2])):raw[match[1]]());}).join('\n')+'\n';
process.stdout.write(structuredDataToLino({owner,lines,translation:selfTranslate(fixture.source,fixture.language,'Rust'),signatures:selfTranslationSignatures(fixture.source,fixture.language)}));
"#;
    for fixture in value["ordinaryModuleFixtures"].as_array().unwrap() {
        let source = fixture["source"].as_str().unwrap();
        let language = fixture["language"].as_str().unwrap();
        let input = meta_language::structured_data_to_lino(fixture).unwrap();
        let output = run(Command::new("node").current_dir(root()).args([
            "--input-type=module",
            "-e",
            script,
            &input,
        ]));
        assert!(output.ok, "{}", output.stderr);
        let javascript = read_structured_data_lino(&output.stdout).unwrap();
        if fixture.get("sourceOwnerValue").is_some() {
            assert_eq!(javascript["owner"]["value"], fixture["sourceOwnerValue"]);
            assert_eq!(
                javascript["owner"]["error"],
                fixture["sourceOwnerCallError"]
            );
        }
        assert_eq!(javascript["lines"], fixture["expectedLines"]);
        let actual = self_translate(source, language, "Rust").unwrap();
        assert_eq!(
            actual.code,
            javascript["translation"]["code"].as_str().unwrap()
        );
        let items=actual.items.iter().map(|item|json!({"term":item.term,"start":item.start,"end":item.end,"status":item.status,"reason":item.reason})).collect::<Vec<_>>();
        assert_eq!(
            serde_json::to_value(items).unwrap(),
            javascript["translation"]["items"]
        );
        let statuses = actual
            .items
            .iter()
            .filter(|item| item.term == "function_declaration" || item.term == "export_statement")
            .map(|item| item.status)
            .collect::<Vec<_>>();
        assert_eq!(
            serde_json::to_value(statuses).unwrap(),
            fixture["functionStatuses"]
        );
        let signatures =
            self_translation_signatures(source, language, &Default::default()).unwrap();
        assert_eq!(
            serde_json::to_value(&signatures).unwrap(),
            javascript["signatures"]
        );
        assert_eq!(
            serde_json::to_value(
                signatures
                    .iter()
                    .map(meta_language::SelfTranslationSignature::name)
                    .collect::<Vec<_>>()
            )
            .unwrap(),
            fixture["signatures"]
        );
        assert_eq!(
            self_translate(&actual.code, "Rust", language).unwrap().code,
            source
        );
        if fixture.get("sourceOwnerValue").is_some() {
            assert!(!actual.code.contains("pub fn second("));
            assert!(actual.code.contains("pub fn independent("));
        }
        compile_and_observe(
            &actual.code,
            &fixture["rustCalls"],
            fixture["expectedLines"].as_str().unwrap(),
            "ordinary-module-binding",
        );
        for restored in archives(&LinkNetwork::parse(
            source,
            language,
            ParseConfiguration::default(),
        )) {
            assert_eq!(restored.reconstruct_text(), source);
        }
    }
}

#[test]
fn every_shared_module_builtin_owner_fixture_retains_actual_module_or_script_observations_and_native_parameter_behavior()
 {
    versions();
    let value = policy();
    let script = r#"
import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';
import {structuredDataToLino,readStructuredDataLino} from './js/src/structured-data-links.js';
import {runInNewContext} from 'node:vm';
const fixture=readStructuredDataLino(process.argv[1]);
const probe=Object.hasOwn(fixture,'sourceProbeExpected')?'('+fixture.sourceProbe+')':'(()=>{try{'+fixture.sourceProbe+';return "";}catch(error){return error.name;}})()';
const observedValues='['+fixture.calls.join(',')+']';
const raw=fixture.sourceMode==='Script'
  ?runInNewContext(fixture.source+'\n({observation:'+probe+',observedValues:'+observedValues+'});',{}, {timeout:1000})
  :await import('data:text/javascript;charset=utf-8,'+encodeURIComponent(fixture.source+'\nexport const observation='+probe+';\nexport const observedValues='+observedValues+';\n'));
// Observation rendering occurs outside the original scope: its actual String
// class binding must never be called by the test harness's output renderer.
const lines=Array.from(raw.observedValues,value=>String(value)).join('\n')+'\n';
process.stdout.write(structuredDataToLino({observation:raw.observation,lines,translation:selfTranslate(fixture.source,fixture.language,'Rust'),signatures:selfTranslationSignatures(fixture.source,fixture.language)}));
"#;
    for fixture in value["moduleOwnerFixtures"].as_array().unwrap() {
        let source = fixture["source"].as_str().unwrap();
        let language = fixture["language"].as_str().unwrap();
        let input = meta_language::structured_data_to_lino(fixture).unwrap();
        let output = run(Command::new("node").current_dir(root()).args([
            "--input-type=module",
            "-e",
            script,
            &input,
        ]));
        assert!(output.ok, "{}", output.stderr);
        let javascript = read_structured_data_lino(&output.stdout).unwrap();
        let expected = fixture
            .get("sourceProbeExpected")
            .unwrap_or(&fixture["sourceProbeError"]);
        assert_eq!(&javascript["observation"], expected, "{}", fixture["id"]);
        assert_eq!(javascript["lines"], fixture["expectedLines"]);
        let actual = self_translate(source, language, "Rust").unwrap();
        assert_eq!(
            actual.code,
            javascript["translation"]["code"].as_str().unwrap(),
            "{}",
            fixture["id"]
        );
        let items=actual.items.iter().map(|item|json!({"term":item.term,"start":item.start,"end":item.end,"status":item.status,"reason":item.reason})).collect::<Vec<_>>();
        assert_eq!(
            serde_json::to_value(items).unwrap(),
            javascript["translation"]["items"]
        );
        let statuses = actual
            .items
            .iter()
            .filter(|item| item.term == "function_declaration" || item.term == "export_statement")
            .map(|item| item.status)
            .collect::<Vec<_>>();
        assert_eq!(
            serde_json::to_value(statuses).unwrap(),
            fixture["functionStatuses"]
        );
        let signatures =
            self_translation_signatures(source, language, &Default::default()).unwrap();
        assert_eq!(
            serde_json::to_value(&signatures).unwrap(),
            javascript["signatures"]
        );
        assert_eq!(
            serde_json::to_value(
                signatures
                    .iter()
                    .map(meta_language::SelfTranslationSignature::name)
                    .collect::<Vec<_>>()
            )
            .unwrap(),
            fixture["signatures"]
        );
        assert_eq!(
            self_translate(&actual.code, "Rust", language).unwrap().code,
            source
        );
        compile_and_observe(
            &actual.code,
            &fixture["rustCalls"],
            fixture["expectedLines"].as_str().unwrap(),
            "module-builtin-owner",
        );
        for restored in archives(&LinkNetwork::parse(
            source,
            language,
            ParseConfiguration::default(),
        )) {
            assert_eq!(restored.reconstruct_text(), source);
        }
    }
}

#[test]
fn a_real_named_import_called_string_preserves_its_actual_function_call_and_both_runtime_outputs() {
    versions();
    let dependency = "/** @param {number} value @returns {number} */\nexport function increment(value){return value+1;}\n";
    let source = "import { increment as String } from './dependency.mjs';\n/** @returns {number} */\nexport function selected(){return String(7);}\n";
    let signatures =
        self_translation_signatures(dependency, "JavaScript", &Default::default()).unwrap();
    let options = SelfTranslationOptions {
        imports: BTreeMap::from([("./dependency.mjs".to_owned(), signatures)]),
        ..Default::default()
    };
    let actual = self_translate_with(source, "JavaScript", "Rust", &options).unwrap();
    assert!(actual.items.iter().all(|item| item.status == "translated"));
    let script = r#"
import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';
import {structuredDataToLino} from './js/src/structured-data-links.js';
const source=process.argv[1],dependency=process.argv[2];
const options={imports:{'./dependency.mjs':selfTranslationSignatures(dependency,'JavaScript')}};
process.stdout.write(structuredDataToLino({translation:selfTranslate(source,'JavaScript','Rust',options),signatures:selfTranslationSignatures(source,'JavaScript',options)}));
"#;
    let output = run(Command::new("node").current_dir(root()).args([
        "--input-type=module",
        "-e",
        script,
        source,
        dependency,
    ]));
    assert!(output.ok, "{}", output.stderr);
    let javascript = read_structured_data_lino(&output.stdout).unwrap();
    assert_eq!(
        actual.code,
        javascript["translation"]["code"].as_str().unwrap()
    );
    assert_eq!(
        serde_json::to_value(self_translation_signatures(source, "JavaScript", &options).unwrap())
            .unwrap(),
        javascript["signatures"]
    );
    assert_eq!(
        self_translate(&actual.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    let directory = TemporaryDirectory::new("actual-string-import");
    fs::write(directory.0.join("dependency.mjs"), dependency).unwrap();
    let original = javascript_fixture_program(source, &json!(["selected()"]));
    assert!(original.starts_with(source));
    let file = directory.0.join("program.mjs");
    fs::write(&file, original).unwrap();
    let output = run(Command::new("node").arg(file));
    assert!(output.ok, "{}", output.stderr);
    assert_eq!(output.stdout, "8\n");
    let dependency = self_translate(dependency, "JavaScript", "Rust").unwrap();
    compile_and_observe(
        &format!(
            "{}\nmod dependency {{\n{}\n}}",
            actual.code, dependency.code
        ),
        &json!(["selected()"]),
        "8\n",
        "actual-string-import",
    );
}

// Independent finite transitive closure is deliberately unlike the production
// generated two-pass walk. It is only the native test oracle, never host policy.
fn graph_case(mask: u32) -> (Vec<usize>, Vec<usize>, Vec<usize>, [usize; 3]) {
    let mut adjacency = [[false; 3]; 3];
    for (source, row) in adjacency.iter_mut().enumerate() {
        for (target, edge) in row.iter_mut().enumerate() {
            *edge = mask & (1 << (source * 3 + target)) != 0;
        }
    }
    let mut reachable = adjacency;
    for (index, row) in reachable.iter_mut().enumerate() {
        row[index] = true;
    }
    for middle in [0, 1, 2] {
        for source in [0, 1, 2] {
            for target in [0, 1, 2] {
                let next = reachable[source][middle] && reachable[middle][target];
                reachable[source][target] |= next;
            }
        }
    }
    let expected = std::array::from_fn(|source| {
        (0..3)
            .find(|target| reachable[source][*target] && reachable[*target][source])
            .unwrap()
    });
    let (mut starts, mut counts, mut targets) = (Vec::new(), Vec::new(), Vec::new());
    for row in adjacency {
        starts.push(targets.len());
        for (target, present) in row.iter().enumerate() {
            if *present {
                targets.push(target);
            }
        }
        counts.push(targets.len() - *starts.last().unwrap());
    }
    (starts, counts, targets, expected)
}
#[test]
fn the_complete_original_shared_algorithm_and_every_three_node_graph_execute_through_both_semantic_archives_in_ci()
 {
    versions();
    let value = policy();
    let closure = &value["sourceClosure"];
    for input in closure["inputs"].as_array().unwrap() {
        let source = fs::read(root().join(input["path"].as_str().unwrap())).unwrap();
        assert_eq!(
            u64::try_from(source.len()).unwrap(),
            input["bytes"].as_u64().unwrap()
        );
        assert_eq!(
            format!("{:x}", Sha256::digest(&source)),
            input["sourceSha256"].as_str().unwrap()
        );
    }
    let mut source = closure["source"].as_str().unwrap().to_owned();
    let mut expected = String::new();
    for mask in 0..512 {
        let (starts, counts, targets, leaders) = graph_case(mask);
        source.push_str(&format!("\nconst case{mask}=readDeclarationDependencyComponents({starts:?},{counts:?},{targets:?});console.log(case{mask}[0],case{mask}[1],case{mask}[2]);\n"));
        expected.push_str(&format!("{} {} {}\n", leaders[0], leaders[1], leaders[2]));
    }
    source.push_str("\nconsole.log(acceptDeclarationModuleScope(['comment','named function declaration','named import','primitive literal binding'],false));\n");
    expected.push_str("true\n");
    for form in [
        "var declaration",
        "assignment",
        "eval",
        "namespace import",
        "const binding list",
        "class declaration",
        "unknown module form",
    ] {
        let literal = serde_json::to_string(form).unwrap();
        source.push_str(&format!("\nconsole.log(acceptDeclarationModuleScope(['named function declaration',{literal}],false));console.log(acceptDeclarationModuleScope(['named function declaration',{literal}],true));\n"));
        expected.push_str("false\ntrue\n");
    }
    source.push_str("\nconsole.log(acceptDeclarationModuleScope(['empty class declaration'],false));console.log(acceptDeclarationSignatureBinding(true,1));\n");
    expected.push_str("true\ntrue\n");
    for count in [0, 2] {
        source.push_str(&format!("\nconsole.log(acceptDeclarationSignatureBinding(true,{count}));console.log(acceptDeclarationSignatureBinding(false,{count}));\n"));
        expected.push_str("false\nfalse\n");
    }
    source.push_str("\nconsole.log(acceptDeclarationSignatureBinding(false,1));\n");
    expected.push_str("false\n");
    observe_complete_javascript(&source, &expected, "recursive-original-graphs");
    let packet = create_typed_program_links(&source, "JavaScript").unwrap();
    let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    for restored in archives(&packet.network) {
        assert_eq!(
            decode_typed_program_links(&restored, packet.root).unwrap(),
            program
        );
        compile_complete_and_observe(
            &emit_typed_program_links(&restored, packet.root, "Rust")
                .unwrap()
                .code,
            &expected,
            "recursive-dependency-graphs",
        );
        let code = emit_typed_program_links(&restored, packet.root, "JavaScript")
            .unwrap()
            .code;
        observe_complete_javascript(&code, &expected, "recursive-javascript-graphs");
    }
}
