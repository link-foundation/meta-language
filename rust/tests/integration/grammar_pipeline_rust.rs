use meta_language::{emit_rust_parser, import_pest, GrammarFormat, GrammarRule};

use super::grammar_pipeline_support::{
    assert_runtime_accepts_all_strings, assert_runtime_rejects, compile_and_run_rust_parser,
    infer_valid_grammar_from_strings, load_fixture_corpus,
};

#[test]
fn infer_emit_rust_reparses_examples() {
    let examples = load_fixture_corpus("arith");
    let grammar = infer_valid_grammar_from_strings(&examples);
    let (artifacts, report) = emit_rust_parser(&grammar).expect("Rust parser codegen emits");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Inferred));
    assert!(!artifacts.pest_grammar.trim().is_empty());
    assert!(!artifacts.parser_struct.trim().is_empty());
    assert!(!artifacts.ast_types.trim().is_empty());
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("unordered choice")));
    pest_meta::parse_and_optimize(&artifacts.pest_grammar).expect("pest grammar validates");

    let emitted_grammar = import_pest(&artifacts.pest_grammar).expect("emitted pest imports");
    assert_runtime_accepts_all_strings(&emitted_grammar, &examples);
    assert_runtime_rejects(&emitted_grammar, "1++");

    let start_rule = grammar
        .start_rule()
        .map(GrammarRule::name)
        .expect("inferred grammar has a start rule");
    compile_and_run_rust_parser(&artifacts, start_rule, &examples, Some("1++"))
        .expect("generated Rust parser compiles and runs");
}
