//! Shared BNF/EBNF literal quoting corpus: both runtimes must import the same
//! quote and escape spellings, and emit terminals their own importers re-read.

use meta_language::{emit_bnf, emit_ebnf, import_bnf, import_ebnf, Grammar, GrammarImportError};
use serde_json::Value;

use super::grammar_render::render_rule;

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-literal-quoting.json");

fn import(format: &str, source: &str) -> Result<Grammar, GrammarImportError> {
    match format {
        "bnf" => import_bnf(source),
        "ebnf" => import_ebnf(source),
        unexpected => panic!("unexpected literal quoting format {unexpected}"),
    }
}

fn field<'case>(case: &'case Value, key: &str) -> &'case str {
    case[key]
        .as_str()
        .unwrap_or_else(|| panic!("case field {key}"))
}

#[test]
fn shared_literal_quoting_imports_match_javascript() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture parses");
    for case in fixture["imports"].as_array().expect("import cases") {
        let id = field(case, "id");
        let result = import(field(case, "format"), field(case, "source"));
        if case.get("error").is_some() {
            assert!(result.is_err(), "{id} should be rejected, got {result:?}");
            continue;
        }
        let grammar = result.unwrap_or_else(|error| panic!("{id} imports: {error}"));
        let rule = grammar.rule("a").expect("rule a exists");
        assert_eq!(render_rule(rule), field(case, "rule"), "{id}");
    }
}

#[test]
fn shared_literal_quoting_emits_reimport_in_the_same_format() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture parses");
    for case in fixture["emits"].as_array().expect("emit cases") {
        let id = field(case, "id");
        let format = field(case, "format");
        let grammar = Grammar::builder()
            .start("a")
            .rule("a", Grammar::expr().term(field(case, "value")))
            .build();
        let (source, report) = match format {
            "bnf" => emit_bnf(&grammar),
            "ebnf" => emit_ebnf(&grammar),
            unexpected => panic!("unexpected literal quoting format {unexpected}"),
        }
        .unwrap_or_else(|error| panic!("{id} emits: {error}"));
        assert_eq!(source, field(case, "source"), "{id} source");
        let lossy = case["lossy"]
            .as_array()
            .expect("lossy notes")
            .iter()
            .map(|note| note.as_str().expect("lossy note").to_string())
            .collect::<Vec<_>>();
        assert_eq!(report.lossy, lossy, "{id} lossy notes");
        let reimported = import(format, &source).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            render_rule(reimported.rule("a").expect("rule a exists")),
            field(case, "reimported"),
            "{id} re-import"
        );
    }
}
