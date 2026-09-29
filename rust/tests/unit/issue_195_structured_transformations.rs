//! Issue #195 structured transformation evidence: for every language of the
//! shared four-language corpus, each public structured operation runs on a
//! program constructed from fragments and reloaded from a snapshot that never
//! held the original source buffer. Every intermediate program's derived
//! metadata (CST, spans, source mappings, diagnostics) must equal a fresh
//! parse of its emitted text, the edit sequence must reach the intended
//! structure, and a follow-up edit must restore the constructed program.
//! Mirrors `js/tests/issue-195-structured-transformations.test.js`.

use meta_language::{
    ProgramProjectContext, ProgramRange, ProgramRepresentation, construct_program,
    construct_program_from_fragments,
};
use serde_json::Value;

use super::issue_195_observations as observations;

const FIXTURE: &str = include_str!("../../../parity/fixtures/four-language-conformance.json");

const ASSERTIONS: &[&str] = &[
    "publicApiUsed",
    "editSequence",
    "serializeReload",
    "constructionWithoutOriginalSource",
    "treeIntegrity",
    "spansUpdated",
    "sourceMappingsUpdated",
    "diagnosticsUpdated",
    "originalBufferDiscarded",
    "emittedSourceReparses",
    "intendedStructureObserved",
];

const DECLARED: &[&str] = &["first", "second", "third", "primary"];

struct Program<'a> {
    language: &'a str,
    source: &'a str,
    first: &'a str,
    second: &'a str,
    inserted: &'a str,
    phantom_source: &'a str,
    phantom_dependency: &'a str,
}

fn text(value: &Value, key: &str) -> String {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("fixture has {key}"))
        .to_owned()
}

/// Declaration name ranges in source order, as the CST reports them.
fn declarations(program: &ProgramRepresentation) -> Vec<(String, ProgramRange)> {
    program
        .query_syntax("identifier")
        .into_iter()
        .map(|range| {
            (
                program.source()[range.start()..range.end()].to_ascii_lowercase(),
                range,
            )
        })
        .filter(|(name, _)| DECLARED.contains(&name.as_str()))
        .collect()
}

fn declared_names(program: &ProgramRepresentation) -> Vec<String> {
    declarations(program)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

fn name_range(program: &ProgramRepresentation, name: &str) -> ProgramRange {
    declarations(program)
        .into_iter()
        .find(|(declared, _)| declared == name)
        .unwrap_or_else(|| panic!("{} declares {name}", program.language()))
        .1
}

/// The derived metadata of an edited program must be exactly what a fresh
/// parse of its emitted text derives, and every mapping must slice the new
/// source.
fn assert_consistent(program: &ProgramRepresentation, expected: &str, label: &str) {
    let emitted = program.emit();
    assert_eq!(emitted, expected, "{label} emits");
    assert_eq!(program.source(), emitted, "{label} source");
    assert!(
        program.network().verify_full_match(None).is_clean(),
        "{label} tree integrity"
    );
    let root = program
        .source_mappings()
        .iter()
        .map(meta_language::ProgramSourceMapping::range)
        .max_by_key(|range| range.end() - range.start())
        .expect("source mappings");
    assert_eq!(
        root,
        ProgramRange::new(0, emitted.len()),
        "{label} root covers source"
    );
    for mapping in program.source_mappings() {
        let range = mapping.range();
        assert!(
            emitted.get(range.start()..range.end()).is_some(),
            "{label} mapping slices the source on character boundaries"
        );
    }
    let reparsed = construct_program(&emitted, program.language(), program.project().clone())
        .unwrap_or_else(|error| panic!("{label} reparses: {error}"));
    assert_eq!(
        program.source_mappings(),
        reparsed.source_mappings(),
        "{label} source mappings"
    );
    assert_eq!(
        program.diagnostics(),
        reparsed.diagnostics(),
        "{label} diagnostics"
    );
    assert_same_semantics(program, &reparsed, label);
}

fn assert_same_semantics(left: &ProgramRepresentation, right: &ProgramRepresentation, label: &str) {
    assert_eq!(left.language(), right.language(), "{label}");
    assert_eq!(left.scopes(), right.scopes(), "{label} scopes");
    assert_eq!(left.bindings(), right.bindings(), "{label} bindings");
    assert_eq!(
        left.unresolved_references(),
        right.unresolved_references(),
        "{label} unresolved references"
    );
    assert_eq!(left.modules(), right.modules(), "{label} modules");
    assert_eq!(left.types(), right.types(), "{label} types");
    assert_eq!(left.extensions(), right.extensions(), "{label} extensions");
    assert_eq!(left.proofs(), right.proofs(), "{label} proofs");
    assert_eq!(
        left.diagnostics(),
        right.diagnostics(),
        "{label} diagnostics"
    );
    assert_eq!(left.constructs(), right.constructs(), "{label} constructs");
    assert_eq!(left.project(), right.project(), "{label} project");
}

fn reload(
    fragments: &[&str],
    language: &str,
    project: ProgramProjectContext,
) -> ProgramRepresentation {
    // Construct from fragments, persist, and drop the constructed program:
    // the reload is the only input from here on.
    let constructed = construct_program_from_fragments(fragments, language, project)
        .expect("fragment construction");
    let serialized = constructed.serialize_snapshot();
    drop(constructed);
    let snapshot: Value = serde_json::from_str(&serialized).expect("snapshot JSON");
    assert!(
        snapshot.get("source").is_none(),
        "snapshot holds no source buffer"
    );
    ProgramRepresentation::from_snapshot(&serialized).expect("snapshot reload")
}

/// Applies the operation under test and returns the intended emitted text and
/// declaration names.
fn apply(
    operation: &str,
    fixture: &Program<'_>,
    program: &ProgramRepresentation,
) -> (ProgramRepresentation, String, Vec<&'static str>) {
    let Program {
        source,
        first,
        second,
        inserted,
        ..
    } = *fixture;
    let first_range = ProgramRange::new(0, first.len());
    let second_range = ProgramRange::new(first.len(), source.len());
    let replacement = first
        .replacen("first", "primary", 1)
        .replacen("FIRST", "primary", 1);
    let edited = match operation {
        "construct" => Ok(reload(
            &[second, first],
            program.language(),
            program.project().clone(),
        )),
        "query" => program.replace(name_range(program, "first"), "primary"),
        "insert" => program.insert(source.len(), inserted),
        "replace" => program.replace(first_range, &replacement),
        "delete" => program.delete(second_range),
        "clone" => program.clone_range(first_range, source.len()),
        "move" => program.move_range(second_range, 0),
        // Emit after an edit sequence, then rebuild from the emitted text
        // alone: nothing of the edited program survives but its output.
        "emit" => program
            .insert(source.len(), inserted)
            .and_then(|inserted| inserted.move_range(second_range, 0))
            .and_then(|moved| {
                construct_program(&moved.emit(), moved.language(), moved.project().clone())
            }),
        _ => unreachable!("unknown operation {operation}"),
    }
    .unwrap_or_else(|error| panic!("{} {operation}: {error}", fixture.language));
    match operation {
        "construct" | "move" => (edited, format!("{second}{first}"), vec!["second", "first"]),
        "query" | "replace" => (edited, replacement + second, vec!["primary", "second"]),
        "insert" => (
            edited,
            format!("{source}{inserted}"),
            vec!["first", "second", "third"],
        ),
        "delete" => (edited, first.to_owned(), vec!["first"]),
        "emit" => (
            edited,
            format!("{second}{first}{inserted}"),
            vec!["second", "first", "third"],
        ),
        _ => (
            edited,
            format!("{source}{first}"),
            vec!["first", "second", "first"],
        ),
    }
}

/// The follow-up edit that restores the constructed program.
fn restore(
    operation: &str,
    fixture: &Program<'_>,
    edited: &ProgramRepresentation,
) -> ProgramRepresentation {
    let Program {
        language,
        source,
        first,
        second,
        inserted,
        ..
    } = *fixture;
    let replacement_len = first.len() - "first".len() + "primary".len();
    match operation {
        "construct" => edited.move_range(ProgramRange::new(0, second.len()), edited.source().len()),
        "query" => edited.replace(
            name_range(edited, "primary"),
            if language == "Rust" { "FIRST" } else { "first" },
        ),
        "insert" => edited.delete(ProgramRange::new(
            source.len(),
            source.len() + inserted.len(),
        )),
        "replace" => edited.replace(ProgramRange::new(0, replacement_len), first),
        "delete" => edited.insert(first.len(), second),
        "clone" => edited.delete(ProgramRange::new(source.len(), source.len() + first.len())),
        "move" => edited.move_range(ProgramRange::new(second.len(), source.len()), 0),
        "emit" => edited
            .delete(ProgramRange::new(
                source.len(),
                source.len() + inserted.len(),
            ))
            .and_then(|deleted| {
                deleted.move_range(ProgramRange::new(second.len(), source.len()), 0)
            }),
        _ => unreachable!("unknown operation {operation}"),
    }
    .unwrap_or_else(|error| panic!("{language} {operation} restore: {error}"))
}

fn check_operation(operation: &str) {
    let corpus: Value = serde_json::from_str(FIXTURE).expect("shared corpus is JSON");
    let programs = corpus["transformationPrograms"]
        .as_array()
        .expect("transformation programs");
    assert_eq!(programs.len(), 4, "one program per language");
    for entry in programs {
        let language = text(entry, "language");
        let phantom = corpus["phantomImportCases"]
            .as_array()
            .expect("phantom import cases")
            .iter()
            .find(|case| case["language"] == language.as_str())
            .unwrap_or_else(|| panic!("{language} has a phantom import case"));
        let (source, first, second, inserted) = (
            text(entry, "source"),
            text(entry, "first"),
            text(entry, "second"),
            text(entry, "inserted"),
        );
        let (phantom_source, phantom_dependency) =
            (text(phantom, "source"), text(phantom, "dependency"));
        let fixture = Program {
            language: &language,
            source: &source,
            first: &first,
            second: &second,
            inserted: &inserted,
            phantom_source: &phantom_source,
            phantom_dependency: &phantom_dependency,
        };
        check_program(operation, &fixture);

        let requirement_id = format!("I195-XFORM-{}-{operation}", observations::slug(&language));
        let fixture_id = format!("planned:transformation:{language}:{operation}");
        observations::record(&observations::Observation {
            requirement_id: &requirement_id,
            suffix: "positive",
            fixture_id: &fixture_id,
            fixture_file: observations::FOUR_LANGUAGE_FIXTURE,
            assertions: ASSERTIONS,
            test_name: &format!(
                "issue_195_structured_{operation}_{}",
                observations::slug(&language)
            ),
        });
    }
}

fn check_program(operation: &str, fixture: &Program<'_>) {
    let language = fixture.language;
    let project = ProgramProjectContext::new(
        format!("/workspace/{language}"),
        vec!["main".to_owned()],
        Vec::new(),
    );
    let program = reload(&[fixture.first, fixture.second], language, project);
    assert_consistent(&program, fixture.source, &format!("{language} reload"));
    assert_eq!(declared_names(&program), ["first", "second"]);

    let (edited, expected, names) = apply(operation, fixture, &program);
    let label = format!("{language} {operation}");
    assert_eq!(
        program.emit(),
        fixture.source,
        "{label} leaves its input intact"
    );
    assert_consistent(&edited, &expected, &label);
    assert_eq!(declared_names(&edited), names, "{label} structure");

    // Spans follow the text: a prefix shifts every declaration name by its
    // length, and the unresolved import surfaces as a project diagnostic that
    // disappears again with the import.
    let prefix = fixture.phantom_source.len();
    let with_import = edited
        .insert(0, fixture.phantom_source)
        .expect("phantom import inserts");
    assert_consistent(
        &with_import,
        &format!("{}{expected}", fixture.phantom_source),
        &format!("{label} import"),
    );
    let shifted = declarations(&with_import)
        .into_iter()
        .map(|(_, range)| range)
        .collect::<Vec<_>>();
    let original = declarations(&edited)
        .into_iter()
        .map(|(_, range)| ProgramRange::new(range.start() + prefix, range.end() + prefix))
        .collect::<Vec<_>>();
    assert_eq!(shifted, original, "{label} spans shift");
    assert!(
        !edited
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.kind() == "missing-project-context")
    );
    let missing = with_import
        .diagnostics()
        .iter()
        .filter(|diagnostic| diagnostic.kind() == "missing-project-context")
        .collect::<Vec<_>>();
    assert_eq!(missing.len(), 1, "{label} missing dependency");
    assert_eq!(missing[0].term(), fixture.phantom_dependency);
    assert!(missing[0].range().end() <= prefix);
    let without_import = with_import
        .delete(ProgramRange::new(0, prefix))
        .expect("phantom import deletes");
    assert_consistent(
        &without_import,
        &expected,
        &format!("{label} import removed"),
    );
    assert_eq!(without_import.diagnostics(), edited.diagnostics());

    // The follow-up edit restores the constructed program exactly.
    let restored = restore(operation, fixture, &edited);
    assert_consistent(&restored, fixture.source, &format!("{label} restored"));
    assert_same_semantics(&restored, &program, &format!("{label} round trip"));
    assert_eq!(declared_names(&restored), ["first", "second"]);
}

#[test]
fn issue_195_structured_construct_runs_on_reloaded_programs_and_reparses() {
    check_operation("construct");
}

#[test]
fn issue_195_structured_query_runs_on_reloaded_programs_and_reparses() {
    check_operation("query");
}

#[test]
fn issue_195_structured_insert_runs_on_reloaded_programs_and_reparses() {
    check_operation("insert");
}

#[test]
fn issue_195_structured_replace_runs_on_reloaded_programs_and_reparses() {
    check_operation("replace");
}

#[test]
fn issue_195_structured_delete_runs_on_reloaded_programs_and_reparses() {
    check_operation("delete");
}

#[test]
fn issue_195_structured_clone_runs_on_reloaded_programs_and_reparses() {
    check_operation("clone");
}

#[test]
fn issue_195_structured_move_runs_on_reloaded_programs_and_reparses() {
    check_operation("move");
}

#[test]
fn issue_195_structured_emit_runs_on_reloaded_programs_and_reparses() {
    check_operation("emit");
}
