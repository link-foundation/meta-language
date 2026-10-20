//! CI-only consumers of the exact JavaScript-produced TypeScript program graphs.
//! Both real native-package emitters execute against independent expected lines.
use meta_language::{
    LinkId, LinkNetwork, decode_typed_program_links, emit_typed_program_links,
    read_structured_data_lino,
};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

fn check_type_script_graph(index: usize) {
    assert_eq!(std::env::var("CI").as_deref(), Ok("true"));
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let manifest = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/typescript-typed-graph/manifest.lino"
    ))
    .unwrap();
    let cases = manifest["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 4);
    let case = &cases[index];
    let source = fs::read_to_string(repository.join(case["path"].as_str().unwrap())).unwrap();
    let digest = format!("{:x}", Sha256::digest(source.as_bytes()));
    assert_eq!(case["sha256"].as_str(), Some(digest.as_str()));
    let network = LinkNetwork::from_lino(&source).unwrap();
    let complete_graph = network.to_lino();
    assert_eq!(
        complete_graph.trim_end_matches('\n'),
        source.trim_end_matches('\n')
    );
    let root = LinkId::from_u64(case["root"].as_str().unwrap().parse().unwrap());
    decode_typed_program_links(&network, root).unwrap();
    let artifacts = repository.join("issue-195-artifacts/typescript-typed-graph-native");
    fs::create_dir_all(&artifacts).unwrap();
    let mut outputs = Vec::new();
    for (target, extension) in [("Rust", "rs"), ("JavaScript", "js")] {
        let code = emit_typed_program_links(&network, root, target)
            .unwrap()
            .code;
        let file = artifacts.join(format!("rust-runtime-case-{index}.{extension}"));
        fs::write(&file, &code).unwrap();
        outputs.push((target, file, code));
    }
    assert_eq!(network.to_lino(), complete_graph);
    let execution = Command::new("node")
        .arg(repository.join("experiments/typescript-typed-graph/run-rust-emissions.mjs"))
        .args([
            "--case",
            &index.to_string(),
            "--results-dir",
            "issue-195-results",
        ])
        .current_dir(repository)
        .output()
        .unwrap();
    let observations = read_structured_data_lino(
        &fs::read_to_string(artifacts.join(format!("rust-runtime-case-{index}.execution.lino")))
            .unwrap(),
    )
    .unwrap();
    let mut expected = String::new();
    for line in case["expectedLines"].as_array().unwrap() {
        for byte in line.as_str().unwrap().bytes().chain(std::iter::once(b'\n')) {
            write!(expected, "{byte:02x}").unwrap();
        }
    }
    let records = observations["observations"].as_array().unwrap();
    assert_eq!(records.len(), outputs.len());
    for ((target, file, code), observed) in outputs.iter().zip(records) {
        assert_eq!(observed["target"].as_str(), Some(*target));
        assert_eq!(fs::read_to_string(file).unwrap(), *code);
        let digest = format!("{:x}", Sha256::digest(code.as_bytes()));
        assert_eq!(observed["source"]["sha256"].as_str(), Some(digest.as_str()));
        assert_eq!(
            observed["execution"]["stdoutHex"].as_str(),
            Some(expected.as_str())
        );
        assert_eq!(
            observed["execution"]["exitCode"]
                .as_f64()
                .unwrap()
                .to_bits(),
            0.0_f64.to_bits()
        );
        if *target == "Rust" {
            assert_eq!(
                observed["compilation"]["exitCode"]
                    .as_f64()
                    .unwrap()
                    .to_bits(),
                0.0_f64.to_bits()
            );
        }
    }
    assert!(
        execution.status.success(),
        "native TypeScript graph execution: {}\n{}",
        String::from_utf8_lossy(&execution.stdout),
        String::from_utf8_lossy(&execution.stderr)
    );
}

#[test]
fn original_type_script_numeric_graph_executes_in_both_targets() {
    check_type_script_graph(0);
}

#[test]
fn original_type_script_alias_graph_executes_in_both_targets() {
    check_type_script_graph(1);
}

#[test]
fn original_type_script_utf16_graph_executes_in_both_targets() {
    check_type_script_graph(2);
}

#[test]
fn edited_type_script_graph_executes_in_both_targets() {
    check_type_script_graph(3);
}
