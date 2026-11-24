//! Deferred actual shared policies, full archives and recursive mapped CSTs.
#[path = "../../src/grammar_composition_batch_rules.rs"]
mod batch_rules;
use super::issue_195_translation_pairs::run;
use meta_language::{
    BinaryLinksOptions, FeatureParseOptions, GlobalConceptOptions, GlobalConceptSource,
    GrammarCompositionPolicy, LinkNetwork, MetaLanguageArchiveLimits,
    check_grammar_selection_mapped_tree, create_discovered_concept_links,
    create_global_concept_space_parser, create_grammar_composition_batch_links,
    create_meta_language_archive, decode_link_network_binary, decode_structured_data_links,
    encode_link_network_binary, encode_structured_data_links, global_concept_space_to_links,
    read_grammar_composition_batch_links, read_meta_language_archive, read_structured_data_lino,
};
use serde_json::Value;
use std::{process::Command, sync::Arc};
fn fixture() -> Value {
    super::grammar_recursive_selection_mapping::mapped_fixture(
        "fixtures/grammar-composition-batch.lino",
    )
}
#[test]
fn exact_batch_sources_decisions_archives_and_recursive_concrete_products() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = fixture();
    let sources = fixture["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(GlobalConceptSource::from_json)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let policies: Vec<GrammarCompositionPolicy> =
        serde_json::from_value(fixture["policies"].clone()).unwrap();
    let original =
        create_discovered_concept_links(&sources, &GlobalConceptOptions::default()).unwrap();
    let before = original.network.to_lino();
    let limits = MetaLanguageArchiveLimits::default();
    let archive =
        create_grammar_composition_batch_links(&original.network, original.root, &policies, limits)
            .unwrap();
    assert_eq!(original.network.to_lino(), before);
    let produced = run(Command::new("node").args([
        "--max-old-space-size=352",
        "../experiments/grammar-composition/batch.mjs",
    ]));
    assert!(produced.ok, "{}", produced.stderr);
    let product = read_structured_data_lino(&produced.stdout).unwrap();
    assert_eq!(
        archive.network.to_lino(),
        product["links"].as_str().unwrap()
    );
    assert_eq!(archive.root.as_u64(), product["root"].as_u64().unwrap());
    let binary =
        encode_link_network_binary(&archive.network, BinaryLinksOptions::default()).unwrap();
    for network in [
        LinkNetwork::from_lino(&archive.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ] {
        let loaded = read_grammar_composition_batch_links(&network, archive.root, limits).unwrap();
        assert_eq!(loaded.current.sources.len(), sources.len());
        assert_eq!(loaded.current.space, product["space"]);
        let stored = loaded
            .entries
            .iter()
            .find(|entry| entry.identifier == "grammar-composition-original-definitions")
            .unwrap();
        assert_eq!(stored.network.to_lino(), before);
        assert_eq!(stored.root, original.root);
        assert_eq!(loaded.current.sources[2].grammar, sources[2].grammar);
        let parser = create_global_concept_space_parser(
            &global_concept_space_to_links(&loaded.current.space).unwrap(),
            "quoted-a",
            &GlobalConceptOptions::default(),
            FeatureParseOptions::default(),
        )
        .unwrap();
        let outcome = parser
            .parse_tree(
                fixture["input"].as_str().unwrap().as_bytes(),
                &FeatureParseOptions::default(),
            )
            .unwrap();
        assert!(outcome.ok, "{:?}", outcome.rejection);
        let bundle = super::grammar_recursive_selection_mapping::mapped_bundle(&outcome);
        assert_eq!(bundle, product["bundle"]);
        assert_eq!(
            outcome.source_planes.unwrap().len(),
            usize::try_from(fixture["expectedPlanes"].as_u64().unwrap()).unwrap()
        );
        check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
    }
    for row in fixture["refusals"].as_array().unwrap() {
        let policies =
            serde_json::from_value::<Vec<GrammarCompositionPolicy>>(row["policies"].clone())
                .unwrap();
        assert_eq!(
            create_grammar_composition_batch_links(
                &original.network,
                original.root,
                &policies,
                limits
            )
            .unwrap_err(),
            format!(
                "grammar composition batch: {}",
                row["reason"].as_str().unwrap()
            )
        );
    }
    for role in ["configuration", "algorithm", "current"] {
        let mut entries = read_meta_language_archive(&archive.network, archive.root, limits)
            .unwrap()
            .entries;
        let index = entries
            .iter()
            .position(|entry| {
                entry.identifier
                    == format!(
                        "grammar-composition-{}",
                        match role {
                            "configuration" => "batch-configuration",
                            "algorithm" => "batch-algorithm",
                            _ => "current-definitions",
                        }
                    )
            })
            .unwrap();
        if role == "configuration" {
            let mut data =
                decode_structured_data_links(&entries[index].network, entries[index].root).unwrap();
            data["policies"][0]["composition"] = Value::String(
                data["policies"][0]["composition"]
                    .as_str()
                    .unwrap()
                    .replace("any-grammar", "all-grammar"),
            );
            let encoded = encode_structured_data_links(&data).unwrap();
            entries[index].network = Arc::new(encoded.network);
            entries[index].root = encoded.root;
        } else {
            let replacement = entries
                .iter()
                .find(|entry| {
                    entry.identifier
                        == if role == "current" {
                            "grammar-composition-original-definitions"
                        } else {
                            "grammar-composition-batch-configuration"
                        }
                })
                .unwrap()
                .clone();
            entries[index].network = replacement.network;
            entries[index].root = replacement.root;
        }
        let changed = create_meta_language_archive(&entries, limits).unwrap();
        assert!(
            read_grammar_composition_batch_links(&changed.network, changed.root, limits).is_err()
        );
    }
}
#[test]
fn every_declared_batch_binding_decision_uses_the_actual_shipped_definition() {
    use batch_rules::ml_string::Text;
    let fixture = fixture();
    for row in fixture["decisions"].as_array().unwrap() {
        let column = |key: &str| {
            row[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| Text::from_text(value.as_str().unwrap()))
                .collect::<Vec<_>>()
        };
        let result = batch_rules::read_grammar_composition_batch_issue(
            &column("sources"),
            &column("owners"),
            &column("names"),
            &column("kinds"),
            &column("policySources"),
            &column("policyRules"),
            &column("compositions"),
        );
        assert_eq!(
            result.to_string(),
            row["expected"].as_str().unwrap(),
            "{}",
            row["name"]
        );
    }
}

#[test]
fn source_count_and_utf16_text_bounds_precede_column_transport() {
    let fixture = fixture();
    for row in fixture["resources"].as_array().unwrap() {
        assert_eq!(
            batch_rules::read_grammar_composition_batch_count_issue(
                row["sources"].as_f64().unwrap(),
                row["rules"].as_f64().unwrap(),
                row["policies"].as_f64().unwrap()
            )
            .to_string(),
            row["countIssue"].as_str().unwrap()
        );
        assert_eq!(
            batch_rules::read_grammar_composition_batch_text_issue(row["units"].as_f64().unwrap())
                .to_string(),
            row["textIssue"].as_str().unwrap()
        );
    }
}
