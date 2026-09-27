//! Issue #195 binding rename evidence: for every language of the shared
//! four-language corpus, `rename_binding` follows one symbol through
//! shadowing, nested scopes, qualified names, Unicode identifiers, and macro
//! or proof binders, rewrites exactly that symbol's occurrences, leaves
//! comments, strings, and every other name's resolution unchanged, and
//! rejects renames that would capture another name.
//! Mirrors `js/tests/issue-195-binding-rename.test.js`.

use std::collections::BTreeSet;
use std::process::Command;

use meta_language::{
    analyze_program, ProgramBinding, ProgramProjectContext, ProgramRepresentation,
};
use serde_json::Value;

use super::issue_195_observations as observations;

const FIXTURE: &str = include_str!("../../../parity/fixtures/four-language-conformance.json");

const ASSERTIONS: &[&str] = &[
    "symbolIdentity",
    "shadowing",
    "nestedScopes",
    "qualifiedNames",
    "unicodeIdentifiers",
    "macroOrProofBinders",
    "captureAvoidance",
    "commentsStringsAndLiteralsUnaffected",
];

fn key(name: &str, binding: &ProgramBinding) -> String {
    format!("{name}:{}:{}", binding.kind(), binding.references().len())
}

/// Every name's resolution: what each binding is, how often it is used, and
/// which names stay unresolved.
fn resolution(program: &ProgramRepresentation) -> (Vec<String>, Vec<String>) {
    let mut bindings = program
        .bindings()
        .iter()
        .map(|binding| key(binding.name(), binding))
        .collect::<Vec<_>>();
    bindings.sort();
    let mut unresolved = program
        .unresolved_references()
        .iter()
        .map(|reference| reference.name().to_owned())
        .collect::<Vec<_>>();
    unresolved.sort();
    (bindings, unresolved)
}

fn comments(program: &ProgramRepresentation) -> Vec<String> {
    program
        .source_mappings()
        .iter()
        .filter(|mapping| mapping.term().contains("comment"))
        .map(|mapping| program.source()[mapping.range().start()..mapping.range().end()].to_owned())
        .collect()
}

fn observe(source: &str) -> Value {
    let output = Command::new("node")
        .args([
            "-e",
            "eval(process.argv[1]); process.stdout.write(JSON.stringify(globalThis.result));",
            source,
        ])
        .output()
        .expect("Node executes the JavaScript fixture");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("fixture emits a JSON observation")
}

fn text<'a>(fixture: &'a Value, field: &str) -> &'a str {
    fixture[field]
        .as_str()
        .unwrap_or_else(|| panic!("fixture has {field}"))
}

fn check_language(corpus: &Value, language: &str) {
    let cases = corpus["bindingRenameCorpus"]
        .as_array()
        .expect("binding rename corpus")
        .iter()
        .filter(|fixture| fixture["language"] == language)
        .collect::<Vec<_>>();
    let covered = cases
        .iter()
        .flat_map(|fixture| fixture["assertions"].as_array().expect("assertions"))
        .map(|assertion| assertion.as_str().expect("assertion name"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        covered,
        ASSERTIONS.iter().copied().collect::<BTreeSet<_>>(),
        "{language} corpus covers every rename assertion"
    );
    assert!(cases.iter().any(|fixture| fixture["allowed"] == true));
    assert!(cases.iter().any(|fixture| fixture["allowed"] == false));

    for fixture in cases {
        let source = text(fixture, "source");
        let name = text(fixture, "binding");
        let replacement = text(fixture, "replacement");
        let occurrence = usize::try_from(fixture["declarationOccurrence"].as_u64().unwrap())
            .expect("occurrence fits usize");
        let label = format!("{language} {} {name}#{occurrence}", fixture["assertions"]);
        let program = analyze_program(source, language, ProgramProjectContext::default())
            .unwrap_or_else(|error| panic!("{label} parses: {error}"));
        let binding = program
            .bindings()
            .iter()
            .filter(|binding| binding.name() == name)
            .nth(occurrence)
            .unwrap_or_else(|| panic!("{label} selects a binding"));
        let observation = &fixture["expectedObservation"];
        if !observation.is_null() {
            assert_eq!(
                &observe(source),
                observation,
                "{label} original observation"
            );
        }

        if fixture["allowed"] == false {
            let error = program
                .rename_binding(binding.id(), replacement)
                .expect_err(&label)
                .to_string();
            assert!(
                error.contains("capture") || error.contains("conflict"),
                "{label}: {error}"
            );
            assert_eq!(program.emit(), source, "{label} leaves the program intact");
            continue;
        }

        let renamed = program
            .rename_binding(binding.id(), replacement)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        assert_eq!(renamed.emit(), text(fixture, "expected"), "{label}");
        assert!(
            renamed.network().verify_full_match(None).is_clean(),
            "{label} reparses"
        );
        // The renamed symbol keeps every occurrence; every other symbol and
        // every unresolved name resolves as before.
        let (mut bindings, unresolved) = resolution(&program);
        let position = bindings
            .iter()
            .position(|entry| *entry == key(name, binding))
            .expect("renamed binding key");
        bindings.remove(position);
        bindings.push(key(replacement, binding));
        bindings.sort();
        assert_eq!(
            resolution(&renamed),
            (bindings, unresolved),
            "{label} resolution"
        );
        assert_eq!(comments(&renamed), comments(&program), "{label} comments");
        if !observation.is_null() {
            assert_eq!(
                &observe(&renamed.emit()),
                observation,
                "{label} renamed observation"
            );
        }
    }

    let requirement_id = format!("I195-RENAME-{}", observations::slug(language));
    let fixture_id = format!("planned:binding-rename:{language}");
    observations::record(&observations::Observation {
        requirement_id: &requirement_id,
        suffix: "positive-and-negative",
        fixture_id: &fixture_id,
        fixture_file: observations::FOUR_LANGUAGE_FIXTURE,
        assertions: ASSERTIONS,
        test_name: &format!(
            "issue 195 {language} binding rename follows symbol identity and rejects capture"
        ),
    });
}

#[test]
fn issue_195_binding_rename_follows_symbol_identity_and_rejects_capture() {
    let corpus: Value = serde_json::from_str(FIXTURE).expect("four-language corpus");
    for language in corpus["languages"].as_array().expect("languages") {
        check_language(&corpus, text(language, "name"));
    }
}
