//! Issue #195 interchange reverse conversion: for one commented, irregularly
//! formatted source per format in `parity/fixtures/grammar-importers.json`
//! (reverse section), source grammar -> native links -> exported grammar ->
//! native links keeps every rule, kind, definition and documentation while the
//! emitter only sees the grammar decoded from the links; the re-imported
//! grammar accepts and rejects independent samples like the source; and the
//! lossless mode reconstructs the source byte for byte from the grammar links
//! and the layout links alone, keeping every untouched definition and comment
//! when a rule is changed or renamed. Emitters that swap a sequence or drop
//! documentation are reported as different. Mirrors
//! `js/tests/issue-195-interchange-reverse-conversion.test.js`.

use meta_language::{
    EmitReport, GRAMMAR_LOSSLESS_FORMATS, GRAMMAR_ROUND_TRIP_MARKER, Grammar, GrammarEmitError,
    GrammarExpr, GrammarLosslessError, GrammarParser, GrammarReverseConversion,
    GrammarReverseFailureKind, GrammarReverseStatus, GrammarRule, check_grammar_reverse_conversion,
    emit_grammar_lossless, grammar_emitter, grammar_importer, import_grammar_lossless,
    mutate_grammar_start_rule, parse_grammar_layout_links, parse_grammar_links,
    rename_grammar_rule, render_grammar_layout_links, render_grammar_links,
};
use serde_json::Value;

use super::issue_195_observations as observations;

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-importers.json");
const REQUIREMENT_ID: &str = "I195-INTERCHANGE-REVERSE-CONVERSION";

fn reverse_entries() -> Vec<Value> {
    let corpus: Value = serde_json::from_str(FIXTURE).expect("shared importer fixture is JSON");
    corpus["reverse"].as_array().expect("reverse array").clone()
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

type BrokenEmitter = dyn Fn(&Grammar) -> Result<(String, EmitReport), GrammarEmitError>;

/// Rebuilds `grammar` from its rules after `change`, keeping the start rule
/// and source format.
fn rebuild(grammar: &Grammar, change: impl Fn(&mut GrammarRule)) -> Grammar {
    let mut rebuilt = Grammar::new();
    for rule in grammar.rules() {
        let mut rule = rule.clone();
        change(&mut rule);
        rebuilt.add_rule(rule);
    }
    if let Some(start) = grammar.start() {
        rebuilt.set_start(start);
    }
    if let Some(format) = grammar.source_format() {
        rebuilt.set_source_format(format);
    }
    rebuilt
}

#[test]
fn issue_195_grammars_export_from_their_native_links_without_the_source_in_every_format() {
    let entries = reverse_entries();
    let formats = entries
        .iter()
        .map(|entry| text(entry, "format"))
        .collect::<Vec<_>>();
    assert_eq!(formats, GRAMMAR_LOSSLESS_FORMATS);
    for entry in &entries {
        let (id, format) = (text(entry, "id"), text(entry, "format"));
        let import = grammar_importer(format).expect("importer");
        let emit = grammar_emitter(format).expect("emitter");
        let (accepted, rejected) = (strings(entry, "accepts"), strings(entry, "rejects"));
        let conversion =
            GrammarReverseConversion::new(&import, &emit).with_samples(&accepted, &rejected);
        let report = check_grammar_reverse_conversion(text(entry, "source"), &conversion)
            .unwrap_or_else(|error| panic!("{id} converts back: {error}"));
        assert_eq!(report.failures, Vec::new(), "{id}");
        assert_eq!(report.status, GrammarReverseStatus::Equivalent, "{id}");
        assert_eq!(report.status.as_str(), "equivalent", "{id}");
        assert_eq!(report.links, text(entry, "links"), "{id}");
        assert_eq!(report.exported, text(entry, "exported"), "{id}");
        // The links alone, with the source out of reach, give the same export.
        let decoded = parse_grammar_links(text(entry, "links")).expect("links parse");
        assert_eq!(render_grammar_links(&decoded), text(entry, "links"), "{id}");
        let (exported, _) = emit(&decoded).expect("decoded grammar emits");
        assert_eq!(exported, text(entry, "exported"), "{id}");
        assert_ne!(exported, text(entry, "source"), "{id}");
        if ["antlr", "gbnf", "lark"].contains(&format) {
            assert!(text(entry, "links").contains("(doc "), "{id}");
        }
    }
    // EBNF comments are skipped outside string literals only.
    let import_ebnf = grammar_importer("ebnf").expect("importer");
    assert!(import_ebnf("a = \"x\" ; (* open").is_err());
    let quoted = import_ebnf("a = \"(*\" ;").expect("a quoted comment opener imports");
    assert!(accepts(&quoted, "(*"));
    assert!(parse_grammar_links("(grammar (start missing))\n(rule word normal any)\n").is_err());
    assert!(parse_grammar_links("(grammar)\n(rule word normal (bogus))\n").is_err());
    record(
        "exportWithoutOriginalSource",
        "issue_195_grammars_export_from_their_native_links_without_the_source_in_every_format",
    );
}

#[test]
fn issue_195_reimported_exports_are_structurally_and_semantically_equivalent_to_the_source() {
    for entry in &reverse_entries() {
        let (id, format) = (text(entry, "id"), text(entry, "format"));
        let import = grammar_importer(format).expect("importer");
        let emit = grammar_emitter(format).expect("emitter");
        let (accepted, rejected) = (strings(entry, "accepts"), strings(entry, "rejects"));
        let imported = import(text(entry, "source")).expect("source imports");
        let reimported = import(text(entry, "exported")).expect("export re-imports");
        assert_eq!(
            render_grammar_links(&reimported),
            text(entry, "reimportedLinks"),
            "{id}"
        );
        assert_eq!(reimported.rule_names(), imported.rule_names(), "{id}");
        assert_eq!(
            reimported.start_rule().map(|rule| rule.name.clone()),
            imported.start_rule().map(|rule| rule.name.clone()),
            "{id}"
        );
        for sample in &accepted {
            assert!(accepts(&reimported, sample), "{id} accepts {sample}");
        }
        for sample in &rejected {
            assert!(!accepts(&reimported, sample), "{id} rejects {sample}");
        }

        // An emitter that swaps every top-level sequence or drops the rule
        // documentation is reported as different.
        let documented = text(entry, "links").contains("(doc ");
        let swapped = move |grammar: &Grammar| -> Result<(String, EmitReport), GrammarEmitError> {
            emit(&rebuild(grammar, |rule| {
                if let GrammarExpr::Sequence(items) = &rule.expr {
                    rule.expr = GrammarExpr::Sequence(items.iter().rev().cloned().collect());
                }
            }))
        };
        let undocumented =
            move |grammar: &Grammar| -> Result<(String, EmitReport), GrammarEmitError> {
                emit(&rebuild(grammar, |rule| rule.doc = None))
            };
        let broken: [(GrammarReverseFailureKind, &BrokenEmitter); 2] = [
            (GrammarReverseFailureKind::RulesChanged, &swapped),
            (GrammarReverseFailureKind::DocChanged, &undocumented),
        ];
        for (kind, emit_broken) in broken {
            let conversion = GrammarReverseConversion::new(&import, emit_broken)
                .with_samples(&accepted, &rejected);
            let report = check_grammar_reverse_conversion(text(entry, "source"), &conversion)
                .expect("broken pair still converts");
            if kind == GrammarReverseFailureKind::DocChanged && !documented {
                assert_eq!(report.status, GrammarReverseStatus::Equivalent, "{id}");
                continue;
            }
            assert_eq!(report.status, GrammarReverseStatus::Different, "{id}");
            assert!(
                report.failures.iter().any(|failure| failure.kind == kind),
                "{id} {}",
                kind.as_str()
            );
        }
    }
    record(
        "reimportEquivalent",
        "issue_195_reimported_exports_are_structurally_and_semantically_equivalent_to_the_source",
    );
}

#[test]
fn issue_195_lossless_mode_reconstructs_every_source_exactly_from_links() {
    for entry in &reverse_entries() {
        let (id, format) = (text(entry, "id"), text(entry, "format"));
        let import = grammar_importer(format).expect("importer");
        let (grammar, layout) =
            import_grammar_lossless(text(entry, "source"), format).expect("lossless import");
        assert_eq!(render_grammar_links(&grammar), text(entry, "links"), "{id}");
        assert_eq!(
            render_grammar_layout_links(&layout),
            text(entry, "layout"),
            "{id}"
        );
        let decoded_layout = parse_grammar_layout_links(text(entry, "layout")).expect("layout");
        assert_eq!(decoded_layout, layout, "{id}");
        let decoded = parse_grammar_links(text(entry, "links")).expect("links parse");
        let (exact, report) = emit_grammar_lossless(&decoded, &decoded_layout).expect("emits");
        assert_eq!(exact, text(entry, "source"), "{id}");
        assert_eq!(report.lossy, Vec::<String>::new(), "{id}");

        let renamed = &entry["renamed"];
        let edits = [
            (
                mutate_grammar_start_rule(&decoded, GRAMMAR_ROUND_TRIP_MARKER).expect("mutates"),
                &entry["mutated"],
            ),
            (
                rename_grammar_rule(
                    &decoded,
                    text(renamed, "from"),
                    text(renamed, "to"),
                    None,
                    &[],
                )
                .expect("renames")
                .grammar,
                renamed,
            ),
        ];
        for (edited, expected) in &edits {
            assert_eq!(
                render_grammar_links(edited),
                text(expected, "links"),
                "{id}"
            );
            let (source, report) = emit_grammar_lossless(edited, &decoded_layout).expect("emits");
            assert_eq!(source, text(expected, "source"), "{id}");
            assert_eq!(report.lossy, strings(expected, "lossy"), "{id}");
            let reimported = import(&source).expect("lossless output re-imports");
            assert_eq!(
                render_grammar_links(&reimported),
                text(expected, "links"),
                "{id}"
            );
            // Every definition the edit left alone keeps its original text.
            let edited_links = render_grammar_links(edited);
            let lines = edited_links.lines().collect::<Vec<_>>();
            let untouched = layout
                .members
                .iter()
                .filter(|member| lines.contains(&member.fingerprint.as_str()))
                .collect::<Vec<_>>();
            assert!(
                !untouched.is_empty() && untouched.len() < layout.members.len(),
                "{id}"
            );
            for member in untouched {
                assert!(source.contains(&member.text), "{id} keeps {}", member.name);
            }
        }
    }
    assert!(matches!(
        import_grammar_lossless("x", "native"),
        Err(GrammarLosslessError::UnsupportedFormat(_))
    ));
    assert!(parse_grammar_layout_links("(layout native %)\n").is_err());
    record(
        "losslessModeExact",
        "issue_195_lossless_mode_reconstructs_every_source_exactly_from_links",
    );
}
