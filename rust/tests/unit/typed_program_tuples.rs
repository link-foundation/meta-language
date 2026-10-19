//! Shared tuple source oracles and source-free graph edits; compilers run in CI.
use super::issue_195_translation_pairs::TemporaryDirectory;
use meta_language::{
    LinkId, LinkNetwork, LinkType, create_typed_program_links, decode_typed_program_links,
    emit_typed_program_links, percent_encode_links_text, read_structured_data_lino,
};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/typed-program-tuples.lino"
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
        let replacement = match edit["kind"].as_str().unwrap() {
            "references" => (
                None,
                edit["values"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_u64().unwrap().to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            "definition" => (
                Some(format!(
                    "(def: {})",
                    percent_encode_links_text(link.metadata().definition().unwrap())
                )),
                format!(
                    "(def: {})",
                    percent_encode_links_text(edit["value"].as_str().unwrap())
                ),
            ),
            _ => panic!("unknown tuple edit"),
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

fn expected(case: &Value) -> Vec<String> {
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

fn identities(network: &LinkNetwork) -> Vec<(LinkId, String, LinkId)> {
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

#[test]
fn source_free_tuple_programs_match_both_independent_originals_and_native_targets() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 10);
    let directory = TemporaryDirectory::new("typed-program-tuples");
    for (index, case) in cases.iter().enumerate() {
        let (network, root) = graph(case);
        decode_typed_program_links(&network, root).expect("the immutable tuple graph is checked");
        let expected = expected(case);
        assert_eq!(
            run_javascript(case["oracleSource"].as_str().unwrap()),
            expected
        );
        assert_eq!(
            run_rust(
                case["rustOracle"].as_str().unwrap(),
                &directory.0,
                &format!("original_{index}")
            ),
            expected
        );
        let javascript = emit_typed_program_links(&network, root, "JavaScript").unwrap();
        assert_eq!(run_javascript(&javascript.code), expected);
        let rust = emit_typed_program_links(&network, root, "Rust").unwrap();
        assert_eq!(
            run_rust(&rust.code, &directory.0, &format!("target_{index}")),
            expected
        );
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
                .is_some_and(|value| value.contains(case["source"].as_str().unwrap()))
        }));
    }
}

#[test]
fn linked_tuple_models_layouts_and_binding_identities_survive_reload() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let (network, root) = graph(case);
        decode_typed_program_links(&network, root).unwrap();
        let tuples: Vec<_> = network
            .links()
            .filter(|link| {
                field(&network, link.id(), "construct kind").is_some_and(|kind| {
                    network.link(kind).unwrap().metadata().definition() == Some("tuple")
                })
            })
            .collect();
        if case["program"] == "unit-and-group" {
            assert!(tuples.is_empty(), "Rust unit is not a zero-field tuple");
            continue;
        }
        assert_ne!(tuples, [] as [&meta_language::Link; 0]);
        for tuple in tuples {
            let frozen = field(&network, tuple.id(), "constructor immutability").unwrap();
            assert_eq!(
                network.link(frozen).unwrap().metadata().definition(),
                Some("true")
            );
            let model = field(&network, tuple.id(), "product model").unwrap();
            assert_eq!(
                network.link(model).unwrap().metadata().definition(),
                Some("immutable value product")
            );
            let items = field(&network, tuple.id(), "items").unwrap();
            assert_ne!(network.link(items).unwrap().references(), []);
        }
        let identities = identities(&network);
        assert!(
            identities
                .iter()
                .any(|(_, label, _)| label == "binding reference")
        );
        assert!(
            identities
                .iter()
                .any(|(_, label, _)| label == "declaration reference")
        );
    }
}

#[test]
fn linked_tuple_literal_edit_changes_execution_without_source_replay() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    let mutations = fixture["mutations"].as_array().unwrap();
    assert_eq!(mutations.len(), 1);
    let directory = TemporaryDirectory::new("typed-program-tuple-edits");
    for (index, mutation) in mutations.iter().enumerate() {
        let case = cases
            .iter()
            .find(|case| case["id"] == mutation["case"])
            .unwrap();
        let (network, root) = graph(case);
        let before = identities(&network);
        let edited = apply_edits(network, &mutation["edits"]);
        let reloaded = LinkNetwork::from_lino(&edited.to_lino()).unwrap();
        assert_eq!(identities(&reloaded), before);
        let expected = expected(mutation);
        assert_eq!(expected, ["false", "false"]);
        assert_eq!(
            run_javascript(mutation["oracleSource"].as_str().unwrap()),
            expected
        );
        assert_eq!(
            run_rust(
                mutation["rustOracle"].as_str().unwrap(),
                &directory.0,
                &format!("oracle_{index}")
            ),
            expected
        );
        assert_eq!(
            run_javascript(
                &emit_typed_program_links(&reloaded, root, "JavaScript")
                    .unwrap()
                    .code
            ),
            expected
        );
        assert_eq!(
            run_rust(
                &emit_typed_program_links(&reloaded, root, "Rust")
                    .unwrap()
                    .code,
                &directory.0,
                &format!("edited_{index}")
            ),
            expected
        );
    }
}

#[test]
fn malformed_tuple_layouts_indices_and_product_models_refuse_native_boundaries() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    let invalid = fixture["invalid"].as_array().unwrap();
    assert_eq!(invalid.len(), 9);
    for mutation in invalid {
        let case = cases
            .iter()
            .find(|case| case["id"] == mutation["case"])
            .unwrap();
        let (network, root) = graph(case);
        let edited = apply_edits(network, &mutation["edits"]);
        assert!(
            decode_typed_program_links(&edited, root).is_err(),
            "{}",
            mutation["id"]
        );
        for target in ["JavaScript", "Rust"] {
            assert!(
                emit_typed_program_links(&edited, root, target).is_err(),
                "{}: {target}",
                mutation["id"]
            );
        }
    }
}

#[test]
fn array_observers_external_callers_and_noncopy_sources_refuse_native_admission() {
    let fixture = fixture();
    let cases = fixture["refusals"].as_array().unwrap();
    assert_eq!(cases.len(), 23);
    for case in cases {
        assert!(
            create_typed_program_links(
                case["source"].as_str().unwrap(),
                case["language"].as_str().unwrap()
            )
            .is_err(),
            "{}",
            case["id"]
        );
    }
}

#[test]
fn chained_rust_product_projections_keep_exact_utf16_ranges_after_unicode() {
    let fixture = fixture();
    let witness = &fixture["syntaxWitness"];
    let parsed =
        meta_language::translation::rust::parse_rust(witness["source"].as_str().unwrap()).unwrap();
    let surface = serde_json::to_value(parsed).unwrap();
    let outer = &surface["items"][0]["body"];
    assert_eq!(outer["k"], "tupleIndex");
    assert_eq!(outer["index"], 1);
    assert_eq!(outer["span"], witness["outer"]);
    assert_eq!(outer["object"]["k"], "tupleIndex");
    assert_eq!(outer["object"]["index"], 0);
    assert_eq!(outer["object"]["span"], witness["inner"]);
}

#[test]
fn linked_zero_tuple_positions_accept_equivalent_numeric_spellings() {
    let fixture = fixture();
    let case = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "javascript-copy-choice")
        .unwrap();
    let (original, root) = graph(case);
    let index = original
        .links()
        .find_map(|link| {
            let kind = field(&original, link.id(), "construct kind")?;
            if original.link(kind)?.metadata().definition() != Some("tupleIndex") {
                return None;
            }
            let index = field(&original, link.id(), "index")?;
            (original.link(index)?.metadata().definition() == Some("0")).then_some(index)
        })
        .expect("the authentic tuple projection has a zero position");
    for spelling in ["-0", "0.0", "0e0"] {
        let edited = apply_edits(
            LinkNetwork::from_lino(&original.to_lino()).unwrap(),
            &serde_json::json!([{ "kind": "definition", "link": index.as_u64(), "value": spelling }]),
        );
        decode_typed_program_links(&edited, root)
            .expect("the equivalent spelling still denotes position zero");
        let javascript = emit_typed_program_links(&edited, root, "JavaScript").unwrap();
        assert_eq!(
            run_javascript(&javascript.code),
            expected(case),
            "{spelling}"
        );
    }
}
