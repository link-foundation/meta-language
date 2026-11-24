#[path = "../../src/translation/rust_primitive_associated_call_rules.rs"]
mod associated_rules;
#[path = "../../src/translation/rust_primitive_integer_constant_rules.rs"]
mod constant_rules;
#[path = "../../src/translation/rust_primitive_from_rules.rs"]
mod from_rules;
#[path = "../../src/translation/rust_primitive_owner_path_rules.rs"]
mod owner_rules;
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
            .join("../parity/fixtures/rust-primitive-owner-path.lino"),
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
        rust_source_header_emission_model: Some(data["model"].as_str().unwrap().to_owned()),
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
        .arg("../experiments/rust-records/primitive-owner-path.mjs")
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
    let directory = TemporaryDirectory::new("primitive-owner-path");
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
                for check in row["checks"].as_array().unwrap() {
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
    let directory = TemporaryDirectory::new("primitive-owner-path-output");
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

#[test]
fn every_primitive_path_domain_and_separate_from_extension_stays_explicit() {
    let data = fixture();
    for row in data["owners"].as_array().unwrap() {
        let segments = row["segments"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let ordinary = row["ordinary"].as_bool().unwrap();
        let shadowed = row["shadowed"].as_bool().unwrap();
        let absolute = row["absolute"].as_bool().unwrap();
        let expected = row["expected"].as_str().unwrap();
        assert_eq!(
            owner_rules::read_rust_primitive_owner_path(&segments, ordinary, shadowed, absolute),
            expected
        );
        assert_eq!(
            associated_rules::read_rust_primitive_associated_owner(
                &segments, ordinary, shadowed, absolute
            ),
            expected
        );
        assert_eq!(
            constant_rules::read_rust_primitive_integer_constant_owner(
                &segments, ordinary, shadowed, absolute
            ),
            if expected == "bool" { "" } else { expected }
        );
        assert_eq!(
            from_rules::read_rust_primitive_from_owner(&segments, ordinary, shadowed, absolute),
            row["fromExpected"].as_str().unwrap()
        );
    }
}
#[test]
fn complete_preceding_generation_recipes_retain_all_removed_source_parts() {
    use sha2::{Digest, Sha256};
    let data = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let archive = meta_language::project_data::read_project_data_file(
        root.join(data["retainedRecipes"].as_str().unwrap()),
    )
    .unwrap();
    let receipt = meta_language::project_data::read_project_data_file(
        root.join(data["isolationReceipt"].as_str().unwrap()),
    )
    .unwrap();
    assert_eq!(
        archive["recipes"].as_array().unwrap().len(),
        data["recipes"].as_array().unwrap().len()
    );
    for row in data["recipes"].as_array().unwrap() {
        let retained = archive["recipes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["path"] == row["path"])
            .unwrap();
        let source = retained["source"].as_str().unwrap();
        assert_eq!(source.len() as u64, row["bytes"].as_u64().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            row["sha256"].as_str().unwrap()
        );
    }
    for row in receipt["removed"].as_array().unwrap() {
        let retained = archive["recipes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["path"] == row["retainedRecipe"])
            .unwrap();
        let record =
            meta_language::read_structured_data_lino(retained["source"].as_str().unwrap()).unwrap();
        let part = record["files"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| {
                format!("rust/src/translation/{}", item["path"].as_str().unwrap())
                    == row["path"].as_str().unwrap()
            })
            .unwrap();
        let source = part["source"].as_str().unwrap();
        assert_eq!(source.len() as u64, row["bytes"].as_u64().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            row["sha256"].as_str().unwrap()
        );
    }
}
