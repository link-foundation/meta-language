use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn repository_json(relative_path: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    serde_json::from_str(&fs::read_to_string(path).expect("shared JSON file is readable"))
        .expect("shared JSON file is valid")
}

fn contains_hand_edited_completion(value: &Value) -> bool {
    match value {
        Value::Object(fields) => fields.iter().any(|(key, child)| {
            matches!(key.as_str(), "status" | "complete" | "completed")
                || contains_hand_edited_completion(child)
        }),
        Value::Array(values) => values.iter().any(contains_hand_edited_completion),
        _ => false,
    }
}

#[test]
fn issue_195_manifest_has_atomic_cross_runtime_traceability() {
    let manifest = repository_json("../parity/issue-195-requirements.json");
    assert_eq!(manifest["schemaVersion"], 1);
    assert_eq!(manifest["issue"], 195);

    let requirements = manifest["atomicRequirements"]
        .as_array()
        .expect("atomic requirement rows");
    assert!(requirements.len() >= 150);

    let fixture_catalog = manifest["fixtureCatalog"]
        .as_object()
        .expect("fixture catalog");
    let mut requirement_ids = BTreeSet::new();
    let mut test_ids = BTreeSet::new();
    for requirement in requirements {
        let id = requirement["id"].as_str().expect("stable requirement id");
        assert!(requirement_ids.insert(id), "duplicate requirement id {id}");
        assert!(id.starts_with("I195-"), "unexpected requirement id {id}");
        assert!(requirement["source"]
            .as_str()
            .expect("source permalink")
            .starts_with("https://github.com/link-foundation/meta-language/"));
        assert!(!contains_hand_edited_completion(requirement));

        let scope = requirement["scope"].as_object().expect("atomic scope");
        for field in [
            "language",
            "version",
            "edition",
            "construct",
            "aliases",
            "extensions",
        ] {
            assert!(scope.contains_key(field), "{id} scope is missing {field}");
        }

        let implementations = requirement["implementationEntryPoints"]
            .as_object()
            .expect("implementation entry-point cells");
        let required_runtimes = requirement["requiredRuntimes"]
            .as_array()
            .expect("required runtimes");
        let verifications = requirement["verifications"]
            .as_array()
            .expect("verification cells");
        assert!(
            !required_runtimes.is_empty(),
            "{id} has no required runtimes"
        );
        assert!(!verifications.is_empty(), "{id} has no verification cells");

        for runtime in required_runtimes {
            let runtime = runtime.as_str().expect("runtime name");
            assert!(
                implementations.contains_key(runtime),
                "{id} has no {runtime} implementation cell"
            );
            assert!(
                verifications
                    .iter()
                    .any(|cell| cell["runtime"].as_str() == Some(runtime)),
                "{id} has no {runtime} verification"
            );
        }

        for cell in verifications {
            let test_id = cell["testId"].as_str().expect("stable test id");
            assert!(test_ids.insert(test_id), "duplicate test id {test_id}");
            assert!(
                !cell["assertions"]
                    .as_array()
                    .expect("assertion list")
                    .is_empty(),
                "{test_id} has no observable assertions"
            );
            for fixture_id in cell["fixtureIds"].as_array().expect("fixture IDs") {
                let fixture_id = fixture_id.as_str().expect("fixture ID");
                assert!(
                    fixture_catalog.contains_key(fixture_id),
                    "{test_id} has dangling fixture {fixture_id}"
                );
            }
        }
    }
}

#[test]
fn issue_195_manifest_covers_the_full_inventory_and_translation_matrix() {
    let manifest = repository_json("../parity/issue-195-requirements.json");
    let inventory = repository_json("../parity/language-grammar-inventory.json");
    let requirements = manifest["atomicRequirements"]
        .as_array()
        .expect("atomic requirement rows");

    let by_area = requirements.iter().fold(
        BTreeMap::<&str, Vec<&Value>>::new(),
        |mut groups, requirement| {
            groups
                .entry(requirement["area"].as_str().expect("requirement area"))
                .or_default()
                .push(requirement);
            groups
        },
    );
    assert_eq!(
        by_area["default-cst"].len(),
        inventory["languages"]
            .as_array()
            .expect("language inventory")
            .len()
    );
    assert_eq!(by_area["directed-translation"].len(), 12);

    let translation_pairs = by_area["directed-translation"]
        .iter()
        .map(|requirement| {
            requirement["scope"]["language"]
                .as_str()
                .expect("translation pair")
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(translation_pairs.len(), 12);
    for requirement in &by_area["directed-translation"] {
        let runtimes = requirement["requiredRuntimes"]
            .as_array()
            .expect("translation runtimes")
            .iter()
            .filter_map(Value::as_str)
            .collect::<BTreeSet<_>>();
        assert_eq!(runtimes, BTreeSet::from(["javascript", "rust"]));
    }
}
