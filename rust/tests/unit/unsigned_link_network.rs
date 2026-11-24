//! Deferred actual JavaScript/native byte products and exact u64 boundaries.
use super::issue_195_translation_pairs::run;
use meta_language::{
    BinaryLinksOptions, LinkMetadata, LinkNetwork, create_dense_unsigned_link_network,
    decode_unsigned_link_network_binary, encode_unsigned_link_network_binary,
    read_structured_data_lino, restore_dense_unsigned_link_network,
};
use std::process::Command;

#[test]
fn complete_unsigned_graphs_match_actual_javascript_bytes_and_native_text() {
    let product = run(Command::new("node")
        .arg("--max-old-space-size=256")
        .arg("../experiments/unsigned-link-network/produce.mjs"));
    assert!(product.ok, "{}", product.stderr);
    let fixture = read_structured_data_lino(&product.stdout).expect("actual Node product");
    for row in fixture["cases"].as_array().unwrap() {
        let graph = LinkNetwork::from_lino(row["source"].as_str().unwrap()).unwrap();
        let name = row["name"].as_str().unwrap();
        assert_eq!(graph.to_lino(), row["text"].as_str().unwrap(), "{name}");
        let expected: Vec<u8> = row["bytes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
            .collect();
        let bytes =
            encode_unsigned_link_network_binary(&graph, BinaryLinksOptions::default()).unwrap();
        assert_eq!(bytes, expected, "{name}");
        let loaded =
            decode_unsigned_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap();
        assert_eq!(loaded.to_lino(), graph.to_lino(), "{name}");
        assert_eq!(
            encode_unsigned_link_network_binary(&loaded, BinaryLinksOptions::default()).unwrap(),
            bytes,
            "{name}"
        );
        let view =
            create_dense_unsigned_link_network(&loaded, BinaryLinksOptions::default()).unwrap();
        assert_eq!(
            view.next_identity.map(|id| id.to_string()),
            row["next"].as_str().map(str::to_owned),
            "{name}"
        );
        let restored =
            restore_dense_unsigned_link_network(&view, BinaryLinksOptions::default()).unwrap();
        assert_eq!(restored.to_lino(), graph.to_lino(), "{name}");
        if let Some(term) = row["registered"].as_str() {
            assert_eq!(
                loaded.find_term(term).unwrap().to_string(),
                row["identities"][0].as_str().unwrap()
            );
        }
        let actual: Vec<_> = loaded.links().map(|link| link.id().to_string()).collect();
        let expected: Vec<_> = row["identities"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| id.as_str().unwrap())
            .collect();
        assert_eq!(actual, expected, "{name}");
        for end in 0..bytes.len() {
            assert!(
                decode_unsigned_link_network_binary(&bytes[..end], BinaryLinksOptions::default())
                    .is_err(),
                "{name}: truncated at {end}"
            );
        }
        let mut appended = bytes;
        appended.push(0);
        assert!(
            decode_unsigned_link_network_binary(&appended, BinaryLinksOptions::default()).is_err()
        );
        if let Some(next) = view.next_identity {
            let mut edited = loaded;
            assert_eq!(edited.insert_link([], LinkMetadata::new()), next, "{name}");
        }
    }
}

#[test]
fn exact_native_envelopes_reject_corrupt_directories_flags_and_budgets() {
    let graph = LinkNetwork::from_lino("(1: 2)\n(2: 1)").unwrap();
    let bytes = encode_unsigned_link_network_binary(&graph, BinaryLinksOptions::default()).unwrap();
    for (offset, byte) in [(0, 0), (8, 2)] {
        let mut invalid = bytes.clone();
        invalid[offset] = byte;
        assert!(
            decode_unsigned_link_network_binary(&invalid, BinaryLinksOptions::default()).is_err()
        );
    }
    let mut repeated = bytes.clone();
    let first = repeated[21..29].to_vec();
    repeated[29..37].copy_from_slice(&first);
    assert!(decode_unsigned_link_network_binary(&repeated, BinaryLinksOptions::default()).is_err());
    let mut zero = bytes.clone();
    zero[21..29].fill(0);
    assert!(decode_unsigned_link_network_binary(&zero, BinaryLinksOptions::default()).is_err());
    assert!(
        encode_unsigned_link_network_binary(
            &graph,
            BinaryLinksOptions {
                record_limit: 1,
                byte_limit: 4096
            }
        )
        .is_err()
    );
    assert!(
        encode_unsigned_link_network_binary(
            &graph,
            BinaryLinksOptions {
                record_limit: 100,
                byte_limit: 33
            }
        )
        .is_err()
    );
}

#[test]
fn complete_unsigned_definition_keeps_both_codecs_and_all_target_emissions() {
    use meta_language::{
        create_unsigned_link_identity_program_links, decode_link_network_binary,
        emit_typed_program_links, encode_link_network_binary,
    };
    let packet = create_unsigned_link_identity_program_links().unwrap();
    let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let expected = emit_typed_program_links(&packet.network, packet.root, target).unwrap();
        for graph in [&text, &binary] {
            assert_eq!(
                emit_typed_program_links(graph, packet.root, target).unwrap(),
                expected
            );
        }
    }
}
