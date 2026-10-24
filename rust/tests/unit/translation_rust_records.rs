//! Future complete paired graph/codec/original-Rust controls. CI compilation only.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{check::check_program, emit_rust::emit_rust, rust::parse_rust};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, TypedProgramLinksEmission, create_rust_record_program_links,
    decode_link_network_binary, emit_typed_program_links, encode_link_network_binary,
    encode_typed_program_links, read_structured_data_lino,
};
use serde_json::{Value, json};
use std::fs;
use std::process::Command;

fn emission(value: TypedProgramLinksEmission) -> Value {
    json!({"code":value.code,"language":value.language,"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}

#[test]
fn nominal_records_preserve_complete_graphs_owners_order_and_original_values() {
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
            "../experiments/rust-records/oracle.mjs",
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
    let definition = create_rust_record_program_links().unwrap();
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
            ("original", format!("original::{module}::result()")),
            ("translated", format!("{native}()")),
        ] {
            main.push_str(&format!("{{let observed=ml_record_result(std::panic::catch_unwind(||({call}).to_string()));if observed!=({kind:?},{value:?}.to_owned()){{failures.push(format!(\"{owner} {module}: {{:?}}\",observed));}}}}\n"));
        }
    }
    main.push_str("if !failures.is_empty(){eprintln!(\"{}\",failures.join(\"\n\"));}assert!(failures.is_empty());println!(\"Rust nominal records\");}\n");
    let original = format!(
        "\nmod original{{\n{}\n}}\n",
        expected["source"].as_str().unwrap()
    );
    let directory = TemporaryDirectory::new("rust-nominal-record-product");
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
    assert_eq!(observed.stdout, "Rust nominal records\n");
    for row in expected["refusals"].as_array().unwrap() {
        let result =
            parse_rust(row["source"].as_str().unwrap()).and_then(|surface| check_program(&surface));
        let error = result.expect_err(row["name"].as_str().unwrap());
        assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap());
        assert_eq!(error.reason, row["reason"].as_str().unwrap());
    }
}
