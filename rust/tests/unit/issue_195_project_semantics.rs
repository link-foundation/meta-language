//! Issue #195 project-aware semantics evidence: for every language of the
//! four-language corpus, a real multi-file project (manifest, module tree and
//! entry program) is loaded, every semantic construct links the entry program
//! to the declarations it names in other project files, missing or broken
//! project context is diagnosed without fabricated links, and the valid
//! context enables the construct's behavior (imports, macro and notation
//! expansion, tactics, attributes, effects).
//! Mirrors `js/tests/issue-195-project-semantics.test.js`; the projects
//! themselves are in experiments/issue-195-projects.

use meta_language::{
    analyze_program, ProgramConstructStatus, ProgramProjectContext, ProgramProjectSource,
    ProgramRange, ProgramRepresentation, SEMANTIC_CONSTRUCTS,
};
use serde_json::{json, Value};

use super::issue_195_observations as observations;

const FIXTURE: &str = include_str!("../../../parity/fixtures/four-language-conformance.json");

const ASSERTIONS: &[&str] = &[
    "projectContextLoaded",
    "languageSpecificStructurePreserved",
    "sourceMappingPreserved",
    "missingContextDiagnosed",
    "validContextEnablesBehavior",
];

fn corpus() -> Value {
    serde_json::from_str(FIXTURE).expect("four-language fixture is JSON")
}

fn text(value: &Value, key: &str) -> String {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("fixture has {key}"))
        .to_owned()
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("fixture has a string array")
        .iter()
        .map(|item| item.as_str().expect("fixture string").to_owned())
        .collect()
}

fn sources(value: &Value) -> Vec<ProgramProjectSource> {
    value
        .as_array()
        .expect("fixture has sources")
        .iter()
        .map(|item| ProgramProjectSource::new(text(item, "path"), text(item, "source")))
        .collect()
}

fn slice(source: &str, range: ProgramRange) -> &str {
    &source[range.start()..range.end()]
}

fn analyze(fixture: &Value, sources: Vec<ProgramProjectSource>) -> ProgramRepresentation {
    let entry = text(fixture, "entry");
    let source = fixture["sources"]
        .as_array()
        .expect("fixture has sources")
        .iter()
        .find(|item| item["path"] == entry.as_str())
        .map(|item| text(item, "source"))
        .expect("the entry is a project source");
    let files = sources.iter().map(|item| item.path().to_owned()).collect();
    let project = ProgramProjectContext::new(
        text(fixture, "root"),
        files,
        strings(&fixture["dependencies"]),
    )
    .with_entry(entry, sources);
    analyze_program(&source, &text(fixture, "language"), project)
        .expect("the project program analyzes")
}

fn project_evidence(
    program: &ProgramRepresentation,
    construct: &str,
    sources: &[ProgramProjectSource],
) -> Vec<Value> {
    construct_of(program, construct)
        .evidence()
        .iter()
        .filter_map(|fact| {
            let file = fact.file()?;
            let source = sources
                .iter()
                .find(|item| item.path() == file)
                .expect("evidence names a project file");
            Some(json!({
                "kind": fact.kind(),
                "name": fact.name(),
                "file": file,
                "text": slice(source.source(), fact.range()),
            }))
        })
        .collect()
}

fn construct_of<'a>(
    program: &'a ProgramRepresentation,
    construct: &str,
) -> &'a meta_language::ProgramConstruct {
    program
        .constructs()
        .iter()
        .find(|item| item.kind() == construct)
        .unwrap_or_else(|| panic!("{construct} is reported"))
}

fn diagnostics(program: &ProgramRepresentation) -> Value {
    program
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            json!({
                "kind": diagnostic.kind(),
                "term": diagnostic.term(),
                "text": slice(program.source(), diagnostic.range()),
            })
        })
        .collect()
}

fn project_modules(program: &ProgramRepresentation) -> Value {
    program
        .project_modules()
        .iter()
        .map(|module| {
            json!({
                "request": module.request(),
                "module": module.module(),
                "text": slice(program.source(), module.range()),
            })
        })
        .collect()
}

fn expansions(program: &ProgramRepresentation) -> Value {
    program
        .expansions()
        .iter()
        .map(|expansion| {
            json!({
                "name": expansion.name(),
                "kind": expansion.kind(),
                "text": slice(program.source(), expansion.range()),
                "expansion": expansion.expansion(),
                "target": expansion.target(),
            })
        })
        .collect()
}

fn names(value: &[Value]) -> Vec<Value> {
    value
        .iter()
        .map(|item| json!({ "kind": item["kind"], "name": item["name"], "file": item["file"] }))
        .collect()
}

#[test]
fn issue_195_project_programs_cover_every_language_and_construct() {
    let corpus = corpus();
    let programs = corpus["projectPrograms"]
        .as_array()
        .expect("project programs");
    let languages = corpus["languages"].as_array().expect("languages");
    assert_eq!(
        programs
            .iter()
            .map(|fixture| text(fixture, "language"))
            .collect::<Vec<_>>(),
        languages
            .iter()
            .map(|language| text(language, "name"))
            .collect::<Vec<_>>(),
    );
    for fixture in programs {
        let constructs = fixture["constructs"].as_object().expect("constructs");
        // serde_json sorts object keys, so the constructs compare as sets.
        let mut expected = SEMANTIC_CONSTRUCTS.to_vec();
        expected.sort_unstable();
        assert_eq!(
            constructs.keys().map(String::as_str).collect::<Vec<_>>(),
            expected
        );
        for construct in SEMANTIC_CONSTRUCTS {
            assert!(
                !constructs[construct]
                    .as_array()
                    .expect("construct evidence")
                    .is_empty(),
                "{} {construct} has expected project evidence",
                text(fixture, "language")
            );
        }
    }
}

#[test]
fn issue_195_every_construct_is_project_aware() {
    for fixture in corpus()["projectPrograms"]
        .as_array()
        .expect("project programs")
    {
        check_project(fixture);
    }
}

fn check_project(fixture: &Value) {
    let language = text(fixture, "language");
    let entry = text(fixture, "entry");
    let project_sources = sources(&fixture["sources"]);
    let program = analyze(fixture, project_sources.clone());
    let entry_source = project_sources
        .iter()
        .find(|item| item.path() == entry)
        .cloned()
        .expect("the entry is a project source");
    let bare = analyze(fixture, Vec::new());
    let alone = analyze(fixture, vec![entry_source]);
    let broken = fixture["brokenContexts"]
        .as_array()
        .expect("broken contexts")
        .iter()
        .map(|context| {
            let replaced = sources(&context["sources"]);
            let sources = project_sources
                .iter()
                .map(|source| {
                    replaced
                        .iter()
                        .find(|item| item.path() == source.path())
                        .unwrap_or(source)
                        .clone()
                })
                .collect();
            (context, analyze(fixture, sources))
        })
        .collect::<Vec<_>>();

    for construct in SEMANTIC_CONSTRUCTS {
        let requirement_id = format!("I195-SEM-{}-{construct}", observations::slug(&language));
        let test_name = format!("issue 195 {language} {construct} is project-aware");

        // projectContextLoaded: every module request resolves to a project file,
        // every source file and manifest is read, and nothing is diagnosed.
        assert_eq!(diagnostics(&program), json!([]), "{test_name}");
        assert_eq!(project_modules(&program), fixture["modules"], "{test_name}");
        let files = program
            .project_facts()
            .iter()
            .filter(|fact| fact.kind() == "project-file")
            .map(|fact| fact.name().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            files,
            project_sources
                .iter()
                .map(|item| item.path().to_owned())
                .collect::<Vec<_>>()
        );
        assert!(
            program
                .project_facts()
                .iter()
                .any(|fact| fact.kind() == "project-manifest"),
            "{language}: the project manifest is read"
        );

        // languageSpecificStructurePreserved: the construct's links carry the
        // language's own symbol identity, roles and declaration kinds.
        let evidence = project_evidence(&program, construct, &project_sources);
        let expected = fixture["constructs"][construct]
            .as_array()
            .expect("construct evidence");
        assert_eq!(
            construct_of(&program, construct).status(),
            ProgramConstructStatus::Represented,
            "{test_name}"
        );
        assert_eq!(names(&evidence), names(expected), "{test_name}");

        // sourceMappingPreserved: every link spans exactly the expected source
        // text, and every declaration range lies within its project file.
        assert_eq!(&evidence, expected, "{test_name}");
        for reference in program.project_references() {
            let target = project_sources
                .iter()
                .find(|item| item.path() == reference.file())
                .unwrap_or_else(|| panic!("{} is declared in a project file", reference.symbol()));
            let declaration = reference.declaration();
            assert!(
                declaration.start() < declaration.end()
                    && declaration.end() <= target.source().len()
            );
            assert!(
                reference
                    .symbol()
                    .starts_with(&format!("{}#", reference.file())),
                "{}",
                reference.symbol()
            );
        }

        // missingContextDiagnosed: without the other project files, the module
        // requests are diagnosed and no project link is fabricated; a broken
        // project reports exactly what is wrong.
        for missing in [&bare, &alone] {
            assert_eq!(
                diagnostics(missing),
                fixture["missingContext"],
                "{test_name}"
            );
            assert!(missing.project_modules().is_empty(), "{test_name}");
            assert!(missing.project_references().is_empty(), "{test_name}");
            assert!(missing.expansions().is_empty(), "{test_name}");
        }
        for (context, broken_program) in &broken {
            assert_eq!(
                diagnostics(broken_program),
                context["diagnostics"],
                "{}",
                text(context, "description")
            );
        }

        // validContextEnablesBehavior: the valid project enables the links and
        // expansions that the missing context cannot produce.
        assert!(!evidence.is_empty());
        assert_eq!(expansions(&program), fixture["expansions"], "{test_name}");
        assert!(construct_of(&bare, construct)
            .evidence()
            .iter()
            .all(|fact| fact.file().is_none()));
        assert!(!program.project_references().is_empty() && !program.project_modules().is_empty());

        observations::record(&observations::Observation {
            requirement_id: &requirement_id,
            suffix: "positive-and-negative",
            fixture_id: &format!("planned:semantic:{language}:{construct}"),
            fixture_file: observations::FOUR_LANGUAGE_FIXTURE,
            assertions: ASSERTIONS,
            test_name: &test_name,
        });
    }
}
