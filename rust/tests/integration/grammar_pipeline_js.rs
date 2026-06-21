use meta_language::emit_javascript_parser;

use super::grammar_pipeline_support::{
    infer_valid_grammar_from_strings, load_fixture_corpus, run_javascript_parser, ExternalRun,
};

#[test]
fn infer_emit_js_reparses_examples() {
    let examples = load_fixture_corpus("csv-row");
    let grammar = infer_valid_grammar_from_strings(&examples);
    let (artifacts, _report) =
        emit_javascript_parser(&grammar).expect("JavaScript parser codegen emits");

    assert!(!artifacts.peggy_grammar.trim().is_empty());
    assert!(artifacts.module.contains("peggy.generate"));

    match run_javascript_parser(&artifacts.module, &examples, Some("a,,"))
        .expect("generated JavaScript parser runner succeeds")
    {
        ExternalRun::Ran => {}
        ExternalRun::Skipped(reason) => eprintln!("skipping JavaScript parser execution: {reason}"),
    }
}
