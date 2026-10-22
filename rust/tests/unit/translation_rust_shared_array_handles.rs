//! Real complete source, both independent emitters and codecs; execute only in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::self_translation::{
    SelfTranslationOptions, self_translate, self_translate_with, self_translation_signatures,
};
use meta_language::translation::{
    check::check_program,
    emit_javascript::emit_javascript,
    emit_rust::emit_rust,
    rust::{parse_rust, parse_rust_with_shared_array_handle_source_model},
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
use serde_json::Value;
use std::{fs, path::Path, process::Command};
const MODEL: &str = "standard Rust shared array identity handles 1";
fn versions() {
    let node = run(Command::new("node").args(["-p", "process.versions.node"]));
    assert!(node.ok, "{}", node.stderr);
    let parts = node
        .stdout
        .trim()
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 22 && parts[1] >= 15 || parts[0] == 24,
        "uncovered Node {}; supported >=22.15 <23 or >=24 <25",
        node.stdout.trim()
    );
    let rust = run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let version = rust.stdout.split_whitespace().nth(1).unwrap();
    let parts = version
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {version}; supported >=1.90 <1.100"
    );
}
fn observe(source: &str) -> String {
    let result = run(Command::new("node").args(["--input-type=module", "-e", source]));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
}
fn compiled(source: &str) -> String {
    let directory = TemporaryDirectory::new("rust-shared-array-handles");
    let input = directory.0.join("handles.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "handles.exe"
    } else {
        "handles"
    });
    fs::write(&input, source).unwrap();
    let build = run(Command::new("rustc")
        .args(["--edition", "2024"])
        .arg(&input)
        .arg("-o")
        .arg(&binary));
    assert!(build.ok, "{}", build.stderr);
    let result = run(&mut Command::new(binary));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
}
fn contract(source: &str, call: &str) -> Value {
    let script = r#"
import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';
import {parseRust} from './js/src/translation/rust.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
import {structuredDataToLino} from './js/src/structured-data-links.js';
const model='standard Rust shared array identity handles 1';
const library=selfTranslate(process.argv[1],'JavaScript','Rust',{referenceSemantics:true}).code;
const source=library+'\nfn main(){println!("{}",'+process.argv[2]+'());}\n';
const surface=parseRust(source,{sharedArrayHandleSourceModel:model}),program=checkProgram(surface);
const publication=selfTranslate(library,'Rust','JavaScript',{rustSharedArrayHandleSourceModel:model});
console.log(structuredDataToLino({source,library,surface,javascript:emitJavaScript(program),rust:emitRust(program),publication,signatures:selfTranslationSignatures(library,'Rust',{rustSharedArrayHandleSourceModel:model})}));
"#;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let result = run(Command::new("node").current_dir(root).args([
        "--input-type=module",
        "-e",
        script,
        source,
        call,
    ]));
    assert!(result.ok, "{}", result.stderr);
    read_structured_data_lino(&result.stdout).unwrap()
}
#[test]
fn complete_actual_shared_array_sources_keep_independent_behavior_and_full_runtime_parity() {
    versions();
    let policy = read_structured_data_lino(include_str!(
        "../../src/data/rust-shared-array-handles.lino"
    ))
    .unwrap();
    for row in policy["cases"].as_array().unwrap() {
        let call = row["call"].as_str().unwrap();
        let expected = format!("{}\n", row["expected"].as_bool().unwrap());
        let source = row["source"].as_str().unwrap();
        assert_eq!(
            observe(&format!("{source}\nconsole.log({call}());")),
            expected,
            "independent original JavaScript"
        );
        let contract = contract(source, call);
        let input = contract["source"].as_str().unwrap();
        assert_eq!(
            compiled(input),
            expected,
            "independent actual Rust source, with its explicit fixture entry point"
        );
        let surface = parse_rust_with_shared_array_handle_source_model(input, MODEL).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), contract["surface"]);
        let program = check_program(&surface).unwrap();
        let packet = encode_typed_program_links(&program).unwrap();
        let archives = [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ];
        for network in archives {
            let restored = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(restored, program);
            encode_typed_program_links(&restored).unwrap();
            let javascript = emit_javascript(&restored).unwrap();
            let rust = emit_rust(&restored).unwrap();
            // Emitted's public Serde contract deliberately omits these internal
            // ordered fields; compare their actual values independently.
            assert_eq!(
                serde_json::to_value(&javascript.definitions).unwrap(),
                contract["javascript"]["definitions"]
            );
            assert_eq!(
                serde_json::to_value(&javascript.preludes).unwrap(),
                contract["javascript"]["preludes"]
            );
            assert_eq!(
                serde_json::to_value(&rust.definitions).unwrap(),
                contract["rust"]["definitions"]
            );
            assert_eq!(
                serde_json::to_value(&rust.preludes).unwrap(),
                contract["rust"]["preludes"]
            );
            for field in ["text", "mappings", "assumptions", "encodings"] {
                assert_eq!(
                    serde_json::to_value(&javascript).unwrap()[field],
                    contract["javascript"][field],
                    "complete JavaScript field {field}"
                );
                assert_eq!(
                    serde_json::to_value(&rust).unwrap()[field],
                    contract["rust"][field],
                    "complete Rust field {field}"
                );
            }
            assert_eq!(observe(&javascript.text), expected);
            assert_eq!(compiled(&rust.text), expected);
        }
        let library = contract["library"].as_str().unwrap();
        assert!(
            parse_rust(library).is_err(),
            "legacy source interpretation remains closed"
        );
        let options = SelfTranslationOptions {
            rust_shared_array_handle_source_model: MODEL.to_owned(),
            ..SelfTranslationOptions::default()
        };
        let publication = self_translate_with(library, "Rust", "JavaScript", &options).unwrap();
        assert_eq!(
            publication.code,
            contract["publication"]["code"].as_str().unwrap()
        );
        assert_eq!(
            serde_json::to_value(self_translation_signatures(library, "Rust", &options).unwrap())
                .unwrap(),
            contract["signatures"]
        );
        assert_eq!(
            self_translate(&publication.code, "JavaScript", "Rust")
                .unwrap()
                .code,
            library
        );
        let created=meta_language::typed_program_links::create_typed_program_links_with_rust_shared_array_handle_source_model(input,"Rust",MODEL,None).unwrap();
        assert_eq!(
            decode_typed_program_links(&created.network, created.root).unwrap(),
            program
        );
    }
}
#[test]
fn shared_array_source_and_closed_surface_refuse_unproved_identity() {
    let handle = "std::rc::Rc<std::cell::RefCell<Vec<bool>>>";
    let sources = [
        format!("fn value(input:{handle})->{handle}{{input}}"),
        format!("fn value(input:{handle})->{handle}{{let alias=input;alias.clone()}}"),
        format!("fn value(left:{handle},right:{handle})->bool{{left==right}}"),
        format!("fn value(input:&{handle})->bool{{true}}"),
        "fn value(input:Vec<bool>)->Vec<bool>{input.clone()}".to_owned(),
        "fn value(input:std::rc::Rc<Vec<bool>>)->bool{true}".to_owned(),
        " ".repeat(65_537),
    ];
    for source in sources {
        assert!(
            parse_rust_with_shared_array_handle_source_model(&source, MODEL).is_err(),
            "{source}"
        );
    }
    assert!(parse_rust_with_shared_array_handle_source_model("fn main(){}", "unknown").is_err());
    for field in ["unknown", "block", "tagTest", "parenthesized", "rounding"] {
        let mut value = serde_json::json!({"k":"referenceIdentity","op":"eq","left":{"k":"name","path":["left"]},"right":{"k":"name","path":["right"]}});
        value[field] = Value::Null;
        assert!(
            serde_json::from_value::<meta_language::translation::surface::SExpr>(value).is_err(),
            "unowned field {field}"
        );
    }
}
