//! Issue 195 generative tests: the property compositions, fuzz mutations,
//! metamorphic pairs, edit sequences and kept reproducers of
//! `parity/fixtures/issue-195-generative/`, generated from the conformance
//! inputs by a seeded PRNG and each compared with the tree the native
//! tree-sitter CLI printed for it (`js/scripts/generate-issue-195-generative.mjs`).
//! Every parse must also keep the oracle-free properties of
//! `generative_support`; edit sequences go through the incremental
//! `apply_edit` path and must equal a fresh parse. A run-time fuzz pass (seed
//! `ISSUE_195_GENERATIVE_SEED`, default the fixture seed; count
//! `ISSUE_195_GENERATIVE_CASES`, default 48) checks the properties, the
//! blank-line relation of clean trees and incremental edits on inputs no
//! fixture contains. Where a language parses with its native grammar, a
//! malformed case whose native recovery differs from tree-sitter's must
//! instead match its justified record in `parity/fixtures/native-recovery.json`
//! (`native_recovery_records`). Mirrors `js/tests/issue-195-generative.test.js`.

use std::fs;
use std::path::PathBuf;

use meta_language::{ByteRange, LinkNetwork, ParseConfiguration};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::cst_lines::{
    NetworkIndex, document_grammar_roots, document_oracle_problems, first_difference,
    parse_cst_lines, render_cst_lines,
};
use super::generative_support::{
    EDIT_CASES, EDIT_STEPS, Edit, FUZZ_CASES, METAMORPHIC_CASES, PROPERTY_CASES, RELATIONS, Random,
    apply_text_edit, generate_inputs, property_problems, random_edit, relation_applies,
    relation_holds, relation_transform, seed_sources,
};
use super::issue_195_observations::{GENERATIVE_FIXTURE, Observation, record};
use super::native_recovery_records::NativeRecovery;

const ASSERTIONS: [&str; 8] = [
    "propertyBasedCasesExecuted",
    "fuzzCasesExecuted",
    "metamorphicCasesExecuted",
    "roundTripPropertiesChecked",
    "malformedInputPropertiesChecked",
    "unicodeSpanPropertiesChecked",
    "editSequencePropertiesChecked",
    "independentOracleUsed",
];

fn repository_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(path)
}

fn read(path: &str) -> Vec<u8> {
    let path = repository_path("parity/fixtures/issue-195-generative").join(path);
    fs::read(&path).unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()))
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

fn edit_of(step: &Value) -> Edit {
    let offset = |key: &str| {
        usize::try_from(step[key].as_u64().expect("offset")).expect("offset fits in usize")
    };
    Edit {
        start: offset("start"),
        end: offset("end"),
        replacement: text(step, "replacement").to_string(),
    }
}

/// The rendered public tree of a network.
fn public_tree(network: &LinkNetwork, language: &str) -> String {
    let index = NetworkIndex::new(network);
    render_cst_lines(&document_grammar_roots(&index, language)).0
}

/// Oracle and property problems of one parse of `source`, with its rendered public tree.
fn parse_problems(language: &str, source: &str, cst: &str) -> (Vec<String>, String) {
    let network = parse(source, language);
    let (mut problems, tree) = document_oracle_problems(&network, language, source, cst);
    problems.extend(property_problems(&network, source));
    (problems, tree)
}

/// `parse_problems` of case `id`, those of its recovery record where it has one.
fn case_problems(
    recovery: &NativeRecovery,
    language: &str,
    id: &str,
    source: &str,
    cst: &str,
) -> (Vec<String>, String) {
    let (problems, tree) = parse_problems(language, source, cst);
    let problems = recovery.resolve(id, source, cst, &tree, problems, |text| {
        parse_problems(language, source, text).0
    });
    (problems, tree)
}

/// Whether a CST has an error or missing node.
fn is_malformed(cst: &str) -> bool {
    parse_cst_lines(cst)
        .iter()
        .any(|node| node.error || node.missing)
}

#[test]
fn issue_195_generative_fixtures_record_their_seed_and_oracle_and_the_seed_regenerates_their_inputs()
 {
    let manifest = read_json("manifest.json");
    let reproducers = read_json("reproducers.json");
    let lock: Value = serde_json::from_slice(
        &fs::read(repository_path("rust/src/data/grammar-lock.json")).expect("grammar lock"),
    )
    .expect("grammar lock JSON");
    assert_eq!(manifest["oracle"]["tool"], lock["treeSitterCli"]);
    assert_eq!(
        text(&manifest["inputs"], "reproducers.json"),
        sha256(read("reproducers.json"))
    );
    assert_eq!(
        text(&manifest["inputs"], "issue-195-conformance/manifest.json"),
        sha256(
            fs::read(repository_path(
                "parity/fixtures/issue-195-conformance/manifest.json"
            ))
            .expect("conformance manifest")
        )
    );
    let counts = &manifest["counts"];
    for (key, count) in [
        ("property", PROPERTY_CASES),
        ("fuzz", FUZZ_CASES),
        ("metamorphic", METAMORPHIC_CASES),
        ("edit", EDIT_CASES),
        ("editSteps", EDIT_STEPS),
    ] {
        assert_eq!(counts[key].as_u64(), Some(count as u64), "{key}");
    }
    let seed = text(&manifest, "seed");
    for (language, details) in manifest["languages"].as_object().expect("languages") {
        let file = text(details, "file");
        assert_eq!(sha256(read(file)), text(details, "sha256"), "{language}");
        let fixture = read_json(file);
        assert_eq!(fixture["seed"], seed);
        let seeds = seed_sources(language);
        assert_eq!(
            details["seedSources"].as_u64(),
            Some(seeds.len() as u64),
            "{language} seed sources"
        );
        let mut regenerated: Vec<(String, String, Option<String>, Vec<Edit>)> =
            generate_inputs(language, &seeds, seed)
                .into_iter()
                .map(|input| (input.id, input.source, input.variant, input.steps))
                .collect();
        for entry in reproducers["cases"].as_array().expect("reproducers") {
            if entry["language"] == language.as_str() {
                regenerated.push((
                    format!("reproducer/{}", text(entry, "id")),
                    text(entry, "source").to_string(),
                    None,
                    Vec::new(),
                ));
            }
        }
        let stored: Vec<(String, String, Option<String>, Vec<Edit>)> = fixture["cases"]
            .as_array()
            .expect("cases")
            .iter()
            .map(|entry| {
                (
                    text(entry, "id").to_string(),
                    text(entry, "source").to_string(),
                    entry["variant"].as_str().map(ToOwned::to_owned),
                    entry["steps"]
                        .as_array()
                        .map(|steps| steps.iter().map(edit_of).collect())
                        .unwrap_or_default(),
                )
            })
            .collect();
        assert!(
            stored == regenerated,
            "{language} inputs are what seed {seed} generates"
        );
    }
}

#[test]
fn issue_195_generative_prng_and_edits_are_the_ones_the_javascript_suite_uses() {
    let manifest = read_json("manifest.json");
    let vectors = manifest["prngVectors"].as_object().expect("PRNG vectors");
    assert_eq!(
        vectors.len(),
        manifest["languages"].as_object().expect("languages").len()
    );
    for (stream, vector) in vectors {
        let mut random = Random::new(stream);
        let drawn: Vec<u64> = (0..4).map(|_| u64::from(random.next_u32())).collect();
        let expected: Vec<u64> = vector
            .as_array()
            .expect("vector")
            .iter()
            .map(|value| value.as_u64().expect("u32"))
            .collect();
        assert_eq!(drawn, expected, "{stream}");
    }
    let source = "a𝒳b\r\n";
    let edit = random_edit(&mut Random::from_number(7), source, "Rust");
    assert!(source.is_char_boundary(edit.start) && source.is_char_boundary(edit.end));
    let edit = Edit {
        start: 1,
        end: 3,
        replacement: "e".to_string(),
    };
    assert_eq!(apply_text_edit("héllo", &edit), "hello");
}

#[test]
fn issue_195_generative_property_checks_reject_a_network_that_breaks_them() {
    let network = parse("let a = 1;\n", "JavaScript");
    assert_eq!(
        property_problems(&network, "let a = 1;\n"),
        [] as [String; 0]
    );
    assert_ne!(
        property_problems(&network, "let a = 2;\n"),
        [] as [String; 0]
    );
    assert_ne!(
        property_problems(&network, "let a = 1;\n\n"),
        [] as [String; 0]
    );
    let base = public_tree(&network, "JavaScript");
    let variant = public_tree(&parse("\n\nlet a = 1;\n", "JavaScript"), "JavaScript");
    assert!(relation_holds("prepend-blank-lines", &base, &variant));
    assert!(!relation_holds("prepend-blank-lines", &base, &base));
}

/// Applies the steps of an edit case through the incremental path, checking
/// each against its oracle, the properties and a fresh parse.
fn edit_sequence_problems(recovery: &NativeRecovery, language: &str, entry: &Value) -> Vec<String> {
    let id = text(entry, "id");
    let mut failures = Vec::new();
    let mut source = text(entry, "source").to_string();
    let mut network = parse(&source, language);
    for (position, step) in entry["steps"].as_array().expect("steps").iter().enumerate() {
        let edit = edit_of(step);
        source = apply_text_edit(&source, &edit);
        if !network.apply_edit(ByteRange::new(edit.start, edit.end), &edit.replacement) {
            failures.push(format!("step {position}: apply_edit refused {edit:?}"));
            break;
        }
        let step_problems = |cst: &str| {
            let (mut problems, tree) = document_oracle_problems(&network, language, &source, cst);
            problems.extend(property_problems(&network, &source));
            (problems, tree)
        };
        let oracle = text(step, "cst");
        let (problems, tree) = step_problems(oracle);
        let mut problems = recovery.resolve(
            &format!("{id} step {position}"),
            &source,
            oracle,
            &tree,
            problems,
            |cst| step_problems(cst).0,
        );
        if tree != public_tree(&parse(&source, language), language) {
            problems.push("the incremental tree differs from a fresh parse".to_string());
        }
        if !problems.is_empty() {
            failures.push(format!(
                "step {position} {source:?}: {}",
                problems.join("; ")
            ));
        }
    }
    failures
}

fn runtime_seed(default: &str) -> String {
    std::env::var("ISSUE_195_GENERATIVE_SEED").unwrap_or_else(|_| default.to_string())
}

fn runtime_cases() -> usize {
    std::env::var("ISSUE_195_GENERATIVE_CASES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(48)
}

/// Appends a run-time case (its base source and edits) as a JSON line to the
/// file named by `ISSUE_195_GENERATIVE_TRACE`, before the case is parsed, so a
/// case that crashes a grammar can be replayed outside the suite. Off by default.
fn trace_runtime_case(language: &str, base: &str, edits: &[Edit]) {
    let Some(path) = std::env::var_os("ISSUE_195_GENERATIVE_TRACE") else {
        return;
    };
    let edits: Vec<Value> = edits
        .iter()
        .map(|edit| {
            serde_json::json!({
                "start": edit.start,
                "end": edit.end,
                "replacement": edit.replacement,
            })
        })
        .collect();
    let line = serde_json::json!({ "language": language, "base": base, "edits": edits });
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("trace file");
    std::io::Write::write_all(&mut file, format!("{line}\n").as_bytes()).expect("trace line");
}

/// The run-time fuzz pass: new inputs from the run seed, checked for the
/// properties, the blank-line relation and incremental edits.
fn runtime_problems(language: &str, seed: &str, cases: usize) -> (Vec<String>, usize) {
    let mut random = Random::new(&format!("{seed}:{language}:runtime"));
    let seeds = seed_sources(language);
    let mut failures = Vec::new();
    let mut clean_cases = 0;
    for index in 0..cases {
        let base = random.pick(&seeds).source.clone();
        let mut source = base.clone();
        let mut network = parse(&source, language);
        let mut problems = Vec::new();
        let mut edits = Vec::new();
        for _ in 0..=random.int(6) {
            let edit = random_edit(&mut random, &source, language);
            source = apply_text_edit(&source, &edit);
            edits.push(edit.clone());
            trace_runtime_case(language, &base, &edits);
            if !network.apply_edit(ByteRange::new(edit.start, edit.end), &edit.replacement) {
                problems.push(format!("apply_edit refused {edit:?}"));
            }
        }
        let fresh = parse(&source, language);
        let tree = public_tree(&fresh, language);
        problems.extend(property_problems(&fresh, &source));
        problems.extend(property_problems(&network, &source));
        let incremental = public_tree(&network, language);
        if incremental != tree {
            problems.push(format!(
                "the incremental tree of {base:?} with edits {edits:?} differs from a fresh parse at {}",
                first_difference(&incremental, &tree)
            ));
        }
        let variant_source = relation_transform("prepend-blank-lines", &source);
        let variant = parse(&variant_source, language);
        problems.extend(property_problems(&variant, &variant_source));
        let clean = fresh.verify_full_match(None).is_clean();
        clean_cases += usize::from(clean);
        if relation_applies("prepend-blank-lines", &source, clean)
            && !relation_holds(
                "prepend-blank-lines",
                &tree,
                &public_tree(&variant, language),
            )
        {
            problems.push("prepend-blank-lines does not hold".to_string());
        }
        if !problems.is_empty() {
            failures.push(format!(
                "seed {seed:?} case {index} source {source:?}: {} (keep it in parity/fixtures/issue-195-generative/reproducers.json)",
                problems.join("; ")
            ));
        }
    }
    (failures, clean_cases)
}

fn check_language(language: &str) {
    let manifest = read_json("manifest.json");
    let reproducers = read_json("reproducers.json");
    let fixture = read_json(text(&manifest["languages"][language], "file"));
    assert_eq!(fixture["language"], language);
    let cases = fixture["cases"].as_array().expect("cases");
    let recovery = NativeRecovery::new("generative", language);
    let mut failures = Vec::new();
    for entry in cases {
        let id = text(entry, "id");
        let source = text(entry, "source");
        let (problems, tree) = case_problems(&recovery, language, id, source, text(entry, "cst"));
        if !problems.is_empty() {
            failures.push(format!("{id} {source:?}: {}", problems.join("; ")));
        }
        match text(entry, "kind") {
            "metamorphic" => {
                let variant = text(entry, "variant");
                let (problems, variant_tree) = case_problems(
                    &recovery,
                    language,
                    &format!("{id} (variant)"),
                    variant,
                    text(entry, "variantCst"),
                );
                if !problems.is_empty() {
                    failures.push(format!(
                        "{id} (variant) {variant:?}: {}",
                        problems.join("; ")
                    ));
                }
                let relation = text(entry, "relation");
                if Some(relation_holds(relation, &tree, &variant_tree))
                    != entry["relationHolds"].as_bool()
                {
                    failures.push(format!(
                        "{id} (relation): {relation} holds for the public trees iff it holds for the oracle"
                    ));
                }
            }
            "edit" => failures.extend(
                edit_sequence_problems(&recovery, language, entry)
                    .into_iter()
                    .map(|problem| format!("{id} {problem}")),
            ),
            _ => {}
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert_eq!(
        recovery.unused(),
        [] as [String; 0],
        "{language} recovery records without a generative case"
    );

    // Every family is present, and each exercises what it claims.
    let of_kind = |kind: &str| -> Vec<&Value> {
        cases.iter().filter(|entry| entry["kind"] == kind).collect()
    };
    assert_eq!(of_kind("property").len(), PROPERTY_CASES);
    assert_eq!(of_kind("fuzz").len(), FUZZ_CASES);
    assert_eq!(
        of_kind("metamorphic").len(),
        METAMORPHIC_CASES * RELATIONS.len()
    );
    assert_eq!(of_kind("edit").len(), EDIT_CASES);
    assert!(
        of_kind("edit")
            .iter()
            .all(|entry| entry["steps"].as_array().map(Vec::len) == Some(EDIT_STEPS))
    );
    assert!(
        of_kind("metamorphic")
            .iter()
            .any(|entry| entry["relationHolds"] == true)
    );
    let kept = reproducers["cases"]
        .as_array()
        .expect("reproducers")
        .iter()
        .filter(|entry| entry["language"] == language)
        .count();
    assert_eq!(of_kind("reproducer").len(), kept);
    let malformed: Vec<&Value> = cases
        .iter()
        .filter(|entry| entry["clean"] == false)
        .collect();
    assert_ne!(malformed, [] as [&Value; 0]);
    assert!(
        malformed
            .iter()
            .all(|entry| is_malformed(text(entry, "cst")))
    );
    assert!(
        cases.iter().any(|entry| text(entry, "source")
            .chars()
            .any(|character| u32::from(character) >= 0x10000)),
        "astral characters"
    );
    assert!(
        cases
            .iter()
            .any(|entry| text(entry, "source").contains('\r')),
        "carriage returns"
    );

    record(&Observation {
        requirement_id: &format!("I195-GENERATIVE-{}", language.to_uppercase()),
        suffix: "positive-and-negative",
        fixture_id: &format!("planned:generative:{language}"),
        fixture_file: GENERATIVE_FIXTURE,
        assertions: &ASSERTIONS,
        test_name: &format!("issue_195_generative_{}", language.to_lowercase()),
    });
}

/// The run-time pass of a language: inputs generated from the manifest seed
/// (or `ISSUE_195_GENERATIVE_SEED`), a test of its own so that it runs beside
/// the fixture cases, not after them (native Lean's malformed inputs repair
/// over many rounds, and one test of both outran the job's timeout).
fn check_runtime(language: &str) {
    let manifest = read_json("manifest.json");
    let seed = runtime_seed(text(&manifest, "seed"));
    let total = runtime_cases();
    let (failures, clean_cases) = runtime_problems(language, &seed, total);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(
        clean_cases > 0 && clean_cases < total,
        "{clean_cases} of {total} run-time cases are clean"
    );
}

#[test]
fn issue_195_generative_javascript() {
    check_language("JavaScript");
}

#[test]
fn issue_195_generative_javascript_runtime() {
    check_runtime("JavaScript");
}

#[test]
fn issue_195_generative_lean() {
    check_language("Lean");
}

#[test]
fn issue_195_generative_lean_runtime() {
    check_runtime("Lean");
}

#[test]
fn issue_195_generative_rocq() {
    check_language("Rocq");
}

#[test]
fn issue_195_generative_rocq_runtime() {
    check_runtime("Rocq");
}

#[test]
fn issue_195_generative_rust() {
    check_language("Rust");
}

#[test]
fn issue_195_generative_rust_runtime() {
    check_runtime("Rust");
}

/// Edit sequences the run-time pass found, kept as reproducers: after each
/// incremental edit the public tree must equal a fresh parse. The JavaScript
/// sequence (seed "d") made native incremental error recovery keep an ERROR
/// node a fresh parse does not produce; the Rust one (the fixture seed) put a
/// zero-width MISSING node after a sibling that starts at the same byte.
#[test]
fn issue_195_generative_kept_edit_sequences_match_a_fresh_parse() {
    type Sequence = (
        &'static str,
        &'static str,
        &'static [(usize, usize, &'static str)],
    );
    let sequences: [Sequence; 2] = [
        (
            "JavaScript",
            "\nif (x)\n  y;\nelse if (a)\n  b;\n\nif (a) {\n  c;\n  d;\n} else {\n  e;\n}\n",
            &[
                (61, 66, "日本"),
                (35, 67, ""),
                (12, 12, "`"),
                (29, 29, "/*"),
                (29, 29, "=>"),
                (27, 40, ""),
            ],
        ),
        (
            "Rust",
            "\nstd::sizeof::<u32>();\nfoo::<8>();\n",
            &[(24, 35, ""), (3, 5, ""), (13, 16, "")],
        ),
    ];
    for (language, base, edits) in sequences {
        let mut source = base.to_string();
        let mut network = parse(&source, language);
        for (start, end, replacement) in edits {
            let edit = Edit {
                start: *start,
                end: *end,
                replacement: (*replacement).to_string(),
            };
            source = apply_text_edit(&source, &edit);
            assert!(network.apply_edit(ByteRange::new(edit.start, edit.end), replacement));
            let fresh = parse(&source, language);
            assert_eq!(
                public_tree(&network, language),
                public_tree(&fresh, language),
                "{language} {source:?}"
            );
            assert_eq!(property_problems(&network, &source), [] as [String; 0]);
        }
    }
}
