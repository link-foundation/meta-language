//! CI-only actual source, generated decision and complete emitted-program parity.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{
    check::check_program,
    emit_javascript::emit_javascript,
    emit_rust::{RustEmissionOptions, emit_rust_with_array_consumption},
    javascript::{
        ModuleContext, parse_javascript, parse_javascript_in,
        parse_javascript_with_scalar_loop_model,
    },
    surface::External,
    types::FLOAT,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, ParseConfiguration, create_typed_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, read_structured_data_lino,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

#[path = "../../src/translation/javascript_record_parameter_rules.rs"]
mod generated_decisions;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}
fn policy() -> Value {
    read_structured_data_lino(include_str!(
        "../../src/data/javascript-record-parameter-rules.lino"
    ))
    .unwrap()
}
fn versions() {
    static CHECKED: OnceLock<()> = OnceLock::new();
    CHECKED.get_or_init(|| {
        let policy = policy();
        assert_eq!(policy["assertionVersions"]["Rust"], ">=1.90.0 <1.100.0");
        assert_eq!(
            policy["assertionVersions"]["Node"],
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

fn javascript_bridge(mode: &str, source: &str) -> Value {
    versions();
    let script = r#"
import {runInNewContext} from 'node:vm';
import {structuredDataToLino} from './js/src/structured-data-links.js';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
import {tokenize} from './js/src/translation/lexer.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from './js/src/translation/array-binding-policy.js';
import * as rules from './js/src/translation/javascript-record-parameter-rules.js';
const mode=process.argv[1],source=process.argv[2];
if(mode==='raw-error'){
 let name=null;try{runInNewContext(source,{console:{log(){}}},{timeout:1000});}catch(error){name=error.name;}
 process.stdout.write(structuredDataToLino({name}));
}else if(mode==='context'){
 const surface=parseJavaScript(source,{externals:[{k:'const',name:'record_parameter_value1',type:{kind:'float'},literal:false}]});
 process.stdout.write(structuredDataToLino({surface}));
}else if(mode==='plans'){
 const tokens=tokenize(source,'JavaScript').tokens,kinds=tokens.map(t=>t.kind),values=tokens.map(t=>t.value);
 const rows=[];for(let end=0;end<=tokens.length;end++)rows.push({kinds:kinds.slice(0,end),values:values.slice(0,end),
 plan:rules.planJavaScriptRecordParameters(kinds.slice(0,end),values.slice(0,end),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings),
 found:rules.findJavaScriptRecordParameterPattern(kinds.slice(0,end),values.slice(0,end),0)});
 process.stdout.write(structuredDataToLino(rows));
}else if(mode==='directives'){
 const tokens=tokenize(source,'JavaScript').tokens,kinds=tokens.map(t=>t.kind),values=tokens.map(t=>t.value);
 const raws=tokens.map(t=>source.slice(t.start,t.end));
 const lines=tokens.map((t,i)=>/[\n\r\u2028\u2029]/u.test(source.slice(i===0?0:tokens[i-1].end,t.start)));
 const rows=[];for(let end=0;end<=tokens.length;end++){
  const columns=[kinds,values,raws,lines].map(values=>values.slice(0,end));
  rows.push({kinds:columns[0],values:columns[1],raws:columns[2],lineBreaks:columns[3],
   decision:rules.findJavaScriptRecordParameterStrictDirective(...columns,0)});
 }
 process.stdout.write(structuredDataToLino(rows));
}else{
 try{const surface=parseJavaScript(source),checked=checkProgram(surface),javascript=emitJavaScript(checked).text;
 const lines=[];runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});
 process.stdout.write(structuredDataToLino({surface,javascript,rust:emitRust(checked,{arrayConsumption:true}).text,lines}));
 }catch(error){process.stdout.write(structuredDataToLino({error:{kind:error.kind??null,reason:error.reason??String(error),span:error.span??null}}));}
}
"#;
    let result = run(Command::new("node").current_dir(root()).args([
        "--input-type=module",
        "-e",
        script,
        mode,
        source,
    ]));
    assert!(result.ok, "{}", result.stderr);
    read_structured_data_lino(&result.stdout).unwrap()
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
fn observe_javascript(source: &str, expected: &str) {
    let result = run(Command::new("node").args(["--input-type=module", "-e", source]));
    assert!(result.ok, "{}", result.stderr);
    assert_eq!(result.stdout, expected);
}
fn compile(source: &str, directory: &TemporaryDirectory, name: &str) -> PathBuf {
    let file = directory.0.join(format!("{name}.rs"));
    let binary = directory
        .0
        .join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    fs::write(&file, source).unwrap();
    let mut command = Command::new("rustc");
    command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        command.arg("-C").arg(format!("linker={linker}"));
    }
    let result = run(command.arg(&file).arg("-o").arg(&binary));
    assert!(result.ok, "{}\n{}", result.stdout, result.stderr);
    binary
}
fn observe_native(binary: &Path, expected: &str) {
    let result = run(&mut Command::new(binary));
    assert!(result.ok, "{}", result.stderr);
    assert_eq!(result.stdout, expected);
}
fn require_source(source: &str, expected: &str) {
    let current = javascript_bridge("source", source);
    assert!(current.get("error").is_none(), "{current}");
    let expected_lines = expected.lines().collect::<Vec<_>>();
    assert_eq!(
        current["lines"],
        serde_json::to_value(expected_lines).unwrap(),
        "independent original source"
    );
    let surface = parse_javascript(source).unwrap_or_else(|error| panic!("{source}: {error}"));
    assert_eq!(
        serde_json::to_value(&surface).unwrap(),
        current["surface"],
        "native source transport retains actual owners and spans"
    );
    let program = check_program(&surface).unwrap();
    let rust = emit_rust_with_array_consumption(&program, RustEmissionOptions::default())
        .unwrap()
        .text;
    let javascript = emit_javascript(&program).unwrap().text;
    assert_eq!(rust, current["rust"].as_str().unwrap());
    assert_eq!(javascript, current["javascript"].as_str().unwrap());
    let concrete = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    assert!(
        concrete
            .links()
            .all(|link| !link.metadata().flags().has_error()
                && !link.metadata().flags().is_missing())
    );
    for network in archives(&concrete) {
        assert_eq!(network.reconstruct_text().as_bytes(), source.as_bytes());
    }
    let native_concrete = LinkNetwork::parse(&rust, "Rust", ParseConfiguration::default());
    assert!(
        native_concrete
            .links()
            .all(|link| !link.metadata().flags().has_error()
                && !link.metadata().flags().is_missing())
    );
    for network in archives(&native_concrete) {
        assert_eq!(network.reconstruct_text().as_bytes(), rust.as_bytes());
    }
    let directory = TemporaryDirectory::new("record-parameters");
    let binary = compile(&rust, &directory, "forward");
    observe_javascript(source, expected);
    observe_javascript(&javascript, expected);
    observe_native(&binary, expected);
    let packet = create_typed_program_links(source, "JavaScript").unwrap();
    for network in archives(&packet.network) {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(restored, program);
        let linked_rust = emit_typed_program_links(&network, packet.root, "Rust")
            .unwrap()
            .code;
        let linked_javascript = emit_typed_program_links(&network, packet.root, "JavaScript")
            .unwrap()
            .code;
        assert_eq!(linked_rust, rust);
        assert_eq!(linked_javascript, javascript);
        observe_native(&binary, expected);
        observe_javascript(&linked_javascript, expected);
    }
    // Reverse the complete actual emitted JavaScript, never a definition-only
    // fragment stripped of its required source/runtime context.
    let reverse = create_typed_program_links(&javascript, "JavaScript").unwrap();
    let reverse_rust = emit_typed_program_links(&reverse.network, reverse.root, "Rust")
        .unwrap()
        .code;
    let reverse_binary = if reverse_rust == rust {
        None
    } else {
        Some(compile(&reverse_rust, &directory, "reverse"))
    };
    for network in archives(&reverse.network) {
        assert_eq!(
            emit_typed_program_links(&network, reverse.root, "Rust")
                .unwrap()
                .code,
            reverse_rust
        );
        observe_native(reverse_binary.as_deref().unwrap_or(&binary), expected);
        observe_javascript(
            &emit_typed_program_links(&network, reverse.root, "JavaScript")
                .unwrap()
                .code,
            expected,
        );
    }
}

#[test]
fn original_required_field_sources_keep_complete_execution_and_both_source_free_directions() {
    for (source, expected) in [
        (
            "function selected({left:a,right:b},scale){return (a+b)*scale;}\nfunction repeated({left:a,left:b}){return a===b;}\nfunction mixed(before,{left,right},after){return before+left+right+after;}\nconsole.log(selected({left:2,right:3},4));\nconsole.log(repeated({left:7}));\nconsole.log(mixed(1,{left:2,right:3},4));",
            "20\ntrue\n10\n",
        ),
        (
            "function selected({left:record_parameter_value0,right:b}){return record_parameter_value0+b;}\nconsole.log(selected({left:2,right:3}));",
            "5\n",
        ),
        (
            "// prefix π😀\nconst record_parameter_value0=9;\nfunction unrelated(value){return value;}\nfunction selected(before,{left:a},after){return before+a+after;}console.log(selected(1,{left:2},3));",
            "6\n",
        ),
        (
            "function selected({a}){if(a>0){const a=7;return a;}else{return a;}}console.log(selected({a:1}));console.log(selected({a:-2}));",
            "7\n-2\n",
        ),
        (
            "function selected({left:a,right:b}){return a+b;}\nfunction forward(value){return selected(value);}\nconsole.log(forward({right:3,left:2}));",
            "5\n",
        ),
        (
            "function selected({left:a}){return a===null;}console.log(selected({left:null}));",
            "true\n",
        ),
        (
            "function selected({length}){return length;}console.log(selected(\"A😀\"));",
            "3\n",
        ),
        (
            "function selected({length}){return length;}console.log(selected([1,2,3]));",
            "3\n",
        ),
        (
            "function selected({size}){return size;}console.log(selected(new Map([[\"second\",2],[\"first\",1],[\"second\",3]])));",
            "2\n",
        ),
        (
            "function selected({size}){return size;}console.log(selected(new Set([1,1,2])));",
            "2\n",
        ),
        (
            "function selected({Map,Set,console}){return Map+Set+console;}console.log(selected({Map:1,Set:2,console:3}));",
            "6\n",
        ),
        (
            "function selected({left:a}){for(;;){return a;}}console.log(selected({left:2}));",
            "2\n",
        ),
        (
            "// original source π\nfunction f({left:a,right:b}) { return a+b; }\nconsole.log(f({left:2,right:3}));\n",
            "5\n",
        ),
        (
            "function f(value=3){return value;}console.log(f());console.log(f(7));",
            "3\n7\n",
        ),
        (
            "function f(value=({literal:1})){return value.literal;}console.log(f());console.log(f({literal:7}));",
            "1\n7\n",
        ),
    ] {
        require_source(source, expected);
    }
}

#[test]
fn actual_source_policy_captures_and_scalar_loop_exclusion_keep_their_original_owners() {
    versions();
    let policy = policy();
    for owner in policy["sourceClosure"]["inputs"]
        .as_array()
        .unwrap()
        .iter()
        .chain(policy["originalDeclarations"].as_array().unwrap())
    {
        let source = fs::read(root().join(owner["path"].as_str().unwrap())).unwrap();
        assert_eq!(source.len() as u64, owner["bytes"].as_u64().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(&source)),
            owner["sourceSha256"].as_str().unwrap()
        );
        if let Some(declaration) = owner["declaration"].as_str() {
            assert!(String::from_utf8(source).unwrap().contains(declaration));
            let concrete =
                LinkNetwork::parse(declaration, "JavaScript", ParseConfiguration::default());
            for network in archives(&concrete) {
                assert_eq!(network.reconstruct_text(), declaration);
            }
        }
    }
    let source = "function selected({left:a}){for(;;){return a;}}console.log(selected({left:2}));";
    let surface =
        parse_javascript_with_scalar_loop_model(source, "closed primitive scalar loop 1").unwrap();
    let parsed = serde_json::to_value(surface).unwrap();
    assert!(parsed["items"][0].get("sourceLoop").is_none());
    let source = "// prefix π😀\nconst record_parameter_value0=9;\nfunction selected(before,{left:a},after){return before+a+after;}console.log(selected(1,{left:2},3));";
    let context = ModuleContext {
        externals: vec![External::Constant {
            name: "record_parameter_value1".to_owned(),
            ty: FLOAT,
            literal: false,
        }],
        module_directory: None,
    };
    let native = serde_json::to_value(parse_javascript_in(source, &context).unwrap()).unwrap();
    assert_eq!(native, javascript_bridge("context", source)["surface"]);
    let selected = native["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["name"] == "selected")
        .unwrap();
    assert_eq!(
        selected["params"][1]["name"], "record_parameter_value2",
        "context is installed before the first selected hygiene inventory"
    );
}

#[test]
fn generated_decisions_keep_every_truncated_actual_column_and_binding_profile_control() {
    use generated_decisions::{self as rules, ml_string::Text};
    let policy = policy();
    let reserved = policy["sourceClosure"]["inputs"].as_array().unwrap();
    assert_eq!(reserved.len(), 3);
    let binding_policy =
        read_structured_data_lino(include_str!("../../src/data/array-bindings.lino")).unwrap();
    let reserved = binding_policy["reservedBindings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| Text::from_text(name.as_str().unwrap()))
        .collect::<Vec<_>>();
    for source in [
        "before,{left:a,right:b,left:c,},after)",
        "{left,right:alias},tail)",
        "{left},)",
        "{a=1})",
        "{a}={})",
        "before=effect(),{a})",
        "{a},after=effect())",
        "[a],{b})",
        "{a:[b]})",
        "{a:{b}})",
        "{...rest})",
        "{[key]:a})",
        "{\"a\":a})",
        "{})",
        "{a},a)",
        "{a:x,b:x})",
        "{a:class})",
        "{a},...rest)",
        "{a},,b)",
        "expression=/{a,b}/,after)",
        "expression=/[,{}()]/,after)",
        "text=\"{a,b}\",after)",
        "text=`{a,b}`,after)",
        "value=call({a:1,b:2}),after)",
        "a=({literal:1}),b)",
        "value,other)",
        "callback=()=>({a:1}))",
    ] {
        let rows = javascript_bridge("plans", source);
        for row in rows.as_array().unwrap() {
            let texts = |key: &str| {
                row[key]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| Text::from_text(value.as_str().unwrap()))
                    .collect::<Vec<_>>()
            };
            let kinds = texts("kinds");
            let values = texts("values");
            let plan = rules::plan_java_script_record_parameters(&kinds, &values, 0.0, &reserved);
            let expected = row["plan"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap().to_bits())
                .collect::<Vec<_>>();
            assert_eq!(
                plan.iter().map(|value| value.to_bits()).collect::<Vec<_>>(),
                expected,
                "{source}"
            );
            assert_eq!(
                rules::find_java_script_record_parameter_pattern(&kinds, &values, 0.0).to_bits(),
                row["found"].as_f64().unwrap().to_bits(),
                "{source}"
            );
            for start in [-1.0, f64::NAN, f64::INFINITY, 0.5] {
                let invalid =
                    rules::plan_java_script_record_parameters(&kinds, &values, start, &reserved);
                assert!(invalid[0] < 0.0);
                assert!(
                    rules::find_java_script_record_parameter_pattern(&kinds, &values, start) < 0.0
                );
            }
        }
    }
    for name in ["class", "arguments", "eval", "ml_hidden", "same"] {
        assert!(!rules::accept_java_script_record_parameter_binding(
            &Text::from_text("identifier"),
            &Text::from_text(name),
            &[Text::from_text("same")],
            &reserved
        ));
    }
    assert!(rules::accept_java_script_record_parameter_profile(
        &Text::from_text("ordinary named function declaration"),
        false,
        0.0,
        false,
        false
    ));
    for (context, asynchronous, documentation, writes, variables) in [
        ("closure", false, 0.0, false, false),
        (
            "ordinary named function declaration",
            true,
            0.0,
            false,
            false,
        ),
        (
            "ordinary named function declaration",
            false,
            1.0,
            false,
            false,
        ),
        (
            "ordinary named function declaration",
            false,
            0.0,
            true,
            false,
        ),
        (
            "ordinary named function declaration",
            false,
            0.0,
            false,
            true,
        ),
    ] {
        assert!(!rules::accept_java_script_record_parameter_profile(
            &Text::from_text(context),
            asynchronous,
            documentation,
            writes,
            variables
        ));
    }
}

#[test]
fn generated_standard_owners_and_actual_directive_columns_keep_their_independent_boundaries() {
    use generated_decisions::{self as rules, ml_string::Text};
    for name in ["Map", "Set", "console"] {
        assert!(rules::accept_java_script_standard_builtin_owner(
            &Text::from_text(name),
            false
        ));
        assert!(!rules::accept_java_script_standard_builtin_owner(
            &Text::from_text(name),
            true
        ));
    }
    assert!(!rules::accept_java_script_standard_builtin_owner(
        &Text::from_text("Unknown"),
        false
    ));
    for (source, expected) in [
        ("\"use strict\";}", 0.0),
        ("'ordinary';'use strict';}", 2.0),
        ("'ordinary'\n'use strict'\nreturn a;}", 1.0),
        ("\"use strict\"/*\n*/return a;}", 0.0),
        ("\"use strict\"\rreturn a;}", 0.0),
        ("\"use strict\"\u{2028}return a;}", 0.0),
        ("\"use strict\"\u{2029}return a;}", 0.0),
        ("\"use\\x20strict\";return a;}", -1.0),
        ("(\"use strict\");return a;}", -1.0),
        ("\"use strict\"+\"\";return a;}", -1.0),
        ("\"use strict\"\n+\"\";return a;}", -1.0),
        ("\"use strict\".length;return a;}", -1.0),
        (";\"use strict\";return a;}", -1.0),
        ("\"ordinary\"\n+1;\"use strict\";return a;}", -1.0),
        ("\"use strict\"", -2.0),
        ("\"ordinary\";", -2.0),
    ] {
        let rows = javascript_bridge("directives", source);
        let rows = rows.as_array().unwrap();
        assert_eq!(
            rows.last().unwrap()["decision"].as_f64().unwrap().to_bits(),
            f64::to_bits(expected),
            "{source}"
        );
        for row in rows {
            let texts = |key: &str| {
                row[key]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| Text::from_text(value.as_str().unwrap()))
                    .collect::<Vec<_>>()
            };
            let kinds = texts("kinds");
            let values = texts("values");
            let raws = texts("raws");
            let lines = row["lineBreaks"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_bool().unwrap())
                .collect::<Vec<_>>();
            let decision = rules::find_java_script_record_parameter_strict_directive(
                &kinds, &values, &raws, &lines, 0.0,
            );
            assert_eq!(
                decision.to_bits(),
                row["decision"].as_f64().unwrap().to_bits(),
                "{source}: {row}"
            );
            for start in [-1.0, f64::NAN, f64::INFINITY, 0.5] {
                assert_eq!(
                    rules::find_java_script_record_parameter_strict_directive(
                        &kinds, &values, &raws, &lines, start
                    )
                    .to_bits(),
                    (-2.0f64).to_bits()
                );
            }
            if !kinds.is_empty() {
                for (kinds, values, raws, lines) in [
                    (
                        &kinds[1..],
                        values.as_slice(),
                        raws.as_slice(),
                        lines.as_slice(),
                    ),
                    (
                        kinds.as_slice(),
                        &values[1..],
                        raws.as_slice(),
                        lines.as_slice(),
                    ),
                    (
                        kinds.as_slice(),
                        values.as_slice(),
                        &raws[1..],
                        lines.as_slice(),
                    ),
                    (
                        kinds.as_slice(),
                        values.as_slice(),
                        raws.as_slice(),
                        &lines[1..],
                    ),
                ] {
                    assert_eq!(
                        rules::find_java_script_record_parameter_strict_directive(
                            kinds, values, raws, lines, 0.0
                        )
                        .to_bits(),
                        (-2.0f64).to_bits()
                    );
                }
            }
        }
    }
}

#[test]
fn actual_parameter_sources_keep_every_unsupported_profile_scope_and_call_boundary() {
    for source in [
        "function f({a}){a=2;return a;}",
        "function f({a}){var a=2;return a;}",
        "function f({a}){const change=()=>{a=2;return a;};return change();}",
        "function f({a},other){other=2;return a+other;}",
        "function f({a}){const a=2;return a;}",
        "function f({a}){let a=2;return a;}",
        "const f=({a})=>a;",
        "const f=function({a}){return a;};",
        "const Namespace={f({a}){return a;}};",
        "async function f({a}){return a;}",
        "/** @param {{a:number}} options */function f({a}){return a;}",
        "function ordinary(value=/{a,b}/){return 1;}",
        "function ordinary(value=/[,{}()]/){return 1;}",
    ] {
        assert!(
            parse_javascript(source)
                .and_then(|surface| check_program(&surface))
                .is_err(),
            "{source}"
        );
        assert!(
            javascript_bridge("source", source).get("error").is_some(),
            "{source}"
        );
    }
    for argument in [
        "null",
        "undefined",
        "",
        "{}",
        "{right:3}",
        "3",
        "true",
        "\"text\"",
        "[1,2]",
        "external()",
        "{get left(){return 2;}}",
        "Object.create({left:2})",
        "new Proxy({left:2},{})",
    ] {
        for body in ["return a;", "return 1;"] {
            let source = format!(
                "function selected({{left:a}}){{{body}}}console.log(selected({argument}));"
            );
            assert!(
                create_typed_program_links(&source, "JavaScript").is_err(),
                "{source}"
            );
            assert!(
                javascript_bridge("source", &source).get("error").is_some(),
                "{source}"
            );
        }
    }
}

#[test]
fn actual_shadowed_builtin_owners_and_non_simple_strict_directives_cannot_be_silently_normalized() {
    for source in [
        "function f({Map}) { return new Map([[1,2]]).size; }\nconsole.log(f({Map:0}));",
        "function g({Set}) { return new Set([1,2]).size; }\nconsole.log(g({Set:0}));",
        "function h({console}) { console.log(7); return 1; }\nh({console:0});",
        "function f(Map){return new Map([[1,2]]).size;}console.log(f(0));",
        "function f(console){console.log(7);return 1;}f(0);",
        "function f(){return new Map([[1,2]]).size;}const Map=0;console.log(f());",
        "function f(){console.log(7);return 1;}const console=0;f();",
        "function f({Map}){const g=()=>new Map([[1,2]]);return g().size;}console.log(f({Map:0}));",
    ] {
        assert_eq!(
            javascript_bridge("raw-error", source)["name"],
            "TypeError",
            "independent unchanged source"
        );
        let error = parse_javascript(source).unwrap_err();
        assert!(
            error.construct.as_deref().is_some_and(|construct| construct
                == "shadowed standard collection owner"
                || construct == "shadowed standard console owner"),
            "{source}: {error}"
        );
        assert!(javascript_bridge("source", source).get("error").is_some());
    }
    for body in [
        "\"use strict\";return a;",
        "'ordinary';'use strict';return a;",
        "'ordinary'\n'use strict'\nreturn a;",
        "\"use strict\"/*\n*/return a;",
    ] {
        let source = format!("function f({{a}}){{{body}}}console.log(f({{a:2}}));");
        assert_eq!(
            javascript_bridge("raw-error", &source)["name"],
            "SyntaxError"
        );
        assert_eq!(
            parse_javascript(&source).unwrap_err().construct.as_deref(),
            Some("record parameter strict directive")
        );
        assert!(javascript_bridge("source", &source).get("error").is_some());
    }
    for body in [
        "(\"use strict\");return a;",
        "\"use\\x20strict\";return a;",
        "\"use strict\"+\"\";return a;",
        "\"use strict\"\n+\"\";return a;",
        ";\"use strict\";return a;",
        "\"use strict\".length;return a;",
    ] {
        require_source(
            &format!("function f({{a}}){{{body}}}console.log(f({{a:2}}));"),
            "2\n",
        );
    }
    require_source(
        "function ordinary(a){\"use strict\";return a;}console.log(ordinary(2));",
        "2\n",
    );
    let dead_zone = "function f({a}){console.log(a);let console=0;return a;}console.log(f({a:2}));";
    assert_eq!(
        parse_javascript(dead_zone)
            .unwrap_err()
            .construct
            .as_deref(),
        Some("shadowed standard console owner")
    );
    for name in ["Map", "Set", "console"] {
        let source = if name == "console" {
            "console.log(1);".to_owned()
        } else {
            format!("console.log(new {name}().size);")
        };
        let context = ModuleContext {
            externals: vec![External::Constant {
                name: name.to_owned(),
                ty: FLOAT,
                literal: false,
            }],
            module_directory: None,
        };
        assert!(
            parse_javascript_in(&source, &context)
                .unwrap_err()
                .construct
                .as_deref()
                .is_some_and(|construct| construct.starts_with("shadowed standard"))
        );
    }
}
