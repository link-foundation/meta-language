use meta_language::{
    GLOBAL_CONCEPT_PROOF_METHOD, GlobalConceptOptions, GlobalConceptSource,
    create_global_concept_space, global_concept_space_to_links, read_global_concept_space_links,
};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../parity/fixtures/global-concept-space.json"
    ))
    .unwrap()
}

fn sources(data: &Value) -> Vec<GlobalConceptSource> {
    data["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| GlobalConceptSource::from_json(source).unwrap())
        .collect()
}

fn settings(data: &Value) -> GlobalConceptOptions {
    GlobalConceptOptions {
        records: Some(data["records"].clone()),
        previous: None,
        equivalences: data["equivalences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|pair| {
                [
                    pair[0].as_str().unwrap().to_owned(),
                    pair[1].as_str().unwrap().to_owned(),
                ]
            })
            .collect(),
    }
}

fn observe(requirement_id: &str, assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id,
        suffix: "behavior",
        fixture_id: "global-concept-space",
        fixture_file: "parity/fixtures/global-concept-space.json",
        assertions,
        test_name,
    });
}

#[test]
fn copied_global_productions_share_only_closed_recursive_correspondences() {
    let data = fixture();
    let input = sources(&data);
    let before: Vec<_> = input.iter().map(|source| source.grammar.clone()).collect();
    let space = create_global_concept_space(&input, &settings(&data)).unwrap();
    assert_eq!(space["sources"].as_array().unwrap().len(), 2);
    assert_eq!(space["mappings"].as_object().unwrap().len(), 6);
    assert_eq!(space["concepts"].as_array().unwrap().len(), 4);
    let concepts = space["concepts"].as_array().unwrap();
    assert_eq!(
        concepts
            .iter()
            .map(|concept| concept["origins"].as_array().unwrap().len())
            .sum::<usize>(),
        6
    );
    assert_eq!(
        concepts
            .iter()
            .map(|concept| concept["definitions"].as_array().unwrap().len())
            .sum::<usize>(),
        6
    );
    assert_eq!(
        space["mappings"]["first:root"],
        space["mappings"]["second:page"]
    );
    assert_eq!(
        space["mappings"]["first:atom"],
        space["mappings"]["second:letter"]
    );
    assert_ne!(
        space["mappings"]["first:unique"],
        space["mappings"]["second:unique"]
    );
    assert_eq!(space["failures"], json!([]));
    assert_eq!(
        input
            .iter()
            .map(|source| source.grammar.clone())
            .collect::<Vec<_>>(),
        before
    );
    let proof = space["proofs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|proof| proof["first"] == "first:root")
        .unwrap();
    assert_eq!(proof["method"], GLOBAL_CONCEPT_PROOF_METHOD);
    assert_eq!(
        proof["correspondence"],
        json!([
            ["first:root", "second:page"],
            ["first:atom", "second:letter"]
        ])
    );
    let root = concepts
        .iter()
        .find(|concept| concept["id"] == space["mappings"]["first:root"])
        .unwrap();
    assert_eq!(root["origins"][0]["language"], "first-language");
    assert_eq!(root["origins"][0]["edition"], "1");
    assert_eq!(
        root["origins"][0]["sourceNames"],
        json!([{"source":"upstream:first","name":"original_root"}])
    );
    assert_eq!(space, data["expectedSpace"]);
    let test_name = "copied_global_productions_share_only_closed_recursive_correspondences";
    observe(
        "I195-GLOBAL-CONCEPT-SPACE",
        &["allRulesInOneSpace"],
        test_name,
    );
    observe(
        "I195-GLOBAL-CONCEPT-COPY",
        &["copiesBeforeDeduplication"],
        test_name,
    );
    observe(
        "I195-GLOBAL-CONCEPT-REUSE",
        &["reuseEveryVerifiedCorrespondence"],
        test_name,
    );
    observe(
        "I195-GLOBAL-CONCEPT-EQUIVALENCE",
        &["recursiveOneToOneProof"],
        test_name,
    );
    observe(
        "I195-GLOBAL-CONCEPT-PROVENANCE",
        &["preserveQualifiedProvenance"],
        test_name,
    );
}

#[test]
fn ambiguous_and_ungrounded_meanings_cannot_share_global_identities() {
    let data = fixture();
    let input = sources(&data);
    let mut options = settings(&data);
    options.records = Some(json!([]));
    let ungrounded = create_global_concept_space(&input, &options).unwrap();
    assert_eq!(ungrounded["proofs"], json!([]));
    assert_eq!(ungrounded["concepts"].as_array().unwrap().len(), 6);
    assert_eq!(ungrounded["failures"].as_array().unwrap().len(), 2);
    let mut records = data["records"].as_array().unwrap().clone();
    records.push(json!({"id":"fixture.other-letter","definition":"A letter with a separate semantic role.","constraints":["This semantic role is distinct."],"sourceAliases":[{"source":"second","name":"letter"}]}));
    options.records = Some(json!(records));
    let ambiguous = create_global_concept_space(&input, &options).unwrap();
    assert_eq!(
        ambiguous["ambiguities"],
        json!([{"alias":"second:letter","meanings":["fixture.letter","fixture.other-letter"]}])
    );
    assert_ne!(
        ambiguous["mappings"]["first:atom"],
        ambiguous["mappings"]["second:letter"]
    );
    let mut distinct_data = data.clone();
    distinct_data["sources"][1]["grammar"]["rules"][1]["concept"] = json!("fixture.other-letter");
    let mut distinct_records = data["records"].as_array().unwrap().clone();
    distinct_records[1]["sourceAliases"]
        .as_array_mut()
        .unwrap()
        .retain(|alias| alias["source"] != "second");
    distinct_records.push(json!({"id":"fixture.other-letter","definition":"A letter with a separate semantic role.","constraints":["This semantic role is distinct."],"sourceAliases":[{"source":"second","name":"letter"}]}));
    options.records = Some(json!(distinct_records));
    let distinct = create_global_concept_space(&sources(&distinct_data), &options).unwrap();
    assert_ne!(
        distinct["mappings"]["first:atom"],
        distinct["mappings"]["second:letter"]
    );
    assert_ne!(
        distinct["mappings"]["first:root"],
        distinct["mappings"]["second:page"]
    );
    let test_name = "ambiguous_and_ungrounded_meanings_cannot_share_global_identities";
    observe(
        "I195-GLOBAL-CONCEPT-UNPROVED",
        &["rejectUnprovedEquivalence", "preserveAmbiguity"],
        test_name,
    );
    observe(
        "I195-GLOBAL-CONCEPT-DISTINCTIONS",
        &["preserveDistinctMeanings"],
        test_name,
    );
}

#[test]
fn operational_context_and_rule_actions_constrain_global_correspondences() {
    let data = fixture();
    for declarations in [
        json!({"matching":"peg"}),
        json!({"extras":[{"kind":"literal","value":" "}]}),
        json!({"scanners":[{"name":"scanner","tokens":["external"],"operations":[{"operation":"advance"}]}]}),
        json!({"imports":["external-grammar"]}),
    ] {
        let mut changed = data.clone();
        changed["sources"][1]["grammar"]["declarations"] = declarations;
        let space = create_global_concept_space(&sources(&changed), &settings(&data)).unwrap();
        assert_eq!(space["proofs"], json!([]));
        assert_eq!(space["failures"].as_array().unwrap().len(), 2);
    }
    for (field, value) in [
        ("parameters", json!(["value"])),
        ("channel", json!("comment")),
        ("modes", json!(["quoted"])),
        ("action", json!([{"operation":"advance"}])),
    ] {
        let mut changed = data.clone();
        changed["sources"][1]["grammar"]["rules"][1][field] = value;
        let space = create_global_concept_space(&sources(&changed), &settings(&data)).unwrap();
        assert_ne!(
            space["mappings"]["first:atom"],
            space["mappings"]["second:letter"]
        );
        assert_ne!(
            space["mappings"]["first:root"],
            space["mappings"]["second:page"]
        );
    }
    observe(
        "I195-GLOBAL-CONCEPT-EQUIVALENCE",
        &["matchingOperationalContext"],
        "operational_context_and_rule_actions_constrain_global_correspondences",
    );
}

#[test]
fn global_identity_survives_reordering_incremental_import_and_safe_splitting() {
    let data = fixture();
    let input = sources(&data);
    let options = settings(&data);
    let initial = create_global_concept_space(&input, &options).unwrap();
    let mut reverse = data.clone();
    reverse["sources"].as_array_mut().unwrap().reverse();
    for source in reverse["sources"].as_array_mut().unwrap() {
        source["grammar"]["rules"].as_array_mut().unwrap().reverse();
    }
    assert_eq!(
        create_global_concept_space(&sources(&reverse), &options).unwrap()["mappings"],
        initial["mappings"]
    );
    let incremental = GlobalConceptOptions {
        previous: Some(initial.clone()),
        ..options.clone()
    };
    assert_eq!(
        create_global_concept_space(&input, &incremental).unwrap(),
        initial
    );
    let mut third = input[0].clone();
    third.id = "earlier".to_owned();
    third.language = "third-language".to_owned();
    let expanded =
        create_global_concept_space(&[input[0].clone(), input[1].clone(), third], &incremental)
            .unwrap();
    for (alias, identity) in initial["mappings"].as_object().unwrap() {
        assert_eq!(&expanded["mappings"][alias], identity);
    }
    assert_eq!(
        expanded["mappings"]["earlier:atom"],
        initial["mappings"]["first:atom"]
    );
    let mut changed = data.clone();
    changed["sources"][1]["grammar"]["rules"][1]["expression"] =
        json!({"kind":"literal","value":"β"});
    let split = create_global_concept_space(&sources(&changed), &incremental).unwrap();
    assert_eq!(
        split["mappings"]["first:atom"],
        initial["mappings"]["first:atom"]
    );
    assert_ne!(
        split["mappings"]["second:letter"],
        initial["mappings"]["first:atom"]
    );
    observe(
        "I195-GLOBAL-CONCEPT-SPACE",
        &["stableIdentities"],
        "global_identity_survives_reordering_incremental_import_and_safe_splitting",
    );
}

#[test]
fn global_links_reload_source_free_and_reject_invalid_or_omitted_proofs() {
    let data = fixture();
    let options = settings(&data);
    let space = create_global_concept_space(&sources(&data), &options).unwrap();
    let links = global_concept_space_to_links(&space).unwrap();
    assert!(links.contains("(source-alias first%3Aroot upstream%3Afirst original_root)"));
    assert!(links.contains("(concept-proof recursive-structural-bijection"));
    assert!(links.contains("(literal %CE%B1)"));
    assert_eq!(links.matches("(concept-definition ").count(), 6);
    assert_eq!(links, data["expectedLinks"].as_str().unwrap());
    let reloaded = read_global_concept_space_links(&links, &options).unwrap();
    assert_eq!(reloaded, space);
    assert_eq!(global_concept_space_to_links(&reloaded).unwrap(), links);
    assert!(
        read_global_concept_space_links(
            &links.replacen("(literal %CE%B1)", "(literal %CE%B2)", 1),
            &options
        )
        .unwrap_err()
        .contains("unproved global concept correspondence")
    );
    let removed = links
        .lines()
        .filter(|line| !line.starts_with("(concept-proof "))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        read_global_concept_space_links(&removed, &options)
            .unwrap_err()
            .contains("lacks its proof witnesses")
    );
    observe(
        "I195-GLOBAL-CONCEPT-SERIALIZATION",
        &["linksRoundTrip"],
        "global_links_reload_source_free_and_reject_invalid_or_omitted_proofs",
    );
}

#[test]
fn translated_global_proof_decisions_match_every_shared_fixture() {
    let data = fixture();
    for entry in data["proofCases"].as_array().unwrap() {
        let strings = |key: &str| {
            entry[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect()
        };
        let numbers = |key: &str| {
            entry[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap())
                .collect()
        };
        assert_eq!(
            meta_language::verify_global_concept_correspondence(
                strings("shapes"),
                strings("meanings"),
                numbers("offsets"),
                numbers("edges"),
                numbers("first"),
                numbers("second")
            ),
            entry["expected"].as_bool().unwrap(),
            "{}",
            entry["name"]
        );
    }
    observe(
        "I195-GLOBAL-CONCEPT-RUNTIME-PARITY",
        &["generatedRuntimeDecisions"],
        "translated_global_proof_decisions_match_every_shared_fixture",
    );
}

#[test]
fn global_links_retain_implicit_starts_and_empty_imported_grammars() {
    let data = fixture();
    let options = settings(&data);
    let mut input = sources(&data);
    input[0].grammar.clear_start();
    input.push(GlobalConceptSource {
        id: "empty".to_owned(),
        language: "empty-language".to_owned(),
        edition: String::new(),
        grammar: meta_language::Grammar::new(),
    });
    let space = create_global_concept_space(&input, &options).unwrap();
    let links = global_concept_space_to_links(&space).unwrap();
    assert!(links.contains("(start unspecified) (rule-order root atom unique)"));
    assert!(
        links.contains("(grammar-origin empty empty-language % (start unspecified) (rule-order ))")
    );
    assert_eq!(
        read_global_concept_space_links(&links, &options).unwrap(),
        space
    );
}

#[test]
fn global_recursive_proofs_reject_many_to_one_reference_collapse() {
    let root = json!({"id":"fixture.root","definition":"A pair of letter references.","constraints":["The references retain their bindings."],"sourceAliases":[]});
    let leaf = json!({"id":"fixture.leaf","definition":"The letter x.","constraints":["One scalar is consumed."],"sourceAliases":[]});
    let source = |id: &str, references: &[&str], names: &[&str]| {
        let mut rules = vec![
            json!({"name":"root","kind":"normal","concept":"fixture.root","expression":{"kind":"seq","items":references.iter().map(|name| json!({"kind":"ref","name":name})).collect::<Vec<_>>()}}),
        ];
        rules.extend(names.iter().map(|name| json!({"name":name,"kind":"normal","concept":"fixture.leaf","expression":{"kind":"literal","value":"x"}})));
        GlobalConceptSource::from_json(&json!({"id":id,"language":id,"grammar":{"schemaVersion":1,"start":"root","rules":rules}})).unwrap()
    };
    let space = create_global_concept_space(
        &[
            source("one", &["leaf", "leaf"], &["leaf"]),
            source("two", &["first", "second"], &["first", "second"]),
        ],
        &GlobalConceptOptions {
            records: Some(json!([root, leaf])),
            previous: None,
            equivalences: vec![["one:root".to_owned(), "two:root".to_owned()]],
        },
    )
    .unwrap();
    assert_ne!(space["mappings"]["one:root"], space["mappings"]["two:root"]);
    assert_eq!(
        space["failures"],
        json!([{"members":["one:root","two:root"],"reason":"no-one-to-one-proof"}])
    );
}
