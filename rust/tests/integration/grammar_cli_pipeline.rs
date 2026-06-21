use std::process::Command;

use meta_language::{import_gbnf, GrammarFormat};

use super::grammar_pipeline_support::{
    assert_runtime_accepts_all_strings, load_fixture_corpus, unique_temp_path,
    write_example_directory,
};

#[test]
fn cli_infer_then_emit_grammar() {
    let examples = load_fixture_corpus("csv-row");
    let examples_dir = write_example_directory("cli-grammar-corpus", &examples);
    let inferred_path = unique_temp_path("cli-inferred-grammar").with_extension("lino");

    let infer = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .arg("infer")
        .arg(&examples_dir)
        .arg("--out")
        .arg(&inferred_path)
        .output()
        .expect("failed to run infer command");
    assert_success(&infer);

    let emit = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .args(["emit-grammar", "--format", "gbnf"])
        .arg(&inferred_path)
        .output()
        .expect("failed to run emit-grammar command");
    assert_success(&emit);

    let stdout = String::from_utf8_lossy(&emit.stdout);
    assert!(!stdout.trim().is_empty());
    let grammar = import_gbnf(&stdout).expect("CLI-emitted GBNF imports");
    assert_eq!(grammar.source_format(), Some(GrammarFormat::Gbnf));
    assert_runtime_accepts_all_strings(&grammar, &examples);
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
