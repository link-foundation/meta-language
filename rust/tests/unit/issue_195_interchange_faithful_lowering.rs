//! Issue #195 interchange faithful lowering: for the grammars in
//! `parity/fixtures/grammar-importers.json` (lowering section), lowering into
//! every less expressive notation writes the recorded executable text, which
//! that notation's own importer reads back as the lowered grammar and which
//! accepts and rejects the samples like the original when every encoding is
//! exact; the recorded reconstruction metadata names every helper rule, its
//! construct, its encoding and its original expression, every rename, kind and
//! documentation step, and reconstructs the original from the executable; and
//! a package missing any step, or an emitter that writes the unlowered grammar,
//! is reported as broken instead of dropping the feature silently. Mirrors
//! `js/tests/issue-195-interchange-faithful-lowering.test.js`.

use meta_language::{
    EmitReport, GRAMMAR_LOWERING_FORMATS, Grammar, GrammarEmitError, GrammarFormat,
    GrammarLoweringEncoding, GrammarLoweringError, GrammarLoweringFailureKind,
    GrammarLoweringOptions, GrammarLoweringStatus, GrammarLoweringStep, GrammarParser,
    check_grammar_lowering, dropped_grammar_features, grammar_emitter, grammar_importer,
    lower_grammar, parse_grammar_links, parse_lowering_metadata, reconstruct_grammar,
    render_grammar_links, render_links_expression, render_lowering_metadata,
};
use serde_json::Value;

use super::issue_195_observations as observations;

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-importers.json");
const REQUIREMENT_ID: &str = "I195-INTERCHANGE-FAITHFUL-LOWERING";

fn lowering_entries() -> Vec<Value> {
    let corpus: Value = serde_json::from_str(FIXTURE).expect("shared importer fixture is JSON");
    corpus["lowering"]
        .as_array()
        .expect("lowering array")
        .clone()
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

/// One lowering target of a fixture grammar.
struct Case {
    label: String,
    grammar: Grammar,
    accepted: Vec<String>,
    rejected: Vec<String>,
    target: Value,
}

/// Every lowering target of every fixture grammar, with the parsed original.
fn targets() -> Vec<Case> {
    let mut targets = Vec::new();
    for entry in lowering_entries() {
        let grammar = parse_grammar_links(text(&entry, "links")).expect("fixture links parse");
        assert_eq!(render_grammar_links(&grammar), text(&entry, "links"));
        for target in entry["targets"].as_array().expect("targets") {
            targets.push(Case {
                label: format!("{} {}", text(&entry, "id"), text(target, "format")),
                grammar: grammar.clone(),
                accepted: strings(&entry, "accepts"),
                rejected: strings(&entry, "rejects"),
                target: target.clone(),
            });
        }
    }
    targets
}

/// The metadata lines of the steps that rewrite an original rule; a helper
/// lowered from another helper is restored together with its owner.
fn top_level_step_lines(metadata: &str) -> Vec<String> {
    let parsed = parse_lowering_metadata(metadata).expect("metadata parses");
    let lines = metadata.lines().collect::<Vec<_>>();
    parsed
        .steps
        .iter()
        .enumerate()
        .filter(|(_, step)| {
            !matches!(step, GrammarLoweringStep::Helper { owner, .. } if !parsed.order.contains(owner))
        })
        .map(|(index, _)| lines[4 + index].to_owned())
        .collect()
}

fn without_line(metadata: &str, line: &str) -> String {
    metadata
        .split('\n')
        .filter(|other| *other != line)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn issue_195_grammars_lower_into_an_executable_grammar_in_every_less_expressive_notation() {
    let entries = lowering_entries();
    let ids = entries
        .iter()
        .map(|entry| text(entry, "id"))
        .collect::<Vec<_>>();
    assert_eq!(ids, ["features:lowering", "lookahead:lowering"]);
    for entry in &entries {
        let formats = entry["targets"]
            .as_array()
            .expect("targets")
            .iter()
            .map(|target| text(target, "format"))
            .collect::<Vec<_>>();
        assert_eq!(formats, GRAMMAR_LOWERING_FORMATS);
    }
    let mut exact = Vec::new();
    for Case {
        label,
        grammar,
        accepted,
        rejected,
        target,
    } in &targets()
    {
        let format = text(target, "format");
        let lowering = lower_grammar(grammar, format, &GrammarLoweringOptions::default())
            .unwrap_or_else(|error| panic!("{label} lowers: {error}"));
        assert_eq!(lowering.status.as_str(), text(target, "status"), "{label}");
        assert_eq!(lowering.executable, text(target, "executable"), "{label}");
        assert_eq!(lowering.metadata, text(target, "metadata"), "{label}");
        assert_eq!(lowering.report.lossy, Vec::<String>::new(), "{label}");
        // The target's own importer reads the executable back as the lowered grammar.
        let import = grammar_importer(format).expect("importer");
        let imported = import(text(target, "executable")).expect("executable imports");
        assert_eq!(
            imported.rule_names(),
            lowering.grammar.rule_names(),
            "{label}"
        );
        assert_eq!(
            imported.start_rule().map(|rule| &rule.name),
            lowering.grammar.start_rule().map(|rule| &rule.name),
            "{label}"
        );
        if lowering.status == GrammarLoweringStatus::Exact {
            exact.push(label.clone());
            for sample in accepted {
                assert!(accepts(&imported, sample), "{label} accepts {sample}");
            }
            for sample in rejected {
                assert!(!accepts(&imported, sample), "{label} rejects {sample}");
            }
        }
        let options = GrammarLoweringOptions {
            accepts: accepted,
            rejects: rejected,
            ..GrammarLoweringOptions::default()
        };
        let report = check_grammar_lowering(grammar, format, &options).expect("checks");
        assert_eq!(report.failures, Vec::new(), "{label}");
        assert_eq!(report.status, lowering.status, "{label}");
    }
    // Every feature grammar lowering is exact; the lookahead grammar is exact
    // only in pest, which writes lookaheads and ordered choices itself.
    let mut expected = GRAMMAR_LOWERING_FORMATS
        .iter()
        .map(|format| format!("features:lowering {format}"))
        .collect::<Vec<_>>();
    expected.push("lookahead:lowering pest".to_owned());
    assert_eq!(exact, expected);
    let grammar = parse_grammar_links(text(&entries[0], "links")).expect("links parse");
    let options = GrammarLoweringOptions::default();
    assert_eq!(
        lower_grammar(&grammar, "cobol", &options).map(|lowering| lowering.executable),
        Err(GrammarLoweringError::UnsupportedFormat("cobol".to_owned()))
    );
    assert_eq!(
        lower_grammar(&Grammar::new(), "bnf", &options).map(|lowering| lowering.executable),
        Err(GrammarLoweringError::NoStartRule)
    );
    record(
        "loweringExecutable",
        "issue_195_grammars_lower_into_an_executable_grammar_in_every_less_expressive_notation",
    );
}

#[test]
fn issue_195_lowering_metadata_explicitly_reconstructs_the_original_grammar() {
    for Case {
        label,
        grammar,
        accepted,
        rejected,
        target,
    } in &targets()
    {
        let (format, source) = (text(target, "format"), text(target, "metadata"));
        let metadata = parse_lowering_metadata(source).expect("metadata parses");
        assert_eq!(render_lowering_metadata(&metadata), source, "{label}");
        assert_eq!(metadata.format, format, "{label}");
        assert_eq!(metadata.status.as_str(), text(target, "status"), "{label}");
        assert_eq!(metadata.source.map(GrammarFormat::as_str), Some("peg"));
        assert_eq!(
            Some(&metadata.start),
            grammar.start_rule().map(|rule| &rule.name)
        );
        assert_eq!(metadata.order, grammar.rule_names(), "{label}");
        // Every helper rule of the executable is a step naming its owner, its
        // construct, its encoding and the original expression it replaces.
        let import = grammar_importer(format).expect("importer");
        let executable = import(text(target, "executable")).expect("executable imports");
        let original_names = metadata
            .order
            .iter()
            .map(|name| {
                metadata
                    .steps
                    .iter()
                    .find_map(|step| match step {
                        GrammarLoweringStep::Rename { rule, value } if rule == name => {
                            Some(value.as_str())
                        }
                        _ => None,
                    })
                    .unwrap_or(name)
            })
            .collect::<Vec<_>>();
        let mut helpers = Vec::new();
        let mut approximate = false;
        for step in &metadata.steps {
            if let GrammarLoweringStep::Helper {
                helper,
                construct,
                encoding,
                note,
                original,
                ..
            } = step
            {
                assert!(
                    !construct.is_empty() && !note.is_empty(),
                    "{label} {helper}"
                );
                let original = format!(" {})\n", render_links_expression(original));
                assert!(source.contains(&original), "{label} {helper}");
                approximate |= *encoding == GrammarLoweringEncoding::Approximate;
                helpers.push(helper.as_str());
            }
        }
        let added = executable
            .rule_names()
            .into_iter()
            .filter(|name| !original_names.contains(name))
            .collect::<Vec<_>>();
        assert_eq!(added, helpers, "{label}");
        assert_eq!(
            metadata.status == GrammarLoweringStatus::Approximate,
            approximate,
            "{label}"
        );
        // The executable alone loses the features the steps carry; the package
        // (executable plus metadata) reconstructs every one of them.
        if !metadata.steps.is_empty() {
            let lost = dropped_grammar_features(grammar, &executable).expect("compares");
            assert!(!lost.is_empty(), "{label}");
        }
        let reconstructed =
            reconstruct_grammar(text(target, "executable"), source, None).expect("reconstructs");
        assert_eq!(
            render_grammar_links(&reconstructed),
            text(target, "reconstructed"),
            "{label}"
        );
        let dropped = dropped_grammar_features(grammar, &reconstructed).expect("compares");
        assert_eq!(dropped, Vec::<String>::new(), "{label}");
        let recorded = parse_grammar_links(text(target, "reconstructed")).expect("links parse");
        let dropped = dropped_grammar_features(grammar, &recorded).expect("compares");
        assert_eq!(dropped, Vec::<String>::new(), "{label}");
        if metadata.status == GrammarLoweringStatus::Exact {
            for sample in accepted {
                assert!(accepts(&reconstructed, sample), "{label} accepts {sample}");
            }
            for sample in rejected {
                assert!(!accepts(&reconstructed, sample), "{label} rejects {sample}");
            }
        }
    }
    // The metadata states every approximation: the BNF executable of the
    // lookahead grammar accepts x, which the original and its reconstruction reject.
    let lookahead = &lowering_entries()[1];
    let bnf = lookahead["targets"]
        .as_array()
        .expect("targets")
        .iter()
        .find(|target| text(target, "format") == "bnf")
        .expect("bnf target");
    let (executable, metadata) = (text(bnf, "executable"), text(bnf, "metadata"));
    let import = grammar_importer("bnf").expect("importer");
    assert!(accepts(&import(executable).expect("imports"), "x"));
    let reconstructed = reconstruct_grammar(executable, metadata, None).expect("reconstructs");
    assert!(!accepts(&reconstructed, "x"));
    let approximations = parse_lowering_metadata(metadata)
        .expect("metadata parses")
        .steps
        .into_iter()
        .filter_map(|step| match step {
            GrammarLoweringStep::Helper {
                construct,
                encoding: GrammarLoweringEncoding::Approximate,
                ..
            } => Some(construct),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(approximations, ["orderedChoice", "not", "any"]);
    for malformed in [
        metadata.replace("(lowering bnf approximate)", "(lowering bnf maybe)"),
        metadata.replace("(source peg)", "(source cobol)"),
        metadata.replace(" repeat0 exact ", " repeat0 lossless "),
        format!("{metadata}(kind list fancy)\n"),
        format!("{metadata}(swap list item)\n"),
        "(lowering bnf exact)\n(source)\n".to_owned(),
    ] {
        assert!(
            matches!(
                parse_lowering_metadata(&malformed),
                Err(GrammarLoweringError::Import(_))
            ),
            "{malformed}"
        );
    }
    assert_eq!(
        parse_lowering_metadata(&metadata.replace("(lowering bnf", "(lowering cobol")),
        Err(GrammarLoweringError::UnsupportedFormat("cobol".to_owned()))
    );
    record(
        "reconstructionMetadataExplicit",
        "issue_195_lowering_metadata_explicitly_reconstructs_the_original_grammar",
    );
}

#[test]
fn issue_195_lowering_never_drops_a_grammar_feature_silently() {
    let mut removed = 0;
    for Case {
        label,
        grammar,
        accepted,
        rejected,
        target,
    } in &targets()
    {
        // A package missing any step that rewrites an original rule is broken.
        for line in top_level_step_lines(text(target, "metadata")) {
            let edit = |metadata: &str| without_line(metadata, &line);
            let options = GrammarLoweringOptions {
                accepts: accepted,
                rejects: rejected,
                edit_metadata: Some(&edit),
                ..GrammarLoweringOptions::default()
            };
            let report =
                check_grammar_lowering(grammar, text(target, "format"), &options).expect("checks");
            assert_eq!(
                report.status,
                GrammarLoweringStatus::Broken,
                "{label} without {line}"
            );
            assert!(!report.failures.is_empty(), "{label} without {line}");
            assert!(
                report
                    .failures
                    .iter()
                    .all(|failure| failure.kind == GrammarLoweringFailureKind::FeatureDropped),
                "{label} without {line}"
            );
            removed += 1;
        }
    }
    assert_eq!(removed, 80);
    // The detail names the dropped feature.
    let features = &lowering_entries()[0];
    let grammar = parse_grammar_links(text(features, "links")).expect("links parse");
    let details = |format: &str, pattern: &str| {
        let edit = |metadata: &str| {
            metadata
                .split('\n')
                .filter(|line| !line.starts_with(pattern))
                .collect::<Vec<_>>()
                .join("\n")
        };
        let options = GrammarLoweringOptions {
            edit_metadata: Some(&edit),
            ..GrammarLoweringOptions::default()
        };
        check_grammar_lowering(&grammar, format, &options)
            .expect("checks")
            .failures
            .into_iter()
            .map(|failure| failure.detail)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        details("gbnf", "(kind sep"),
        [
            "rule sep lost its kind token",
            "rule sep changed its definition"
        ]
    );
    assert_eq!(
        details("bnf", "(doc message"),
        ["rule message changed its documentation"]
    );
    assert_eq!(
        details("gbnf", "(helper lowered2 shout"),
        ["rule shout changed its definition"]
    );
    assert_eq!(
        details("gbnf", "(rename message root"),
        ["the executable grammar has no rule message"]
    );
    // An emitter that writes the unlowered grammar reports what the target
    // cannot carry instead of a faithful lowering.
    let (accepted, rejected) = (strings(features, "accepts"), strings(features, "rejects"));
    for format in GRAMMAR_LOWERING_FORMATS {
        let emit = grammar_emitter(format).expect("emitter");
        let raw =
            |_: &Grammar| -> Result<(String, EmitReport), GrammarEmitError> { emit(&grammar) };
        let options = GrammarLoweringOptions {
            emit: Some(&raw),
            accepts: &accepted,
            rejects: &rejected,
            ..GrammarLoweringOptions::default()
        };
        let report = check_grammar_lowering(&grammar, format, &options).expect("checks");
        assert_eq!(report.status, GrammarLoweringStatus::Broken, "{format}");
        for kind in [
            GrammarLoweringFailureKind::LossyEmission,
            GrammarLoweringFailureKind::NotExecutable,
        ] {
            assert!(
                report.failures.iter().any(|failure| failure.kind == kind),
                "{format} {}",
                kind.as_str()
            );
        }
    }
    assert_eq!(
        dropped_grammar_features(&grammar, &Grammar::new()).expect("compares"),
        [
            "rules [message, greeting, sep, name, letter, digit, shout] became []",
            "the start rule changed"
        ]
    );
    record(
        "noSilentlyDroppedFeature",
        "issue_195_lowering_never_drops_a_grammar_feature_silently",
    );
}
