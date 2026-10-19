//! Algebraic source-free graph edits; native compiler execution belongs to CI.
use super::issue_195_translation_pairs::TemporaryDirectory;
use meta_language::{
    LinkId, LinkNetwork, LinkType, create_typed_program_links, decode_typed_program_links,
    emit_typed_program_links, percent_encode_links_text, read_structured_data_lino,
};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/typed-program-algebraic.lino"
    ))
    .unwrap()
}

fn graph(case: &Value) -> (LinkNetwork, LinkId) {
    (
        LinkNetwork::from_lino(case["lino"].as_str().unwrap()).unwrap(),
        LinkId::from_u64(case["root"].as_u64().unwrap()),
    )
}

fn apply_edits(mut network: LinkNetwork, edits: &Value) -> LinkNetwork {
    for edit in edits.as_array().unwrap() {
        let id = LinkId::from_u64(edit["link"].as_u64().unwrap());
        let link = network.link(id).expect("the edited link exists");
        let kind = edit["kind"].as_str().unwrap();
        let replacement = if kind == "references" {
            let references = edit["values"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_u64().unwrap().to_string())
                .collect::<Vec<_>>()
                .join(" ");
            (None, references)
        } else {
            let (label, before) = match kind {
                "definition" => ("def", link.metadata().definition()),
                "language" => ("lang", link.metadata().language()),
                "term" => ("term", link.metadata().term()),
                _ => panic!("unknown algebraic edit"),
            };
            let marker = format!(
                "({label}: {})",
                percent_encode_links_text(before.expect("the edited metadata exists"))
            );
            (
                Some(marker),
                format!(
                    "({label}: {})",
                    percent_encode_links_text(edit["value"].as_str().unwrap())
                ),
            )
        };
        let text = network
            .to_lino()
            .lines()
            .map(|line| {
                if !line.starts_with(&format!("({id}: ")) {
                    return line.to_owned();
                }
                if let Some(marker) = &replacement.0 {
                    assert!(line.contains(marker), "the exact metadata is present");
                    line.replacen(marker, &replacement.1, 1)
                } else {
                    let metadata = &line[line.find("(meta:").unwrap()..];
                    format!("({id}: {} {metadata}", replacement.1)
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        network = LinkNetwork::from_lino(&text).unwrap();
    }
    network
}

fn output(case: &Value) -> Vec<String> {
    case["output"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect()
}

fn run_javascript(code: &str) -> Vec<String> {
    let script = format!(
        "const output=[];require('node:vm').runInNewContext({},{{console:{{log:(...values)=>output.push(values.map(String).join(' '))}},process:{{argv:[]}}}},{{timeout:1000}});process.stdout.write(JSON.stringify(output));",
        serde_json::to_string(code).unwrap()
    );
    let result = Command::new("node").arg("-e").arg(script).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}

fn run_rust(code: &str, directory: &Path, name: &str) -> Vec<String> {
    let source = directory.join(format!("{name}.rs"));
    let binary = directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, code).unwrap();
    let mut invocation = Command::new("rustc");
    invocation.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        invocation.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = invocation
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let result = Command::new(binary).output().unwrap();
    assert!(
        result.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}

fn referenced_identities(network: &LinkNetwork) -> Vec<(LinkId, String, LinkId)> {
    network
        .links()
        .filter_map(|link| match link.references() {
            [parent, label, child] if link.metadata().link_type() == Some(LinkType::Field) => {
                let label = network.link(*label)?.metadata().term()?;
                label
                    .ends_with("reference")
                    .then(|| (*parent, label.to_owned(), *child))
            }
            _ => None,
        })
        .collect()
}

fn field(network: &LinkNetwork, parent: LinkId, wanted: &str) -> Option<LinkId> {
    network.links().find_map(|link| match link.references() {
        [owner, label, child]
            if link.metadata().link_type() == Some(LinkType::Field)
                && *owner == parent
                && network.link(*label)?.metadata().term()? == wanted =>
        {
            Some(*child)
        }
        _ => None,
    })
}

#[test]
fn source_free_algebraic_programs_execute_against_javascript_and_native_originals() {
    let fixture = fixture();
    let directory = TemporaryDirectory::new("typed-program-algebraic");
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    for (index, case) in cases.iter().enumerate() {
        let (network, root) = graph(case);
        decode_typed_program_links(&network, root).expect("the algebraic graph is checked");
        let expected = output(case);
        assert_eq!(
            run_javascript(case["oracleSource"].as_str().unwrap()),
            expected
        );
        let javascript = emit_typed_program_links(&network, root, "JavaScript").unwrap();
        assert_eq!(javascript.code, case["javascript"].as_str().unwrap());
        assert_eq!(run_javascript(&javascript.code), expected);
        let rust = emit_typed_program_links(&network, root, "Rust").unwrap();
        assert_eq!(rust.code, case["rust"].as_str().unwrap());
        assert_eq!(
            run_rust(&rust.code, &directory.0, &format!("target_{index}")),
            expected
        );
        if case["language"] == "Rust" {
            assert_eq!(
                run_rust(
                    case["source"].as_str().unwrap(),
                    &directory.0,
                    &format!("original_{index}")
                ),
                expected
            );
        }
        let created = create_typed_program_links(
            case["source"].as_str().unwrap(),
            case["language"].as_str().unwrap(),
        )
        .unwrap();
        let reloaded = LinkNetwork::from_lino(&created.network.to_lino()).unwrap();
        assert_eq!(
            run_javascript(
                &emit_typed_program_links(&reloaded, created.root, "JavaScript")
                    .unwrap()
                    .code
            ),
            expected
        );
        assert!(network.links().all(|link| {
            !link
                .metadata()
                .definition()
                .is_some_and(|definition| definition.contains(case["source"].as_str().unwrap()))
        }));
        let identities = referenced_identities(&network);
        let datatypes: BTreeSet<_> = identities
            .iter()
            .filter(|(_, label, _)| label == "data type declaration reference")
            .map(|(_, _, child)| *child)
            .collect();
        assert!(!datatypes.is_empty());
        assert!(
            identities
                .iter()
                .any(|(_, label, _)| label == "constructor declaration reference")
        );
        assert!(
            identities
                .iter()
                .any(|(_, label, _)| label == "binding reference")
        );
        let (parent, _, child) = identities
            .iter()
            .find(|(_, label, _)| label == "data type declaration reference")
            .unwrap();
        let label = network
            .links()
            .find_map(|link| match link.references() {
                [owner, label, value] if owner == parent && value == child => Some(*label),
                _ => None,
            })
            .unwrap();
        let legacy = apply_edits(
            LinkNetwork::from_lino(&network.to_lino()).unwrap(),
            &serde_json::json!([{ "kind": "term", "link": label.as_u64(), "value": "datatype declaration reference" }]),
        );
        assert_eq!(
            run_javascript(
                &emit_typed_program_links(&legacy, root, "JavaScript")
                    .unwrap()
                    .code
            ),
            expected
        );
        let mut duplicate = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        duplicate.insert_field(*parent, "datatype declaration reference", *child);
        assert!(decode_typed_program_links(&duplicate, root).is_err());
    }
}

#[test]
fn linked_algebraic_renames_and_recursive_edits_execute_in_both_targets() {
    let fixture = fixture();
    let directory = TemporaryDirectory::new("typed-program-algebraic-edits");
    let cases = fixture["cases"].as_array().unwrap();
    let mutations = fixture["mutations"].as_array().unwrap();
    assert_eq!(mutations.len(), 6);
    for (index, mutation) in mutations.iter().enumerate() {
        let case = cases
            .iter()
            .find(|case| case["id"] == mutation["case"])
            .unwrap();
        let (original, root) = graph(case);
        let identities = referenced_identities(&original);
        let changed = apply_edits(original, &mutation["edits"]);
        let reloaded = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
        assert_eq!(referenced_identities(&reloaded), identities);
        let expected = output(mutation);
        assert_eq!(
            run_javascript(mutation["oracleSource"].as_str().unwrap()),
            expected
        );
        let javascript = emit_typed_program_links(&reloaded, root, "JavaScript").unwrap();
        assert_eq!(javascript.code, mutation["javascript"].as_str().unwrap());
        assert_eq!(run_javascript(&javascript.code), expected);
        let rust = emit_typed_program_links(&reloaded, root, "Rust").unwrap();
        assert_eq!(rust.code, mutation["rust"].as_str().unwrap());
        assert_eq!(
            run_rust(&rust.code, &directory.0, &format!("edited_{index}")),
            expected
        );
        assert_eq!(
            run_rust(
                mutation["nativeOracle"].as_str().unwrap(),
                &directory.0,
                &format!("oracle_{index}")
            ),
            expected
        );
    }
}

#[test]
fn invalid_algebraic_identity_scope_type_arity_and_metadata_edits_are_refused() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    let invalid = fixture["invalid"].as_array().unwrap();
    assert_eq!(invalid.len(), 13);
    for mutation in invalid {
        let case = cases
            .iter()
            .find(|case| case["id"] == mutation["case"])
            .unwrap();
        let (network, root) = graph(case);
        let changed = apply_edits(network, &mutation["edits"]);
        assert!(
            decode_typed_program_links(&changed, root).is_err(),
            "{}",
            mutation["id"]
        );
        for target in ["JavaScript", "Rust"] {
            assert!(
                emit_typed_program_links(&changed, root, target).is_err(),
                "{}: {target}",
                mutation["id"]
            );
        }
    }
}

#[test]
fn native_neutral_fields_are_admitted_only_in_their_actual_semantic_role() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    let original = cases
        .iter()
        .find(|case| case["id"] == "rust-choice")
        .unwrap();
    let (network, root) = graph(original);
    let false_value = network
        .links()
        .find(|link| {
            link.metadata().term() == Some("boolean value")
                && link.metadata().definition() == Some("false")
        })
        .unwrap()
        .id();
    let pattern = network
        .links()
        .find(|link| field(&network, link.id(), "pattern bindings").is_some())
        .unwrap()
        .id();
    let constructor = field(&network, pattern, "constructor declaration reference").unwrap();
    for (owner, label) in [
        (constructor, "generated"),
        (pattern, "constructor immutability"),
    ] {
        let mut changed = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        changed.insert_field(owner, label, false_value);
        assert!(
            decode_typed_program_links(&changed, root).is_err(),
            "{label}"
        );
        for target in ["JavaScript", "Rust"] {
            assert!(emit_typed_program_links(&changed, root, target).is_err());
        }
    }
    let javascript = cases
        .iter()
        .find(|case| case["id"] == "javascript-choice")
        .unwrap();
    let (mut changed, root) = graph(javascript);
    let false_value = changed
        .links()
        .find(|link| {
            link.metadata().term() == Some("boolean value")
                && link.metadata().definition() == Some("false")
        })
        .unwrap()
        .id();
    let value = changed
        .links()
        .find(|link| {
            field(&changed, link.id(), "constructor declaration reference").is_some()
                && field(&changed, link.id(), "arguments").is_some()
        })
        .unwrap()
        .id();
    changed.insert_field(value, "constructor immutability", false_value);
    decode_typed_program_links(&changed, root)
        .expect("false is neutral for an actual constructor expression");
    for target in ["JavaScript", "Rust"] {
        let emitted = emit_typed_program_links(&changed, root, target).unwrap();
        if target == "JavaScript" {
            assert_eq!(run_javascript(&emitted.code), output(javascript));
        }
    }
}
