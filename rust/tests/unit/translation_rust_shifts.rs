//! Future paired four-target checked Rust shift controls; native products run in CI.
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
fn checked_rust_shifts_match_original_values_aborts_and_all_target_graphs() {
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
            "../experiments/rust-shifts/oracle.mjs",
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
    let functions = expected["functions"].as_array().unwrap();
    let mut main=r#"
fn ml_shift_case_result(result:std::thread::Result<String>)->(&'static str,String){
 match result{Ok(value)=>("value",value),Err(error)=>{
  let message=error.downcast_ref::<&str>().map(|value|(*value).to_owned()).or_else(||error.downcast_ref::<String>().cloned()).expect("an original panic retains its actual text");
  ("abort",message)
 }}
}
fn main(){std::panic::set_hook(Box::new(|_|{}));let cases:&[(usize,&str,&str,&str,&str)]=&[
"#.to_owned();
    for case in expected["cases"].as_array().unwrap() {
        main.push_str(&format!(
            "({},{:?},{:?},{:?},{:?}),",
            case[0].as_u64().unwrap(),
            case[1].as_str().unwrap(),
            case[2].as_str().unwrap(),
            case[3].as_str().unwrap(),
            case[4].as_str().unwrap()
        ));
    }
    main.push_str("];let mut failures=Vec::new();for(index,left,count,kind,expected)in cases{");
    for (owner, prefix) in [("original", "original::"), ("translated", "")] {
        main.push_str(&format!(
            "let {owner}=ml_shift_case_result(std::panic::catch_unwind(||match *index{{"
        ));
        for (index, row) in functions.iter().enumerate() {
            let name = if prefix.is_empty() {
                row["nativeTarget"].as_str().unwrap()
            } else {
                row["name"].as_str().unwrap()
            };
            let ty = row["type"].as_str().unwrap();
            let count_ty = row["countType"].as_str().unwrap();
            assert!(matches!(
                ty,
                "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "u128" | "i128"
            ));
            assert!(matches!(
                count_ty,
                "i8" | "u8" | "i32" | "u32" | "i128" | "u128"
            ));
            main.push_str(&format!("{index}=>{prefix}{name}(left.parse::<{ty}>().unwrap(),count.parse::<{count_ty}>().unwrap()).to_string(),"));
        }
        main.push_str("_=>panic!(\"unknown declared function\") }));");
        main.push_str(&format!("if {owner}!=(*kind,(*expected).to_owned()){{failures.push(format!(\"{owner} {{index}}({{left}},{{count}}): expected {{kind}} {{expected}}, observed {{:?}}\",{owner}));}}"));
    }
    main.push_str("}if !failures.is_empty(){eprintln!(\"{}\",failures.join(\"\n\"));}assert!(failures.is_empty());assert_eq!(original::shift_precedence(2,1),shift_precedence(2,1));assert_eq!(original::shift_contextual(),shift_contextual());println!(\"Rust checked shifts\");}\n");
    let original = format!(
        "\nmod original{{\n{}\n}}\n",
        expected["source"].as_str().unwrap()
    );
    let directory = TemporaryDirectory::new("rust-checked-shift-product");
    let source = directory.0.join("product.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "product.exe"
    } else {
        "product"
    });
    fs::write(&source, code + &original + &main).unwrap();
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
    assert_eq!(observed.stdout, "Rust checked shifts\n");
}
