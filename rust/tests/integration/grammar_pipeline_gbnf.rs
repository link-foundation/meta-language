use meta_language::{
    emit_gbnf, import_gbnf, infer_cfg, GrammarFormat, InferenceOptions, PositiveOnlyOracle,
};

use super::grammar_pipeline_support::{
    assert_runtime_accepts_all_strings, infer_valid_grammar_from_strings, load_fixture_corpus,
};

#[test]
fn infer_emit_gbnf_round_trips() {
    let examples = load_fixture_corpus("json-ish");
    let grammar = infer_valid_grammar_from_strings(&examples);
    let repeated = infer_cfg(
        &examples,
        &PositiveOnlyOracle::new(),
        InferenceOptions::default(),
    );
    assert_eq!(grammar, repeated.grammar);

    let (gbnf, report) = emit_gbnf(&grammar).expect("GBNF emits");
    assert!(!gbnf.trim().is_empty());
    assert!(
        report
            .lossy
            .iter()
            .all(|note| note.contains("renamed rule")),
        "{report:#?}"
    );

    let reparsed = import_gbnf(&gbnf).expect("emitted GBNF re-imports");
    assert_eq!(reparsed.source_format(), Some(GrammarFormat::Gbnf));
    assert!(reparsed.undefined_nonterminals().is_empty());
    assert_runtime_accepts_all_strings(&reparsed, &examples);

    let (re_emitted, _) = emit_gbnf(&reparsed).expect("re-imported GBNF re-emits");
    assert_eq!(gbnf, re_emitted);
}
