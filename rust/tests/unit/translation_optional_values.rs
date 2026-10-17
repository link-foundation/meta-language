//! Actual modern JavaScript optional values retain runtime effects and types.
use meta_language::TranslationSupport;
use meta_language::translate_program;
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::ir::{Decl, Node};
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::types::Type;
use std::process::Command;

fn observe(source: &str) -> std::process::Output {
    Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .expect("Node.js observes the complete actual module")
}

fn check_runtime(source: &str, expected: &[u8]) {
    let original = observe(source);
    assert!(
        original.status.success(),
        "{}",
        String::from_utf8_lossy(&original.stderr)
    );
    assert_eq!(original.stdout, expected);
    let checked = check_program(&parse_javascript(source).expect("the actual source parses"))
        .expect("the actual source checks");
    let emitted = emit_javascript(&checked).expect("the actual checked module emits");
    let translated = observe(&emitted.text);
    assert_eq!(translated.status.code(), original.status.code());
    assert_eq!(translated.stdout, original.stdout);
    assert_eq!(translated.stderr, original.stderr);
    for target in ["Rust", "Lean", "Rocq"] {
        let translated = translate_program(source, "JavaScript", target)
            .expect("an actual target descriptor exists");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
        assert!(!translated.code().is_empty(), "{target}");
    }
}

#[test]
fn nullish_fallbacks_keep_falsy_values_and_only_run_absent_branch_effects() {
    let source = "function fallback(){console.log('fallback');return 7;}function choose(value){return value??fallback();}console.log(choose(0));console.log(choose(null));console.log(choose(undefined));";
    check_runtime(source, b"0\nfallback\n7\nfallback\n7\n");
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let Some(Decl::Fn(function)) = program.declaration("choose") else {
        panic!("the actual optional function is retained");
    };
    assert_eq!(
        function.params[0].ty,
        Type::Option {
            element: Box::new(Type::Float),
            absence: "both".to_owned()
        }
    );
    assert_eq!(function.ret, Type::Float);
    assert!(matches!(function.body.node, Node::Coalesce { .. }));
}

#[test]
fn optional_fields_numeric_coercions_truth_and_type_queries_follow_actual_javascript() {
    check_runtime(
        "function get(item){return item?.value??7;}console.log(get({value:0}));console.log(get(null));console.log(get(undefined));",
        b"0\n7\n7\n",
    );
    let source = "function inspect(value){console.log(`${value===null} ${value===undefined} ${!!value} ${typeof value} ${value+1}`);return value||5;}console.log(inspect(null));console.log(inspect(undefined));console.log(inspect(0));console.log(inspect(2));";
    check_runtime(source, b"true false false object 1\n5\nfalse true false undefined NaN\n5\nfalse false false number 1\n5\nfalse false true number 3\n2\n");
    for (source, kind) in [
        (
            "/** @param {{item:number}|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect({item:2}));",
            "record",
        ),
        (
            "/** @param {[number,string]|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect([2,'kept']));",
            "tuple",
        ),
        (
            "/** @param {Map<string,number>|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect(new Map([['kept',2]])));",
            "map",
        ),
        (
            "/** @param {Set<number>|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect(new Set([2])));",
            "set",
        ),
    ] {
        check_runtime(source, b"object undefined object\n");
        let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
        let Some(Decl::Fn(function)) = checked.declaration("inspect") else {
            panic!("the declared optional observer remains owned")
        };
        let Type::Option { element, absence } = &function.params[0].ty else {
            panic!("the original absence alternatives remain represented")
        };
        assert_eq!(element.kind(), kind);
        assert_eq!(absence, "both");
        assert!(
            matches!(&function.body.node, Node::OptionalTypeOf { present, .. } if present == "object")
        );
    }
    check_runtime(
        "function read(value){console.log('read');const n=value?.item??0;return value;}function describe(value){return typeof read(value);}console.log('%s',describe(null));console.log('%s',describe(undefined));console.log('%s',describe({item:2}));",
        b"read\nobject\nread\nundefined\nread\nobject\n",
    );
}

#[test]
fn optional_tuple_context_keeps_arity_spread_heterogeneous_array_and_unknown_type_refusals() {
    let optional = "/** @param {[number,number]|null|undefined} pair @returns {string} */function describe(pair){return typeof pair;}";
    check_runtime(
        &format!("{optional}console.log('%s',describe([2,3]),describe(null),describe(undefined));"),
        b"object object undefined\n",
    );
    let wrong_arity =
        parse_javascript(&format!("{optional}console.log('%s',describe([2]));")).unwrap_err();
    assert!(wrong_arity.message().contains("two different lengths"));
    let spread = parse_javascript(&format!(
        "{optional}console.log('%s',describe([...[2,3]]));"
    ))
    .unwrap_err();
    assert_eq!(
        spread.kind,
        meta_language::translation::diagnostics::ErrorKind::Type
    );
    assert!(spread.message().contains("array") && spread.message().contains("tuple"));
    let array = optional.replace("[number,number]", "number[]");
    let mixed =
        parse_javascript(&format!("{array}console.log('%s',describe([2,'kept']));")).unwrap_err();
    assert_eq!(
        mixed.kind,
        meta_language::translation::diagnostics::ErrorKind::Type
    );
    assert!(mixed.message().contains("number") && mixed.message().contains("string"));
    let unresolved: meta_language::translation::surface::SProgram = serde_json::from_value(serde_json::json!({
        "language":"JavaScript", "items":[{"k":"fn", "name":"unknownType",
        "params":[{"name":"value", "type":{"kind":"option", "element":{"kind":"union", "alternatives":[{"kind":"float"},{"kind":"string"}]}, "absence":"both"}}],
        "ret":{"kind":"string"}, "body":{"k":"typeOf", "arg":{"k":"name", "path":["value"]}}}], "main":null,
    })).unwrap();
    let expected = Type::Option {
        element: Box::new(Type::Union {
            alternatives: vec![Type::Float, Type::String],
        }),
        absence: "both".to_owned(),
    };
    let error = check_program(&unresolved).unwrap_err();
    assert_eq!(
        error.kind,
        meta_language::translation::diagnostics::ErrorKind::Unsupported
    );
    assert_eq!(error.construct, Some(format!("typeof {}", expected.key())));
    assert!(
        error
            .message()
            .contains("type queries require a known JavaScript value type")
    );
}

#[test]
fn structural_optional_type_queries_round_trip_and_execute_as_one_complete_rust_program() {
    use super::issue_195_translation_pairs::{TemporaryDirectory, run};
    use meta_language::translation::emit_rust::emit_rust;
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
        encode_link_network_binary, encode_typed_program_links,
    };
    let source = "/** @param {{item:number}|null|undefined} value @returns {string} */function recordType(value){return typeof value;}\n/** @param {[number,string]|null|undefined} value @returns {string} */function tupleType(value){return typeof value;}\n/** @param {Map<string,number>|null|undefined} value @returns {string} */function mapType(value){return typeof value;}\n/** @param {Set<number>|null|undefined} value @returns {string} */function setType(value){return typeof value;}\nconsole.log('%s',recordType(null),recordType(undefined),recordType({item:2}));\nconsole.log('%s',tupleType(null),tupleType(undefined),tupleType([2,'kept']));\nconsole.log('%s',mapType(null),mapType(undefined),mapType(new Map([['kept',2]])));\nconsole.log('%s',setType(null),setType(undefined),setType(new Set([2])));\nfunction read(value){console.log('read');const n=value?.item??0;return value;}function describe(value){return typeof read(value);}console.log('%s',describe(null));console.log('%s',describe(undefined));console.log('%s',describe({item:2}));";
    let expected: &[u8] = b"object undefined object\nobject undefined object\nobject undefined object\nobject undefined object\nread\nobject\nread\nundefined\nread\nobject\n";
    let original = observe(source);
    assert!(
        original.status.success(),
        "{}",
        String::from_utf8_lossy(&original.stderr)
    );
    assert_eq!(original.stdout, expected);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let javascript = emit_javascript(&program).unwrap().text;
    let observed = observe(&javascript);
    assert_eq!(observed.status.code(), original.status.code());
    assert_eq!(observed.stdout, original.stdout);
    assert_eq!(observed.stderr, original.stderr);
    let rust = emit_rust(&program).unwrap().text;
    let archive = encode_typed_program_links(&program).unwrap();
    for network in [
        LinkNetwork::from_lino(&archive.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&archive.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, archive.root).unwrap();
        assert_eq!(restored, program);
        assert_eq!(emit_javascript(&restored).unwrap().text, javascript);
        assert_eq!(emit_rust(&restored).unwrap().text, rust);
    }
    let directory = TemporaryDirectory::new("structural_optional_type_queries");
    let file = directory.0.join("program.rs");
    let executable = directory
        .0
        .join(format!("program{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&file, rust).unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg(&file)
        .arg("-o")
        .arg(&executable));
    assert!(compiled.ok, "{}", compiled.stderr);
    let result = run(&mut Command::new(&executable));
    assert!(result.ok, "{}", result.stderr);
    assert_eq!(result.stdout.as_bytes(), original.stdout);
    assert_eq!(result.stderr.as_bytes(), original.stderr);
}

fn optional_equality_contracts() -> serde_json::Value {
    use super::issue_195_translation_pairs::run;
    let script = r#"
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from './js/src/typed-program-links.js';
import {recheckLinkedTypedProgram} from './js/src/typed-program-checked-projection.js';
import {LinkNetwork} from './js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from './js/src/link-network-binary.js';
const source=readFileSync('./js/tests/typed-program-strict-equality-projection.test.js','utf8');
function literal(name){
 const marker='const '+name+' = [';
 assert.equal(source.split(marker).length,2,'the actual JavaScript control has one literal '+name);
 const start=source.indexOf(marker),boundary=name==='cases'?'\n  ];':'\n  for (const declarations of [readers';
 const closing=source.indexOf(boundary,start);
 assert.ok(closing>start,'the complete original declaration is retained');
 const end=closing+(name==='cases'?boundary.length:0);
 return vm.runInNewContext(source.slice(start,end)+'\n'+name+';');
}
const cases=literal('cases'),readers=literal('readers');
assert.equal(cases.length,6);assert.equal(readers.length,2);
for(const [index,declarations]of [readers,[...readers].reverse()].entries())cases.push(['deferred-'+index,
 '/** @param {number[]} numbers @param {string[]} texts @returns {boolean} */function compare(numbers,texts){return readNumber(numbers)===readText(texts);}'
 +declarations.join('\n')+'console.log(compare([],[]));console.log(compare([0],[\'0\']));',
 ['left','right','true','left','right','false']]);
function observe(source){const lines=[];vm.runInNewContext(source,{console:{log:value=>lines.push(String(value))}},{timeout:1000});return lines;}
const contracts=[];
for(const [name,source,expected]of cases){
 assert.deepEqual(observe(source),Array.from(expected));
 const surface=parseJavaScript(source),program=checkProgram(surface),javascript=emitJavaScript(program).text,rust=emitRust(program,{arrayConsumption:true}).text;
 assert.deepEqual(observe(javascript),Array.from(expected));
 const packet=encodeTypedProgramLinks(program);
 for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root);recheckLinkedTypedProgram(restored,{wholeProgram:true});
  assert.equal(emitJavaScript(restored).text,javascript);assert.equal(emitRust(restored,{arrayConsumption:true}).text,rust);
 }
 const changed=source.includes('!==')?source.replace('!==','==='):source.replace('===','!==');
 const edited=checkProgram(parseJavaScript(changed)),editedExpected=Array.from(expected,line=>line==='true'?'false':line==='false'?'true':line);
 assert.deepEqual(observe(changed),editedExpected);assert.deepEqual(observe(emitJavaScript(edited).text),editedExpected);
 contracts.push({name,source,expected:Array.from(expected),surface,javascript,rust,links:packet.network.toLino(),root:packet.root.asU64(),
  editedExpected,editedJavaScript:emitJavaScript(edited).text,editedRust:emitRust(edited,{arrayConsumption:true}).text});
}
console.log(JSON.stringify(contracts));
"#;
    let output = run(Command::new("node")
        .current_dir(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap(),
        )
        .args(["--input-type=module", "-e", script]));
    assert!(output.ok, "{}", output.stderr);
    serde_json::from_str(&output.stdout).unwrap()
}

#[test]
fn optional_strict_equality_keeps_primitive_domains_deferred_constraints_and_both_archives() {
    use super::issue_195_translation_pairs::{TemporaryDirectory, run};
    use meta_language::translation::emit_rust::{
        RustEmissionOptions, emit_rust_with_array_consumption,
    };
    use meta_language::{
        BinaryLinksOptions, LinkId, LinkNetwork, decode_link_network_binary,
        decode_typed_program_links, encode_link_network_binary, encode_typed_program_links,
    };
    let contracts = optional_equality_contracts();
    assert_eq!(contracts.as_array().unwrap().len(), 8);
    for contract in contracts.as_array().unwrap() {
        let name = contract["name"].as_str().unwrap();
        let source = contract["source"].as_str().unwrap();
        let surface = parse_javascript(source).unwrap();
        let program = check_program(&surface).unwrap();
        let javascript = emit_javascript(&program).unwrap().text;
        let rust = emit_rust_with_array_consumption(&program, RustEmissionOptions::default())
            .unwrap()
            .text;
        let expected = contract["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        assert_eq!(
            serde_json::to_value(surface).unwrap(),
            contract["surface"],
            "{name}: independently parsed JavaScript source"
        );
        assert_eq!(
            javascript,
            contract["javascript"].as_str().unwrap(),
            "{name}: JavaScript bytes"
        );
        assert_eq!(
            rust,
            contract["rust"].as_str().unwrap(),
            "{name}: Rust bytes"
        );
        let observed = observe(&javascript);
        assert!(
            observed.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&observed.stderr)
        );
        assert_eq!(
            observed.stdout,
            expected.as_bytes(),
            "{name}: actual native-checker JavaScript observations"
        );
        let authored = LinkNetwork::from_lino(contract["links"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(contract["root"].as_u64().unwrap());
        let from_javascript = decode_typed_program_links(&authored, root).unwrap();
        assert_eq!(
            from_javascript, program,
            "{name}: actual JavaScript-authored checked graph"
        );
        let packet = encode_typed_program_links(&program).unwrap();
        for (network, root) in [
            (authored, root),
            (
                LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
                packet.root,
            ),
            (
                decode_link_network_binary(
                    &encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                        .unwrap(),
                    BinaryLinksOptions::default(),
                )
                .unwrap(),
                packet.root,
            ),
        ] {
            let mut restored = decode_typed_program_links(&network, root).unwrap();
            assert_eq!(restored, program, "{name}: complete source-free graph");
            assert_eq!(emit_javascript(&restored).unwrap().text, javascript);
            assert_eq!(
                emit_rust_with_array_consumption(&restored, RustEmissionOptions::default())
                    .unwrap()
                    .text,
                rust
            );
            let Some(Decl::Fn(compare)) = restored.declarations.iter_mut().find(
                |declaration| matches!(declaration, Decl::Fn(function) if function.name=="compare"),
            ) else {
                panic!("the real comparison declaration remains present")
            };
            let Node::StrictEqual { negate, .. } = &mut compare.body.node else {
                panic!("the actual optional comparison retains strict semantics")
            };
            *negate = !*negate;
            let edited = encode_typed_program_links(&restored).unwrap();
            for saved in [
                LinkNetwork::from_lino(&edited.network.to_lino()).unwrap(),
                decode_link_network_binary(
                    &encode_link_network_binary(&edited.network, BinaryLinksOptions::default())
                        .unwrap(),
                    BinaryLinksOptions::default(),
                )
                .unwrap(),
            ] {
                let edited = decode_typed_program_links(&saved, edited.root).unwrap();
                let emitted = emit_javascript(&edited).unwrap().text;
                assert_eq!(
                    emitted,
                    contract["editedJavaScript"].as_str().unwrap(),
                    "{name}: independently rechecked source edit"
                );
                assert_eq!(
                    emit_rust_with_array_consumption(&edited, RustEmissionOptions::default())
                        .unwrap()
                        .text,
                    contract["editedRust"].as_str().unwrap()
                );
                let output = observe(&emitted);
                assert!(output.status.success());
                let expected = contract["editedExpected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap())
                    .collect::<Vec<_>>()
                    .join("\n")
                    + "\n";
                assert_eq!(output.stdout, expected.as_bytes());
            }
        }
        let directory = TemporaryDirectory::new(name);
        let file = directory.0.join("program.rs");
        let executable = directory
            .0
            .join(format!("program{}", std::env::consts::EXE_SUFFIX));
        std::fs::write(&file, rust).unwrap();
        let compiled = run(Command::new("rustc")
            .arg("--edition=2024")
            .arg(&file)
            .arg("-o")
            .arg(&executable));
        assert!(compiled.ok, "{name}: {}", compiled.stderr);
        let output = run(&mut Command::new(&executable));
        assert!(output.ok, "{name}: {}", output.stderr);
        assert_eq!(
            output.stdout, expected,
            "{name}: actual emitted Rust execution in CI"
        );
    }
    let unknown = check_program(
        &parse_javascript("function compare(left,right){return left===right;}").unwrap(),
    )
    .unwrap();
    let Some(Decl::Fn(compare)) = unknown.declaration("compare") else {
        panic!("the actual unknown function remains present")
    };
    assert!(
        compare
            .params
            .iter()
            .all(|parameter| parameter.ty == Type::Float)
    );
    let optional = check_program(&parse_javascript("/** @param {number[]} values @returns {number|undefined} */function read(values){return values[0];}function compare(value){return value===read([1]);}").unwrap()).unwrap();
    let Some(Decl::Fn(compare)) = optional.declaration("compare") else {
        panic!("the actual optional constraint owner remains present")
    };
    assert!(
        matches!(&compare.params[0].ty, Type::Option{element,absence} if element.as_ref()==&Type::Float&&absence=="undefined")
    );
    for source in [
        "/** @param {number} left @param {string} right @returns {boolean} */function compare(left,right){return left===right;}console.log(compare(0,'0'));",
        "/** @param {number[]} left @param {string[]} right @returns {boolean} */function compare(left,right){return left[0]<right[0];}console.log(compare([0],['0']));",
        "function maybe(present){if(present)return [1];}function compare(present){return maybe(present)===maybe(present);}console.log(compare(false));",
    ] {
        assert!(
            parse_javascript(source)
                .and_then(|surface| check_program(&surface))
                .is_err(),
            "{source}: existing nonoptional, relational and reference identity boundaries remain enforced"
        );
    }
}

#[test]
fn strict_equality_raw_flags_are_mandatory_booleans_in_both_graph_archives() {
    use meta_language::{
        BinaryLinksOptions, LinkMetadata, LinkNetwork, LinkType, SubstitutionRule,
        decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
        encode_link_network_binary, encode_typed_program_links, typed_program_links_schema,
    };
    let source = "/** @param {number[]} left @param {number[]} right @returns {boolean} */function compare(left,right){return left[0]===right[0];}console.log(compare([],[]));console.log(compare([1],[]));";
    let original = observe(source);
    assert!(original.status.success());
    assert_eq!(original.stdout, b"true\nfalse\n");
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let packet = encode_typed_program_links(&program).unwrap();
    let schema = typed_program_links_schema();
    let construct_label = schema["fields"]["k"].as_str().unwrap_or("k");
    let owner = packet
        .network
        .links()
        .find_map(|link| match link.references() {
            [owner, label, value]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && packet
                        .network
                        .link(*label)
                        .and_then(|link| link.metadata().term())
                        == Some(construct_label)
                    && packet
                        .network
                        .link(*value)
                        .and_then(|link| link.metadata().definition())
                        == Some("strictEqual") =>
            {
                Some(*owner)
            }
            _ => None,
        })
        .unwrap();
    let negate_label = schema["fields"]["negate"].as_str().unwrap_or("negate");
    let field = packet
        .network
        .links()
        .find_map(|link| match link.references() {
            [parent, label, value]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && *parent == owner
                    && packet
                        .network
                        .link(*label)
                        .and_then(|link| link.metadata().term())
                        == Some(negate_label) =>
            {
                Some([*parent, *label, *value])
            }
            _ => None,
        })
        .unwrap();
    for (kind, definition, expected) in [
        ("boolean", Some("false"), Some(b"true\nfalse\n".as_slice())),
        ("boolean", Some("true"), Some(b"false\ntrue\n".as_slice())),
        ("missing", None, None),
        ("null", None, None),
        ("string", Some("false"), None),
        ("string", Some("true"), None),
    ] {
        let mut network = packet.network.clone();
        if kind == "missing" {
            let result = network.apply_substitution(&SubstitutionRule::delete(field));
            assert_eq!(
                result.deleted().len(),
                1,
                "only the actual negate relation is removed"
            );
        } else {
            let mut metadata = LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_language(schema["language"].as_str().unwrap())
                .with_term(schema["scalars"][kind].as_str().unwrap());
            if let Some(definition) = definition {
                metadata = metadata.with_definition(definition);
            }
            let scalar = network.insert_link([], metadata);
            let result = network
                .apply_substitution(&SubstitutionRule::new(field, [field[0], field[1], scalar]));
            assert_eq!(
                result.updated().len(),
                1,
                "only the actual negate relation is retargeted"
            );
        }
        for saved in [
            LinkNetwork::from_lino(&network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&network, BinaryLinksOptions::default()).unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            if let Some(expected) = expected {
                let restored = decode_typed_program_links(&saved, packet.root).unwrap();
                let Some(Decl::Fn(compare)) = restored.declaration("compare") else {
                    panic!("the actual comparison remains present")
                };
                assert!(
                    matches!(&compare.body.node, Node::StrictEqual{negate,..} if *negate==(definition==Some("true")))
                );
                let rechecked = encode_typed_program_links(&restored).unwrap();
                assert_eq!(
                    decode_typed_program_links(&rechecked.network, rechecked.root).unwrap(),
                    restored
                );
                let emitted = emit_typed_program_links(&saved, packet.root, "JavaScript").unwrap();
                let observed = observe(&emitted.code);
                assert!(observed.status.success());
                assert_eq!(observed.stdout, expected);
                assert!(emit_typed_program_links(&saved, packet.root, "Rust").is_ok());
            } else {
                assert!(
                    decode_typed_program_links(&saved, packet.root).is_err(),
                    "{kind}/{definition:?}: malformed flags are refused before whole-program recheck"
                );
                for target in ["JavaScript", "Rust"] {
                    assert!(
                        emit_typed_program_links(&saved, packet.root, target).is_err(),
                        "{kind}/{definition:?}/{target}"
                    );
                }
            }
        }
    }
}
