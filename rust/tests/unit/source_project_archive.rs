//! Deferred exact cross-runtime archives, complete graph state and original CSTs.
use super::issue_195_translation_pairs::run;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, create_source_project_archive,
    decode_link_network_binary, encode_link_network_binary, extend_source_project_archive,
    read_source_project_archive, read_structured_data_lino,
};
use serde_json::{Value, json};
use std::process::Command;
fn check_product(network: &LinkNetwork, root: LinkId, row: &Value, options: &Value) {
    let actual = read_source_project_archive(network, root, options).unwrap();
    assert_eq!(actual.directory, row["directory"]);
    assert_eq!(
        actual.sources.len(),
        row["sources"].as_array().unwrap().len()
    );
    for (file, expected) in actual
        .sources
        .iter()
        .zip(row["sources"].as_array().unwrap())
    {
        assert_eq!(file.path, expected["path"].as_str().unwrap());
        assert_eq!(file.language, expected["language"].as_str().unwrap());
        assert_eq!(file.source, expected["source"].as_str().unwrap());
        assert_eq!(file.network.reconstruct_text(), file.source);
        assert_eq!(file.root.as_u64(), expected["root"].as_u64().unwrap());
        assert_eq!(file.network.to_lino(), expected["links"].as_str().unwrap());
        assert_eq!(
            serde_json::to_value(
                encode_link_network_binary(&file.network, BinaryLinksOptions::default()).unwrap()
            )
            .unwrap(),
            expected["binary"]
        );
        let view = file
            .view
            .iter()
            .map(|link| link.id().as_u64())
            .collect::<Vec<_>>();
        assert_eq!(serde_json::to_value(view).unwrap(), expected["view"]);
    }
}
#[test]
fn original_concrete_source_graphs_and_cache_states_reconstruct_without_source_buffers() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let fields = version
        .stdout
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    assert_eq!(fields[0], "1", "uncovered Rust {}", version.stdout);
    let minor = fields[1].parse::<u32>().unwrap();
    assert!(
        (90..100).contains(&minor),
        "uncovered Rust {}",
        version.stdout
    );
    let produced =
        run(Command::new("node").arg("../experiments/source-project-archive/products.mjs"));
    assert!(produced.ok, "{}", produced.stderr);
    let product = read_structured_data_lino(&produced.stdout).unwrap();
    for row in product["cases"].as_array().unwrap() {
        let node = LinkNetwork::from_lino(row["links"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(row["root"].as_u64().unwrap());
        check_product(&node, root, row, &row["options"]);
        let archive = create_source_project_archive(&row["inputs"], &row["options"]).unwrap();
        // Recreated native output must retain the same full original graph products.
        check_product(&archive.network, archive.root, row, &row["options"]);
        let text = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        let binary = decode_link_network_binary(
            &encode_link_network_binary(&archive.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap();
        for graph in [&text, &binary] {
            check_product(graph, archive.root, row, &row["options"]);
        }
        if row["unclean"] == true {
            for file in read_source_project_archive(&archive.network, archive.root, &row["options"])
                .unwrap()
                .sources
            {
                assert!(!file.network.verify_full_match(None).is_clean());
            }
        }
        let original =
            read_source_project_archive(&archive.network, archive.root, &row["options"]).unwrap();
        let abstract_view = read_source_project_archive(
            &archive.network,
            archive.root,
            &json!({"projection":"AbstractSyntax"}),
        )
        .unwrap();
        for (before, after) in original.sources.iter().zip(abstract_view.sources) {
            assert_eq!(before.network.to_lino(), after.network.to_lino());
            assert_eq!(
                after.projection,
                meta_language::NetworkProjection::AbstractSyntax
            );
        }
    }
    for row in product["extensions"].as_array().unwrap() {
        let first = meta_language::MetaLanguageArchive {
            network: LinkNetwork::from_lino(row["before"]["links"].as_str().unwrap()).unwrap(),
            root: LinkId::from_u64(row["before"]["root"].as_u64().unwrap()),
        };
        let before =
            read_source_project_archive(&first.network, first.root, &row["options"]).unwrap();
        let extended = extend_source_project_archive(
            &first.network,
            first.root,
            &row["added"],
            &row["options"],
        )
        .unwrap();
        check_product(
            &extended.network,
            extended.root,
            &row["after"],
            &row["options"],
        );
        let after =
            read_source_project_archive(&extended.network, extended.root, &row["options"]).unwrap();
        for old in before.sources {
            let current = after
                .sources
                .iter()
                .find(|file| file.path == old.path)
                .unwrap();
            assert_eq!(current.network.to_lino(), old.network.to_lino());
            assert_eq!(
                encode_link_network_binary(&current.network, BinaryLinksOptions::default())
                    .unwrap(),
                encode_link_network_binary(&old.network, BinaryLinksOptions::default()).unwrap()
            );
            assert_eq!(
                current
                    .network
                    .reconstruct_concept("snapshot marker", "JavaScript"),
                Some("preserved selected syntax")
            );
        }
        assert!(
            extend_source_project_archive(
                &first.network,
                first.root,
                &row["initial"],
                &row["options"]
            )
            .unwrap_err()
            .contains("cannot replace an existing source")
        );
    }
    for row in product["refusals"].as_array().unwrap() {
        let error = create_source_project_archive(&row["inputs"], &row["options"]).unwrap_err();
        assert!(error.contains(row["message"].as_str().unwrap()), "{error}");
    }
}
