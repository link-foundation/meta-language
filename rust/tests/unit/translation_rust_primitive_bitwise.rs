//! Future paired four-target Rust primitive bitwise controls; native products run in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{check::check_program, emit_rust::emit_rust, rust::parse_rust};
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
fn rust_primitive_bitwise_matches_original_native_functions_and_all_target_graphs() {
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
            "../experiments/rust-primitive-bitwise/oracle.mjs",
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
    for (index, case) in expected["cases"].as_array().unwrap().iter().enumerate() {
        let name = case["name"].as_str().unwrap();
        let target = case["nativeTarget"].as_str().unwrap();
        let ty = case["type"].as_str().unwrap();
        assert!(matches!(
            ty,
            "bool" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "u128" | "i128"
        ));
        let left = format!(
            "{:?}.parse::<{ty}>().unwrap()",
            case["left"].as_str().unwrap()
        );
        let right = format!(
            "{:?}.parse::<{ty}>().unwrap()",
            case["right"].as_str().unwrap()
        );
        main.push_str(&format!("assert_eq!(original::{name}({left},{right}).to_string(),{:?},\"original {index}\");assert_eq!({target}({left},{right}).to_string(),{:?},\"translated {index}\");",case["expected"].as_str().unwrap(),case["expected"].as_str().unwrap()));
    }
    main.push_str("assert_eq!(original::precedence(8,3),precedence(8,3));assert_eq!(original::contextual(),contextual());");
    for name in ["eager_and", "eager_or", "eager_xor"] {
        main.push_str(&format!("assert!(std::panic::catch_unwind(original::{name}).is_err());assert!(std::panic::catch_unwind({name}).is_err());"));
    }
    main.push_str("println!(\"Rust primitive bitwise\");}\n");
    let eager = emit_rust(
        &check_program(&parse_rust(expected["eagerSource"].as_str().unwrap()).unwrap()).unwrap(),
    )
    .unwrap();
    let eager_code = eager
        .preludes
        .into_iter()
        .chain(eager.definitions)
        .collect::<Vec<_>>()
        .join("\n\n");
    assert_eq!(eager_code, expected["eagerLibrary"].as_str().unwrap());
    let original = format!(
        "\nmod original{{\n{}\n{}\n}}\n",
        expected["source"].as_str().unwrap(),
        expected["eagerSource"].as_str().unwrap()
    );
    let directory = TemporaryDirectory::new("rust-primitive-bitwise-product");
    let source = directory.0.join("product.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "product.exe"
    } else {
        "product"
    });
    fs::write(&source, code + &eager_code + &original + &main).unwrap();
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
    assert_eq!(observed.stdout, "Rust primitive bitwise\n");
}
