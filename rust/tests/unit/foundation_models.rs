//! Foundation neutrality (requirement I195-VISION-FOUNDATION-NEUTRAL): integer
//! and overflow models, universes, effects, proof systems and logics are
//! distinct data with explicit correspondences, the translations' encodings
//! are justified by those correspondences, no production source builds in a
//! consumer's own foundation, and the check rejects a hard-coded universal
//! logic. The JavaScript twin is js/tests/foundation-models.test.js.

use std::path::{Path, PathBuf};

use meta_language::{
    FoundationCorrespondence, FoundationRegister, check_foundation_models,
    foundation_correspondences, foundation_model, foundation_register, language_foundation_models,
    translate_program,
};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

const REGISTER: &str = "parity/foundation-models.json";
const LANGUAGES: [&str; 4] = ["JavaScript", "Rust", "Lean", "Rocq"];
const FOUNDATION_ENCODINGS: [&str; 4] = [
    "machine-integer",
    "numbers",
    "non-aborting-executions",
    "theorem-properties",
];
/// The files that record which foundation each downstream consumer brings.
const CONSUMER_REGISTRIES: [&str; 2] = ["rust/src/parity.rs", "rust/src/parity_fixtures.rs"];

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-VISION-FOUNDATION-NEUTRAL",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-vision-foundation-neutral",
        fixture_file: REGISTER,
        assertions,
        test_name,
    });
}

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn register_copy() -> Value {
    let text = std::fs::read_to_string(repository().join(REGISTER)).expect("the register reads");
    serde_json::from_str(&text).expect("the register is JSON")
}

/// The problem kinds of a register, or `parse` when the runtime refuses to read it.
fn kinds(register: Value) -> Vec<&'static str> {
    serde_json::from_value::<FoundationRegister>(register).map_or_else(
        |_| vec!["parse"],
        |register| {
            check_foundation_models(&register)
                .into_iter()
                .map(|problem| problem.kind)
                .collect()
        },
    )
}

fn model_mut<'a>(register: &'a mut Value, id: &str) -> &'a mut Value {
    register["models"]
        .as_array_mut()
        .expect("models")
        .iter_mut()
        .find(|model| model["id"] == id)
        .expect("the model is registered")
}

fn ids(language: &str) -> Vec<&'static str> {
    language_foundation_models(language, None)
        .into_iter()
        .map(|model| model.id.as_str())
        .collect()
}

#[test]
fn foundations_are_data_and_every_language_records_its_own() {
    let register = foundation_register();
    let embedded = include_str!("../../src/data/foundation-models.json");
    let canonical = std::fs::read_to_string(repository().join(REGISTER)).expect("register");
    assert_eq!(embedded, canonical, "the embedded copy is the register");
    assert_eq!(check_foundation_models(register), []);
    let families: Vec<&str> = register
        .families
        .iter()
        .map(|family| family.id.as_str())
        .collect();
    assert_eq!(
        families,
        [
            "integer-model",
            "overflow-model",
            "universe-model",
            "effect-model",
            "proof-system",
            "logic-model"
        ]
    );
    for family in &families {
        let count = register
            .models
            .iter()
            .filter(|model| model.family == *family)
            .count();
        assert!(count >= 2, "{family} has several models");
        for language in LANGUAGES {
            assert!(
                !language_foundation_models(language, Some(family)).is_empty(),
                "{language} records its {family}"
            );
        }
    }
    let proof_systems: Vec<Vec<&str>> = LANGUAGES
        .iter()
        .map(|language| {
            language_foundation_models(language, Some("proof-system"))
                .into_iter()
                .map(|model| model.id.as_str())
                .collect()
        })
        .collect();
    assert_eq!(
        proof_systems,
        [
            vec!["proof-system.no-proof-checker"],
            vec!["proof-system.no-proof-checker"],
            vec!["proof-system.lean-kernel"],
            vec!["proof-system.rocq-kernel"],
        ]
    );
    assert_eq!(
        foundation_model("overflow-model.abort")
            .expect("abort")
            .family,
        "overflow-model"
    );
    let kernels =
        foundation_correspondences("proof-system.rocq-kernel", "proof-system.lean-kernel");
    assert_eq!(kernels[0].kind, "restatement");
    observe(
        &["foundationsRepresentedAsData"],
        "foundations_are_data_and_every_language_records_its_own",
    );
}

fn justified(encoding: &str, source: &[&str], target: &[&str]) -> bool {
    let register = foundation_register();
    let by_correspondence = |entry: &&FoundationCorrespondence| {
        entry.translation_encodings.iter().any(|id| id == encoding)
            && source.contains(&entry.from.as_str())
            && target.contains(&entry.to.as_str())
    };
    register
        .correspondences
        .iter()
        .any(|entry| by_correspondence(&entry))
        || register.distinctions.iter().any(|entry| {
            entry.translation_encodings.iter().any(|id| id == encoding)
                && entry
                    .models
                    .iter()
                    .any(|model| source.contains(&model.as_str()))
        })
}

#[test]
fn distinct_models_stay_distinct_and_translation_encodings_are_justified() {
    let homonyms: Vec<String> = foundation_register()
        .distinctions
        .iter()
        .map(|entry| entry.models.join(" / "))
        .collect();
    for pair in [
        "effect-model.abort / effect-model.panic-with-default-value",
        "logic-model.constructive-propositions / logic-model.classical-propositions",
        "proof-system.bounded-property-check / proof-system.lean-kernel",
    ] {
        assert!(
            homonyms.iter().any(|homonym| homonym == pair),
            "{pair} is recorded as distinct"
        );
    }

    let mut merged = register_copy();
    let wrapping = model_mut(&mut merged, "overflow-model.wrapping")["properties"].clone();
    model_mut(&mut merged, "overflow-model.saturating")["properties"] = wrapping;
    assert_eq!(kinds(merged), ["indistinct-models"]);
    let mut exact = register_copy();
    exact["correspondences"][0]["kind"] = "exact".into();
    assert_eq!(kinds(exact), ["correspondence-kind"]);
    let mut unconditioned = register_copy();
    unconditioned["correspondences"][0]["condition"] = "".into();
    assert_eq!(kinds(unconditioned), ["unconditioned-correspondence"]);

    let stages: Value = serde_json::from_str(
        &std::fs::read_to_string(repository().join("parity/fixtures/translation-stages.json"))
            .expect("translation stages"),
    )
    .expect("translation stages are JSON");
    let lean = stages["programs"]
        .as_array()
        .expect("programs")
        .iter()
        .find(|program| program["name"] == "check/lean-theorem-cases")
        .and_then(|program| program["source"].as_str())
        .expect("the Lean theorem program");
    let rust =
        "fn add(a: u8, b: u8) -> u8 { a + b }\nfn main() { println!(\"{}\", add(200, 50)); }\n";
    let cases = [
        ("Rust", "JavaScript", rust),
        ("Rust", "Lean", rust),
        ("Rust", "Rocq", rust),
        ("Lean", "JavaScript", lean),
        ("Lean", "Rust", lean),
    ];
    let mut covered = std::collections::BTreeSet::new();
    for (source, target, program) in cases {
        let translation = translate_program(program, source, target).expect("translation");
        let semantics = translation.semantics().expect("a semantic translation");
        let recorded = semantics
            .encodings
            .iter()
            .map(|encoding| encoding.id.as_str())
            .chain(
                semantics
                    .assumptions
                    .iter()
                    .map(|assumption| assumption.id.as_str()),
            );
        for id in recorded {
            let id = id.split(':').next().unwrap_or(id);
            if FOUNDATION_ENCODINGS.contains(&id) {
                assert!(
                    justified(id, &ids(source), &ids(target)),
                    "{source} -> {target} encoding {id} is justified by a correspondence between their models"
                );
                covered.insert(id.to_owned());
            }
        }
    }
    let mut expected: Vec<&str> = FOUNDATION_ENCODINGS.to_vec();
    expected.sort_unstable();
    assert_eq!(covered.into_iter().collect::<Vec<_>>(), expected);
    observe(
        &["distinctModelsKeptDistinct"],
        "distinct_models_stay_distinct_and_translation_encodings_are_justified",
    );
}

#[test]
fn the_check_rejects_a_hard_coded_universal_logic() {
    let mut marked = register_copy();
    model_mut(&mut marked, "logic-model.belnap-four-valued")["universal"] = true.into();
    assert_eq!(
        kinds(marked),
        ["parse"],
        "the runtime does not read a universality mark"
    );

    let mut hub = register_copy();
    let hub_id = "logic-model.belnap-four-valued";
    let logics: Vec<String> = hub["models"]
        .as_array()
        .expect("models")
        .iter()
        .filter(|model| model["family"] == "logic-model" && model["id"] != hub_id)
        .map(|model| model["id"].as_str().expect("id").to_owned())
        .collect();
    let correspondences = hub["correspondences"]
        .as_array_mut()
        .expect("correspondences");
    correspondences.retain(|entry| {
        !(entry["to"] == hub_id && logics.iter().any(|id| entry["from"] == id.as_str()))
    });
    for id in &logics {
        correspondences.push(serde_json::json!({
            "from": id,
            "to": hub_id,
            "kind": "embedding",
            "condition": "Every value is read as a four-valued truth value.",
        }));
    }
    assert_eq!(kinds(hub), ["universal-model"]);

    let mut imposed = register_copy();
    for language in imposed["languages"].as_array_mut().expect("languages") {
        let models = language["models"].as_array_mut().expect("models");
        models.retain(|id| !id.as_str().expect("id").starts_with("proof-system."));
        models.push("proof-system.lean-kernel".into());
    }
    assert_eq!(kinds(imposed), ["universal-model"]);

    let mut unrecorded = register_copy();
    unrecorded["languages"][0]["models"]
        .as_array_mut()
        .expect("models")
        .retain(|id| !id.as_str().expect("id").starts_with("proof-system."));
    assert_eq!(kinds(unrecorded), ["language-family"]);
    observe(
        &["hardCodedFoundationRejected"],
        "the_check_rejects_a_hard_coded_universal_logic",
    );
}

/// Whether a line names a consumer's own foundation: `relative-meta-logic` in
/// any spelling, or the word `RML`.
fn names_consumer_foundation(line: &str) -> bool {
    let squeezed: String = line
        .to_lowercase()
        .chars()
        .filter(|character| !matches!(character, '-' | '_' | ' '))
        .collect();
    let word = |character: char| character.is_alphanumeric() || character == '_';
    squeezed.contains("relativemetalogic")
        || line.match_indices("RML").any(|(start, _)| {
            !line[..start].chars().next_back().is_some_and(word)
                && !line[start + 3..].chars().next().is_some_and(word)
        })
}

fn consumer_references(base: &Path, directory: &Path, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.map(|entry| entry.expect("directory entry")) {
        let path = entry.path();
        let relative = path
            .strip_prefix(base)
            .expect("inside the base")
            .to_string_lossy()
            .replace('\\', "/");
        if path.is_dir() {
            if entry.file_name() != "vendor" {
                consumer_references(base, &path, found);
            }
            continue;
        }
        let scanned = ["js", "mjs", "ts", "rs", "json", "lino"]
            .iter()
            .any(|extension| path.extension().is_some_and(|actual| actual == *extension));
        if !scanned || CONSUMER_REGISTRIES.contains(&relative.as_str()) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for (index, line) in text.lines().enumerate() {
            if names_consumer_foundation(line) {
                found.push(format!("{relative}:{}", index + 1));
            }
        }
    }
}

fn scan(base: &Path) -> Vec<String> {
    let mut found = Vec::new();
    for root in ["js/src", "rust/src"] {
        consumer_references(base, &base.join(root), &mut found);
    }
    found
}

#[test]
fn no_production_source_names_a_consumer_foundation() {
    let base = repository().canonicalize().expect("repository");
    assert_eq!(scan(&base), Vec::<String>::new());
    let directory =
        std::env::temp_dir().join(format!("meta-language-foundations-{}", std::process::id()));
    std::fs::create_dir_all(directory.join("js/src")).expect("js/src");
    std::fs::create_dir_all(directory.join("rust/src")).expect("rust/src");
    std::fs::write(
        directory.join("js/src/logic.js"),
        "// the RML truth tables are the logic of every proof\nexport const logic = 1;\n",
    )
    .expect("logic.js");
    std::fs::write(
        directory.join("rust/src/parity.rs"),
        "// upstream: relative-meta-logic\n",
    )
    .expect("parity.rs");
    let found = scan(&directory);
    std::fs::remove_dir_all(&directory).expect("cleanup");
    assert_eq!(found, ["js/src/logic.js:1"]);
    assert!(!names_consumer_foundation("let html = HTML_RMLS;"));
    observe(
        &["noRelativeMetaLogicSpecificSyntax"],
        "no_production_source_names_a_consumer_foundation",
    );
}
