//! Future full checkpoint/corpus/native-codec parity, only executed in CI.
use super::regular_grammar_inference::{examples, versions};
use meta_language::{
    MetaLanguageArchiveLimits, RegularGrammarInferenceOptions, accept_inferred_grammar_symbols,
    decode_structured_data_links, infer_regular_grammar, meta_language_archive_to_binary,
    meta_language_archive_to_text, read_meta_language_archive, read_meta_language_archive_binary,
    read_meta_language_archive_text, read_refined_regular_grammar_inference_links,
    read_structured_data_lino, refine_regular_grammar_inference, serialize_grammar,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::process::Command;
use std::sync::Arc;
#[test]
fn complete_refinement_history_resumes_both_codecs_and_matches_independent_native_products() {
    versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/regular-grammar-refinement.lino"
    ))
    .unwrap();
    let limits = MetaLanguageArchiveLimits {
        node_limit: 1000000,
        reference_limit: 4000000,
        byte_limit: 134217728,
    };
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let mut parameters = row["options"].clone();
        parameters.as_object_mut().unwrap().remove("identifier");
        let options = RegularGrammarInferenceOptions {
            parameters: serde_json::from_value(parameters).unwrap(),
            identifier: row["options"]["identifier"].as_str().unwrap().to_owned(),
            ..Default::default()
        };
        let mut current = infer_regular_grammar(
            &examples(&row["positives"]),
            &examples(&row["negatives"]),
            &options,
        )
        .unwrap();
        let mut positives = examples(&row["positives"]);
        let mut negatives = examples(&row["negatives"]);
        for (ordinal, step) in row["steps"].as_array().unwrap().iter().enumerate() {
            let original = current.network.to_lino();
            let next = refine_regular_grammar_inference(
                &current.network,
                current.root,
                &examples(&step["positives"]),
                &examples(&step["negatives"]),
                None,
            )
            .unwrap();
            assert_eq!(current.network.to_lino(), original);
            positives.extend(examples(&step["positives"]));
            negatives.extend(examples(&step["negatives"]));
            let archive = meta_language::MetaLanguageArchive {
                network: next.network.clone(),
                root: next.root,
            };
            let text = meta_language_archive_to_text(&archive, limits).unwrap();
            let binary = meta_language_archive_to_binary(&archive, limits).unwrap();
            for loaded in [
                read_meta_language_archive_text(&text, limits).unwrap(),
                read_meta_language_archive_binary(&binary, limits).unwrap(),
            ] {
                let restored =
                    read_refined_regular_grammar_inference_links(&loaded.network, loaded.root)
                        .unwrap();
                assert_eq!(restored.table, next.table);
                for input in &positives {
                    assert!(accept_inferred_grammar_symbols(&restored, input).unwrap());
                }
                for input in &negatives {
                    assert!(!accept_inferred_grammar_symbols(&restored, input).unwrap());
                }
                assert_eq!(
                    loaded
                        .entries
                        .iter()
                        .filter(|entry| entry.identifier == "regular-discovery-algorithm")
                        .count(),
                    1
                );
                assert_eq!(loaded.entries.iter().filter(|entry|entry.identifier=="regular-discovery-refinement-algorithm").count(),1);
                for index in 0..=ordinal {
                    assert!(loaded.entries.iter().any(|entry| entry.identifier
                        == format!("regular-discovery-history-{index}-model")));
                }
            }
            current = next;
        }
        let before = current.network.to_lino();
        assert!(
            refine_regular_grammar_inference(
                &current.network,
                current.root,
                &[],
                &[],
                Some(&options.identifier)
            )
            .unwrap_err()
            .contains("new source-qualified")
        );
        assert!(
            refine_regular_grammar_inference(
                &current.network,
                current.root,
                &[],
                &[positives[0].clone()],
                None
            )
            .unwrap_err()
            .contains("contradictory")
        );
        assert_eq!(current.network.to_lino(), before);
        let producer = Command::new("node")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args([
                "--max-old-space-size=352",
                "--max-semi-space-size=1",
                "../experiments/grammar-inference/refinement-oracle.mjs",
                &index.to_string(),
            ])
            .output()
            .unwrap();
        assert!(
            producer.status.success(),
            "{}",
            String::from_utf8_lossy(&producer.stderr)
        );
        let expected =
            read_structured_data_lino(&String::from_utf8(producer.stdout).unwrap()).unwrap();
        let grammar: Value = serde_json::from_str(&serialize_grammar(&current.grammar)).unwrap();
        assert_eq!(
            json!({"name":row["name"],"identifier":current.source.id,"alphabet":current.alphabet,"table":current.table,"grammar":grammar,"root":current.root.as_u64(),"archiveSha256":format!("{:x}",Sha256::digest(current.network.to_lino().as_bytes()))}),
            expected
        );
        let entries = read_meta_language_archive(&current.network, current.root, limits)
            .unwrap()
            .entries;
        let journal = entries
            .iter()
            .find(|entry| entry.identifier == "regular-discovery-refinement-journal")
            .unwrap();
        let journal = decode_structured_data_links(&journal.network, journal.root).unwrap();
        assert_eq!(journal["steps"].as_array().unwrap().len(), 2);
        let mut altered = journal;
        altered["steps"][0]["positives"] = json!([]);
        let data = meta_language::encode_structured_data_links(&altered).unwrap();
        let mut changed_entries = entries.clone();
        let journal_entry = changed_entries
            .iter_mut()
            .find(|entry| entry.identifier == "regular-discovery-refinement-journal")
            .unwrap();
        journal_entry.network = Arc::new(data.network);
        journal_entry.root = data.root;
        let changed =
            meta_language::create_meta_language_archive(&changed_entries, limits).unwrap();
        assert!(
            read_refined_regular_grammar_inference_links(&changed.network, changed.root)
                .unwrap_err()
                .contains("invalid refinement history")
        );
        for missing in [
            "regular-discovery-history-0-grammar",
            "regular-discovery-refinement-algorithm",
        ] {
            let retained = entries
                .iter()
                .filter(|entry| entry.identifier != missing)
                .cloned()
                .collect::<Vec<_>>();
            let changed = meta_language::create_meta_language_archive(&retained, limits).unwrap();
            assert!(
                read_refined_regular_grammar_inference_links(&changed.network, changed.root)
                    .is_err()
            );
        }
    }
}
