//! Future paired four-target collection controls; native products run in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{
    check::check_program, emit_rust::emit_rust, javascript::parse_javascript,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, TypedProgramLinksEmission, decode_link_network_binary,
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
fn complete_collection_model_replays_all_targets_and_executes_one_native_product() {
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
            "../experiments/collection-access/oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let expected = read_structured_data_lino(&producer.stdout).unwrap();
    let program =
        check_program(&parse_javascript(expected["source"].as_str().unwrap()).unwrap()).unwrap();
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
    let emitted = emit_rust(&program).unwrap();
    let header = emitted
        .text
        .lines()
        .find(|line| line.starts_with("#![allow("))
        .unwrap()
        .to_owned();
    let code = std::iter::once(header)
        .chain(emitted.preludes)
        .chain(emitted.definitions)
        .collect::<Vec<_>>()
        .join("\n\n");
    assert_eq!(code, expected["nativeLibrary"].as_str().unwrap());
    let mut main = "\nfn main(){".to_owned();
    for (index, value) in expected["expected"].as_array().unwrap().iter().enumerate() {
        let value = value.as_str().unwrap().parse::<f64>().unwrap();
        assert!(value.is_finite());
        main.push_str(&format!(
            "assert_eq!(compute{index}(),{value}f64,\"fixture {index}\");"
        ));
    }
    main.push_str("println!(\"collection access\");}\n");
    let directory = TemporaryDirectory::new("collection-access-product");
    let source = directory.0.join("product.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "product.exe"
    } else {
        "product"
    });
    fs::write(&source, code + &main).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let built = run(compiler.arg(&source).arg("-o").arg(&binary));
    assert!(built.ok, "{}", built.stderr);
    let observed = run(&mut Command::new(&binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, "collection access\n");
}
