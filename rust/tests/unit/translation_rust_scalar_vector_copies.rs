//! Deferred standard source copies, IEEE values and independent native products.
#[path = "../../src/translation/rust_scalar_vector_copy_rules.rs"]
mod rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_rust_source_owner_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary,
};
use serde_json::Value;
use std::{fs, process::Command};
fn fixture() -> Value {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-scalar-vector-copies.lino"),
    )
    .unwrap()
}
fn emission_value(value: &meta_language::TypedProgramLinksEmission) -> Value {
    serde_json::json!({"code":value.code,"language":value.language.as_str(),"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}
#[test]
fn every_actual_copy_selection_retains_its_complete_source_and_value_premises() {
    let data = fixture();
    for row in data["queries"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_scalar_vector_copy_query(
                row["ordinary"].as_bool().unwrap(),
                row["operation"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["layers"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["receivers"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_scalar_vector_copy_receiver(
                row["kind"].as_str().unwrap(),
                row["element"].as_str().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["references"].as_bool().unwrap(),
                row["copy"].as_bool().unwrap(),
                row["count"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["owners"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_scalar_vector_copy_owner(
                row["kind"].as_str().unwrap(),
                row["path"].as_str().unwrap(),
                row["paths"].as_f64().unwrap(),
                row["children"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["occurrences"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_scalar_vector_copy_occurrence(
                row["kind"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["recorded"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["values"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_scalar_vector_copy_value(
                row["language"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["operation"].as_str().unwrap(),
                row["receiver"].as_str().unwrap(),
                row["element"].as_str().unwrap(),
                row["reference"].as_bool().unwrap(),
                row["count"].as_f64().unwrap(),
                row["startKind"].as_str().unwrap(),
                row["start"].as_str().unwrap(),
                row["endKind"].as_str().unwrap(),
                row["end"].as_str().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
#[test]
fn complete_original_and_both_generated_products_keep_standard_scalar_vector_copies() {
    fixture();
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/scalar-vector-copies.mjs",
        "ordinary Rust scalar vector copies",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-scalar-vector-copy-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn both_actual_copy_products_keep_ieee_values_new_allocations_and_source_free_contracts() {
    let data = fixture();
    let controls = &data["controls"];
    let produced = run(Command::new("node")
        .arg("../experiments/rust-records/scalar-vector-copy-controls.mjs")
        .current_dir(env!("CARGO_MANIFEST_DIR")));
    assert!(produced.ok, "{}\n{}", produced.stdout, produced.stderr);
    let javascript = meta_language::read_structured_data_lino(&produced.stdout).unwrap();
    let packet =
        create_rust_source_owner_program_links(controls["source"].as_str().unwrap()).unwrap();
    let before = decode_typed_program_links(&packet.network, packet.root).unwrap();
    let views = [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ];
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let expected = emit_typed_program_links(&packet.network, packet.root, target).unwrap();
        assert_eq!(emission_value(&expected), javascript["emissions"][target]);
        for network in &views {
            assert_eq!(
                decode_typed_program_links(network, packet.root).unwrap(),
                before
            );
            assert_eq!(
                emit_typed_program_links(network, packet.root, target).unwrap(),
                expected
            );
        }
    }
    for row in javascript["observations"].as_array().unwrap() {
        assert_eq!(row["fromInput"], true);
        assert_eq!(row["separate"], true);
        for key in ["input", "first", "second"] {
            assert_eq!(row[key], controls["values"]);
        }
    }
    let native = emit_typed_program_links(&packet.network, packet.root, "Rust").unwrap();
    assert!(native.code.contains("::std::clone::Clone::clone"));
    assert!(!native.code.contains("ml_array::slice("));
    let directory = TemporaryDirectory::new("scalar-vector-copy-controls");
    for (product, code) in [
        (
            "javascript",
            javascript["emissions"]["Rust"]["code"].as_str().unwrap(),
        ),
        ("native", native.code.as_str()),
    ] {
        let mut source = format!(
            "{code}\nmod original{{\n{}\n}}\n#[test]fn scalar_copy_observations(){{\nlet expected=[f64::NAN,-0.0,f64::INFINITY,f64::NEG_INFINITY,7.5];\n",
            controls["source"].as_str().unwrap()
        );
        for name in controls["names"].as_array().unwrap() {
            let name = name.as_str().unwrap();
            for prefix in ["original::", ""] {
                source.push_str(&format!("{{let input=expected.to_vec();let original_address=input.as_ptr();let copied={prefix}{name}(input);assert_ne!(original_address,copied.as_ptr());assert_eq!(copied.len(),expected.len());for (value,expected) in copied.iter().zip(expected){{assert_eq!(value.to_bits(),expected.to_bits());}}}}\n"));
            }
        }
        source.push_str("}\n");
        let input = directory.0.join(format!("{product}.rs"));
        let binary = directory
            .0
            .join(format!("{product}{}", std::env::consts::EXE_SUFFIX));
        fs::write(&input, source).unwrap();
        let mut command = Command::new("rustc");
        command.args(["--edition", "2024", "--test"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            command.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(command.arg(&input).arg("-o").arg(&binary));
        assert!(
            compiled.ok,
            "{product}\n{}\n{}",
            compiled.stdout, compiled.stderr
        );
        let observed = run(Command::new(&binary).args(["--exact", "scalar_copy_observations"]));
        assert!(
            observed.ok,
            "{product}\n{}\n{}",
            observed.stdout, observed.stderr
        );
        assert!(observed.stdout.contains("1 passed"));
    }
}
