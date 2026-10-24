//! Future complete paired graph/codec/original-Rust controls. CI compilation only.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::rust::parse_rust_with_source_signature_model;
use meta_language::translation::{check::check_program, emit_rust::emit_rust, rust::parse_rust};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, TypedProgramLinks, TypedProgramLinksEmission,
    TypedProgramLinksError, create_rust_record_program_links, decode_link_network_binary,
    emit_typed_program_links, encode_link_network_binary, encode_typed_program_links,
    read_structured_data_lino,
};
use serde_json::{Value, json};
use std::fs;
use std::process::Command;

fn emission(value: TypedProgramLinksEmission) -> Value {
    json!({"code":value.code,"language":value.language,"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}

#[test]
fn nominal_records_preserve_complete_graphs_owners_order_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/rust-records/oracle.mjs",
        "Rust nominal records",
        create_rust_record_program_links,
    );
}

#[test]
fn named_fields_preserve_declarations_calls_patterns_and_mixed_encoding() {
    assert_nominal_translation_product(
        "../experiments/rust-records/layout.mjs",
        "Rust named constructor fields",
        create_rust_record_program_links,
    );
}

pub(super) fn assert_nominal_translation_product(
    producer_path: &str,
    product_label: &str,
    create_definition: fn() -> Result<TypedProgramLinks, TypedProgramLinksError>,
) {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let version = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = version
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {version}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            producer_path,
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let expected = read_structured_data_lino(&producer.stdout).unwrap();
    let program =
        check_program(&parse_rust(expected["source"].as_str().unwrap()).unwrap()).unwrap();
    let packet = encode_typed_program_links(&program).unwrap();
    assert_eq!(packet.root.as_u64(), expected["root"].as_u64().unwrap());
    assert_eq!(
        packet.network.to_lino(),
        expected["links"].as_str().unwrap()
    );
    let binary =
        encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ] {
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            assert_eq!(
                emission(emit_typed_program_links(&network, packet.root, target).unwrap()),
                expected["emissions"][target]
            );
        }
    }
    let definition = create_definition().unwrap();
    assert_eq!(
        definition.root.as_u64(),
        expected["definitionRoot"].as_u64().unwrap()
    );
    assert_eq!(
        definition.network.to_lino(),
        expected["definitionLinks"].as_str().unwrap()
    );
    let emitted = emit_rust(&program).unwrap();
    let header = emitted
        .text
        .lines()
        .find(|line| line.starts_with("#![allow("))
        .unwrap()
        .to_owned();
    let library = std::iter::once(header)
        .chain(emitted.preludes)
        .chain(emitted.definitions)
        .collect::<Vec<_>>()
        .join("\n\n");
    assert_eq!(library, expected["nativeLibrary"].as_str().unwrap());
    if let Some(layout) = expected.get("layout") {
        for piece in layout["pieces"].as_array().unwrap() {
            assert!(emitted.text.contains(piece.as_str().unwrap()), "{piece}");
        }
        assert!(emitted.text.lines().any(|line| {
            line.contains(layout["patternPrefix"].as_str().unwrap()) && line.contains(" } => ")
        }));
        assert_eq!(
            emitted
                .encodings
                .iter()
                .find(|encoding| encoding.id == "data")
                .unwrap()
                .statement,
            layout["encoding"].as_str().unwrap()
        );
    }
    let mut main = r#"
fn ml_record_result(result:std::thread::Result<String>)->(&'static str,String){
 match result{Ok(value)=>("value",value),Err(error)=>{
  let message=error.downcast_ref::<&str>().map(|value|(*value).to_owned()).or_else(||error.downcast_ref::<String>().cloned()).expect("the original panic retains its actual text");
  ("abort",message)
 }}
}
fn main(){std::panic::set_hook(Box::new(|_|{}));let mut failures=Vec::new();
"#.to_owned();
    for row in expected["cases"].as_array().unwrap() {
        let module = row["module"].as_str().unwrap();
        if row.get("previousExpectation").is_some() {
            assert!(
                expected["source"]
                    .as_str()
                    .unwrap()
                    .starts_with(row["source"].as_str().unwrap())
            );
            assert_eq!(
                row["previousExpectation"],
                json!({"kind":"unsupported","reason":"Rust record update: update syntax needs complete move and field provenance"})
            );
        }
        let native = row["nativeTarget"].as_str().unwrap().replace('.', "::");
        let (kind, value) = if let Some(message) = row["abort"].as_str() {
            ("abort", message)
        } else {
            ("value", row["value"].as_str().unwrap())
        };
        assert_eq!(
            row["observed"],
            json!({"kind":kind,"value":value}),
            "{}",
            row["name"]
        );
        for (owner, call) in [
            (
                "original",
                if module.is_empty() {
                    "original::result()".to_owned()
                } else {
                    format!("original::{module}::result()")
                },
            ),
            ("translated", format!("{native}()")),
        ] {
            main.push_str(&format!("{{let observed=ml_record_result(std::panic::catch_unwind(||({call}).to_string()));if observed!=({kind:?},{value:?}.to_owned()){{failures.push(format!(\"{owner} {module}: {{:?}}\",observed));}}}}\n"));
        }
    }
    main.push_str("if !failures.is_empty(){eprintln!(\"{}\",failures.join(\"\n\"));}assert!(failures.is_empty());");
    main.push_str(&format!("println!({product_label:?});}}\n"));
    let original = format!(
        "\nmod original{{\n{}\n}}\n",
        expected["source"].as_str().unwrap()
    );
    let directory = TemporaryDirectory::new(product_label);
    let source = directory.0.join("product.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "product.exe"
    } else {
        "product"
    });
    fs::write(&source, library + &original + &main).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024", "-C", "overflow-checks=on"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let built = run(compiler.arg(&source).arg("-o").arg(&binary));
    assert!(built.ok, "{}", built.stderr);
    let observed = run(&mut Command::new(&binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, format!("{product_label}\n"));
    for row in expected["refusals"].as_array().unwrap() {
        let surface = if let Some(model) = row["sourceSignatureModel"].as_str() {
            parse_rust_with_source_signature_model(row["source"].as_str().unwrap(), model)
        } else {
            parse_rust(row["source"].as_str().unwrap())
        };
        let result = surface.and_then(|surface| check_program(&surface));
        let error = result.expect_err(row["name"].as_str().unwrap());
        assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap());
        assert_eq!(error.reason, row["reason"].as_str().unwrap());
    }
}
