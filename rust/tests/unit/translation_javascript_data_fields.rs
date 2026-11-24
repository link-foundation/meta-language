//! CI-only original/generated products and complete frontend parity controls.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::read_structured_data_lino;
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript,
};
use std::{fs, process::Command};
#[path = "../../src/translation/nominal_array_loan_rules.rs"]
mod loan_rules;
#[path = "../../src/translation/javascript_data_field_rules.rs"]
mod rules;

#[test]
fn datatype_fields_keep_complete_source_and_both_native_products() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/data-field-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let observations = read_structured_data_lino(&producer.stdout).unwrap();
    for row in observations["loanCases"].as_array().unwrap() {
        assert_eq!(
            loan_rules::accept_nominal_array_projection_loan(
                row["result"].as_str().unwrap(),
                row["reference"].as_bool().unwrap(),
                row["receiver"].as_str().unwrap(),
                row["constructors"].as_f64().unwrap(),
                row["cases"].as_f64().unwrap(),
                row["owner"].as_bool().unwrap(),
                row["fields"].as_f64().unwrap(),
                row["bindings"].as_f64().unwrap(),
                row["slot"].as_f64().unwrap(),
                row["matches"].as_f64().unwrap(),
                row["direct"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in observations["ownerFallbackCases"].as_array().unwrap() {
        assert_eq!(
            rules::accept_java_script_data_field_owner_fallback(
                row["kind"].as_str().unwrap(),
                row["owners"].as_f64().unwrap(),
                row["settled"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in observations["plans"].as_array().unwrap() {
        let positions = row["positions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let types = row["types"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let (error, mode, optional) = rules::read_java_script_data_field_plan(
            row["language"].as_str().unwrap(),
            row["field"].as_str().unwrap(),
            row["inherited"].as_bool().unwrap(),
            &positions,
            &types,
        );
        assert_eq!(error, row["expected"]["error"].as_str().unwrap());
        assert_eq!(mode, row["expected"]["mode"].as_str().unwrap());
        assert_eq!(optional, row["expected"]["optional"].as_bool().unwrap());
        assert_eq!(
            rules::read_java_script_data_field_inference_form(
                row["field"].as_str().unwrap(),
                &positions
            ),
            row["inference"].as_str().unwrap()
        );
    }
    for (index, row) in observations["rows"].as_array().unwrap().iter().enumerate() {
        let surface = parse_javascript(row["source"].as_str().unwrap()).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), row["surface"]);
        let program = check_program(&surface).unwrap();
        let native = emit_rust(&program).unwrap();
        assert_eq!(serde_json::to_value(&native).unwrap(), row["native"]);
        assert_eq!(
            serde_json::to_value(emit_javascript(&program).unwrap()).unwrap(),
            row["javascript"]
        );
        let entry = native
            .mappings
            .iter()
            .find(|mapping| mapping.kind == "function" && mapping.source == "compute")
            .unwrap()
            .target
            .clone();
        let expected = &row["value"];
        let assertion = if let Some(value) = expected.as_bool() {
            format!("assert_eq!({entry}(),{value});")
        } else if let Some(value) = expected.as_str() {
            format!("assert_eq!({entry}().to_string(),{value:?});")
        } else {
            format!("assert_eq!({entry}(),{}f64);", expected.as_f64().unwrap())
        };
        let javascript_product = row["native"]["preludes"]
            .as_array()
            .unwrap()
            .iter()
            .chain(row["native"]["definitions"].as_array().unwrap())
            .map(|part| part.as_str().unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        for (owner, code) in [
            (
                "native",
                native.preludes.join("\n") + "\n" + &native.definitions.join("\n"),
            ),
            ("javascript", javascript_product),
        ] {
            let directory = TemporaryDirectory::new(&format!("data-field-{index}-{owner}"));
            let file = directory.0.join("product.rs");
            let binary = directory.0.join(if cfg!(windows) {
                "product.exe"
            } else {
                "product"
            });
            fs::write(&file, code + "\nfn main(){" + &assertion + "}\n").unwrap();
            let mut compiler = Command::new("rustc");
            compiler.args(["--edition", "2024"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                compiler.arg("-C").arg(format!("linker={linker}"));
            }
            let built = run(compiler.arg(file).arg("-o").arg(&binary));
            assert!(built.ok, "{}: {}", row["name"], built.stderr);
            let observed = run(&mut Command::new(binary));
            assert!(observed.ok, "{}", observed.stderr);
        }
    }
    for row in observations["refusals"].as_array().unwrap() {
        let surface = parse_javascript(row["source"].as_str().unwrap()).unwrap();
        assert!(check_program(&surface).is_err(), "{}", row["name"]);
    }
}
