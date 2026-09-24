use std::fs;
use std::path::PathBuf;

use meta_language::{
    analyze_program, ByteRange, LinkNetwork, LinkType, ParseConfiguration, ProgramProjectContext,
    ProgramRepresentation,
};
use serde_json::Value;

fn evidence() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/issue-195-evidence.json");
    serde_json::from_str(&fs::read_to_string(path).expect("shared evidence corpus is readable"))
        .expect("shared evidence corpus is valid JSON")
}

#[test]
fn pinned_external_corpora_and_projects_exercise_every_four_language_frontend() {
    let evidence = evidence();
    for fixture in evidence["externalConformance"]
        .as_array()
        .expect("external fixtures")
    {
        let provenance = &fixture["provenance"];
        assert!(provenance["url"]
            .as_str()
            .expect("provenance URL")
            .starts_with("https://github.com/"));
        assert!(!provenance["revision"].as_str().unwrap().is_empty());
        assert!(!provenance["license"].as_str().unwrap().is_empty());
        assert_structured_source(
            fixture["source"].as_str().unwrap(),
            fixture["language"].as_str().unwrap(),
            fixture["root"].as_str().unwrap(),
            strings(&fixture["requiredTerms"]),
        );
    }

    for fixture in evidence["representativeProjects"]
        .as_array()
        .expect("project fixtures")
    {
        let language = fixture["language"].as_str().unwrap();
        let source = fixture["source"].as_str().unwrap();
        let project = ProgramProjectContext::new(
            "/project",
            strings(&fixture["files"]),
            strings(&fixture["dependencies"]),
        );
        let program = analyze_program(source, language, project).expect("project analysis");
        assert_eq!(program.emit(), source, "{language} project reconstruction");
        assert!(
            program.network().verify_full_match(None).is_clean(),
            "{language} project CST"
        );
        assert!(
            !program.bindings().is_empty(),
            "{language} project bindings"
        );
        assert!(
            !program.source_mappings().is_empty(),
            "{language} project mappings"
        );
        assert_eq!(
            program
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.kind() == "missing-project-context"),
            matches!(language, "JavaScript" | "Rust"),
            "{language} project context"
        );
    }
}

#[test]
fn shared_embedded_fixtures_connect_host_boundaries_to_target_grammar_roots() {
    let evidence = evidence();
    for fixture in evidence["embedded"].as_array().expect("embedded fixtures") {
        let source = fixture["source"].as_str().unwrap();
        let language = fixture["regionLanguage"]
            .as_str()
            .or_else(|| fixture["target"].as_str())
            .unwrap();
        let region_source = fixture["regionSource"].as_str().unwrap();
        let start = source.find(region_source).expect("region source");
        let range = ByteRange::new(start, start + region_source.len());
        let network = LinkNetwork::parse(
            source,
            fixture["parseLanguage"].as_str().unwrap(),
            ParseConfiguration::default(),
        );
        let region = network
            .links()
            .find(|link| {
                link.metadata().link_type() == Some(LinkType::Region)
                    && link
                        .metadata()
                        .language()
                        .is_some_and(|candidate| candidate.eq_ignore_ascii_case(language))
                    && link
                        .metadata()
                        .span()
                        .is_some_and(|span| span.byte_range() == range)
            })
            .expect("region with exact boundary");
        assert!(network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax)
                && link
                    .metadata()
                    .language()
                    .is_some_and(|candidate| candidate.eq_ignore_ascii_case(language))
                && link.metadata().term() == fixture["root"].as_str()
                && link.references().contains(&region.id())
                && link
                    .metadata()
                    .span()
                    .is_some_and(|span| span.byte_range() == range)
        }));
        assert_eq!(network.reconstruct_text(), source);
    }
}

#[test]
fn seeded_generative_fuzz_and_metamorphic_cases_obey_independent_source_oracles() {
    let evidence = evidence();
    let generative = &evidence["generative"];
    let seed = u32::try_from(generative["seed"].as_u64().unwrap()).expect("seed fits u32");
    let mut random = Lcg::new(seed);
    let case_count = generative["casesPerLanguage"].as_u64().unwrap();
    let identifiers = strings(&generative["unicodeIdentifiers"]);
    for language in ["JavaScript", "Rust", "Lean", "Rocq"] {
        for index in 0..case_count {
            let number = random.next() % 10_000;
            let identifier = format!(
                "{}{}",
                identifiers[random.next() as usize % identifiers.len()],
                index
            );
            let source = generated_source(language, &identifier, number, index);
            let network = LinkNetwork::parse(&source, language, ParseConfiguration::default());
            assert_eq!(
                network.reconstruct_text(),
                source,
                "{language} case {index}"
            );
            assert!(
                network.verify_full_match(None).is_clean(),
                "{language} generated parse {index}"
            );
            assert_utf8_token_spans(&network, source.len(), language, index);

            let prefixed = format!(
                "{}{}",
                comment(language, &format!("metamorphic {number}")),
                source
            );
            let transformed =
                LinkNetwork::parse(&prefixed, language, ParseConfiguration::default());
            assert_eq!(transformed.reconstruct_text(), prefixed);
            assert!(
                transformed.verify_full_match(None).is_clean(),
                "{language} metamorphic parse {index}"
            );

            let program = analyze_program(&source, language, ProgramProjectContext::default())
                .expect("generated program analysis");
            let insertion = comment(language, &format!("edit {index}"));
            let edited = program
                .insert(source.len(), &insertion)
                .expect("structured insertion");
            assert_eq!(edited.emit(), format!("{source}{insertion}"));
            let restored = ProgramRepresentation::from_snapshot(&edited.serialize_snapshot())
                .expect("snapshot reload");
            assert_eq!(restored.emit(), edited.emit());

            let malformed = format!("{source}\0");
            let recovered = LinkNetwork::parse(&malformed, language, ParseConfiguration::default());
            assert_eq!(recovered.reconstruct_text(), malformed);
            assert!(
                !recovered.verify_full_match(None).is_clean(),
                "{language} malformed diagnostic {index}"
            );
        }
    }
}

fn assert_structured_source(source: &str, language: &str, root: &str, required: Vec<String>) {
    let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    assert_eq!(
        network.reconstruct_text(),
        source,
        "{language} external reconstruction"
    );
    assert!(
        network.verify_full_match(None).is_clean(),
        "{language} external parse"
    );
    let terms = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
        .filter_map(|link| link.metadata().term().map(str::to_string))
        .collect::<Vec<_>>();
    assert!(terms.iter().any(|term| term == root), "{language} {root}");
    for term in required {
        assert!(terms.contains(&term), "{language} {term}");
    }
}

fn assert_utf8_token_spans(network: &LinkNetwork, byte_length: usize, language: &str, index: u64) {
    let spans = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Token))
        .filter_map(|link| link.metadata().span())
        .collect::<Vec<_>>();
    assert!(spans.len() > 1, "{language} generated tokens {index}");
    assert!(spans.iter().all(|span| {
        let range = span.byte_range();
        range.start() <= range.end() && range.end() <= byte_length
    }));
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("string array")
        .iter()
        .map(|item| item.as_str().expect("string").to_string())
        .collect()
}

struct Lcg(u32);

impl Lcg {
    const fn new(seed: u32) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.0
    }
}

fn generated_source(language: &str, identifier: &str, number: u32, index: u64) -> String {
    match language {
        "JavaScript" => format!(
            "const {identifier} = {number};\nfunction f{index}(value) {{ return value + {identifier}; }}\n"
        ),
        "Rust" => format!(
            "const {identifier}: u64 = {number};\nfn f{index}(value: u64) -> u64 {{ value + {identifier} }}\n"
        ),
        "Lean" => format!(
            "def {identifier} : Nat := {number}\ndef f{index} (value : Nat) : Nat := value + {identifier}\n"
        ),
        "Rocq" => format!(
            "Definition {identifier} : nat := {number}.\nDefinition f{index} (value : nat) : nat := value + {identifier}.\n"
        ),
        _ => unreachable!("four-language fixture"),
    }
}

fn comment(language: &str, text: &str) -> String {
    match language {
        "Lean" => format!("-- {text}\n"),
        "Rocq" => format!("(* {text} *)\n"),
        _ => format!("// {text}\n"),
    }
}
