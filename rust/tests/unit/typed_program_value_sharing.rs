//! Deferred cross-runtime graph reload, scope ownership and codec controls.
use super::issue_195_translation_pairs::run;
use meta_language::{
    LinkId, LinkNetwork, decode_typed_program_links, encode_typed_program_links,
    encode_typed_program_links_with_value_sharing, read_structured_data_lino,
    read_typed_program_binding_identities,
};
use std::process::Command;
#[test]
fn exact_values_share_while_actual_binding_owners_remain_separate() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let active = version.stdout.split_whitespace().nth(1).unwrap();
    let components = active
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        components[0] == 1 && (90..100).contains(&components[1]),
        "uncovered Rust {active}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/typed-sharing-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let data = read_structured_data_lino(&producer.stdout).unwrap();
    for row in data["programs"].as_array().unwrap() {
        let legacy = LinkNetwork::from_lino(row["legacy"].as_str().unwrap()).unwrap();
        let shared = LinkNetwork::from_lino(row["shared"].as_str().unwrap()).unwrap();
        let first = decode_typed_program_links(
            &legacy,
            LinkId::from_u64(row["legacyRoot"].as_str().unwrap().parse().unwrap()),
        )
        .unwrap();
        let second = decode_typed_program_links(
            &shared,
            LinkId::from_u64(row["sharedRoot"].as_str().unwrap().parse().unwrap()),
        )
        .unwrap();
        assert_eq!(first, second);
        let before = encode_typed_program_links(&second).unwrap();
        let after = encode_typed_program_links_with_value_sharing(
            &second,
            None,
            data["model"].as_str().unwrap(),
        )
        .unwrap();
        assert!(after.network.links().count() < before.network.links().count());
        assert_eq!(
            decode_typed_program_links(&after.network, after.root).unwrap(),
            second
        );
        let identities = read_typed_program_binding_identities(&after.network, after.root).unwrap();
        let unique = identities.iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), identities.len());
        assert!(identities.len() >= 2);
        let text = LinkNetwork::from_lino(&after.network.to_lino()).unwrap();
        assert_eq!(
            decode_typed_program_links(&text, after.root).unwrap(),
            second
        );
        assert!(
            encode_typed_program_links_with_value_sharing(&second, None, "unknown profile")
                .is_err()
        );
    }
}
