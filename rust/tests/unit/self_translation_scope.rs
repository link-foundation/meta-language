//! Regression checks for combined declaration scopes and source preservation.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    SelfTranslationItem, SelfTranslationOptions, self_translate, self_translate_with,
    self_translation_signatures,
};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::process::Command;

#[test]
fn actual_external_bindings_emit_calls_and_reads_through_both_semantic_codecs() {
    use meta_language::translation::{
        check::check_program,
        emit_javascript::emit_javascript,
        javascript::{ModuleContext, parse_javascript_in},
        surface::External,
    };
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
        emit_typed_program_links, encode_link_network_binary, encode_typed_program_links,
        read_structured_data_lino,
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let version = run(Command::new("node").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let parts = version
        .stdout
        .trim()
        .trim_start_matches('v')
        .split('.')
        .collect::<Vec<_>>();
    let major = parts[0].parse::<u32>().unwrap();
    let minor = parts[1].parse::<u32>().unwrap();
    assert!(
        major == 22 && minor >= 15 || major == 24,
        "uncovered Node {}",
        version.stdout
    );
    let script = r#"
import {runInNewContext} from 'node:vm';
import {selfTranslationSignatures} from './js/src/self-translation.js';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {structuredDataToLino} from './js/src/structured-data-links.js';
const [name,provider,source]=process.argv.slice(1);
const externals=selfTranslationSignatures(provider,'JavaScript').map(row=>({...row,name}));
const scope=provider.replace('export ','')+(name==='increment'||name==='limit'?'':'\nconst '+name+'=increment;');
const program=checkProgram(parseJavaScript(source,{externals,moduleBindingNames:[name]}));
const javascript=emitJavaScript(program).text;
const original=runInNewContext(scope+'\n'+source+'\nselected();');
const actual=runInNewContext(scope+'\n'+javascript+'\nselected();');
process.stdout.write(structuredDataToLino({javascript,original,actual}));
"#;
    for name in ["increment", "String", "_value", "$increment", "limit"] {
        let provider = if name == "limit" {
            "export const limit=7;"
        } else {
            "/** @param {number} value @returns {number} */export function increment(value){return value+1;}"
        };
        let expression = if name == "limit" {
            "limit+1".to_owned()
        } else {
            format!("{name}(7)")
        };
        let source =
            format!("/** @returns {{number}} */function selected(){{return {expression};}}");
        let mut externals =
            self_translation_signatures(provider, "JavaScript", &SelfTranslationOptions::default())
                .unwrap();
        assert_eq!(externals.len(), 1, "{name}");
        match &mut externals[0] {
            External::Function { name: binding, .. } | External::Constant { name: binding, .. } => {
                *binding = name.to_owned()
            }
        }
        let context = ModuleContext {
            externals,
            module_directory: None,
        };
        let program = check_program(&parse_javascript_in(&source, &context).unwrap()).unwrap();
        let emitted = emit_javascript(&program).unwrap();
        let observation = run(Command::new("node").current_dir(root).args([
            "--input-type=module",
            "-e",
            script,
            name,
            provider,
            &source,
        ]));
        assert!(observation.ok, "{name}: {}", observation.stderr);
        let observation = read_structured_data_lino(&observation.stdout).unwrap();
        assert_eq!(observation["original"], 8, "{name}");
        assert_eq!(observation["actual"], 8, "{name}");
        assert_eq!(observation["javascript"], emitted.text, "{name}");
        let packet = encode_typed_program_links(&program).unwrap();
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            assert_eq!(
                decode_typed_program_links(&network, packet.root).unwrap(),
                program
            );
            assert_eq!(
                emit_typed_program_links(&network, packet.root, "JavaScript")
                    .unwrap()
                    .code,
                emitted.text
            );
        }
    }
}

#[test]
fn resource_limited_executor_preserves_source_and_restores_exactly() {
    let source = include_str!("../../../js/src/grammar-runtime/executor.js");
    let translated = self_translate(source, "JavaScript", "Rust").unwrap();
    assert_ne!(translated.items, [] as [SelfTranslationItem; 0]);
    let mut cursor = 0;
    for item in &translated.items {
        assert!(item.start >= cursor && item.end > item.start && item.end <= source.len());
        assert!(
            source[cursor..item.start]
                .encode_utf16()
                .all(meta_language::translation::lexer::is_js_space)
        );
        cursor = item.end;
        if item.term == "ERROR" {
            assert_eq!(item.status, "carried");
        }
    }
    assert!(
        source[cursor..]
            .encode_utf16()
            .all(meta_language::translation::lexer::is_js_space)
    );
    assert_eq!(
        self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
}

#[test]
fn source_prefix_restoration_rejects_appended_code() {
    let source = "/** @param {number} value @returns {number} */\nfunction identity(value) { return value; }";
    let translated = self_translate(source, "JavaScript", "Rust").unwrap();
    assert_eq!(
        self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    let edited = self_translate(
        &format!("{}\nconst EXTRA: f64 = 2.0;\n", translated.code),
        "Rust",
        "JavaScript",
    )
    .unwrap();
    assert_ne!(edited.code, source);
}

#[test]
fn scope_retry_exports_signatures_to_importing_modules() {
    let source = include_str!("../../../js/src/translation/frontend-rules.js");
    let expected: Vec<_> = source
        .lines()
        .filter_map(|line| line.strip_prefix("export function "))
        .map(|tail| tail.split('(').next().unwrap())
        .filter(|name| *name != "decodeUnicodeEscape")
        .collect();
    let _ = std::io::stderr().write_all(b"self-translation phase: frontend signatures start\n");
    let signatures =
        self_translation_signatures(source, "JavaScript", &SelfTranslationOptions::default())
            .unwrap();
    let _ = std::io::stderr().write_all(b"self-translation phase: frontend signatures returned\n");
    let names: Vec<_> = signatures
        .iter()
        .map(meta_language::SelfTranslationSignature::name)
        .collect();
    assert_eq!(names, expected);
    let options = SelfTranslationOptions {
        imports: BTreeMap::from([("./frontend-rules.js".to_owned(), signatures)]),
        ..SelfTranslationOptions::default()
    };
    let _ =
        std::io::stderr().write_all(b"self-translation phase: frontend importing module start\n");
    let imported = self_translate_with("import { readArrayMethodForm as form } from './frontend-rules.js';\n/** @returns {string} */\nexport function select() { return form('concat'); }\n", "JavaScript", "Rust", &options).unwrap();
    let _ = std::io::stderr()
        .write_all(b"self-translation phase: frontend importing module returned\n");
    assert!(
        imported
            .items
            .iter()
            .all(|item| item.status == "translated")
    );
    assert!(
        imported
            .code
            .contains("use crate::frontend_rules::read_array_method_form as form;")
    );
}

#[test]
fn selected_utf16_prelude_retains_scalar_source_and_carries_unproved_text_values() {
    let source = "/** @param {number} value @returns {number} */\nexport function keep(value) { return value; }\n";
    let options = SelfTranslationOptions {
        utf16_strings: true,
        ..SelfTranslationOptions::default()
    };
    let default = self_translate(source, "JavaScript", "Rust").unwrap();
    assert!(!default.code.contains("pub mod ml_string {"));
    let selected = self_translate_with(source, "JavaScript", "Rust", &options).unwrap();
    assert_eq!(selected.code.matches("pub mod ml_string {").count(), 1);
    assert!(selected.items.iter().all(|item| item.status != "carried"));
    assert_eq!(
        self_translate(&selected.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    // Keep the historical inputs and test identity, but require the Text
    // lowering now proved by the paired JavaScript emitter and native execution.
    let directory = TemporaryDirectory::new("selected-utf16-text");
    fs::write(
        directory.0.join("text.mjs"),
        "export function readLabel(){return String.fromCharCode(65,55357,56832,66,55296,57343);}\n",
    )
    .unwrap();
    let observe = |source: &str,
                   options: &SelfTranslationOptions,
                   name: &str,
                   javascript_observer: &str,
                   rust_observer: &str,
                   expected: &str| {
        let translated = self_translate_with(source, "JavaScript", "Rust", options).unwrap();
        assert!(translated.items.iter().all(|item| item.status != "carried"));
        assert_eq!(translated.code.matches("pub mod ml_string {").count(), 1);
        assert_eq!(
            self_translate(&translated.code, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
        let script = "import{selfTranslate}from'./js/src/self-translation.js';process.stdout.write(selfTranslate(process.argv[1],'JavaScript','Rust',JSON.parse(process.argv[2])).code);";
        let javascript_options = serde_json::json!({
            "utf16Strings": options.utf16_strings,
            "imports": &options.imports,
        });
        let paired = run(Command::new("node")
            .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
            .args([
                "--input-type=module",
                "-e",
                script,
                source,
                &javascript_options.to_string(),
            ]));
        assert!(paired.ok, "{name}: {}", paired.stderr);
        assert_eq!(translated.code, paired.stdout, "{name}: paired emission");

        let javascript_file = directory.0.join(format!("{name}.mjs"));
        fs::write(&javascript_file, format!("{source}\n{javascript_observer}")).unwrap();
        let original = run(Command::new("node").arg(&javascript_file));
        assert!(original.ok, "{name}: {}", original.stderr);
        assert_eq!(original.stdout, expected, "{name}: original JavaScript");

        let rust_file = directory.0.join(format!("{name}.rs"));
        let executable = directory
            .0
            .join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
        fs::write(&rust_file, format!("{}\n{rust_observer}", translated.code)).unwrap();
        let compiled = run(Command::new("rustc")
            .arg("--edition=2024")
            .arg(&rust_file)
            .arg("-o")
            .arg(&executable));
        assert!(compiled.ok, "{name}: {}", compiled.stderr);
        let native = run(&mut Command::new(&executable));
        assert!(native.ok, "{name}: {}", native.stderr);
        assert_eq!(native.stdout, expected, "{name}: emitted Rust");
        assert_eq!(native.stdout, original.stdout, "{name}: execution parity");
    };
    for (name, source, javascript_observer, rust_observer) in [
        (
            "string_identity",
            "/** @param {string} value @returns {string} */\nexport function keep(value) { return value; }\n",
            "for(const units of [[65,55357,56832,66],[55296,65,57343],[]]){const value=keep(String.fromCharCode(...units));console.log(Array.from({length:value.length},(_,index)=>value.charCodeAt(index)).join(','));}",
            r#"fn main(){for units in [vec![65_u16,55357,56832,66],vec![55296,65,57343],vec![]]{let value=keep(crate::ml_string::Text::from_units(units));println!("{}",value.0.iter().map(u16::to_string).collect::<Vec<_>>().join(","));}}"#,
        ),
        (
            "string_array_identity",
            "/** @param {string[]} values @returns {string[]} */\nexport function keep(values) { return values; }\n",
            "const values=[[65,55357,56832,66],[55296,65,57343],[]].map(units=>String.fromCharCode(...units));for(const value of keep(values)){console.log(Array.from({length:value.length},(_,index)=>value.charCodeAt(index)).join(','));}",
            r#"fn main(){let values=vec![crate::ml_string::Text::from_units(vec![65,55357,56832,66]),crate::ml_string::Text::from_units(vec![55296,65,57343]),crate::ml_string::Text::from_units(vec![])];for value in keep(values){println!("{}",value.0.iter().map(u16::to_string).collect::<Vec<_>>().join(","));}}"#,
        ),
    ] {
        observe(
            source,
            &options,
            name,
            javascript_observer,
            rust_observer,
            "65,55357,56832,66\n55296,65,57343\n\n",
        );
    }
    let external = SelfTranslationOptions {
        imports: BTreeMap::from([(
            "./text.mjs".to_owned(),
            vec![meta_language::translation::surface::External::Function {
                name: "readLabel".to_owned(),
                params: Vec::new(),
                ret: meta_language::translation::types::Type::String,
            }],
        )]),
        utf16_strings: true,
        ..SelfTranslationOptions::default()
    };
    let source = "import { readLabel } from './text.mjs';\n/** @returns {number} */\nexport function count() { return readLabel().length; }\n";
    observe(
        source,
        &external,
        "imported_string_length",
        "console.log(count());",
        r#"mod text{pub fn read_label()->crate::ml_string::Text{crate::ml_string::Text::from_units(vec![65,55357,56832,66,55296,57343])}}fn main(){println!("{}",count());}"#,
        "6\n",
    );
}
