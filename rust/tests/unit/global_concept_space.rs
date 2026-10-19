use meta_language::{
    GLOBAL_CONCEPT_PROOF_METHOD, GlobalConceptOptions, GlobalConceptSource,
    create_global_concept_space, global_concept_space_to_links, read_global_concept_space_links,
    translate_global_construct,
};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};

fn fixture() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/global-concept-space.lino"
    ))
    .unwrap()
}

fn identity_controls() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/global-concept-identities.lino"
    ))
    .unwrap()
}

#[test]
fn restored_changed_origins_retain_distinct_current_identities() {
    let data = fixture();
    let controls = identity_controls();
    let control = &controls["restoration"];
    let input = sources(&data);
    let mut options = settings(&data);
    let initial = create_global_concept_space(&input, &options).unwrap();
    options.previous = Some(initial);
    let remaining = create_global_concept_space(&input[1..], &options).unwrap();
    let mut changed_data = data["sources"][0].clone();
    let rule = changed_data["grammar"]["rules"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|rule| rule["name"] == control["changedRule"])
        .unwrap();
    rule["expression"] = json!({"kind":"literal","value":control["changedLiteral"]});
    let changed = GlobalConceptSource::from_json(&changed_data).unwrap();
    let restored_sources = [changed, input[1].clone()];
    options.previous = Some(remaining.clone());
    let restored = create_global_concept_space(&restored_sources, &options).unwrap();
    let concepts = restored["concepts"].as_array().unwrap();
    assert_eq!(json!(concepts.len()), control["concepts"]);
    assert_eq!(
        json!(
            concepts
                .iter()
                .map(|entry| entry["id"].as_str().unwrap())
                .collect::<std::collections::BTreeSet<_>>()
                .len()
        ),
        control["uniqueIdentities"]
    );
    assert_eq!(
        json!(restored["proofs"].as_array().unwrap().len()),
        control["proofs"]
    );
    assert_eq!(
        json!(restored["failures"].as_array().unwrap().len()),
        control["failures"]
    );
    for (origin, identity) in remaining["mappings"].as_object().unwrap() {
        assert_eq!(&restored["mappings"][origin], identity);
    }
    assert_ne!(
        restored["mappings"]["first:atom"],
        restored["mappings"]["second:letter"]
    );
    assert_ne!(
        restored["mappings"]["first:root"],
        restored["mappings"]["second:page"]
    );
    let mut reordered = restored_sources.clone();
    reordered.reverse();
    assert_eq!(
        create_global_concept_space(&reordered, &options).unwrap(),
        restored
    );
    options.previous = Some(restored.clone());
    assert_eq!(
        create_global_concept_space(&restored_sources, &options).unwrap(),
        restored
    );
    let links = global_concept_space_to_links(&restored).unwrap();
    assert_eq!(
        read_global_concept_space_links(&links, &options).unwrap(),
        restored
    );
    let merged = create_global_concept_space(&input, &options).unwrap();
    assert_eq!(
        merged["mappings"]["first:atom"],
        merged["mappings"]["second:letter"]
    );
}

#[test]
fn shared_history_controls_refuse_duplicate_or_empty_identities_and_origins() {
    let data = fixture();
    for control in identity_controls()["histories"].as_array().unwrap() {
        let concepts: Vec<_> = control["identities"]
            .as_array().unwrap().iter().enumerate()
            .map(|(index, identity)| json!({"id":identity,"origins":[{"alias":control["aliases"][index]}]}))
            .collect();
        let mut options = settings(&data);
        options.previous = Some(json!({"concepts":concepts}));
        let result = create_global_concept_space(&sources(&data), &options);
        let expected = control["expected"].as_bool().unwrap();
        assert_eq!(result.is_ok(), expected);
        if !expected {
            assert_eq!(result.unwrap_err(), "invalid global concept history");
        }
    }
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
        formal_definitions: data.get("formalDefinitions").cloned(),
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

// Extend only source provenance recorded by the original input grammars. The
// fixture's identities, definitions, proofs and legacy archive stay untouched.
fn with_source_format_provenance(expected: &Value, input: &[GlobalConceptSource]) -> Value {
    let mut expected = expected.clone();
    for source in expected["sources"].as_array_mut().unwrap() {
        let original = input
            .iter()
            .find(|original| source["id"] == original.id)
            .unwrap();
        if let Some(format) = original.grammar.source_format() {
            source["sourceFormat"] = json!(format.as_str());
        }
    }
    expected
}

fn with_source_format_statements(links: &str, input: &[GlobalConceptSource]) -> String {
    use meta_language::grammar::interchange::percent_encode_links_text;

    let mut statements = Vec::new();
    for line in links.split('\n') {
        statements.push(line.to_owned());
        for source in input {
            let id = percent_encode_links_text(&source.id);
            if line.starts_with(&format!("(grammar-origin {id} "))
                && let Some(format) = source.grammar.source_format()
            {
                statements.push(format!("(grammar-source-format {id} {})", format.as_str()));
            }
        }
    }
    statements.join("\n")
}

#[test]
fn closed_global_context_retains_lexical_competition_modes_channels_actions_and_owners() {
    let data = fixture();
    let options = settings(&data);
    assert_eq!(
        create_global_concept_space(&sources(&data), &options).unwrap(),
        with_source_format_provenance(&data["expectedSpace"], &sources(&data))
    );
    let mut scoped = data;
    for (index, source) in scoped["sources"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        let binding = if index == 0 { "atom" } else { "letter" };
        source["grammar"]["declarations"]["lexers"] = json!([{
            "name":"source lexer", "expression":{"kind":"seq", "items":[
                {"kind":"lexicalMode","name":"quoted"},
                {"kind":"lexicalChannel","name":"CUSTOM"},
                {"kind":"lexicalRule","name":"ATOM","type":"ATOM","mode":"default","channel":"default","disposition":"emit","item":{"kind":"literal","value":"a"},"action":[]},
                {"kind":"lexicalRule","name":"FALLBACK","type":"FALLBACK","mode":"default","channel":"CUSTOM","disposition":"emit","item":{"kind":"any"},"action":[]},
                {"kind":"lexicalBinding","name":"ATOM","item":{"kind":"ref","name":binding}},
                {"kind":"lexicalObligation","name":"","category":"option","source":" superClass = FirstBase; /* original context */ "}
            ]}
        }]);
    }
    let shared = create_global_concept_space(&sources(&scoped), &options).unwrap();
    assert_eq!(
        shared["mappings"]["first:atom"],
        shared["mappings"]["second:letter"]
    );
    assert_eq!(
        shared["mappings"]["first:root"],
        shared["mappings"]["second:page"]
    );
    assert_ne!(
        shared["proofs"].as_array().unwrap().as_slice(),
        [] as [Value; 0]
    );
    let links = global_concept_space_to_links(&shared).unwrap();
    assert!(links.contains("lexicalRule"));
    assert_eq!(
        read_global_concept_space_links(&links, &options).unwrap(),
        shared
    );
    for mutation in 0..8 {
        let mut changed = scoped.clone();
        let entries =
            changed["sources"][1]["grammar"]["declarations"]["lexers"][0]["expression"]["items"]
                .as_array_mut()
                .unwrap();
        match mutation {
            0 => entries.swap(2, 3),
            1 => entries[2]["channel"] = json!("CUSTOM"),
            2 => entries[2]["mode"] = json!("quoted"),
            3 => entries[2]["disposition"] = json!("skip"),
            4 => entries[2]["action"] = json!([{"operation":"pushMode","mode":"quoted"}]),
            5 => entries.push(json!({"kind":"lexicalObligation","name":"ATOM","category":"predicate","source":"unresolvedGuard()"})),
            6 => entries.push(json!({"kind":"lexicalOwner","name":"original parser owner","item":{"kind":"ref","name":"page"}})),
            _ => entries[5]["source"] = json!(" superClass = SecondBase; /* original context */ "),
        }
        let compared = create_global_concept_space(&sources(&changed), &options).unwrap();
        assert_ne!(
            compared["mappings"]["first:atom"],
            compared["mappings"]["second:letter"]
        );
        assert_ne!(
            compared["mappings"]["first:root"],
            compared["mappings"]["second:page"]
        );
        assert_eq!(
            compared["proofs"].as_array().unwrap().as_slice(),
            [] as [Value; 0]
        );
        assert_eq!(
            compared["failures"].as_array().unwrap().len(),
            options.equivalences.len()
        );
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
    assert_eq!(
        space,
        with_source_format_provenance(&data["expectedSpace"], &input)
    );
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
        ..options
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
    let mut changed = data;
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
    let legacy =
        read_global_concept_space_links(data["expectedLinks"].as_str().unwrap(), &options).unwrap();
    assert_eq!(legacy, data["expectedSpace"]);
    assert_eq!(
        global_concept_space_to_links(&legacy).unwrap(),
        data["expectedLinks"].as_str().unwrap()
    );
    assert!(
        legacy["sources"]
            .as_array()
            .unwrap()
            .iter()
            .all(|source| source.get("sourceFormat").is_none())
    );
    let space = create_global_concept_space(&sources(&data), &options).unwrap();
    let links = global_concept_space_to_links(&space).unwrap();
    assert!(links.contains("(source-alias first%3Aroot upstream%3Afirst original_root)"));
    assert!(links.contains("(concept-proof recursive-structural-bijection"));
    assert!(links.contains("(literal %CE%B1)"));
    assert_eq!(links.matches("(concept-definition ").count(), 6);
    assert_eq!(links.matches("(formal-origin-definition ").count(), 4);
    assert_eq!(
        links,
        with_source_format_statements(data["expectedLinks"].as_str().unwrap(), &sources(&data))
    );
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
    let missing_basis = links
        .lines()
        .filter(|line| !line.starts_with("(formal-origin-definition "))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(read_global_concept_space_links(&missing_basis, &options).is_err());
    let counterfeit = links.replacen(
        "(def: literal%20source%20text%20value)",
        "(def: unrelated%20primitive)",
        1,
    );
    assert_ne!(counterfeit, links);
    assert!(read_global_concept_space_links(&counterfeit, &options).is_err());
    observe(
        "I195-GLOBAL-CONCEPT-SERIALIZATION",
        &["linksRoundTrip"],
        "global_links_reload_source_free_and_reject_invalid_or_omitted_proofs",
    );
}

#[test]
fn global_semantic_premises_require_supported_operational_archives() {
    let data = fixture();
    let mut options = settings(&data);
    options.formal_definitions = None;
    let prose = create_global_concept_space(&sources(&data), &options).unwrap();
    assert_eq!(prose["proofs"], json!([]));
    assert_eq!(prose["formalDefinitions"], json!({}));
    assert_eq!(prose["concepts"].as_array().unwrap().len(), 6);
    options.formal_definitions =
        Some(json!({"fixture.letter":{"links":"(claim same-meaning)","root":1}}));
    let counterfeit = create_global_concept_space(&sources(&data), &options).unwrap();
    assert_eq!(counterfeit["proofs"], json!([]));
    assert_eq!(counterfeit["formalDefinitions"], json!({}));
    assert_eq!(counterfeit["concepts"].as_array().unwrap().len(), 6);
    let mut alias_basis = serde_json::Map::new();
    for alias in ["first:root", "first:atom", "second:page", "second:letter"] {
        alias_basis.insert(
            alias.to_owned(),
            data["formalDefinitions"]["fixture.letter"].clone(),
        );
    }
    options.records = Some(json!([]));
    options.formal_definitions = Some(Value::Object(alias_basis));
    let explicit = create_global_concept_space(&sources(&data), &options).unwrap();
    assert_eq!(explicit["concepts"].as_array().unwrap().len(), 4);
    assert_eq!(
        explicit["mappings"]["first:root"],
        explicit["mappings"]["second:page"]
    );
    assert_eq!(explicit["formalDefinitions"].as_object().unwrap().len(), 4);
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
            formal_definitions: Some(json!({"fixture.root":fixture()["formalDefinitions"]["fixture.letter"],"fixture.leaf":fixture()["formalDefinitions"]["fixture.letter"]})),
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

#[test]
fn global_proof_discovery_resolves_catalog_aliases_of_real_native_grammars() {
    let input: Vec<_> = [
        (
            "scheme",
            "Scheme",
            include_str!("../../src/data/native-grammars/scheme.lino"),
        ),
        (
            "racket",
            "Racket",
            include_str!("../../src/data/native-grammars/racket.lino"),
        ),
    ]
    .iter()
    .map(|(identity, language, listing)| {
        let original = meta_language::parse_grammar_links(listing).unwrap();
        let mut grammar = meta_language::Grammar::new();
        if let Some(start) = original.start() {
            grammar.set_start(start);
        }
        grammar.set_declarations(original.declarations().clone());
        grammar.set_kinds(original.kinds().to_vec());
        for mut rule in original.rules().to_vec() {
            rule.concept = None;
            grammar.add_rule(rule);
        }
        GlobalConceptSource {
            id: format!("pinned-{identity}"),
            language: (*language).to_owned(),
            edition: String::new(),
            grammar,
        }
    })
    .collect();
    let trivia = fixture()["triviaDefinition"].clone();
    let options = GlobalConceptOptions {
        formal_definitions: Some(
            json!({"pinned-scheme:block_comment":trivia,"pinned-racket:block_comment":trivia}),
        ),
        ..GlobalConceptOptions::default()
    };
    let space = create_global_concept_space(&input, &options).unwrap();
    let shared_id = &space["mappings"]["pinned-scheme:block_comment"];
    assert_eq!(shared_id, &space["mappings"]["pinned-racket:block_comment"]);
    let shared = space["concepts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|concept| &concept["id"] == shared_id)
        .unwrap();
    assert_eq!(shared["origins"].as_array().unwrap().len(), 2);
    assert_ne!(
        shared["proofs"].as_array().unwrap().as_slice(),
        [] as [Value; 0]
    );
    assert_eq!(
        space["mappings"].as_object().unwrap().len(),
        input
            .iter()
            .map(|source| source.grammar.rules().len())
            .sum::<usize>()
    );
    assert_ne!(
        space["mappings"]["pinned-scheme:program"],
        space["mappings"]["pinned-racket:program"]
    );
    let translated = translate_global_construct(&space, "pinned-scheme:block_comment", "rkt", None);
    assert_eq!(translated["relation"], "translated");
    assert_eq!(&translated["concept"], shared_id);
    assert_eq!(
        translated["origins"][0]["alias"],
        "pinned-racket:block_comment"
    );
    assert_ne!(
        translated["proofs"].as_array().unwrap().as_slice(),
        [] as [Value; 0]
    );
    assert_eq!(
        translate_global_construct(&space, "pinned-scheme:program", "Racket", None)["relation"],
        "untranslatable"
    );
    assert_eq!(
        read_global_concept_space_links(&global_concept_space_to_links(&space).unwrap(), &options)
            .unwrap(),
        space
    );
}

#[test]
fn global_construct_queries_preserve_meaning_ambiguity_and_target_editions() {
    let data = fixture();
    let mut input = sources(&data);
    let mut third = input[1].clone();
    third.id = "third".to_owned();
    third.edition = "3".to_owned();
    input.push(third);
    let options = settings(&data);
    let space = create_global_concept_space(&input, &options).unwrap();
    let ambiguous = translate_global_construct(&space, "first:root", "second-language", None);
    assert_eq!(ambiguous["relation"], "ambiguous");
    assert_eq!(
        ambiguous["origins"]
            .as_array()
            .unwrap()
            .iter()
            .map(|origin| origin["alias"].clone())
            .collect::<Vec<_>>(),
        vec![json!("second:page"), json!("third:page")]
    );
    let edition = translate_global_construct(&space, "first:root", "second-language", Some("3"));
    assert_eq!(edition["relation"], "translated");
    assert_eq!(edition["origins"][0]["alias"], "third:page");
    assert_eq!(
        translate_global_construct(&space, "first:unique", "second-language", None)["relation"],
        "untranslatable"
    );
    assert_eq!(
        translate_global_construct(&space, "missing:rule", "second-language", None),
        json!({"relation":"unknown","concept":null,"origins":[],"proofs":[]})
    );
    let mut records = data["records"].as_array().unwrap().clone();
    records.push(json!({"id":"fixture.other-letter","definition":"A separate meaning of the letter.","constraints":["The role is distinct."],"sourceAliases":[{"source":"first","name":"root"}]}));
    let uncertain = create_global_concept_space(
        &sources(&data),
        &GlobalConceptOptions {
            records: Some(json!(records)),
            ..options
        },
    )
    .unwrap();
    assert_eq!(
        translate_global_construct(&uncertain, "first:root", "second-language", None),
        json!({"relation":"ambiguous","concept":null,"origins":[],"proofs":[]})
    );
}
