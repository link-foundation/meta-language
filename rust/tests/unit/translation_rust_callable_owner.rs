#[path = "../../src/translation/rust_callable_owner_rules.rs"]
mod callable_rules;
// Deferred complete original/JavaScript/native products with independent types.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::emit_rust::{RustEmissionOptions, emit_rust_with_options};
use meta_language::translation::ir::{Decl, FnDecl, Program};
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, TypedProgramLinksEmissionOptions,
    create_rust_source_owner_program_links, decode_link_network_binary, decode_typed_program_links,
    emit_typed_program_links, emit_typed_program_links_with_options, encode_link_network_binary,
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
            .join("../parity/fixtures/rust-callable-owner.lino"),
    )
    .unwrap()
}
fn function<'a>(program: &'a Program, name: &str) -> &'a FnDecl {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("actual retained function required")
    };
    function
}
fn emission_value(value: &meta_language::TypedProgramLinksEmission) -> Value {
    serde_json::json!({"code":value.code,"language":value.language.as_str(),"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}
fn options(data: &Value) -> TypedProgramLinksEmissionOptions {
    TypedProgramLinksEmissionOptions {
        rust_source_header_emission_model: data["model"].as_str().map(str::to_owned),
        ..TypedProgramLinksEmissionOptions::default()
    }
}

fn views(packet: &meta_language::TypedProgramLinks) -> Vec<LinkNetwork> {
    vec![
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}
#[test]
fn complete_source_products_keep_independent_scalar_function_types_and_observations() {
    let data = fixture();
    let produced = run(Command::new("node")
        .arg("../experiments/rust-records/callable-owner.mjs")
        .current_dir(env!("CARGO_MANIFEST_DIR")));
    assert!(produced.ok, "{}\n{}", produced.stdout, produced.stderr);
    let javascript = meta_language::read_structured_data_lino(&produced.stdout).unwrap();
    let packet = create_rust_source_owner_program_links(data["source"].as_str().unwrap()).unwrap();
    let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    let selected = emit_rust_with_options(
        &program,
        RustEmissionOptions {
            source_header_model: data["model"].as_str(),
        },
    )
    .unwrap();
    for row in data["cases"].as_array().unwrap() {
        assert_eq!(
            row["checks"].as_array().unwrap().len(),
            row["canonicalChecks"].as_array().unwrap().len()
        );
        assert_eq!(
            row["checks"].as_array().unwrap().len(),
            row["sourceContracts"].as_array().unwrap().len()
        );
        for contract in row["sourceContracts"].as_array().unwrap() {
            let name = format!(
                "{}.{}",
                row["module"].as_str().unwrap(),
                contract["name"].as_str().unwrap()
            );
            let actual =
                serde_json::to_value(function(&program, &name).source_signature.as_ref().unwrap())
                    .unwrap();
            let parameters = actual["parameters"]
                .as_array()
                .unwrap()
                .iter()
                .map(|position| position["owner"].clone())
                .collect::<Vec<_>>();
            assert_eq!(
                serde_json::to_value(parameters).unwrap(),
                contract["parameters"]
            );
            assert_eq!(actual["result"]["owner"], contract["result"]);
        }
    }
    let expected = emit_typed_program_links_with_options(
        &packet.network,
        packet.root,
        "Rust",
        &options(&data),
    )
    .unwrap();
    assert_eq!(emission_value(&expected), javascript["emission"]);
    let shared = LinkNetwork::from_lino(javascript["links"].as_str().unwrap()).unwrap();
    let shared_root = LinkId::from_u64(javascript["root"].as_u64().unwrap());
    for (network, root) in std::iter::once((&shared, shared_root))
        .chain(views(&packet).iter().map(|network| (network, packet.root)))
    {
        let restored = decode_typed_program_links(network, root).unwrap();
        for entry in &program.declarations {
            assert_eq!(
                function(&restored, entry.full_name()).source_signature,
                function(&program, entry.full_name()).source_signature
            );
        }
        let actual =
            emit_typed_program_links_with_options(network, root, "Rust", &options(&data)).unwrap();
        assert_eq!(emission_value(&actual), emission_value(&expected));
    }
    let native_library = format!(
        "{}\n{}",
        selected.preludes.join("\n"),
        selected.definitions.join("\n")
    );
    assert_native_products(&data, &javascript, &native_library);
    assert_printed_products(&data, &javascript);
}

fn assert_native_products(data: &Value, javascript: &Value, native_library: &str) {
    let directory = TemporaryDirectory::new("callable-owner");
    // Each product is a crate root: its actual absolute crate paths must keep
    // their meaning. Batch all cases per product instead of nesting the code.
    for (product, library) in [
        ("javascript", javascript["nativeLibrary"].as_str().unwrap()),
        ("native", native_library),
    ] {
        let mut source = format!(
            "{library}\nmod original {{\n{}\n}}\n",
            data["source"].as_str().unwrap()
        );
        let mut observations = String::from("#[test]fn retained_scalar_observations(){\n");
        for (index, row) in data["cases"].as_array().unwrap().iter().enumerate() {
            assert_eq!(javascript["observations"][index]["value"], row["expected"]);
            assert_eq!(
                javascript["observations"][index]["unit"].as_bool().unwrap(),
                row["resultType"] == "()"
            );
            let module = row["module"].as_str().unwrap();
            for prefix in [format!("original::{module}"), module.to_owned()] {
                let checks = if prefix.starts_with("original::") {
                    &row["checks"]
                } else {
                    &row["canonicalChecks"]
                };
                for check in checks.as_array().unwrap() {
                    source.push_str(&check.as_str().unwrap().replace("CASE", &prefix));
                    source.push('\n');
                }
                observations.push_str(&format!(
                    "assert_eq!({prefix}::result().to_string(),{:?});\n",
                    row["expected"].as_str().unwrap()
                ));
            }
        }
        observations.push_str("}\n");
        source.push_str(&observations);
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
        let observed = run(Command::new(&binary).args(["--exact", "retained_scalar_observations"]));
        assert!(
            observed.ok,
            "{product}\n{}\n{}",
            observed.stdout, observed.stderr
        );
        assert!(observed.stdout.contains("1 passed"));
    }
}

fn assert_printed_products(data: &Value, javascript: &Value) {
    let directory = TemporaryDirectory::new("callable-owner-output");
    for (index, row) in data["printedSources"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let packet =
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap();
        let native = emit_typed_program_links(&packet.network, packet.root, "Rust").unwrap();
        let expected = &javascript["printedProducts"][index];
        assert_eq!(expected["lines"], row["lines"]);
        assert_eq!(emission_value(&native), expected["native"]);
        let lines = row["lines"]
            .as_array()
            .unwrap()
            .iter()
            .map(|line| line.as_str().unwrap())
            .collect::<Vec<_>>();
        for (product, source) in [
            ("original", row["source"].as_str().unwrap()),
            ("javascript", expected["native"]["code"].as_str().unwrap()),
            ("native", native.code.as_str()),
        ] {
            let input = directory.0.join(format!("{index}-{product}.rs"));
            let binary = directory
                .0
                .join(format!("{index}-{product}{}", std::env::consts::EXE_SUFFIX));
            fs::write(&input, source).unwrap();
            let mut command = Command::new("rustc");
            command.args(["--edition", "2024"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                command.arg("-C").arg(format!("linker={linker}"));
            }
            let compiled = run(command.arg(&input).arg("-o").arg(&binary));
            assert!(
                compiled.ok,
                "{product}: {}\n{}",
                compiled.stdout, compiled.stderr
            );
            let observed = run(&mut Command::new(&binary));
            assert!(
                observed.ok,
                "{product}: {}\n{}",
                observed.stdout, observed.stderr
            );
            assert_eq!(observed.stdout.lines().collect::<Vec<_>>(), lines);
        }
    }
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item.as_str().unwrap().to_owned())
        .collect()
}
#[test]
fn every_recursive_callable_owner_contract_join_equality_budget_and_refusal_stays_explicit() {
    let data = fixture();
    for row in data["contractErrors"].as_array().unwrap() {
        assert_eq!(
            callable_rules::read_rust_callable_owner_error(
                &strings(&row["actual"]),
                &strings(&row["expected"])
            ),
            row["expectedError"].as_str().unwrap()
        );
    }
    for row in data["equalities"].as_array().unwrap() {
        assert_eq!(
            callable_rules::accept_rust_callable_owner_descriptors_equal(
                &strings(&row["left"]),
                &strings(&row["right"])
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["joins"].as_array().unwrap() {
        assert_eq!(
            callable_rules::read_rust_callable_owner_join_error(&strings(&row["rows"])),
            row["expectedError"].as_str().unwrap()
        );
    }
    for row in data["limits"].as_array().unwrap() {
        assert_eq!(
            callable_rules::read_rust_callable_owner_projection_limit(
                row["name"].as_str().unwrap()
            ),
            row["expected"].as_f64().unwrap()
        );
    }
    for row in data["oversized"].as_array().unwrap() {
        let descriptor =
            vec![String::new(); usize::try_from(row["size"].as_u64().unwrap()).unwrap()];
        assert_eq!(
            callable_rules::read_rust_callable_owner_error(&descriptor, &[]),
            row["expectedError"].as_str().unwrap()
        );
        assert_eq!(
            callable_rules::read_rust_callable_owner_join_error(&descriptor),
            row["expectedError"].as_str().unwrap()
        );
        assert!(
            !callable_rules::accept_rust_callable_owner_descriptors_equal(&descriptor, &descriptor)
        );
    }
    for row in data["refusals"].as_array().unwrap() {
        let error =
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap_err();
        assert!(
            error.message.contains(row["reason"].as_str().unwrap()),
            "{}: {}",
            row["name"],
            error.message
        );
    }
    for row in data["malformed"].as_array().unwrap() {
        assert!(
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).is_err(),
            "{}",
            row["name"]
        );
    }
}

#[test]
fn original_rust_rejects_each_retained_source_type_mismatch_and_malformed_input() {
    let data = fixture();
    let directory = TemporaryDirectory::new("callable-owner-original-refusals");
    let mut failures = Vec::new();
    for (index, row) in data["refusals"]
        .as_array()
        .unwrap()
        .iter()
        .chain(data["malformed"].as_array().unwrap())
        .enumerate()
    {
        let input = directory.0.join(format!("refusal-{index}.rs"));
        let output = directory.0.join(format!("refusal-{index}.rmeta"));
        fs::write(&input, row["source"].as_str().unwrap()).unwrap();
        let observed = run(Command::new("rustc")
            .args([
                "--edition",
                "2024",
                "--crate-type",
                "lib",
                "--emit",
                "metadata",
                "--color",
                "never",
            ])
            .arg(&input)
            .arg("-o")
            .arg(&output));
        if observed.ok {
            failures.push(format!(
                "{}: original Rust accepted a retained invalid input",
                row["name"]
            ));
        }
        if let Some(codes) = row["originalRustErrorCodes"].as_array() {
            for code in codes {
                if !observed
                    .stderr
                    .contains(&format!("error[{}]", code.as_str().unwrap()))
                {
                    failures.push(format!(
                        "{}: original Rust omitted {}\n{}",
                        row["name"], code, observed.stderr
                    ));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn corrected_emitter_scope_retains_every_original_source_and_type_assertion() {
    use sha2::{Digest, Sha256};
    let data = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for correction in data["assertionCorrections"].as_array().unwrap() {
        let path = root.join(correction["retainedDraft"].as_str().unwrap());
        let bytes = fs::read(&path).unwrap();
        assert_eq!(bytes.len() as u64, correction["bytes"].as_u64().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            correction["sha256"].as_str().unwrap()
        );
        let prior = meta_language::project_data::read_project_data_file(path).unwrap();
        assert!(
            prior["cases"].as_array().unwrap().len() <= data["cases"].as_array().unwrap().len()
        );
        for (index, before) in prior["cases"].as_array().unwrap().iter().enumerate() {
            let after = &data["cases"][index];
            for field in ["name", "module", "source", "expected", "resultType"] {
                assert_eq!(after[field], before[field]);
            }
            for check in before["checks"].as_array().unwrap() {
                assert!(after["checks"].as_array().unwrap().contains(check));
            }
        }
    }
}
