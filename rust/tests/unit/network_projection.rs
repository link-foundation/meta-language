//! Finite public projection API and original-link provenance parity.
//! These controls do not establish complete parsing for the language catalog.
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use meta_language::{
    ByteRange, LinkFlags, LinkMetadata, LinkNetwork, LinkType, NetworkProjection,
    ParseConfiguration, Point, SourceSpan,
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
