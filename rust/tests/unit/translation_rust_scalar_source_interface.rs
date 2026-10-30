//! Deferred complete original/JavaScript/native products with independent types.
#[path = "../../src/translation/rust_scalar_source_interface_rules.rs"]
mod rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::emit_rust::{RustEmissionOptions, emit_rust_with_options};
use meta_language::translation::ir::{Decl, FnDecl, Program};
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, TypedProgramLinksEmissionOptions,
    create_rust_source_owner_program_links, decode_link_network_binary, decode_typed_program_links,
    emit_typed_program_links_with_options, encode_link_network_binary, encode_typed_program_links,
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
            .join("../parity/fixtures/rust-scalar-source-interface.lino"),
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
        .arg("../experiments/rust-records/scalar-source-interface.mjs")
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
    let directory = TemporaryDirectory::new("retained-scalar-source-interface");
    // Each product is a crate root: its actual absolute crate paths must keep
    // their meaning. Batch all cases per product instead of nesting the code.
    for (product, library) in [
        ("javascript", javascript["nativeLibrary"].as_str().unwrap()),
        ("native", native_library.as_str()),
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
                    "assert_eq!(format!(\"{{:?}}\",{prefix}::result()),{:?});\n",
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
#[test]
fn every_declared_generated_scalar_decision_keeps_its_explicit_domain() {
    let data = fixture();
    assert!(rules::accept_rust_scalar_source_interface_model(
        data["model"].as_str().unwrap()
    ));
    assert!(!rules::accept_rust_scalar_source_interface_model("unknown"));
    for row in data["values"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_scalar_value_type(
                row["kind"].as_str().unwrap(),
                row["bits"].as_f64().unwrap(),
                row["signed"].as_bool().unwrap()
            ),
            row["expected"]
        );
    }
    for row in data["owners"].as_array().unwrap() {
        assert_eq!(
            rules::read_rust_scalar_source_type(
                row["value"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["owner"].as_str().unwrap(),
                row["paths"].as_f64().unwrap(),
                row["children"].as_f64().unwrap()
            ),
            row["expected"]
        );
    }
    for row in data["calls"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_scalar_source_call(
                row["owned"].as_bool().unwrap(),
                row["arguments"].as_bool().unwrap(),
                row["result"].as_bool().unwrap(),
                row["static"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["budgets"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_scalar_source_budget(row["visits"].as_f64().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["positions"].as_array().unwrap() {
        assert_eq!(
            rules::accept_rust_scalar_source_position(
                row["model"].as_str().unwrap(),
                row["source"].as_str().unwrap(),
                row["invalid"].as_bool().unwrap(),
                row["borrowed"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
#[test]
fn actual_source_free_header_edits_refuse_unsupported_call_contracts() {
    let data = fixture();
    let row = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["name"]
                .as_str()
                .unwrap()
                .starts_with("borrowed Boolean")
        })
        .unwrap();
    let packet = create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap();
    let original = decode_typed_program_links(&packet.network, packet.root).unwrap();
    for (change, diagnostic) in [
        ("static", "nonstatic copied borrows"),
        ("result", "direct results"),
        ("legacy", "version-two scalar owners"),
    ] {
        let mut program = original.clone();
        let Some(Decl::Fn(function)) = program
            .declarations
            .iter_mut()
            .find(|entry| entry.full_name() == "identity")
        else {
            panic!("actual function")
        };
        let signature = function.source_signature.as_mut().unwrap();
        match change {
            "static" => signature.parameters[0].lifetimes = vec![Some("static".into())],
            "result" => {
                signature.result.wrappers = vec!["reference".into()];
                signature.result.lifetimes = vec![None];
            }
            _ => signature.model = "Rust source function signature 1".into(),
        }
        let edited = encode_typed_program_links(&program).unwrap();
        for network in views(&edited) {
            let error = emit_typed_program_links_with_options(
                &network,
                edited.root,
                "Rust",
                &options(&data),
            )
            .unwrap_err();
            assert!(error.message.contains(diagnostic), "{error}");
        }
    }
    let nominal = create_rust_source_owner_program_links(
        "struct Value{value:usize}pub fn read(value:Value)->usize{value.value}fn main(){}",
    )
    .unwrap();
    assert!(
        emit_typed_program_links_with_options(
            &nominal.network,
            nominal.root,
            "Rust",
            &options(&data)
        )
        .unwrap_err()
        .message
        .contains("original owned function declaration")
    );
}
