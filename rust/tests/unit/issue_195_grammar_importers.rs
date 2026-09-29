//! Issue #195 grammar importer evidence: every shared importer case is
//! imported through the public API, its declared rules match the
//! runtime-neutral rendering recorded in `parity/fixtures/grammar-importers.json`,
//! the generated Rust parser compiles and runs the corpus, the same-format
//! emitter reproduces the recorded text, and both re-import and links-notation
//! serialization preserve the grammar. Mirrors
//! `js/tests/issue-195-grammar-importers.test.js`.

use std::collections::BTreeMap;

use meta_language::{
    EmitReport, Grammar, GrammarEmitError, GrammarImportError, GrammarParser, emit_abnf, emit_bnf,
    emit_ebnf, emit_pest, emit_rust_parser, emit_tree_sitter_json, grammar_from_lino,
    grammar_to_lino, import_abnf, import_bnf, import_ebnf, import_pest, import_tree_sitter_json,
};
use serde_json::Value;

use super::grammar_pipeline_support::compile_and_run_rust_parser_on_corpus;
use super::grammar_render::render_rule;
use super::issue_195_observations as observations;

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-importers.json");

const ASSERTIONS: &[&str] = &[
    "ordinaryPublicImportApi",
    "grammarExpressionsPreserved",
    "generatedParserExecuted",
    "generatedEmitterExecuted",
    "independentCorpusParses",
    "roundTripPreservesGrammar",
];

type Importer = fn(&str) -> Result<Grammar, GrammarImportError>;
type Emitter = fn(&Grammar) -> Result<(String, EmitReport), GrammarEmitError>;

fn strings(case: &Value, key: &str) -> Vec<String> {
    case[key]
        .as_array()
        .unwrap_or_else(|| panic!("case has {key}"))
        .iter()
        .map(|value| value.as_str().expect("string entry").to_owned())
        .collect()
}

fn declared_rules(grammar: &Grammar, rules: &[String]) -> BTreeMap<String, String> {
    rules
        .iter()
        .map(|name| {
            let rule = grammar
                .rule(name)
                .unwrap_or_else(|| panic!("rule {name} is declared"));
            (name.clone(), render_rule(rule))
        })
        .collect()
}

fn assert_membership(id: &str, grammar: &Grammar, accepts: &[String], rejects: &[String]) {
    let parser = GrammarParser::new(grammar.clone());
    for source in accepts {
        assert!(parser.accepts(source), "{id} accepts {source:?}");
    }
    for source in rejects {
        assert!(!parser.accepts(source), "{id} rejects {source:?}");
    }
}

fn check_importer(format: &str, importer: &str, import: Importer, emit: Emitter) {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("shared importer fixture is JSON");
    let cases = fixture["cases"]
        .as_array()
        .expect("cases array")
        .iter()
        .filter(|case| case["format"] == format)
        .collect::<Vec<_>>();
    assert!(cases.len() >= 2, "{format} has message and construct cases");

    for case in cases {
        let id = case["id"].as_str().expect("case id");
        let start = case["start"].as_str().expect("start rule");
        let rules = strings(case, "rules");
        let accepts = strings(case, "accepts");
        let rejects = strings(case, "rejects");
        let expressions = case["expressions"]
            .as_object()
            .expect("expressions")
            .iter()
            .map(|(name, value)| (name.clone(), value.as_str().expect("rendering").to_owned()))
            .collect::<BTreeMap<_, _>>();

        let grammar = import(case["source"].as_str().expect("source"))
            .unwrap_or_else(|error| panic!("{id} imports: {error}"));
        assert_eq!(
            grammar.start_rule().map(|rule| rule.name.as_str()),
            Some(start),
            "{id}"
        );
        assert!(grammar.undefined_nonterminals().is_empty(), "{id}");
        assert_eq!(declared_rules(&grammar, &rules), expressions, "{id}");
        assert_membership(id, &grammar, &accepts, &rejects);

        let (artifacts, _report) = emit_rust_parser(&grammar).expect("Rust parser emits");
        compile_and_run_rust_parser_on_corpus(&artifacts, start, &accepts, &rejects)
            .unwrap_or_else(|error| panic!("{id} generated Rust parser: {error}"));

        let (emitted, report) =
            emit(&grammar).unwrap_or_else(|error| panic!("{id} emits: {error}"));
        assert_eq!(
            emitted,
            case["emitted"]["source"].as_str().expect("emitted source"),
            "{id}"
        );
        assert_eq!(report.lossy, strings(&case["emitted"], "lossy"), "{id}");

        // Emission is idempotent from the first re-import on: ABNF appends the
        // referenced core rules once, every other format is a fixpoint.
        let reimported =
            import(&emitted).unwrap_or_else(|error| panic!("{id} re-imports: {error}"));
        assert_eq!(
            reimported.start_rule().map(|rule| rule.name.as_str()),
            Some(start),
            "{id}"
        );
        assert_eq!(declared_rules(&reimported, &rules), expressions, "{id}");
        let (reemitted, _) = emit(&reimported).expect("re-imported grammar emits");
        let (stable, _) = emit(&import(&reemitted).expect("re-emitted text imports"))
            .expect("stable grammar emits");
        assert_eq!(stable, reemitted, "{id}");
        if format != "abnf" {
            assert_eq!(reemitted, emitted, "{id}");
        }
        assert_membership(id, &reimported, &accepts, &rejects);

        let restored = grammar_from_lino(&grammar_to_lino(&grammar)).expect("LiNo round trip");
        assert_eq!(restored, grammar, "{id}");
    }

    let requirement_id = format!("I195-IMPORT-{}", observations::slug(importer));
    let fixture_id = format!("planned:grammar-importer:{importer}");
    observations::record(&observations::Observation {
        requirement_id: &requirement_id,
        suffix: "positive",
        fixture_id: &fixture_id,
        fixture_file: observations::GRAMMAR_IMPORTER_FIXTURE,
        assertions: ASSERTIONS,
        test_name: &format!("issue_195_{}_importer_cases", observations::slug(importer)),
    });
}

#[test]
fn issue_195_abnf_importer_cases_import_generate_emit_and_round_trip() {
    check_importer("abnf", "ABNF", import_abnf, emit_abnf);
}

#[test]
fn issue_195_bnf_importer_cases_import_generate_emit_and_round_trip() {
    check_importer("bnf", "BNF", import_bnf, emit_bnf);
}

#[test]
fn issue_195_ebnf_importer_cases_import_generate_emit_and_round_trip() {
    check_importer("ebnf", "EBNF", import_ebnf, emit_ebnf);
}

#[test]
fn issue_195_pest_importer_cases_import_generate_emit_and_round_trip() {
    check_importer("pest", "pest", import_pest, emit_pest);
}

#[test]
fn issue_195_tree_sitter_json_importer_cases_import_generate_emit_and_round_trip() {
    check_importer(
        "tree-sitter-json",
        "tree-sitter grammar JSON",
        import_tree_sitter_json,
        emit_tree_sitter_json,
    );
}
