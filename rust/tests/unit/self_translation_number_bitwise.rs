//! One native binary executes every declared original-JavaScript boundary cell.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, TypedProgramLinksEmission,
    create_number_bitwise_program_links, decode_link_network_binary, emit_typed_program_links,
    encode_link_network_binary, read_structured_data_lino, self_translate,
    self_translation_signatures,
};
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

fn emission(value: TypedProgramLinksEmission) -> Value {
    json!({"code":value.code,"language":value.language,"mappings":value.mappings,
        "assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}

fn integer_bits(values: &Value) -> Vec<String> {
    values
        .as_array()
        .unwrap()
        .iter()
        .map(|value| {
            let text = value.as_str().unwrap();
            assert_eq!(text.len(), 16);
            let bits = u64::from_str_radix(text, 16).unwrap();
            format!("0x{bits:016x}u64")
        })
        .collect()
}

#[test]
fn complete_number_bitwise_product_has_full_runtime_codec_and_definition_parity() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let rust = run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let version = rust.stdout.split_whitespace().nth(1).unwrap();
    let parts = version
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {version}; >=1.90 <1.100"
    );
    let oracle = run(Command::new("node").current_dir(root).args([
        "--max-old-space-size=352",
        "--max-semi-space-size=1",
        "experiments/number-bitwise/oracle.mjs",
    ]));
    assert!(oracle.ok, "{}", oracle.stderr);
    let expected = read_structured_data_lino(&oracle.stdout).unwrap();
    let code = check_translation(&expected);
    check_definition_archives(&expected);
    execute_product(&expected, &code);
}

fn check_translation(expected: &Value) -> String {
    let source = expected["source"].as_str().unwrap();
    let actual = self_translate(source, "JavaScript", "Rust").unwrap();
    assert_eq!(
        json!({"sourceLanguage":actual.source_language,"targetLanguage":actual.target_language,
        "code":&actual.code,"items":actual.items.iter().map(|item| json!({
            "term":&item.term,"start":item.start,"end":item.end,"status":item.status,"reason":&item.reason,
        })).collect::<Vec<_>>()}),
        expected["translation"]
    );
    assert!(
        actual
            .items
            .iter()
            .all(|item| item.status == "translated" || item.status == "comment")
    );
    assert_eq!(
        serde_json::to_value(
            self_translation_signatures(source, "JavaScript", &Default::default()).unwrap()
        )
        .unwrap(),
        expected["signatures"]
    );
    assert_eq!(
        self_translate(&actual.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    assert_eq!(
        actual.code.matches("pub mod ml_number_bitwise {").count(),
        1
    );
    actual.code
}

fn check_definition_archives(expected: &Value) {
    let definition = create_number_bitwise_program_links().unwrap();
    assert_eq!(
        definition.root.as_u64(),
        expected["definition"]["root"].as_u64().unwrap()
    );
    assert_eq!(
        definition.network.to_lino(),
        expected["definition"]["links"].as_str().unwrap()
    );
    for key in ["program", "definition"] {
        let packet = &expected[key];
        let network = LinkNetwork::from_lino(packet["links"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(packet["root"].as_u64().unwrap());
        let binary = encode_link_network_binary(&network, BinaryLinksOptions::default()).unwrap();
        for loaded in [
            network,
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            assert_eq!(
                emission(emit_typed_program_links(&loaded, root, "Rust").unwrap()),
                packet["emission"]
            );
        }
    }
}

fn execute_product(expected: &Value, code: &str) {
    let inputs = integer_bits(&expected["values"]);
    let results = integer_bits(&expected["expected"]);
    assert_eq!(inputs.len(), 30);
    assert_eq!(results.len(), 6300);
    let input = inputs
        .iter()
        .map(|bits| format!("f64::from_bits({bits})"))
        .collect::<Vec<_>>()
        .join(",");
    let branches = (0..7)
        .map(|index| format!("{index}=>compute{index}(left,right),"))
        .collect::<String>();
    let main = format!(
        "\nfn main(){{let inputs=[{input}];let expected=[{}];let mut count=0;for operation in 0..7{{for left in inputs{{for right in inputs{{let actual=match operation{{{branches}_=>unreachable!(),}};assert_eq!(actual.to_bits(),expected[count],\"cell {{count}}\");count+=1;}}}}}}assert_eq!(count,6300);println!(\"{{count}}\");}}\n",
        results.join(",")
    );
    let directory = TemporaryDirectory::new("number-bitwise-product");
    let input = directory.0.join("product.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "product.exe"
    } else {
        "product"
    });
    fs::write(&input, code.to_owned() + &main).unwrap();
    let build = run(Command::new("rustc")
        .args(["--edition", "2024"])
        .arg(&input)
        .arg("-o")
        .arg(&binary));
    assert!(build.ok, "{}", build.stderr);
    let observed = run(&mut Command::new(&binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, "6300\n");
}
