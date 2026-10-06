//! Issue #195 interchange mutation guards: every shared importer case is round
//! tripped through `check_grammar_round_trip`, which changes the grammar
//! before export and requires the change in the exported text and the
//! re-imported grammar; every malformed source of
//! `parity/fixtures/grammar-importers.json` is rejected; and two
//! importer/emitter pairs that a plain round trip cannot tell from correct
//! ones (mutually reversing sequences, re-emitting a stale source) are
//! reported as broken. Mirrors
//! `js/tests/issue-195-interchange-mutation-guards.test.js`.

use std::collections::BTreeSet;

use meta_language::{
    EmitReport, GRAMMAR_ROUND_TRIP_MARKER, Grammar, GrammarEmitError, GrammarExpr,
    GrammarImportError, GrammarParser, GrammarRoundTrip, GrammarRoundTripError,
    GrammarRoundTripFailureKind, GrammarRoundTripStage, GrammarRoundTripStatus,
    check_grammar_round_trip, emit_abnf, emit_bnf, emit_ebnf, emit_pest, emit_tree_sitter_json,
    import_abnf, import_bnf, import_ebnf, import_pest, import_tree_sitter_json,
};
use serde_json::Value;

use super::grammar_render::render_rule;
use super::issue_195_observations as observations;

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-importers.json");
const REQUIREMENT_ID: &str = "I195-INTERCHANGE-MUTATION-GUARDS";

type Importer = fn(&str) -> Result<Grammar, GrammarImportError>;
type Emitter = fn(&Grammar) -> Result<(String, EmitReport), GrammarEmitError>;

fn pair(format: &str) -> (Importer, Emitter) {
    match format {
        "abnf" => (import_abnf, emit_abnf),
        "bnf" => (import_bnf, emit_bnf),
        "ebnf" => (import_ebnf, emit_ebnf),
        "pest" => (import_pest, emit_pest),
        "tree-sitter-json" => (import_tree_sitter_json, emit_tree_sitter_json),
        other => panic!("unknown format {other}"),
    }
}

fn corpus() -> Value {
    serde_json::from_str(FIXTURE).expect("shared importer fixture is JSON")
}

fn strings(value: &Value, key: &str) -> Vec<String> {
    value[key]
        .as_array()
        .unwrap_or_else(|| panic!("has {key}"))
        .iter()
        .map(|item| item.as_str().expect("string entry").to_owned())
        .collect()
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("has {key}"))
}

fn accepts(grammar: &Grammar, text: &str) -> bool {
    GrammarParser::new(grammar.clone()).accepts(text)
}

fn record(assertion: &str, test_name: &str) {
    let fixture_id = format!(
        "planned:repository-directive:{}",
        REQUIREMENT_ID.to_lowercase()
    );
    observations::record(&observations::Observation {
        requirement_id: REQUIREMENT_ID,
        suffix: "behavior",
        fixture_id: &fixture_id,
        fixture_file: observations::GRAMMAR_IMPORTER_FIXTURE,
        assertions: &[assertion],
        test_name,
    });
}

/// A plain round trip, as in `issue_195_grammar_importers.rs`: import,
/// export, re-import and compare the declared rules.
fn plain_round_trip_agrees(case: &Value, round_trip: &GrammarRoundTrip<'_>) -> bool {
    let imported = (round_trip.import)(text(case, "source")).expect("source imports");
    let (exported, _) = (round_trip.emit)(&imported).expect("grammar emits");
    let reimported = (round_trip.import)(&exported).expect("export re-imports");
    strings(case, "rules")
        .iter()
        .all(|name| reimported.rule(name).map(render_rule) == imported.rule(name).map(render_rule))
}

fn reverse(expr: &GrammarExpr) -> GrammarExpr {
    let boxed = |inner: &GrammarExpr| Box::new(reverse(inner));
    match expr {
        GrammarExpr::Sequence(items) => {
            GrammarExpr::Sequence(items.iter().rev().map(reverse).collect())
        }
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives.iter().map(reverse).collect(),
        },
        GrammarExpr::Optional(inner) => GrammarExpr::Optional(boxed(inner)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::ZeroOrMore(boxed(inner)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::OneOrMore(boxed(inner)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: boxed(expr),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(inner) => GrammarExpr::And(boxed(inner)),
        GrammarExpr::Not(inner) => GrammarExpr::Not(boxed(inner)),
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: boxed(expr),
        },
        other => other.clone(),
    }
}

fn reverse_sequences(grammar: &Grammar) -> Grammar {
    let mut reversed = Grammar::new();
    for rule in grammar.rules() {
        let mut rule = rule.clone();
        rule.expr = reverse(&rule.expr);
        reversed.add_rule(rule);
    }
    if let Some(start) = grammar.start() {
        reversed.set_start(start);
    }
    if let Some(format) = grammar.source_format() {
        reversed.set_source_format(format);
    }
    reversed
}

#[test]
fn issue_195_mutated_grammars_survive_export_and_reimport_in_every_shared_format() {
    let corpus = corpus();
    let cases = corpus["cases"].as_array().expect("cases array");
    assert_eq!(cases.len(), 10);
    for case in cases {
        let id = text(case, "id");
        let (import, emit) = pair(text(case, "format"));
        let (accepted, rejected) = (strings(case, "accepts"), strings(case, "rejects"));
        let round_trip = GrammarRoundTrip::new(&import, &emit).with_samples(&accepted, &rejected);
        let report = check_grammar_round_trip(text(case, "source"), &round_trip)
            .unwrap_or_else(|error| panic!("{id} round trips: {error}"));
        assert_eq!(report.failures, Vec::new(), "{id}");
        assert_eq!(report.status, GrammarRoundTripStatus::Preserved, "{id}");
        assert_eq!(report.status.as_str(), "preserved", "{id}");
        // The recorded emission of the unmutated grammar is an independent
        // reference: the mutated export must differ from it and carry the
        // marker.
        assert_ne!(report.exported, text(&case["emitted"], "source"), "{id}");
        assert!(report.exported.contains(GRAMMAR_ROUND_TRIP_MARKER), "{id}");
        let imported = import(text(case, "source")).expect("source imports");
        assert!(!accepts(&imported, GRAMMAR_ROUND_TRIP_MARKER), "{id}");
        assert!(
            accepts(&report.reimported, GRAMMAR_ROUND_TRIP_MARKER),
            "{id}"
        );
        assert_eq!(
            report
                .reimported
                .start_rule()
                .map(|rule| rule.name.as_str()),
            Some(text(case, "start")),
            "{id}"
        );
    }
    record(
        "mutationVisibleAfterRoundTrip",
        "issue_195_mutated_grammars_survive_export_and_reimport_in_every_shared_format",
    );
}

#[test]
fn issue_195_malformed_grammars_and_negative_samples_are_rejected() {
    let corpus = corpus();
    let malformed = corpus["malformed"].as_array().expect("malformed array");
    let formats = malformed
        .iter()
        .map(|entry| text(entry, "format"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        formats,
        BTreeSet::from(["abnf", "bnf", "ebnf", "pest", "tree-sitter-json"])
    );
    for entry in malformed {
        let (format, reason) = (text(entry, "format"), text(entry, "reason"));
        let (import, emit) = pair(format);
        let error = check_grammar_round_trip(
            text(entry, "source"),
            &GrammarRoundTrip::new(&import, &emit),
        )
        .expect_err("malformed source is rejected");
        assert!(
            matches!(error, GrammarRoundTripError::Import(_)),
            "{format} rejects a source with {reason}: {error}"
        );
    }
    for case in corpus["cases"].as_array().expect("cases array") {
        let id = text(case, "id");
        let (import, emit) = pair(text(case, "format"));
        let rejected = strings(case, "rejects");
        assert!(!rejected.is_empty(), "{id}");
        let report =
            check_grammar_round_trip(text(case, "source"), &GrammarRoundTrip::new(&import, &emit))
                .unwrap_or_else(|error| panic!("{id} round trips: {error}"));
        let imported = import(text(case, "source")).expect("source imports");
        for sample in &rejected {
            assert!(!accepts(&imported, sample), "{id} rejects {sample:?}");
            assert!(
                !accepts(&report.reimported, sample),
                "{id} re-import rejects {sample:?}"
            );
        }
    }
    record(
        "malformedGrammarsRejected",
        "issue_195_malformed_grammars_and_negative_samples_are_rejected",
    );
}

#[test]
fn issue_195_mutually_wrong_importer_and_exporter_pairs_are_detected() {
    let corpus = corpus();
    for case in corpus["cases"].as_array().expect("cases array") {
        let id = text(case, "id");
        let source = text(case, "source");
        let (import, emit) = pair(text(case, "format"));
        let (accepted, rejected) = (strings(case, "accepts"), strings(case, "rejects"));

        // Both halves reverse every sequence, so each undoes the other's error.
        let reversing_import = |text: &str| import(text).map(|grammar| reverse_sequences(&grammar));
        let reversing_emit = |grammar: &Grammar| emit(&reverse_sequences(grammar));
        let reversing = GrammarRoundTrip::new(&reversing_import, &reversing_emit)
            .with_samples(&accepted, &rejected);
        assert!(
            plain_round_trip_agrees(case, &reversing),
            "{id} fools a plain round trip"
        );
        let report = check_grammar_round_trip(source, &reversing).expect("guard runs");
        assert_eq!(report.status, GrammarRoundTripStatus::Broken, "{id}");
        assert!(
            report.failures.iter().any(|failure| {
                failure.kind == GrammarRoundTripFailureKind::SampleRejected
                    && failure.stage == GrammarRoundTripStage::Imported
            }),
            "{id}"
        );

        // The exporter prints the source it was first given, whatever it
        // receives.
        let stale_emit = |_: &Grammar| -> Result<(String, EmitReport), GrammarEmitError> {
            Ok((source.to_owned(), EmitReport::default()))
        };
        let stale = GrammarRoundTrip::new(&import, &stale_emit).with_samples(&accepted, &rejected);
        assert!(
            plain_round_trip_agrees(case, &stale),
            "{id} fools a plain round trip"
        );
        let report = check_grammar_round_trip(source, &stale).expect("guard runs");
        assert_eq!(report.status, GrammarRoundTripStatus::Broken, "{id}");
        let kinds = report
            .failures
            .iter()
            .map(|failure| failure.kind.as_str())
            .collect::<BTreeSet<_>>();
        for kind in [
            "mutation-not-exported",
            "mutation-not-visible",
            "rules-changed",
        ] {
            assert!(kinds.contains(kind), "{id} reports {kind}");
        }
    }
    record(
        "mutuallyWrongPairDetected",
        "issue_195_mutually_wrong_importer_and_exporter_pairs_are_detected",
    );
}
