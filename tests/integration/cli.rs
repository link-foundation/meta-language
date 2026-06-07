use std::process::Command;

use meta_language::{LinkNetwork, ParseConfiguration};

#[test]
fn describe_cli_reports_self_description_roots() {
    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .arg("describe")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("(link:"));
    assert!(stdout.contains("(reference:"));
    assert!(stdout.contains("(Type: Type Type)"));
    assert!(stdout
        .lines()
        .all(|line| line.starts_with('(') && line.ends_with(')')));

    let network = LinkNetwork::parse(&stdout, "LiNo", ParseConfiguration::default());
    assert_eq!(network.reconstruct_text(), stdout);
}

#[test]
fn verify_cli_reports_clean_lossless_text_region() {
    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .args(["verify", "--language", "plain-text", "--text", "alpha beta"])
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "clean");
}
