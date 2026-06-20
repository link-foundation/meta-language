use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use meta_language::{
    grammar_from_lino, grammar_to_lino, import_bnf, Grammar, GrammarFormat, GrammarRule,
    LinkNetwork, ParseConfiguration,
};

const ARITHMETIC_BNF: &str = include_str!("../fixtures/grammar/bnf/arithmetic.bnf");

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

#[test]
fn import_grammar_cli_imports_bnf_to_lino() {
    let fixture = fixture_path("tests/fixtures/grammar/bnf/arithmetic.bnf");
    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .args(["import-grammar", "--format", "bnf"])
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let grammar = grammar_from_lino(&stdout).expect("stdout is LiNo grammar");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Bnf));
    assert_eq!(grammar.start_rule().map(GrammarRule::name), Some("expr"));
    assert!(grammar.rule("digit").is_some());
}

#[test]
fn emit_grammar_cli_emits_bnf_from_lino_input() {
    let grammar = import_bnf(ARITHMETIC_BNF).expect("BNF fixture imports");
    let input = write_temp_file("emit-input", "lino", &grammar_to_lino(&grammar));
    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .args(["emit-grammar", "--format", "bnf"])
        .arg(&input)
        .output()
        .expect("failed to execute binary");

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("<expr> ::= <term>"));
    assert!(stdout.contains("<digit> ::= \"0\""));
}

#[test]
fn infer_cli_reads_example_directory_and_reports_metrics_to_stderr() {
    let examples = unique_temp_path("infer-examples");
    fs::create_dir_all(&examples).expect("create example directory");
    fs::write(examples.join("empty.txt"), "[]").expect("write example");
    fs::write(examples.join("one.txt"), "[a]").expect("write example");
    fs::write(examples.join("two.txt"), "[a,b]").expect("write example");

    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .arg("infer")
        .arg(&examples)
        .arg("--metrics")
        .output()
        .expect("failed to execute binary");

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let grammar = grammar_from_lino(&stdout).expect("stdout is LiNo grammar");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Inferred));
    assert!(!grammar.rules().is_empty());
    assert!(stderr.contains("precision="), "{stderr}");
    assert!(stderr.contains("recall="), "{stderr}");
    assert!(stderr.contains("f1="), "{stderr}");
    assert!(stderr.contains("runtime_ms="), "{stderr}");
}

#[test]
fn translate_grammar_cli_translates_rule_surface_to_target_language() {
    let input = write_temp_file(
        "translate-input",
        "lino",
        &grammar_to_lino(&concept_aligned_grammar()),
    );
    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .arg("translate-grammar")
        .arg(&input)
        .args(["--from-language", "en", "--to-language", "ru"])
        .output()
        .expect("failed to execute binary");

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let grammar = grammar_from_lino(&stdout).expect("stdout is LiNo grammar");

    assert_eq!(
        grammar.start(),
        Some("\u{0432}\u{044b}\u{0440}\u{0430}\u{0436}\u{0435}\u{043d}\u{0438}\u{0435}")
    );
    assert!(grammar.undefined_nonterminals().is_empty());
}

#[test]
fn emit_grammar_cli_reports_unsupported_formats_without_panic() {
    let grammar = import_bnf(ARITHMETIC_BNF).expect("BNF fixture imports");
    let input = write_temp_file("unsupported-input", "lino", &grammar_to_lino(&grammar));
    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .args(["emit-grammar", "--format", "antlr"])
        .arg(&input)
        .output()
        .expect("failed to execute binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unsupported output format: antlr"),
        "{stderr}"
    );
    assert!(!stderr.contains("panicked"), "{stderr}");
}

fn concept_aligned_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .source_format(GrammarFormat::Peg)
        .start("expression")
        .grammar_rule(
            GrammarRule::new("expression", expr.nt("term")).with_concept("grammar.expression"),
        )
        .grammar_rule(GrammarRule::new("term", expr.term("n")).with_concept("grammar.term"))
        .build()
}

fn fixture_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn write_temp_file(stem: &str, extension: &str, contents: &str) -> PathBuf {
    let path = unique_temp_path(stem).with_extension(extension);
    fs::write(&path, contents).expect("write temporary file");
    path
}

fn unique_temp_path(stem: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "meta-language-{stem}-{}-{nanos}",
        std::process::id()
    ))
}

fn assert_success(output: &std::process::Output) {
    assert!(
        output.status.success(),
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
