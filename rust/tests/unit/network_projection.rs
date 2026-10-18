//! Finite public projection API and original-link provenance parity.
//! These controls do not establish complete parsing for the language catalog.
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use meta_language::{
    BinaryLinksOptions, ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType,
    NetworkProjection, ParseConfiguration, Point, SourceSpan, decode_link_network_binary,
    encode_link_network_binary,
};
use serde_json::{Value, json};

fn fixture() -> Value {
    meta_language::project_data::read_project_data_file(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/network-projection-expectations.lino"),
    )
    .expect("the independent shared projection matrix loads")
}

fn projection(name: &str) -> NetworkProjection {
    match name {
        "Lossless" => NetworkProjection::Lossless,
        "ConcreteSyntax" => NetworkProjection::ConcreteSyntax,
        "AbstractSyntax" => NetworkProjection::AbstractSyntax,
        "Semantic" => NetworkProjection::Semantic,
        _ => panic!("the finite fixture names a public projection"),
    }
}

fn role(name: &str) -> LinkType {
    match name {
        "Dynamic" => LinkType::Link,
        "Reference" => LinkType::Reference,
        "Relation" => LinkType::Relation,
        "Language" => LinkType::Language,
        "Grammar" => LinkType::Grammar,
        "Type" => LinkType::Type,
        "Concept" => LinkType::Concept,
        "Syntax" => LinkType::Syntax,
        "Field" => LinkType::Field,
        "Trivia" => LinkType::Trivia,
        "SourceToken" => LinkType::Token,
        "Document" => LinkType::Document,
        "Semantic" => LinkType::Semantic,
        "Region" => LinkType::Region,
        "Object" => LinkType::Object,
        _ => panic!("the finite fixture names a public link role"),
    }
}

fn role_network(data: &Value) -> LinkNetwork {
    let mut network = LinkNetwork::new();
    for kind in data["linkTypes"].as_array().unwrap() {
        let metadata = kind.as_str().map_or_else(LinkMetadata::new, |name| {
            LinkMetadata::new().with_link_type(role(name))
        });
        network.insert_link([], metadata);
    }
    network
}

#[test]
fn every_public_projection_matches_the_independent_role_matrix_without_copying_links() {
    let data = fixture();
    let network = role_network(&data);
    let before = network.to_lino();
    for entry in data["cases"].as_array().unwrap() {
        let selected = projection(entry["projection"].as_str().unwrap());
        let actual = network
            .projected_links(selected)
            .map(|link| {
                assert!(std::ptr::eq(link, network.link(link.id()).unwrap()));
                link.id().as_u64()
            })
            .collect::<Vec<_>>();
        assert_eq!(json!(actual), entry["included"], "{}", selected.label());
    }
    assert_eq!(network.to_lino(), before);
    assert_eq!(network.len(), 16);
}

#[test]
fn abstract_results_keep_original_concrete_references_fields_spans_and_recovery() {
    let mut network = LinkNetwork::new();
    let span = SourceSpan::new(ByteRange::new(0, 2), Point::new(0, 0), Point::new(0, 2));
    let token = network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_term("id")
            .with_language("JavaScript")
            .with_span(span)
            .with_flags(LinkFlags::missing().with_extra()),
    );
    let trivia = network.insert_link(
        [token],
        LinkMetadata::new()
            .with_link_type(LinkType::Trivia)
            .with_span(span),
    );
    let syntax = network.insert_link(
        [token, trivia],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_term("identifier")
            .with_language("JavaScript")
            .with_named(true)
            .with_span(span)
            .with_flags(LinkFlags::containing_error()),
    );
    let field = network.insert_field(syntax, "name", token);
    let label = network.link(field).unwrap().references()[1];
    let region = network.insert_link(
        [syntax, token],
        LinkMetadata::new()
            .with_link_type(LinkType::Region)
            .with_span(span),
    );
    let provenance = network.insert_link(
        [syntax, token, trivia],
        LinkMetadata::new().with_link_type(LinkType::Reference),
    );
    let before = network.to_lino();
    let selected = network
        .projected_links(NetworkProjection::AbstractSyntax)
        .collect::<Vec<_>>();
    assert_eq!(
        selected.iter().map(|link| link.id()).collect::<Vec<_>>(),
        [syntax, label, field, region, provenance]
    );
    let actual = selected
        .iter()
        .copied()
        .find(|link| link.id() == syntax)
        .unwrap();
    assert!(std::ptr::eq(actual, network.link(syntax).unwrap()));
    assert_eq!(actual.metadata().span(), Some(span));
    assert!(actual.metadata().flags().has_error());
    assert_eq!(
        network.link(field).unwrap().references(),
        &[syntax, label, token]
    );
    assert_eq!(network.link(label).unwrap().metadata().term(), Some("name"));
    assert_eq!(actual.references(), &[token, trivia]);
    for reference in actual.references() {
        assert!(network.link(*reference).is_some());
        assert!(!selected.iter().any(|link| link.id() == *reference));
    }
    let token_flags = network.link(token).unwrap().metadata().flags();
    assert!(token_flags.is_missing());
    assert!(token_flags.is_extra());
    assert_eq!(network.to_lino(), before);
}

#[test]
fn default_parse_keeps_concrete_source_after_explicit_abstract_selection() {
    let source = "const value = 1; // café\n";
    let network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let before = network.to_lino();
    let concrete = network
        .projected_links(NetworkProjection::ConcreteSyntax)
        .collect::<Vec<_>>();
    assert!(
        concrete
            .iter()
            .any(|link| link.metadata().link_type() == Some(LinkType::Token))
    );
    assert!(
        concrete
            .iter()
            .any(|link| link.metadata().link_type() == Some(LinkType::Trivia))
    );
    assert!(
        concrete
            .iter()
            .any(|link| link.metadata().link_type() == Some(LinkType::Syntax))
    );
    let abstract_links = network
        .projected_links(NetworkProjection::AbstractSyntax)
        .collect::<Vec<_>>();
    assert!(!abstract_links.is_empty());
    assert!(abstract_links.iter().all(|link| !matches!(
        link.metadata().link_type(),
        Some(LinkType::Token | LinkType::Trivia)
    )));
    assert!(
        abstract_links
            .iter()
            .any(|link| link
                .references()
                .iter()
                .any(|id| network.link(*id).is_some_and(|child| matches!(
                    child.metadata().link_type(),
                    Some(LinkType::Token | LinkType::Trivia)
                ))))
    );
    assert_eq!(network.reconstruct_text(), source);
    assert_eq!(network.to_lino(), before);
    assert_eq!(
        network.projected_links(NetworkProjection::Lossless).count(),
        network.len()
    );
}

#[test]
fn actual_native_and_javascript_apis_select_the_same_original_ids_from_one_saved_network() {
    // Cross-runtime identity/provenance is observed on an actual public parse,
    // independently of the complete, literal role matrix in the first control.
    let source = "const value = 1; // café\n";
    let network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let before = network.to_lino();
    let script = r"
import { readFileSync } from 'node:fs';
import { LinkNetwork, LinkType, NetworkProjection } from './js/src/index.js';
const network = LinkNetwork.fromLino(readFileSync(0, 'utf8'));
const before = network.toLino(), projections = {}, operands = {};
for (const selection of Object.values(NetworkProjection)) {
  const links = network.projectedLinks(selection);
  projections[selection] = links.map(link => {
    if (link !== network.link(link.id())) throw new Error('a projected Link lost source identity');
    if (link.metadata() !== network.link(link.id()).metadata()) throw new Error('projected metadata lost source identity');
    return link.id().asU64();
  });
  operands[selection] = links.map(link => [link.id().asU64(), link.references().map(reference => {
    if (!network.link(reference)) throw new Error('a source provenance reference no longer resolves');
    return reference.asU64();
  })]);
}
const abstract = network.projectedLinks(NetworkProjection.AbstractSyntax);
process.stdout.write(JSON.stringify({
  projections, operands, network: network.toLino(), unchanged: before === network.toLino(),
  concreteDefault: network.projectedLinks().map(link => link.id().asU64()),
  aliasDefault: network.projected_links().map(link => link.id().asU64()),
  source: network.reconstructText(),
  abstractHasConcreteProvenance: abstract.some(link => link.references().some(reference =>
    [LinkType.Token, LinkType.Trivia].includes(network.link(reference).metadata().linkType)))
}));
";
    let mut child = Command::new("node")
        .args(["--input-type=module", "-e", script])
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("CI provides the existing Node parity runtime");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(before.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let observed: Value = serde_json::from_slice(&output.stdout)
        .expect("the complete actual process result is valid JSON");
    for entry in fixture()["cases"].as_array().unwrap() {
        let name = entry["projection"].as_str().unwrap();
        let selected = network
            .projected_links(projection(name))
            .collect::<Vec<_>>();
        let native = selected
            .iter()
            .map(|link| {
                assert!(std::ptr::eq(*link, network.link(link.id()).unwrap()));
                link.id().as_u64()
            })
            .collect::<Vec<_>>();
        let operands = selected
            .iter()
            .map(|link| {
                (
                    link.id().as_u64(),
                    link.references()
                        .iter()
                        .map(|reference| {
                            assert!(network.link(*reference).is_some());
                            reference.as_u64()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(observed["projections"][name], json!(native));
        assert_eq!(observed["operands"][name], json!(operands));
    }
    let concrete = network
        .projected_links(NetworkProjection::ConcreteSyntax)
        .map(|link| link.id().as_u64())
        .collect::<Vec<_>>();
    assert_eq!(observed["network"], before);
    assert_eq!(observed["unchanged"], true);
    assert_eq!(observed["concreteDefault"], json!(concrete));
    assert_eq!(observed["aliasDefault"], json!(concrete));
    assert_eq!(observed["source"], source);
    assert_eq!(observed["abstractHasConcreteProvenance"], true);
    assert_eq!(network.reconstruct_text(), source);
    assert_eq!(network.to_lino(), before);
}

fn projection_data(name: &str) -> Value {
    meta_language::project_data::read_project_data_file(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity")
            .join(name),
    )
    .expect("the existing independent projection source or CST fixture loads")
}

fn projection_document_root(network: &LinkNetwork) -> LinkId {
    let document = network
        .links()
        .find(|link| link.metadata().link_type() == Some(LinkType::Document))
        .expect("ordinary parsing retains its document")
        .id();
    network
        .links()
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax) && link.references() == [document]
        })
        .expect("ordinary parsing exposes its actual grammar root")
        .id()
}

fn projection_syntax_rows(network: &LinkNetwork, root: LinkId, base: usize) -> Vec<Value> {
    let mut children: HashMap<LinkId, Vec<LinkId>> = HashMap::new();
    let mut fields: HashMap<(LinkId, LinkId), String> = HashMap::new();
    for link in network.links() {
        match link.metadata().link_type() {
            Some(LinkType::Syntax) => {
                if let [parent] = link.references() {
                    children.entry(*parent).or_default().push(link.id());
                }
            }
            Some(LinkType::Field) if link.metadata().term().is_none() => {
                if let [parent, label, child] = link.references() {
                    fields.insert(
                        (*parent, *child),
                        network
                            .link(*label)
                            .unwrap()
                            .metadata()
                            .term()
                            .unwrap()
                            .to_owned(),
                    );
                }
            }
            _ => {}
        }
    }
    let mut rows = Vec::new();
    let mut stack = vec![(root, 0_usize, None::<String>)];
    while let Some((id, depth, field)) = stack.pop() {
        let metadata = network.link(id).unwrap().metadata();
        let flags = metadata.flags();
        let range = metadata
            .span()
            .expect("a CST node keeps its exact byte span")
            .byte_range();
        rows.push(json!([
            depth,
            field,
            metadata.term(),
            i32::from(metadata.is_named()),
            range.start() - base,
            range.end() - base,
            format!(
                "{}{}{}",
                if flags.is_error() { "E" } else { "" },
                if flags.is_missing() { "M" } else { "" },
                if flags.is_extra() { "X" } else { "" }
            )
        ]));
        for child in children.get(&id).into_iter().flatten().rev() {
            stack.push((*child, depth + 1, fields.get(&(id, *child)).cloned()));
        }
    }
    rows
}

fn assert_projection_replay(
    network: &LinkNetwork,
    source: &str,
    trees: &[(LinkId, usize, &Value)],
    label: &str,
) {
    // Ordinary parsing must retain the full concrete graph before view selection.
    assert_eq!(
        network.reconstruct_text(),
        source,
        "{label}: original source bytes"
    );
    for role in [LinkType::Syntax, LinkType::Token, LinkType::Trivia] {
        assert!(
            network
                .links()
                .any(|link| link.metadata().link_type() == Some(role)),
            "{label}: original CST role {role:?}"
        );
    }
    for (root, base, expected) in trees {
        assert_eq!(
            json!(projection_syntax_rows(network, *root, *base)),
            **expected,
            "{label}: original independent CST rows"
        );
    }
    let before = network.to_lino();
    let concrete_ids = network
        .projected_links(NetworkProjection::ConcreteSyntax)
        .map(meta_language::Link::id)
        .collect::<Vec<_>>();
    assert_eq!(
        concrete_ids,
        network
            .projected_links(NetworkProjection::ConcreteSyntax)
            .map(meta_language::Link::id)
            .collect::<Vec<_>>()
    );
    for role in [LinkType::Syntax, LinkType::Token, LinkType::Trivia] {
        assert!(
            network
                .projected_links(NetworkProjection::ConcreteSyntax)
                .any(|link| link.metadata().link_type() == Some(role)),
            "{label}: default concrete role {role:?}"
        );
    }
    let abstract_ids = network
        .projected_links(NetworkProjection::AbstractSyntax)
        .map(meta_language::Link::id)
        .collect::<Vec<_>>();
    assert!(!abstract_ids.is_empty());
    assert_ne!(
        abstract_ids, concrete_ids,
        "abstract syntax requires a distinct explicit selection"
    );
    assert_eq!(
        network.to_lino(),
        before,
        "{label}: selection leaves every original link unchanged"
    );
    let binary = encode_link_network_binary(network, BinaryLinksOptions::default()).unwrap();
    for (encoding, restored) in [
        ("text", LinkNetwork::from_lino(&before).unwrap()),
        (
            "binary",
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ),
    ] {
        assert_eq!(
            restored.to_lino(),
            before,
            "{label}: {encoding} complete graph"
        );
        assert_eq!(
            restored.reconstruct_text(),
            source,
            "{label}: {encoding} exact UTF-8 source"
        );
        assert_eq!(
            restored.links().collect::<Vec<_>>(),
            network.links().collect::<Vec<_>>(),
            "all original references, metadata, spans and recovery flags remain identical"
        );
        assert_eq!(
            restored
                .projected_links(NetworkProjection::ConcreteSyntax)
                .map(meta_language::Link::id)
                .collect::<Vec<_>>(),
            concrete_ids
        );
        let abstract_links = restored
            .projected_links(NetworkProjection::AbstractSyntax)
            .collect::<Vec<_>>();
        assert_eq!(
            abstract_links
                .iter()
                .copied()
                .map(meta_language::Link::id)
                .collect::<Vec<_>>(),
            abstract_ids
        );
        for link in &abstract_links {
            assert!(std::ptr::eq(*link, restored.link(link.id()).unwrap()));
            assert!(!matches!(
                link.metadata().link_type(),
                Some(LinkType::Token | LinkType::Trivia)
            ));
            for reference in link.references() {
                assert!(
                    restored.link(*reference).is_some(),
                    "concrete provenance remains resolvable"
                );
            }
        }
        for link in restored.links().filter(|link| {
            matches!(
                link.metadata().link_type(),
                Some(LinkType::Syntax | LinkType::Field | LinkType::Region)
            )
        }) {
            assert!(
                abstract_links
                    .iter()
                    .any(|selected| selected.id() == link.id()),
                "all syntax, field and region links remain in the explicit view"
            );
        }
        for (root, base, expected) in trees {
            assert_eq!(
                json!(projection_syntax_rows(&restored, *root, *base)),
                **expected,
                "{label}: {encoding} independent CST rows"
            );
        }
        assert_eq!(
            restored
                .projected_links(NetworkProjection::ConcreteSyntax)
                .map(|link| link.id())
                .collect::<Vec<_>>(),
            concrete_ids,
            "AST selection cannot change the concrete view"
        );
        assert_eq!(restored.to_lino(), before);
    }
}

#[test]
fn four_priority_language_csts_keep_original_abstract_provenance_through_both_saved_formats() {
    let inventory = projection_data("language-grammar-inventory.lino");
    let oracle = projection_data("fixtures/default-cst-expected.lino");
    let native = projection_data("fixtures/native-default-cst-expected.lino");
    for name in ["JavaScript", "TypeScript", "Rust", "Markdown"] {
        let language = inventory["languages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["name"] == name)
            .unwrap();
        let expected = native["languages"]
            .get(name)
            .unwrap_or_else(|| &oracle["languages"][name]);
        assert!(
            !expected.is_null(),
            "{name}: independent CST expectations exist"
        );
        for (part, key) in [("positive", "source"), ("recovery", "recoverySource")] {
            let source = language[key].as_str().unwrap();
            let network = LinkNetwork::parse(source, name, ParseConfiguration::default());
            assert_projection_replay(
                &network,
                source,
                &[(projection_document_root(&network), 0, &expected[part])],
                &format!("{name} {part}"),
            );
        }
    }
    super::issue_195_observations::record(&super::issue_195_observations::Observation {
        requirement_id: "I195-CST-EXPLICIT-AST-SELECTION",
        suffix: "priority-projection-replay",
        fixture_id: "cst-projection-priority-rust",
        fixture_file: "parity/language-grammar-inventory.lino",
        assertions: &[
            "abstractSyntaxRequiresExplicitSelection",
            "concreteSyntaxRemainsDefault",
            "abstractProjectionRetainsConcreteProvenance",
        ],
        test_name: "four_priority_language_csts_keep_original_abstract_provenance_through_both_saved_formats",
    });
}

#[test]
fn the_original_markdown_rust_region_keeps_host_and_child_cst_provenance_in_abstract_views() {
    let evidence = projection_data("fixtures/issue-195-evidence.lino");
    let original = evidence["embedded"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| {
            entry["host"] == "Markdown" && entry["target"] == "language-selected fenced code"
        })
        .unwrap();
    let oracle = projection_data("fixtures/default-cst-expected.lino");
    let native = projection_data("fixtures/native-default-cst-expected.lino");
    let label = format!(
        "{} -> {}",
        original["host"].as_str().unwrap(),
        original["target"].as_str().unwrap()
    );
    let language = original["parseLanguage"].as_str().unwrap();
    for (part, key) in [("positive", "source"), ("recovery", "recoverySource")] {
        let source = original[key].as_str().unwrap();
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        let expected = &oracle["embeddedFixtures"][label.as_str()][part];
        let embedded = native["embeddedFixtures"]
            .get(label.as_str())
            .map_or(&expected["embedded"], |entry| &entry[part]["embedded"])
            .as_array()
            .unwrap();
        let regions = network
            .links()
            .filter(|link| {
                link.metadata().link_type() == Some(LinkType::Region)
                    && link.references().len() == 2
            })
            .filter(|link| {
                link.metadata().span().is_some_and(|span| {
                    let range = span.byte_range();
                    !(link.metadata().language() == Some(language)
                        && range.start() == 0
                        && range.end() == source.len())
                })
            })
            .collect::<Vec<_>>();
        let boundaries = regions
            .iter()
            .map(|region| {
                let range = region.metadata().span().unwrap().byte_range();
                json!([region.metadata().language(), range.start(), range.end()])
            })
            .collect::<Vec<_>>();
        let wanted = embedded
            .iter()
            .map(|region| json!([region["language"], region["startByte"], region["endByte"]]))
            .collect::<Vec<_>>();
        assert_eq!(boundaries, wanted);
        let mut trees = vec![(projection_document_root(&network), 0, &expected["rows"])];
        for (region, wanted) in regions.iter().zip(embedded) {
            let root = network
                .links()
                .find(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.references() == [region.id()]
                })
                .expect("the region owns its actual child grammar root");
            assert!(
                network.link(region.references()[0]).is_some(),
                "the actual host document reference remains present"
            );
            trees.push((
                root.id(),
                region.metadata().span().unwrap().byte_range().start(),
                &wanted["rows"],
            ));
        }
        assert!(
            regions
                .iter()
                .any(|region| region.metadata().language() == original["regionLanguage"].as_str())
        );
        assert_projection_replay(&network, source, &trees, &format!("{label} {part}"));
    }
    super::issue_195_observations::record(&super::issue_195_observations::Observation {
        requirement_id: "I195-CST-EXPLICIT-AST-SELECTION",
        suffix: "embedded-projection-replay",
        fixture_id: "cst-projection-embedded-rust",
        fixture_file: "parity/fixtures/issue-195-evidence.lino",
        assertions: &[
            "abstractSyntaxRequiresExplicitSelection",
            "concreteSyntaxRemainsDefault",
            "abstractProjectionRetainsConcreteProvenance",
        ],
        test_name: "the_original_markdown_rust_region_keeps_host_and_child_cst_provenance_in_abstract_views",
    });
}
