use std::collections::{BTreeMap, HashMap};

use meta_language::{
    AccessMode, ByteRange, Link, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType,
    ParseConfiguration, Point, RegionDetectionPolicy, SourceSpan, TriviaAttachmentPolicy,
    detect_embedded_regions,
};
use serde_json::{Value, json};

#[path = "../../src/embedded_region_parser/diagnostics.rs"]
mod embedded_region_diagnostics;
#[path = "../../src/translation/embedded_region_rules.rs"]
mod rules;

use embedded_region_diagnostics::{EmbeddedRegionDiagnostic, insert_embedded_region_diagnostic};

fn recursive_default_region_fixture() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/recursive-default-embedded-regions.lino"
    ))
    .expect("the complete shared recursive default region fixture reads")
}

fn recursive_default_region_links(network: &LinkNetwork) -> Vec<&Link> {
    // Match embedded_regions(): the self-description's Region type point is
    // retained in the network, but only physical occurrences have both fields.
    let regions: Vec<_> = network
        .links()
        .filter(|link| {
            let metadata = link.metadata();
            metadata.link_type() == Some(LinkType::Region)
                && metadata.language().is_some()
                && metadata.span().is_some()
        })
        .collect();
    let definition = network
        .link(
            network
                .find_term("region")
                .expect("the Region type point remains"),
        )
        .expect("the retained Region type point resolves");
    assert_eq!(
        (
            definition.metadata().link_type(),
            definition.metadata().language(),
            definition.metadata().span()
        ),
        (Some(LinkType::Region), None, None)
    );
    assert!(regions.iter().all(|region| region.id() != definition.id()));
    regions
}

struct RecursiveConcreteProjection<'a> {
    network: &'a LinkNetwork,
    children: HashMap<LinkId, Vec<LinkId>>,
    fields: HashMap<LinkId, Vec<(String, LinkId)>>,
}

impl<'a> RecursiveConcreteProjection<'a> {
    fn new(network: &'a LinkNetwork) -> Self {
        let mut projection = Self {
            network,
            children: HashMap::new(),
            fields: HashMap::new(),
        };
        for link in network.links() {
            match (link.metadata().link_type(), link.references()) {
                (Some(LinkType::Syntax | LinkType::Token), [parent]) => {
                    projection
                        .children
                        .entry(*parent)
                        .or_default()
                        .push(link.id());
                }
                (Some(LinkType::Field), [parent, label, child]) => {
                    let label = network
                        .link(*label)
                        .unwrap()
                        .metadata()
                        .term()
                        .unwrap()
                        .to_owned();
                    projection
                        .fields
                        .entry(*parent)
                        .or_default()
                        .push((label, *child));
                }
                _ => {}
            }
        }
        projection
    }

    fn children(&self, parent: LinkId, origin: SourceSpan) -> Value {
        Value::Array(
            self.children
                .get(&parent)
                .into_iter()
                .flatten()
                .map(|child| self.node(*child, origin))
                .collect(),
        )
    }

    fn node(&self, id: LinkId, origin: SourceSpan) -> Value {
        let link = self
            .network
            .link(id)
            .expect("every retained CST reference resolves");
        let metadata = link.metadata();
        assert!(matches!(
            metadata.link_type(),
            Some(LinkType::Syntax | LinkType::Token)
        ));
        let span = metadata
            .span()
            .expect("a parsed CST node retains its source coordinates");
        let range = span.byte_range();
        let flags = metadata.flags();
        let children = self.children.get(&id).map_or(&[][..], Vec::as_slice);
        let fields: Vec<_> = self.fields.get(&id).into_iter().flatten().map(|(label, child)| {
            json!({ "label": label, "child": children.iter().position(|candidate| candidate == child)
                .expect("every field references an actual ordered child") })
        }).collect();
        json!({
            "type": metadata.link_type().unwrap().to_string(), "term": metadata.term(),
            "language": metadata.language(), "named": metadata.is_named(),
            "flags": [flags.is_error(), flags.has_error(), flags.is_missing(), flags.is_extra()],
            "span": { "start": range.start() - origin.byte_range().start(),
                "end": range.end() - origin.byte_range().start(),
                "startPoint": relative_region_point(span.start_point(), origin.start_point()),
                "endPoint": relative_region_point(span.end_point(), origin.start_point()) },
            "fields": fields, "children": self.children(id, origin),
        })
    }
}

fn relative_region_point(point: Point, origin: Point) -> Value {
    json!({ "row": point.row() - origin.row(), "column": point.column()
        - if point.row() == origin.row() { origin.column() } else { 0 } })
}

fn recursive_region_configuration(policy: &str) -> ParseConfiguration {
    ParseConfiguration::default().with_region_detection_policy(match policy {
        "NameDriven" => RegionDetectionPolicy::NameDriven,
        "ContentDriven" => RegionDetectionPolicy::ContentDriven,
        "Both" => RegionDetectionPolicy::Both,
        _ => panic!("the shared fixture names an existing region detection policy"),
    })
}

fn region_document(network: &LinkNetwork) -> &Link {
    network
        .links()
        .find(|link| link.metadata().link_type() == Some(LinkType::Document))
        .expect("the registered parser retains its document")
}

fn assert_recursive_default_regions(network: &LinkNetwork, fixture: &Value) {
    let source = fixture["source"].as_str().unwrap();
    let host = fixture["host"].as_str().unwrap();
    let regions = recursive_default_region_links(network);
    let expected = fixture["regions"].as_array().unwrap();
    assert_eq!(
        regions.len(),
        expected.len(),
        "FIFO retains every authored region exactly once"
    );
    let configuration = recursive_region_configuration(fixture["policy"].as_str().unwrap())
        .with_embedded_region_limits(0, 0, 0)
        .unwrap();
    let projection = RecursiveConcreteProjection::new(network);
    for (index, expected) in expected.iter().enumerate() {
        let region = regions[index];
        let metadata = region.metadata();
        let language = expected["language"].as_str().unwrap();
        let span = metadata.span().unwrap();
        let range = span.byte_range();
        assert_eq!(metadata.language(), Some(language));
        assert_eq!(
            u64::try_from(range.start()).unwrap(),
            expected["startByte"].as_u64().unwrap()
        );
        assert_eq!(
            u64::try_from(range.end()).unwrap(),
            expected["endByte"].as_u64().unwrap()
        );
        for (point, expected) in [
            (span.start_point(), &expected["startPoint"]),
            (span.end_point(), &expected["endPoint"]),
        ] {
            assert_eq!(
                json!({ "row": point.row(), "column": point.column() }),
                *expected
            );
        }
        let region_source = expected["source"].as_str().unwrap();
        assert_eq!(&source[range.start()..range.end()], region_source);
        let expected_parent = expected["parent"].as_i64().unwrap();
        let parent = if expected_parent < 0 {
            region_document(network).id()
        } else {
            regions[usize::try_from(expected_parent).unwrap()].id()
        };
        assert_eq!(
            region.references()[0],
            parent,
            "descendants name their containing Region"
        );
        let direct = LinkNetwork::parse(region_source, language, configuration);
        let document = region_document(&direct);
        let actual = projection.children(region.id(), span);
        assert!(
            !actual.as_array().unwrap().is_empty(),
            "accepted regions own their complete CST"
        );
        assert_eq!(
            actual,
            RecursiveConcreteProjection::new(&direct)
                .children(document.id(), document.metadata().span().unwrap()),
            "every child node, token, flag, ordered field and coordinate survives one absolute offset"
        );
    }
    let direct = LinkNetwork::parse(source, host, configuration);
    let document = region_document(network);
    let direct_document = region_document(&direct);
    assert_eq!(
        projection.children(document.id(), document.metadata().span().unwrap()),
        RecursiveConcreteProjection::new(&direct).children(
            direct_document.id(),
            direct_document.metadata().span().unwrap()
        ),
        "descendant Regions preserve the complete original host CST"
    );
    assert_eq!(network.reconstruct_text(), source);
    assert_eq!(network.render_source(host), source);
    assert!(network.verify_full_match(None).is_clean());
}

#[test]
fn default_recursive_csts_preserve_all_shared_sources_absolute_boundaries_and_reloaded_children() {
    let fixture = recursive_default_region_fixture();
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(fixture["id"], "recursive-default-embedded-regions");
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 4);
    for fixture in fixture["cases"].as_array().unwrap() {
        let source = fixture["source"].as_str().unwrap();
        let host = fixture["host"].as_str().unwrap();
        let policy = fixture["policy"].as_str().unwrap();
        let configuration = recursive_region_configuration(policy);
        let expected: Vec<_> = fixture["regions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|region| region["parent"] == -1)
            .map(|region| {
                (
                    region["language"].as_str().unwrap().to_owned(),
                    region["startByte"].as_u64().unwrap(),
                    region["endByte"].as_u64().unwrap(),
                )
            })
            .collect();
        let direct: Vec<_> =
            detect_embedded_regions(source, host, configuration.region_detection_policy())
                .into_iter()
                .map(|region| {
                    let range = region.span().byte_range();
                    (
                        region.language().to_owned(),
                        u64::try_from(range.start()).unwrap(),
                        u64::try_from(range.end()).unwrap(),
                    )
                })
                .collect();
        assert_eq!(
            direct, expected,
            "the original direct detector is unchanged"
        );
        let network = LinkNetwork::parse(source, host, configuration);
        assert_recursive_default_regions(&network, fixture);
        let text = network.to_lino();
        let restored = LinkNetwork::from_lino(&text).unwrap();
        assert_eq!(restored.to_lino(), text);
        assert_recursive_default_regions(&restored, fixture);
    }
}

fn embedded_diagnostic_fields(
    network: &LinkNetwork,
    diagnostic: LinkId,
) -> BTreeMap<String, String> {
    network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
        .filter_map(|link| {
            let [parent, label, child] = link.references() else {
                return None;
            };
            (*parent == diagnostic).then(|| {
                (
                    network
                        .link(*label)
                        .unwrap()
                        .metadata()
                        .term()
                        .unwrap()
                        .to_owned(),
                    network
                        .link(*child)
                        .unwrap()
                        .metadata()
                        .term()
                        .unwrap()
                        .to_owned(),
                )
            })
        })
        .collect()
}

#[test]
fn default_recursive_limits_preserve_refused_boundaries_counters_and_source_after_reload() {
    let child = "const value = \"π\";";
    let html = format!("<script>{child}</script>\n");
    let source = format!("```html\n{html}```\n");
    for (depth, count, bytes, reason, observed, limit) in [
        (1, 4096, 16_777_216, "embeddedRegionDepthLimit", 2, 1),
        (128, 1, 16_777_216, "embeddedRegionCountLimit", 2, 1),
        (
            128,
            4096,
            html.len(),
            "embeddedRegionByteLimit",
            html.len() + child.len(),
            html.len(),
        ),
    ] {
        let configuration = ParseConfiguration::default()
            .with_embedded_region_limits(depth, count, bytes)
            .unwrap();
        let network = LinkNetwork::parse(&source, "Markdown", configuration);
        let restored = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        for current in [&network, &restored] {
            let regions = recursive_default_region_links(current);
            assert_eq!(
                regions
                    .iter()
                    .map(|region| region.metadata().language().unwrap())
                    .collect::<Vec<_>>(),
                ["HTML", "JavaScript"]
            );
            let projection = RecursiveConcreteProjection::new(current);
            assert!(projection.children.contains_key(&regions[0].id()));
            assert!(!projection.children.contains_key(&regions[1].id()));
            assert_eq!(regions[1].references()[0], regions[0].id());
            let diagnostics: Vec<_> = current
                .links()
                .filter(|link| link.metadata().term() == Some("embedded region diagnostic"))
                .collect();
            assert_eq!(diagnostics.len(), 1);
            let diagnostic = diagnostics[0];
            assert_eq!(diagnostic.metadata().definition(), Some(reason));
            assert_eq!(diagnostic.references(), [regions[1].id()]);
            assert_eq!(
                embedded_diagnostic_fields(current, diagnostic.id()),
                BTreeMap::from([
                    ("depth".to_owned(), "2".to_owned()),
                    ("region count".to_owned(), "1".to_owned()),
                    ("child parse bytes".to_owned(), html.len().to_string()),
                    ("observed".to_owned(), observed.to_string()),
                    ("limit".to_owned(), limit.to_string()),
                ])
            );
            assert!(!current.verify_full_match(None).is_clean());
            assert_eq!(current.reconstruct_text(), source);
            assert_eq!(current.render_source("Markdown"), source);
        }
    }
    let admitted = LinkNetwork::parse(
        &source,
        "Markdown",
        ParseConfiguration::default()
            .with_embedded_region_limits(2, 2, html.len() + child.len())
            .unwrap(),
    );
    assert!(admitted.verify_full_match(None).is_clean());
    assert!(
        RecursiveConcreteProjection::new(&admitted)
            .children
            .contains_key(&recursive_default_region_links(&admitted)[1].id())
    );
    let root_only = LinkNetwork::parse(
        "plain café",
        "txt",
        ParseConfiguration::default()
            .with_embedded_region_limits(0, 0, 0)
            .unwrap(),
    );
    assert_eq!(recursive_default_region_links(&root_only).len(), 1);
    assert!(root_only.verify_full_match(None).is_clean());
    assert_eq!(root_only.reconstruct_text(), "plain café");
}

#[test]
fn recursive_default_siblings_share_counters_and_preserve_original_direct_link_id_order() {
    let child = "const nested = 1;";
    let sibling = "const sibling = 2;\n";
    let source = format!("```html\n<script>{child}</script>\n```\n\n```JavaScript\n{sibling}```\n");
    let configuration = ParseConfiguration::default()
        .with_embedded_region_limits(128, 2, 16_777_216)
        .unwrap();
    let network = LinkNetwork::parse(&source, "Markdown", configuration);
    let regions = recursive_default_region_links(&network);
    assert_eq!(
        regions
            .iter()
            .map(|region| region.metadata().language().unwrap())
            .collect::<Vec<_>>(),
        ["HTML", "JavaScript", "JavaScript"]
    );
    let projection = RecursiveConcreteProjection::new(&network);
    assert!(projection.children.contains_key(&regions[0].id()));
    assert!(projection.children.contains_key(&regions[1].id()));
    assert!(!projection.children.contains_key(&regions[2].id()));
    let diagnostic = network
        .links()
        .find(|link| link.metadata().definition() == Some("embeddedRegionCountLimit"))
        .unwrap();
    assert_eq!(
        embedded_diagnostic_fields(&network, diagnostic.id())["observed"],
        "3"
    );
    let direct_only = LinkNetwork::parse(
        &source,
        "Markdown",
        ParseConfiguration::default()
            .with_embedded_region_limits(1, 4096, 16_777_216)
            .unwrap(),
    );
    let direct_regions = recursive_default_region_links(&direct_only);
    assert_eq!(regions[0].id(), direct_regions[0].id());
    assert_eq!(regions[1].id(), direct_regions[1].id());
    let descendant_id = regions[2].id();
    assert_eq!(
        network
            .links()
            .take_while(|link| link.id() < descendant_id)
            .collect::<Vec<_>>(),
        direct_only
            .links()
            .take_while(|link| link.id() < descendant_id)
            .collect::<Vec<_>>(),
        "every original direct link precedes descendants with its original identity and value"
    );
    assert_eq!(network.render_source("Markdown"), source);
    assert_eq!(network.render_source("JavaScript"), sibling);
    assert_eq!(network.reconstruct_text(), source);
    let mut constructed = LinkNetwork::new();
    let _ = constructed.insert_source_token("fragment", "first");
    let _ = constructed.insert_source_token("fragment", "second");
    assert_eq!(constructed.render_source("fragment"), "firstsecond");
}

#[test]
fn parsed_source_rendering_keeps_unspanned_fragments_and_constructed_reference_order() {
    let source = "# Root\n\n```markdown\n# Inner\n```\n";
    let mut network = LinkNetwork::parse(source, "Markdown", ParseConfiguration::default());
    assert_eq!(
        network.render_source("Markdown"),
        source,
        "same-language descendants cover physical source once"
    );
    let _ = network.insert_source_token("Markdown", "tail");
    assert_eq!(network.render_source("Markdown"), format!("{source}tail"));
    assert_eq!(
        network.render_source_from_document("Markdown"),
        Some(source.to_owned()),
        "the document-owned rendering API retains its original ownership contract"
    );
    let mut constructed = LinkNetwork::new();
    let first = constructed.insert_source_token("fragment", "first");
    let second = constructed.insert_source_token("fragment", "second");
    let root = constructed.insert_syntax_node("fragment", "root", [second, first]);
    assert_eq!(constructed.render_source("fragment"), "secondfirst");
    assert_eq!(
        constructed.render_source_from(root, "fragment"),
        "secondfirst"
    );
    for entries in [
        [("tail", None), ("B", Some(1)), ("A", Some(0))],
        [("B", Some(1)), ("tail", None), ("A", Some(0))],
    ] {
        let mut mixed = LinkNetwork::new();
        for (term, start) in entries {
            let token = mixed.insert_source_token("fragment", term);
            if let Some(start) = start {
                assert!(mixed.set_span(
                    token,
                    SourceSpan::new(
                        ByteRange::new(start, start + 1),
                        Point::new(0, start),
                        Point::new(0, start + 1)
                    )
                ));
            }
        }
        assert_eq!(
            mixed.render_source("fragment"),
            "ABtail",
            "physical source precedes unspanned fragments independently of their IDs"
        );
        let restored = LinkNetwork::from_lino(&mixed.to_lino()).unwrap();
        assert_eq!(restored.render_source("fragment"), "ABtail");
    }
}

#[test]
fn recursive_default_configuration_preserves_const_defaults_and_refuses_unsafe_native_limits() {
    const DEFAULTS: ParseConfiguration = ParseConfiguration::new(TriviaAttachmentPolicy::Both);
    assert_eq!(
        (
            DEFAULTS.embedded_region_depth_limit(),
            DEFAULTS.embedded_region_count_limit(),
            DEFAULTS.embedded_region_byte_limit()
        ),
        (128, 4096, 16_777_216)
    );
    let limited = ParseConfiguration::new(TriviaAttachmentPolicy::TokenLink)
        .with_embedded_region_limits(3, 4, 20)
        .unwrap()
        .with_region_detection_policy(RegionDetectionPolicy::NameDriven)
        .with_access_mode(AccessMode::ReadOnly);
    assert_eq!(
        limited.trivia_attachment_policy(),
        TriviaAttachmentPolicy::TokenLink
    );
    assert_eq!(
        (
            limited.embedded_region_depth_limit(),
            limited.embedded_region_count_limit(),
            limited.embedded_region_byte_limit()
        ),
        (3, 4, 20)
    );
    assert_eq!(DEFAULTS.embedded_region_depth_limit(), 128);
    assert!(limited.with_embedded_region_limits(0, 0, 0).is_ok());
    if let Ok(maximum) = usize::try_from(9_007_199_254_740_991_u64) {
        assert!(
            DEFAULTS
                .with_embedded_region_limits(maximum, maximum, maximum)
                .is_ok()
        );
    }
    for invalid in [9_007_199_254_740_992_u64, u64::MAX] {
        if let Ok(invalid) = usize::try_from(invalid) {
            for limits in [[invalid, 0, 0], [0, invalid, 0], [0, 0, invalid]] {
                assert_eq!(
                    DEFAULTS.with_embedded_region_limits(limits[0], limits[1], limits[2]),
                    Err("embedded region limits must be nonnegative safe integers")
                );
            }
        }
    }
}

#[test]
fn recursive_default_root_aliases_retain_one_boundary_and_the_existing_direct_cst_views() {
    let source = "plain café\n";
    for language in ["txt", "TXT", "text", "TeXt", "plain text", "PLAIN TEXT"] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        let regions = recursive_default_region_links(&network);
        assert_eq!(regions.len(), 1);
        let region = regions[0];
        assert_eq!(region.metadata().language(), Some("txt"));
        assert_eq!(region.references()[0], region_document(&network).id());
        assert_eq!(
            region.metadata().span(),
            region_document(&network).metadata().span()
        );
        let projection = RecursiveConcreteProjection::new(&network);
        assert_eq!(
            projection.children.contains_key(&region.id()),
            !language.eq_ignore_ascii_case("txt")
        );
        let baseline = LinkNetwork::parse(
            source,
            language,
            ParseConfiguration::default()
                .with_embedded_region_limits(0, 0, 0)
                .unwrap(),
        );
        let baseline_document = region_document(&baseline);
        let document = region_document(&network);
        assert_eq!(
            projection.children(document.id(), document.metadata().span().unwrap()),
            RecursiveConcreteProjection::new(&baseline).children(
                baseline_document.id(),
                baseline_document.metadata().span().unwrap()
            )
        );
        assert_eq!(network.reconstruct_text(), source);
        assert_eq!(network.render_source(language), source);
        assert!(network.verify_full_match(None).is_clean());
        assert_eq!(
            LinkNetwork::from_lino(&network.to_lino())
                .unwrap()
                .to_lino(),
            network.to_lino()
        );
    }
}

#[test]
fn synthetic_embedding_cycles_keep_the_actual_ancestor_and_error_after_reload() {
    let mut network = LinkNetwork::new();
    let span = SourceSpan::new(ByteRange::new(0, 2), Point::new(0, 0), Point::new(0, 2));
    let outer = network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Region)
            .with_language("Markdown")
            .with_span(span),
    );
    let inner = network.insert_link(
        [outer],
        LinkMetadata::new()
            .with_link_type(LinkType::Region)
            .with_language("HTML")
            .with_span(span),
    );
    let rejected = network.insert_link(
        [inner],
        LinkMetadata::new()
            .with_link_type(LinkType::Region)
            .with_language("Markdown")
            .with_span(span),
    );
    let ancestor = rules::read_embedded_region_ancestor(
        &[-1.0, 0.0],
        &[0.0, 1.0],
        &[0.0, 0.0],
        &[2.0, 2.0],
        1.0,
        0.0,
        0.0,
        2.0,
    );
    assert_eq!(rules::ml_array::number_index(ancestor), Some(0));
    let reason = rules::read_embedded_region_disposition(
        0.0,
        2.0,
        1.0,
        0.0,
        0.0,
        2.0,
        ancestor,
        2.0,
        1.0,
        2.0,
        128.0,
        4096.0,
        16_777_216.0,
        true,
    );
    assert_eq!(reason, "nonConsumingEmbeddingCycle");
    let diagnostic = insert_embedded_region_diagnostic(
        &mut network,
        rejected,
        "Markdown",
        span,
        &EmbeddedRegionDiagnostic {
            reason: &reason,
            depth: 2.0,
            count: 1.0,
            bytes: 2.0,
            observed: -1.0,
            limit: -1.0,
            ancestor: Some(outer),
        },
    );
    network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_term("π")
            .with_language("Markdown")
            .with_span(span),
    );
    let restored = LinkNetwork::from_lino(&network.to_lino()).unwrap();
    for current in [&network, &restored] {
        assert!(!current.verify_full_match(None).is_clean());
        assert_eq!(
            current.link(diagnostic).unwrap().metadata().definition(),
            Some(reason.as_str())
        );
        let mut fields = current
            .links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
            .filter(|link| link.references().first() == Some(&diagnostic));
        assert!(fields.any(|field| {
            let references = field.references();
            current.link(references[1]).unwrap().metadata().term() == Some("ancestor")
                && references[2] == outer
        }));
        assert!(
            !current
                .links()
                .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
                .filter(|link| link.references().first() == Some(&diagnostic))
                .any(|field| current
                    .link(field.references()[1])
                    .unwrap()
                    .metadata()
                    .term()
                    == Some("observed"))
        );
        assert_eq!(current.reconstruct_text(), "π");
    }
}

#[test]
fn generated_region_policy_keeps_malformed_ancestry_zero_width_and_current_host_reuse_distinct() {
    assert!(rules::accept_embedded_region_bounds(10.0, 20.0, 20.0, 20.0));
    assert!(!rules::accept_embedded_region_bounds(10.0, 20.0, 9.0, 20.0));
    assert!(!rules::accept_embedded_region_limit(f64::NAN));
    assert!(!rules::accept_embedded_region_limit(f64::INFINITY));
    assert!(!rules::accept_embedded_region_limit(-1.0));
    assert!(!rules::accept_embedded_region_limit(0.5));
    let invalid =
        rules::read_embedded_region_ancestor(&[0.0], &[0.0], &[0.0], &[2.0], 0.0, 0.0, 0.0, 2.0);
    assert_eq!(invalid.to_bits(), (-2.0_f64).to_bits());
    assert_eq!(
        rules::read_embedded_region_disposition(
            0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, true
        ),
        "reuseAttached"
    );
    assert_eq!(
        rules::read_embedded_region_disposition(
            0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, true
        ),
        "reuse"
    );
    assert_eq!(
        rules::read_embedded_region_disposition(
            0.0,
            2.0,
            0.0,
            0.0,
            0.0,
            2.0,
            0.0,
            1.0,
            0.0,
            0.0,
            128.0,
            4096.0,
            16_777_216.0,
            false
        ),
        "parse"
    );
}
