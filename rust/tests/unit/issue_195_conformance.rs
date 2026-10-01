//! Issue 195 four-language conformance: the upstream tree-sitter corpora,
//! real-project files and hand-authored construct, Unicode, malformed and
//! mixed-language inputs of `parity/fixtures/issue-195-conformance/`, each
//! compared with the tree the native tree-sitter CLI printed for it
//! (`js/scripts/generate-issue-195-conformance.mjs`): structure, kinds,
//! fields, spans, error and missing nodes, trivia, diagnostics and exact
//! reconstruction. Where the upstream authors' expected tree agrees with the
//! CLI, the public tree must equal it too. Mirrors
//! `js/tests/issue-195-conformance.test.js`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use meta_language::{LinkNetwork, ParseConfiguration};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::cst_lines::{
    NetworkIndex, cst_lines_to_sexp, diagnostic_problems, document_oracle_problems,
    first_difference, parse_cst_lines, region_grammar_roots, render_cst_lines, trivia_problems,
};
use super::cst_sexpression::{CorpusCase, normalize, parse_corpus, strip_fields};
use super::issue_195_observations::{CONFORMANCE_FIXTURE, Observation, record};

const ASSERTIONS: [&str; 7] = [
    "claimedConstructInventoryMapped",
    "upstreamCorpusExecuted",
    "representativeRealProjectExecuted",
    "externalCorpusProvenanceRecorded",
    "malformedRecoveryCasesExecuted",
    "unicodeCasesExecuted",
    "mixedLanguageCasesExecuted",
];

fn repository_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(path)
}

fn read(path: &str) -> Vec<u8> {
    let path = repository_path("parity/fixtures/issue-195-conformance").join(path);
    fs::read(&path).unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()))
}

fn read_text(path: &str) -> String {
    String::from_utf8(read(path)).expect("UTF-8 fixture")
}

fn read_json(path: &str) -> Value {
    serde_json::from_slice(&read(path)).expect("valid JSON fixture")
}

fn sha256(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            use std::fmt::Write as _;
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} is a string"))
}

fn parse(source: &str, language: &str) -> LinkNetwork {
    LinkNetwork::parse(source, language, ParseConfiguration::default())
}

/// The corpus cases and project sources of one language, by oracle case id.
fn case_sources(details: &Value) -> HashMap<String, CorpusCase> {
    let mut sources = HashMap::new();
    let directory = text(&details["corpus"], "directory");
    for file in details["corpus"]["files"]
        .as_object()
        .expect("corpus files")
        .keys()
    {
        for (index, corpus_case) in parse_corpus(&read_text(&format!("{directory}/{file}")))
            .into_iter()
            .enumerate()
        {
            sources.insert(format!("corpus/{file}/{index}"), corpus_case);
        }
    }
    for project in details["projects"].as_array().expect("projects") {
        let file = text(project, "file");
        let name = file.rsplit('/').next().expect("project file name");
        sources.insert(
            format!("project/{name}"),
            CorpusCase {
                name: String::new(),
                attributes: Vec::new(),
                source: read_text(file),
                expected: String::new(),
            },
        );
    }
    sources
}

/// Problems of the public tree of one non-mixed case against its oracle tree.
fn document_problems(
    language: &str,
    entry: &Value,
    source: &str,
    corpus_case: Option<&CorpusCase>,
) -> Vec<String> {
    let network = parse(source, language);
    let (mut problems, tree) =
        document_oracle_problems(&network, language, source, text(entry, "cst"));
    let upstream = entry["upstream"].as_str();
    if let (Some("match"), Some(corpus_case)) = (upstream, corpus_case) {
        let expected = normalize(&corpus_case.expected);
        let mut actual = normalize(&cst_lines_to_sexp(&tree));
        if !expected.contains(": ") {
            actual = strip_fields(&actual);
        }
        if actual != expected {
            problems.push(format!(
                "upstream expected tree differs:\n  actual   {actual}\n  expected {expected}"
            ));
        }
    }
    if upstream == Some("error-expected") && network.verify_full_match(None).is_clean() {
        problems.push(
            "the upstream corpus expects an error, but the network verifies clean".to_string(),
        );
    }
    problems
}

/// Problems of the public region tree of one mixed-language case against its oracle tree.
fn region_problems(language: &str, entry: &Value, host: &Value) -> Vec<String> {
    let source = text(host, "source");
    let network = parse(source, text(host, "host"));
    let index = NetworkIndex::new(&network);
    let start = usize::try_from(entry["startByte"].as_u64().expect("startByte")).expect("offset");
    let end = usize::try_from(entry["endByte"].as_u64().expect("endByte")).expect("offset");
    let Some(region) = region_grammar_roots(&index).into_iter().find(|region| {
        region.language == language
            && region.span.is_some_and(|span| {
                span.byte_range().start() == start && span.byte_range().end() == end
            })
    }) else {
        return vec![format!(
            "no {language} region at {start}..{end} of the {} host",
            text(host, "host")
        )];
    };
    let (tree, rendered) = render_cst_lines(&region);
    let oracle = text(entry, "cst");
    let mut problems = Vec::new();
    if tree != oracle {
        problems.push(format!(
            "region CST differs at {}",
            first_difference(&tree, oracle)
        ));
    }
    if network.reconstruct_text() != source {
        problems.push("reconstruction differs from the host source".to_string());
    }
    problems.extend(trivia_problems(
        &network,
        source,
        oracle,
        Some((start, end)),
    ));
    problems.extend(diagnostic_problems(&network, &rendered, oracle, false));
    problems
}

#[test]
fn issue_195_conformance_fixtures_record_their_provenance_and_match_their_pinned_inputs() {
    let manifest = read_json("manifest.json");
    let lock: Value = serde_json::from_slice(
        &fs::read(repository_path("rust/src/data/grammar-lock.json")).expect("grammar lock"),
    )
    .expect("grammar lock JSON");
    assert_eq!(manifest["oracle"]["tool"], lock["treeSitterCli"]);
    for input in ["cases.json", "error-recovery.json"] {
        assert_eq!(
            text(&manifest["inputs"], input),
            sha256(read(input)),
            "{input}"
        );
    }
    for (language, details) in manifest["languages"].as_object().expect("languages") {
        let grammar = &details["grammar"];
        assert_eq!(
            grammar["parserSha256"],
            lock["grammars"][text(grammar, "id")]["parserSha256"],
            "{language}"
        );
        let revision = text(grammar, "revision");
        assert!(revision.len() == 40 && revision.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_ne!(
            fs::read(repository_path(text(grammar, "license"))).expect("grammar license"),
            [] as [u8; 0]
        );
        assert_eq!(
            sha256(read(text(details, "oracle"))),
            text(details, "oracleSha256"),
            "{language} oracle"
        );
        let directory = text(&details["corpus"], "directory");
        for (file, digest) in details["corpus"]["files"].as_object().expect("files") {
            assert_eq!(
                sha256(read(&format!("{directory}/{file}"))),
                digest.as_str().expect("digest"),
                "{language} {file}"
            );
        }
        let projects = details["projects"].as_array().expect("projects");
        assert!(!projects.is_empty(), "{language} has real projects");
        for project in projects {
            let commit = text(project, "commit");
            assert_eq!(commit.len(), 40);
            assert!(text(project, "url").contains(commit));
            assert_eq!(sha256(read(text(project, "file"))), text(project, "sha256"));
            assert_ne!(read(text(project, "licenseFile")), [] as [u8; 0]);
        }
    }
}

#[test]
fn issue_195_conformance_comparison_rejects_a_tree_that_differs_from_the_oracle() {
    let manifest = read_json("manifest.json");
    let oracle = read_json(text(&manifest["languages"]["JavaScript"], "oracle"));
    let malformed = oracle["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .find(|entry| entry["kind"] == "malformed")
        .expect("a malformed case");
    let source = text(malformed, "source");
    let cst = text(malformed, "cst");
    let with_cst = |cst: String| {
        let mut entry = malformed.clone();
        entry["cst"] = Value::String(cst);
        entry
    };
    let clean = cst.replace('•', "").replacen("ERROR ", "program ", 1);
    assert_ne!(
        document_problems("JavaScript", &with_cst(clean), source, None),
        [] as [String; 0]
    );
    let (head, last) = cst.rsplit_once(':').expect("an end point");
    let shifted = format!("{head}:{}", last.parse::<usize>().expect("column") + 1);
    assert_ne!(
        document_problems("JavaScript", &with_cst(shifted), source, None),
        [] as [String; 0]
    );
}

fn has_astral_character(source: &str) -> bool {
    source
        .chars()
        .any(|character| u32::from(character) >= 0x10000)
}

fn check_language(language: &str) {
    let manifest = read_json("manifest.json");
    let inputs = read_json("cases.json");
    let error_recovery = read_json("error-recovery.json");
    let details = &manifest["languages"][language];
    let oracle = read_json(text(details, "oracle"));
    assert_eq!(oracle["language"], language);
    let sources = case_sources(details);
    let hosts: HashMap<&str, &Value> = inputs["mixed"]
        .as_array()
        .expect("mixed hosts")
        .iter()
        .map(|host| (text(host, "id"), host))
        .collect();
    let cases = oracle["cases"].as_array().expect("oracle cases");
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut failures = Vec::new();
    for entry in cases {
        let kind = text(entry, "kind");
        *counts.entry(kind.to_string()).or_default() += 1;
        let id = text(entry, "id");
        let problems = if kind == "mixed" {
            let host = hosts[text(entry, "hostCase")];
            assert_eq!(
                sha256(text(host, "source")),
                text(entry, "sourceSha256"),
                "{id}"
            );
            region_problems(language, entry, host)
        } else {
            let corpus_case = sources.get(id);
            let source = entry["source"].as_str().map_or_else(
                || corpus_case.expect("case source").source.clone(),
                str::to_string,
            );
            assert_eq!(sha256(&source), text(entry, "sourceSha256"), "{id}");
            document_problems(language, entry, &source, corpus_case)
        };
        if !problems.is_empty() {
            let name = entry["name"]
                .as_str()
                .map_or_else(String::new, |name| format!(" ({name})"));
            failures.push(format!("{id}{name}: {}", problems.join("; ")));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    let expected_counts: BTreeMap<String, u64> = details["cases"]
        .as_object()
        .expect("case counts")
        .iter()
        .map(|(kind, count)| (kind.clone(), count.as_u64().expect("count")))
        .collect();
    assert_eq!(counts, expected_counts);

    // Every case family is present, and each exercises what it claims.
    let by_kind = |kind: &str| -> Vec<&Value> {
        cases.iter().filter(|entry| entry["kind"] == kind).collect()
    };
    let corpus_files = details["corpus"]["files"].as_object().expect("files").len();
    assert!(by_kind("corpus").len() >= corpus_files);
    assert!(
        by_kind("corpus")
            .iter()
            .any(|entry| entry["upstream"] == "match"),
        "upstream expected trees are checked"
    );
    assert_eq!(
        by_kind("project").len(),
        details["projects"].as_array().expect("projects").len()
    );
    let malformed = by_kind("malformed");
    assert!(!malformed.is_empty() && malformed.iter().all(|entry| entry["clean"] == false));
    let unicode = by_kind("unicode");
    assert!(
        !unicode.is_empty()
            && unicode
                .iter()
                .all(|entry| has_astral_character(text(entry, "source")))
    );
    let mixed = by_kind("mixed");
    assert!(!mixed.is_empty() && mixed.iter().any(|entry| entry["clean"] == true));
    assert!(
        by_kind("construct")
            .iter()
            .all(|entry| entry["clean"] == true)
    );
    let of_language = |list: &Value| {
        list.as_array()
            .expect("input list")
            .iter()
            .filter(|entry| entry["language"] == language)
            .count()
    };
    let expected_inline = of_language(&inputs["constructs"])
        + of_language(&inputs["unicode"])
        + of_language(&inputs["malformed"])
        + of_language(&error_recovery["cases"]);
    assert_eq!(
        by_kind("construct").len() + unicode.len() + malformed.len(),
        expected_inline
    );

    // The claimed construct inventory: every visible named kind of the grammar
    // occurs in a checked tree.
    let seen: HashSet<String> = cases
        .iter()
        .flat_map(|entry| parse_cst_lines(text(entry, "cst")))
        .map(|node| node.kind)
        .collect();
    let unseen: Vec<&str> = details["inventory"]
        .as_array()
        .expect("inventory")
        .iter()
        .map(|kind| kind.as_str().expect("kind"))
        .filter(|kind| !seen.contains(*kind))
        .collect();
    assert!(
        unseen.is_empty(),
        "{language} kinds without a conformance case: {unseen:?}"
    );

    record(&Observation {
        requirement_id: &format!("I195-CONFORMANCE-{}", language.to_uppercase()),
        suffix: "positive-and-negative",
        fixture_id: &format!("planned:conformance:{language}"),
        fixture_file: CONFORMANCE_FIXTURE,
        assertions: &ASSERTIONS,
        test_name: &format!("issue_195_conformance_{}", language.to_lowercase()),
    });
}

#[test]
fn issue_195_conformance_javascript() {
    check_language("JavaScript");
}

#[test]
fn issue_195_conformance_lean() {
    check_language("Lean");
}

#[test]
fn issue_195_conformance_rocq() {
    check_language("Rocq");
}

#[test]
fn issue_195_conformance_rust() {
    check_language("Rust");
}
