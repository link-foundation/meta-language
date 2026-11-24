//! Deferred complete cross-runtime layout, formal-basis and malformed inputs.
use super::issue_195_translation_pairs::run;
use meta_language::{
    BinaryLinksOptions, FormalFunctionSelector, LinkId, LinkNetwork, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_formal_function_declaration_value,
    encode_formal_function_definition, encode_link_network_binary, encode_typed_program_links,
    encode_typed_program_links_with_record_shapes, prove_formal_concept_equivalence,
    read_structured_data_lino, read_typed_program_binding_identities, structured_data_to_lino,
    verify_formal_concept_equivalence,
};
use std::io::Write;
use std::process::{Command, Stdio};
#[test]
fn ordered_record_shapes_keep_all_values_owners_and_formal_contexts() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let active = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = active
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {active}; >=1.90 <1.100"
    );
    let output = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/record-shape-oracle.mjs",
        ]));
    assert!(output.ok, "{}", output.stderr);
    let data = read_structured_data_lino(&output.stdout).unwrap();
    let mut native_packets = Vec::new();
    for row in data["programs"].as_array().unwrap() {
        let legacy = LinkNetwork::from_lino(row["legacy"].as_str().unwrap()).unwrap();
        let compact = LinkNetwork::from_lino(row["compact"].as_str().unwrap()).unwrap();
        let a = LinkId::from_u64(row["legacyRoot"].as_str().unwrap().parse().unwrap());
        let b = LinkId::from_u64(row["compactRoot"].as_str().unwrap().parse().unwrap());
        let program = decode_typed_program_links(&legacy, a).unwrap();
        assert_eq!(decode_typed_program_links(&compact, b).unwrap(), program);
        let first = encode_typed_program_links(&program).unwrap();
        let second = encode_typed_program_links_with_record_shapes(
            &program,
            None,
            data["model"].as_str().unwrap(),
        )
        .unwrap();
        assert!(second.network.links().count() < first.network.links().count());
        let mut old_owners =
            read_typed_program_binding_identities(&first.network, first.root).unwrap();
        let new_owners =
            read_typed_program_binding_identities(&second.network, second.root).unwrap();
        old_owners.sort();
        let mut sorted = new_owners.clone();
        sorted.sort();
        assert_eq!(old_owners, sorted);
        assert_eq!(
            new_owners.len(),
            row["declarations"].as_array().unwrap().len()
        );
        let text = LinkNetwork::from_lino(&second.network.to_lino()).unwrap();
        let bytes =
            encode_link_network_binary(&second.network, BinaryLinksOptions::default()).unwrap();
        let binary = decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap();
        for network in [&compact, &second.network, &text, &binary] {
            let root = if std::ptr::eq(network, &compact) {
                b
            } else {
                second.root
            };
            assert_eq!(decode_typed_program_links(network, root).unwrap(), program);
            for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
                assert_eq!(
                    emit_typed_program_links(network, root, target).unwrap(),
                    emit_typed_program_links(&legacy, a, target).unwrap()
                );
            }
        }
        for declaration in row["declarations"].as_array().unwrap() {
            let name = declaration.as_str().unwrap();
            let one = encode_formal_function_definition(
                &legacy,
                a,
                FormalFunctionSelector::Name(name),
                None,
            )
            .unwrap();
            let two = encode_formal_function_definition(
                &compact,
                b,
                FormalFunctionSelector::Name(name),
                None,
            )
            .unwrap();
            let proof = prove_formal_concept_equivalence(&one, &two)
                .expect("layout does not change the actual supported formal basis");
            assert!(verify_formal_concept_equivalence(&one, &two, &proof));
            if row["boolean"] == true {
                let one = encode_formal_function_declaration_value(
                    &legacy,
                    a,
                    FormalFunctionSelector::Name(name),
                    None,
                )
                .unwrap();
                let two = encode_formal_function_declaration_value(
                    &compact,
                    b,
                    FormalFunctionSelector::Name(name),
                    None,
                )
                .unwrap();
                let proof = prove_formal_concept_equivalence(&one, &two).unwrap();
                assert!(verify_formal_concept_equivalence(&one, &two, &proof));
            }
        }
        for bad in row["invalid"].as_array().unwrap() {
            let network = LinkNetwork::from_lino(bad["links"].as_str().unwrap()).unwrap();
            let root = LinkId::from_u64(bad["root"].as_str().unwrap().parse().unwrap());
            let error = decode_typed_program_links(&network, root)
                .expect_err("every malformed layout must be rejected");
            if let Some(reason) = bad.get("reason").and_then(serde_json::Value::as_str) {
                assert!(
                    error.message.contains(reason),
                    "{}: {}",
                    bad["name"],
                    error.message
                );
            }
        }
        if let Some(edited) = row.get("edited") {
            let network = LinkNetwork::from_lino(edited["links"].as_str().unwrap()).unwrap();
            let root = LinkId::from_u64(edited["root"].as_str().unwrap().parse().unwrap());
            let actual = decode_typed_program_links(&network, root).unwrap();
            assert_ne!(actual, program);
        }
        for unknown in ["", "unregistered layout 7"] {
            assert!(
                encode_typed_program_links_with_record_shapes(&program, None, unknown).is_err()
            );
        }
        native_packets.push(serde_json::json!({"legacy":row["legacy"],"legacyRoot":row["legacyRoot"],"native":second.network.to_lino(),"nativeRoot":second.root.as_u64().to_string()}));
    }
    let input = structured_data_to_lino(&serde_json::json!({"programs":native_packets})).unwrap();
    let mut child = Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/record-shape-native-reload.mjs",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "native record layout reload complete\n"
    );
}
