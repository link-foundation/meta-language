//! Native execution of the same complete module sources; run only in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{read_structured_data_lino, self_translate, self_translation_signatures};
use std::{fs, path::Path, process::Command};
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
        "uncovered Node {}; >=22.15 <23 or >=24 <25",
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
        "uncovered Rust {version}; >=1.90 <1.100"
    );
}
#[test]
fn complete_literal_siblings_preserve_actual_signatures_source_and_independent_collection_behavior()
{
    versions();
    for prefix in [
        "const values=[1,2];",
        "const values={enabled:true};",
        "const values=new Set([\"a\",\"b\"]);",
        "const values=new Map([[\"a\",1]]);",
        "const values=Object.freeze({enabled:true});",
    ] {
        assert_literal_sibling_translation(&format!(
            "{prefix}\nexport function create(){{return new Map([[\"a\",1],[\"b\",2]]);}}\n"
        ));
    }
}

fn assert_literal_sibling_translation(source: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let original = run(Command::new("node").args([
        "--input-type=module",
        "-e",
        &format!("{source}\nconsole.log(create().size);"),
    ]));
    assert!(original.ok, "{}", original.stderr);
    assert_eq!(original.stdout, "2\n");
    let script = r#"import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';import {structuredDataToLino} from './js/src/structured-data-links.js';const source=process.argv[1];console.log(structuredDataToLino({translation:selfTranslate(source,'JavaScript','Rust'),signatures:selfTranslationSignatures(source,'JavaScript')}));"#;
    let output = run(Command::new("node").current_dir(root).args([
        "--input-type=module",
        "-e",
        script,
        source,
    ]));
    assert!(output.ok, "{}", output.stderr);
    let expected = read_structured_data_lino(&output.stdout).unwrap();
    let actual = self_translate(source, "JavaScript", "Rust").unwrap();
    assert_eq!(
        actual.code,
        expected["translation"]["code"].as_str().unwrap()
    );
    assert_eq!(
        serde_json::json!({
            "sourceLanguage": actual.source_language,
            "targetLanguage": actual.target_language,
            "code": &actual.code,
            "items": actual.items.iter().map(|item| serde_json::json!({
                "term": &item.term, "start": item.start, "end": item.end,
                "status": item.status, "reason": &item.reason,
            })).collect::<Vec<_>>(),
        }),
        expected["translation"]
    );
    assert!(
        actual
            .items
            .iter()
            .any(|item| item.term == "export_statement" && item.status == "translated")
    );
    let signatures =
        self_translation_signatures(source, "JavaScript", &Default::default()).unwrap();
    assert_eq!(
        serde_json::to_value(&signatures).unwrap(),
        expected["signatures"]
    );
    assert!(
        signatures
            .iter()
            .any(|signature| signature.name() == "create")
    );
    assert_eq!(
        self_translate(&actual.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    let directory = TemporaryDirectory::new("inert-module-bindings");
    let input = directory.0.join("module.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "module.exe"
    } else {
        "module"
    });
    fs::write(
        &input,
        format!(
            "{}\nfn main(){{println!(\"{{}}\",create().len());}}\n",
            actual.code
        ),
    )
    .unwrap();
    let build = run(Command::new("rustc")
        .args(["--edition", "2024"])
        .arg(&input)
        .arg("-o")
        .arg(&binary));
    assert!(build.ok, "{}", build.stderr);
    let observed = run(&mut Command::new(&binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, original.stdout);
}

#[test]
fn empty_collection_siblings_preserve_complete_native_parity_and_independent_behavior() {
    versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/empty-module-collections.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    for case in cases {
        let prefix = case["source"].as_str().unwrap();
        let source = format!(
            "{prefix}\nexport function create(){{return new Map([[\"a\",1],[\"b\",2]]);}}\nfunction deferred(){{return /unsupported/u;}}\n"
        );
        assert_literal_sibling_translation(&source);
        let actual = self_translate(&source, "JavaScript", "Rust").unwrap();
        assert!(
            actual
                .items
                .iter()
                .any(|item| item.term == "function_declaration" && item.status == "carried")
        );
    }
}

#[test]
fn empty_collection_siblings_retain_complete_module_owner_and_effect_refusals() {
    for prefix in [
        "const Map={};const cache=new Map();",
        "class Map{}const cache=new Map;",
        "import {Map} from \"./custom.mjs\";const cache=new Map();",
        "const Set={};const cache=new Set;",
        "const cache=new Map();effect();",
        "const cache=new Set();class Owner{static value=effect();}",
    ] {
        let source = format!(
            "{prefix}\nexport function create(){{return new Map([[\"a\",1]]);}}\nfunction deferred(){{return /unsupported/u;}}\n"
        );
        assert!(
            self_translation_signatures(&source, "JavaScript", &Default::default())
                .unwrap()
                .is_empty(),
            "{source}"
        );
        let actual = self_translate(&source, "JavaScript", "Rust").unwrap();
        assert_eq!(
            self_translate(&actual.code, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
    }
}
#[test]
fn evaluated_or_shadowed_module_initializers_keep_their_original_scope_refusal() {
    for prefix in [
        "const Map={};",
        "const Map=[];",
        "const values=[effect()];",
        "const values={get enabled(){return true;}};",
        "const Set=custom;const values=new Set([1]);",
        "const Object=custom;const values=Object.freeze({});",
        "class Owner{static value=effect();}const values=[];",
    ] {
        let source =
            format!("{prefix}\nexport function create(){{return new Map([[\"a\",1]]);}}\n");
        assert!(
            self_translation_signatures(&source, "JavaScript", &Default::default())
                .unwrap()
                .is_empty(),
            "{source}"
        );
    }
}

#[test]
fn numeric_constant_siblings_preserve_complete_parity_and_independent_behavior() {
    versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/numeric-module-bindings.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    for case in cases {
        let prefix = case["source"].as_str().unwrap();
        let source = format!(
            "{prefix}\nexport function create(){{return new Map([[\"a\",1],[\"b\",2]]);}}\nfunction deferred(){{return /unsupported/u;}}\n"
        );
        assert_literal_sibling_translation(&source);
    }
}

#[test]
fn arithmetic_module_values_preserve_independent_numeric_behavior_and_complete_runtime_parity() {
    versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/numeric-module-values.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let numeric_script = r#"import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';import {structuredDataToLino} from './js/src/structured-data-links.js';const source=process.argv[1];console.log(structuredDataToLino({translation:selfTranslate(source,'JavaScript','Rust'),signatures:selfTranslationSignatures(source,'JavaScript')}));"#;
    for case in cases {
        let source = format!(
            "{}\nexport function verify(){{return {};}}\n",
            case["source"].as_str().unwrap(),
            case["predicate"].as_str().unwrap()
        );
        let original = run(Command::new("node").args([
            "--input-type=module",
            "-e",
            &format!("{source}\nconsole.log(verify());"),
        ]));
        assert!(original.ok, "{}", original.stderr);
        assert_eq!(original.stdout, "true\n", "{source}");
        let output = run(Command::new("node").current_dir(root).args([
            "--input-type=module",
            "-e",
            numeric_script,
            &source,
        ]));
        assert!(output.ok, "{}", output.stderr);
        let expected = read_structured_data_lino(&output.stdout).unwrap();
        let actual = self_translate(&source, "JavaScript", "Rust").unwrap();
        assert_eq!(
            serde_json::json!({
                "sourceLanguage": actual.source_language,
                "targetLanguage": actual.target_language,
                "code": &actual.code,
                "items": actual.items.iter().map(|item| serde_json::json!({
                    "term": &item.term, "start": item.start, "end": item.end,
                    "status": item.status, "reason": &item.reason,
                })).collect::<Vec<_>>(),
            }),
            expected["translation"]
        );
        assert!(
            actual.items.iter().all(|item| item.status == "translated"),
            "{source}"
        );
        assert_eq!(
            serde_json::to_value(
                self_translation_signatures(&source, "JavaScript", &Default::default()).unwrap()
            )
            .unwrap(),
            expected["signatures"]
        );
        assert_eq!(
            self_translate(&actual.code, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
        let directory = TemporaryDirectory::new("numeric-module-value");
        let input = directory.0.join("module.rs");
        let binary = directory.0.join(if cfg!(windows) {
            "module.exe"
        } else {
            "module"
        });
        fs::write(
            &input,
            format!(
                "{}\nfn main(){{println!(\"{{}}\",verify());}}\n",
                actual.code
            ),
        )
        .unwrap();
        let build = run(Command::new("rustc")
            .args(["--edition", "2024"])
            .arg(&input)
            .arg("-o")
            .arg(&binary));
        assert!(build.ok, "{}", build.stderr);
        let observed = run(&mut Command::new(&binary));
        assert!(observed.ok, "{}", observed.stderr);
        assert_eq!(observed.stdout, original.stdout, "{source}");
    }
}

#[test]
fn typed_arithmetic_constants_retain_complete_runtime_parity_and_original_source() {
    versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/numeric-module-values.lino"
    ))
    .unwrap();
    let cases = fixture["typedCases"].as_array().unwrap();
    assert_eq!(cases.len(), 1);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let typed_script = r#"import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';import {structuredDataToLino} from './js/src/structured-data-links.js';const source=process.argv[1];console.log(structuredDataToLino({translation:selfTranslate(source,'TypeScript','Rust'),signatures:selfTranslationSignatures(source,'TypeScript')}));"#;
    for case in cases {
        let source = format!(
            "{}\nexport function verify(){{return {};}}\n",
            case["source"].as_str().unwrap(),
            case["predicate"].as_str().unwrap()
        );
        let output = run(Command::new("node").current_dir(root).args([
            "--input-type=module",
            "-e",
            typed_script,
            &source,
        ]));
        assert!(output.ok, "{}", output.stderr);
        let expected = read_structured_data_lino(&output.stdout).unwrap();
        let actual = self_translate(&source, "TypeScript", "Rust").unwrap();
        assert_eq!(
            serde_json::json!({
                "sourceLanguage": actual.source_language,
                "targetLanguage": actual.target_language,
                "code": &actual.code,
                "items": actual.items.iter().map(|item| serde_json::json!({
                    "term": &item.term,"start":item.start,"end":item.end,"status":item.status,"reason":&item.reason,
                })).collect::<Vec<_>>(),
            }),
            expected["translation"]
        );
        assert!(actual.items.iter().all(|item| item.status == "translated"));
        assert_eq!(
            serde_json::to_value(
                self_translation_signatures(&source, "TypeScript", &Default::default()).unwrap()
            )
            .unwrap(),
            expected["signatures"]
        );
        assert_eq!(
            self_translate(&actual.code, "Rust", "TypeScript")
                .unwrap()
                .code,
            source
        );
    }
}
