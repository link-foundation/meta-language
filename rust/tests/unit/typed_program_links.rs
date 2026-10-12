//! Shared source-free graph reload, edit and rejection cases.
use meta_language::{
    LinkId, LinkNetwork, LinkType, create_typed_program_links, decode_typed_program_links,
    emit_typed_program_links, percent_encode_links_text,
};
use serde_json::Value;
use std::process::Command;

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../parity/fixtures/typed-program-links.json"
    ))
    .unwrap()
}
fn field(network: &LinkNetwork, parent: LinkId, label: &str) -> LinkId {
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, name, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && *owner == parent
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some(label) =>
            {
                Some(*child)
            }
            _ => None,
        })
        .unwrap()
}
fn scalar(network: &LinkNetwork, label: &str, value: &str) -> LinkId {
    network
        .links()
        .find_map(|link| match link.references() {
            [_, name, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some(label)
                    && network
                        .link(*child)
                        .and_then(|child| child.metadata().definition())
                        == Some(value) =>
            {
                Some(*child)
            }
            _ => None,
        })
        .unwrap()
}
fn edit_definition(network: &LinkNetwork, id: LinkId, value: &str) -> LinkNetwork {
    let before = network.link(id).unwrap().metadata().definition().unwrap();
    let marker = format!("(def: {})", percent_encode_links_text(before));
    let replacement = format!("(def: {})", percent_encode_links_text(value));
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({id}: ")) {
                line.replacen(&marker, &replacement, 1)
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&text).unwrap()
}
fn edit_references(network: &LinkNetwork, id: LinkId, references: &[LinkId]) -> LinkNetwork {
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({id}: ")) {
                let metadata = &line[line.find("(meta:").unwrap()..];
                format!(
                    "({id}: {} {metadata}",
                    references
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&text).unwrap()
}
fn run_javascript(code: &str) -> Vec<String> {
    let script = format!(
        "const output=[]; require('node:vm').runInNewContext({},{{console:{{log:(...v)=>output.push(v.map(String).join(' '))}},process:{{argv:[]}}}},{{timeout:1000}}); process.stdout.write(JSON.stringify(output));",
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

#[test]
fn javascript_and_rust_sources_emit_from_reloaded_source_free_graphs() {
    for case in fixture()["cases"].as_array().unwrap() {
        let graph = LinkNetwork::from_lino(case["lino"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["root"].as_u64().unwrap());
        let javascript = emit_typed_program_links(&graph, root, "JavaScript").unwrap();
        assert_eq!(
            javascript.code,
            case["javascript"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        let expected: Vec<_> = case["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect();
        assert_eq!(run_javascript(&javascript.code), expected);
        assert_eq!(
            emit_typed_program_links(&graph, root, "Rust").unwrap().code,
            case["rust"].as_str().unwrap()
        );
        let created = create_typed_program_links(
            case["source"].as_str().unwrap(),
            case["language"].as_str().unwrap(),
        )
        .unwrap();
        let replay = LinkNetwork::from_lino(&created.network.to_lino()).unwrap();
        assert_eq!(
            run_javascript(
                &emit_typed_program_links(&replay, created.root, "JavaScript")
                    .unwrap()
                    .code
            ),
            expected
        );
        assert!(created.network.links().all(|link| {
            !link
                .metadata()
                .definition()
                .is_some_and(|definition| definition.contains(case["source"].as_str().unwrap()))
        }));
    }
}

#[test]
fn native_index_model_emits_from_an_edited_source_free_graph() {
    let fixtures = fixture();
    for case in fixtures["derivedCases"].as_array().unwrap() {
        let graph = LinkNetwork::from_lino(case["lino"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["root"].as_u64().unwrap());
        let emitted = emit_typed_program_links(&graph, root, "JavaScript").unwrap();
        assert_eq!(emitted.code, case["javascript"].as_str().unwrap());
        assert_eq!(run_javascript(&emitted.code), ["2"]);
        assert_eq!(
            emit_typed_program_links(&graph, root, "Rust").unwrap().code,
            case["rust"].as_str().unwrap()
        );
    }
}

#[test]
fn operation_and_binding_edits_change_the_actual_linked_program() {
    let fixtures = fixture();
    let case = &fixtures["cases"][0];
    let graph = LinkNetwork::from_lino(case["lino"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    let changed = edit_definition(&graph, scalar(&graph, "operation", "add"), "mul");
    assert_eq!(
        run_javascript(
            &emit_typed_program_links(&changed, root, "JavaScript")
                .unwrap()
                .code
        ),
        ["6"]
    );
    assert!(
        emit_typed_program_links(&changed, root, "Rust")
            .unwrap()
            .code
            .contains("a * b")
    );
    let binder = graph
        .links()
        .find_map(|link| match link.references() {
            [_, label, child]
                if graph.link(*label).and_then(|label| label.metadata().term())
                    == Some("binding reference") =>
            {
                Some(*child)
            }
            _ => None,
        })
        .unwrap();
    let changed = edit_definition(&graph, field(&graph, binder, "name"), "renamed");
    let emitted = emit_typed_program_links(&changed, root, "JavaScript").unwrap();
    assert!(emitted.code.contains("add(renamed, b)"));
    assert_eq!(run_javascript(&emitted.code), ["5"]);
}

#[test]
fn declaration_retargeting_uses_owned_identity_and_checked_signatures() {
    let fixtures = fixture();
    let case = &fixtures["cases"][2];
    let graph = LinkNetwork::from_lino(case["lino"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    let name = scalar(&graph, "full name", "multiply");
    let target = graph
        .links()
        .find_map(|link| match link.references() {
            [parent, _, child] if *child == name => Some(*parent),
            _ => None,
        })
        .unwrap();
    let reference = graph.links().find(|link| matches!(link.references(), [_, label, _] if graph.link(*label).and_then(|label| label.metadata().term()) == Some("declaration reference"))).unwrap();
    let changed = edit_references(
        &graph,
        reference.id(),
        &[reference.references()[0], reference.references()[1], target],
    );
    assert_eq!(
        run_javascript(
            &emit_typed_program_links(&changed, root, "JavaScript")
                .unwrap()
                .code
        ),
        ["6"]
    );
}

#[test]
fn malformed_literal_type_and_structural_edits_are_rejected_before_emission() {
    let fixtures = fixture();
    let case = &fixtures["cases"][0];
    let graph = LinkNetwork::from_lino(case["lino"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    let changed = edit_definition(&graph, scalar(&graph, "value", "2"), "0); injected(); (0");
    assert!(
        emit_typed_program_links(&changed, root, "JavaScript")
            .unwrap_err()
            .message
            .contains("floating-point literal")
    );
    let changed = edit_definition(&graph, scalar(&graph, "kind", "float"), "string");
    assert!(emit_typed_program_links(&changed, root, "Rust").is_err());
    let body = graph.link(root).unwrap().references()[0];
    let items = field(&graph, body, "items");
    let changed = edit_references(&graph, items, &[items]);
    assert!(
        decode_typed_program_links(&changed, root)
            .unwrap_err()
            .message
            .contains("structural cycle")
    );
}

#[test]
fn shared_graph_paths_have_a_bounded_semantic_walk() {
    let fixtures = fixture();
    for case in fixtures["resourceCases"].as_array().unwrap() {
        let graph = LinkNetwork::from_lino(case["lino"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["root"].as_u64().unwrap());
        assert!(
            graph.len() < 1000,
            "the fixture must exercise repeated paths rather than unique-node bounds"
        );
        assert!(
            decode_typed_program_links(&graph, root)
                .unwrap_err()
                .message
                .contains("semantic traversal resource limit")
        );
    }
}

#[test]
fn machine_integer_width_edits_return_a_diagnostic_before_emission() {
    let fixtures = fixture();
    let case = &fixtures["cases"][1];
    let graph = LinkNetwork::from_lino(case["lino"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    let bits = scalar(&graph, "bits", "32");
    for width in fixtures["invalidMachineIntegerWidths"].as_array().unwrap() {
        let changed = edit_definition(&graph, bits, &width.to_string());
        assert!(
            emit_typed_program_links(&changed, root, "Rust")
                .unwrap_err()
                .message
                .contains("invalid machine integer type")
        );
    }
}
