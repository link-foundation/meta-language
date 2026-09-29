//! The Links Notation compatibility matrix of `docs/vision.md#links-notation-fitness`
//! (`parity/fixtures/lino-compatibility-matrix.json`), the counterpart of
//! `js/tests/lino-compatibility-matrix.test.js`: both packages use the audited
//! links-notation release, and every feature is decoded by the official parser
//! and by meta-language's own reading alike, reconstructed exactly, edited and
//! encoded back into text both read the same way.

use std::fs;
use std::path::PathBuf;

use links_notation::{parse_lino_to_links, LiNo};
use meta_language::{ByteRange, LinkId, LinkNetwork, LinkType, ParseConfiguration, SourceSpan};
use serde_json::{json, Value};

use super::issue_195_observations::{record, Observation, LINO_MATRIX_FIXTURE};

fn repository_file(relative: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(relative);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()))
}

fn matrix() -> Value {
    serde_json::from_str(&repository_file(LINO_MATRIX_FIXTURE)).expect("matrix JSON")
}

fn observe(requirement_id: &str, assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id,
        suffix: "behavior",
        fixture_id: &format!(
            "planned:repository-directive:{}",
            requirement_id.to_ascii_lowercase()
        ),
        fixture_file: LINO_MATRIX_FIXTURE,
        assertions,
        test_name,
    });
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} is a string"))
}

/// The official reading: a reference is its id and a link is `[id, ...values]`.
fn canonical(link: &LiNo<String>) -> Value {
    match link {
        LiNo::Ref(id) => json!(id),
        LiNo::Link { id, values } if values.is_empty() && id.is_some() => json!(id),
        LiNo::Link { id, values } => {
            let mut items = vec![json!(id)];
            items.extend(values.iter().map(canonical));
            Value::Array(items)
        }
    }
}

fn official_reading(source: &str) -> Value {
    let links = parse_lino_to_links(source)
        .unwrap_or_else(|error| panic!("the official parser reads {source:?}: {error:?}"));
    Value::Array(links.iter().map(canonical).collect())
}

fn own_reading(source: &str) -> (LinkNetwork, Vec<LinkId>, Value) {
    let (network, links) = LinkNetwork::parse_links_notation(source, ParseConfiguration::default());
    let readings = links
        .iter()
        .map(|&id| {
            serde_json::to_value(network.links_notation_reading(id).expect("a LiNo link"))
                .expect("reading JSON")
        })
        .collect();
    (network, links, Value::Array(readings))
}

fn has_error_node(network: &LinkNetwork) -> bool {
    network.links().any(|link| {
        let metadata = link.metadata();
        metadata.term() == Some("ERROR") && metadata.flags().is_error()
    })
}

#[test]
fn both_packages_depend_on_the_audited_links_notation_release() {
    let matrix = matrix();
    let crates_io = &matrix["releases"]["cratesIo"];
    let (package, version) = (text(crates_io, "package"), text(crates_io, "version"));
    assert_eq!(package, "links-notation");
    let manifest = repository_file(text(crates_io, "manifest"));
    let prefix = format!("{package} = {{ version = \"");
    let declared: Vec<&str> = manifest
        .lines()
        .filter_map(|line| line.strip_prefix(&prefix))
        .map(|rest| rest.split('"').next().expect("a version"))
        .collect();
    assert!(!declared.is_empty(), "Cargo.toml declares {package}");
    assert!(
        declared.iter().all(|declared| *declared == version),
        "{declared:?}"
    );
    let npm = &matrix["releases"]["npm"];
    let package_json: Value =
        serde_json::from_str(&repository_file(text(npm, "manifest"))).expect("package.json");
    assert_eq!(
        package_json["dependencies"][package],
        json!(text(npm, "version"))
    );
    assert_eq!(
        text(npm, "version"),
        version,
        "both runtimes use the same release"
    );
    observe(
        "I195-LINO-UPGRADE",
        &["declaredReleaseMatchesMatrix"],
        "both_packages_depend_on_the_audited_links_notation_release",
    );
}

#[test]
fn the_locked_links_notation_crate_is_the_audited_release_and_the_matrix_is_its_reading() {
    let matrix = matrix();
    let crates_io = &matrix["releases"]["cratesIo"];
    let (package, version) = (text(crates_io, "package"), text(crates_io, "version"));
    // A Windows checkout may write the lockfile with CRLF line endings.
    let lockfile = repository_file(text(crates_io, "lockfile")).replace("\r\n", "\n");
    assert!(
        lockfile.contains(&format!("name = \"{package}\"\nversion = \"{version}\"\n")),
        "Cargo.lock resolves {package} {version}"
    );
    // The version-coupled fixtures name the release that generated them, and
    // the crate reads every feature of the matrix as the matrix records.
    // The grammar cases nest deeper than serde_json reads, so their oracle
    // line is read as text.
    let cases = repository_file("parity/fixtures/lino-grammar-cases.json");
    assert!(
        cases
            .lines()
            .any(|line| line.starts_with(&format!(" \"oracle\": \"{package}@{version} "))),
        "lino-grammar-cases.json was generated by {package} {version}"
    );
    let expected: Value = serde_json::from_str(&repository_file(
        "parity/fixtures/builtin-cst-expected.json",
    ))
    .expect("CST expectations");
    assert_eq!(expected["generator"]["linksNotation"], json!(version));
    for feature in matrix["features"].as_array().expect("features") {
        assert_eq!(
            official_reading(text(feature, "source")),
            feature["links"],
            "{}: the matrix is the release's reading",
            text(feature, "id")
        );
    }
    observe(
        "I195-LINO-UPGRADE",
        &[
            "installedReleaseMatchesMatrix",
            "fixturesRegeneratedAgainstRelease",
        ],
        "the_locked_links_notation_crate_is_the_audited_release_and_the_matrix_is_its_reading",
    );
}

fn check_feature(feature: &Value) {
    let id = text(feature, "id");
    let source = text(feature, "source");
    let links = &feature["links"];

    // Decoded: the official parser and the native reading agree with the matrix.
    assert_eq!(&official_reading(source), links, "{id}: official reading");
    let (mut network, ids, readings) = own_reading(source);
    assert!(
        !has_error_node(&network),
        "{id}: the native CST has no ERROR node"
    );
    assert_eq!(&readings, links, "{id}: native reading");

    // Reconstructed: the network gives back the source byte for byte.
    assert_eq!(network.reconstruct_text(), source, "{id}: reconstruction");

    // Encoded: the links written back as text read as the same links.
    let encoded = network.links_notation_text(&ids);
    assert_eq!(
        &official_reading(&encoded),
        links,
        "{id}: official reading of {encoded:?}"
    );
    assert_eq!(
        &own_reading(&encoded).2,
        links,
        "{id}: native reading of {encoded:?}"
    );

    // Edited: the token's source range is edited through the network and the
    // edited text is read the way the official parser reads it.
    let (replace, with) = (
        text(&feature["edit"], "replace"),
        text(&feature["edit"], "with"),
    );
    let range = network
        .links()
        .filter(|link| {
            let metadata = link.metadata();
            metadata.link_type() == Some(LinkType::Token) && metadata.term() == Some(replace)
        })
        .filter_map(|link| link.metadata().span())
        .map(SourceSpan::byte_range)
        .min_by_key(|range| range.start())
        .unwrap_or_else(|| panic!("{id} has a {replace} token"));
    assert!(network.apply_edit(ByteRange::new(range.start(), range.end()), with));
    let edited = format!(
        "{}{with}{}",
        &source[..range.start()],
        &source[range.end()..]
    );
    assert_eq!(network.reconstruct_text(), edited, "{id}: edited text");
    let (reread, _, reread_readings) = own_reading(&edited);
    assert_eq!(reread.reconstruct_text(), edited);
    assert_eq!(
        reread_readings,
        official_reading(&edited),
        "{id}: edited reading"
    );
    assert_ne!(&reread_readings, links, "{id}: the edit changes the links");
}

#[test]
fn every_feature_of_the_compatibility_matrix_is_decoded_reconstructed_edited_and_encoded() {
    let matrix = matrix();
    let features = matrix["features"].as_array().expect("features");
    let ids: Vec<&str> = features.iter().map(|feature| text(feature, "id")).collect();
    assert_eq!(
        ids,
        [
            "named-links",
            "anonymous-links",
            "arbitrary-arity",
            "shared-references",
            "recursive-references",
            "forward-references",
            "identity",
            "ordering",
            "indentation",
            "nested-multiline-groups",
            "quoting",
            "escaping",
            "comments-and-trivia",
            "unicode",
            "source-mappings",
        ]
    );
    for feature in features {
        check_feature(feature);
    }
    observe(
        "I195-LINO-COMPATIBILITY-MATRIX",
        &[
            "everyFeatureDecoded",
            "everyFeatureReconstructedExactly",
            "everyFeatureEditable",
            "everyFeatureEncoded",
        ],
        "every_feature_of_the_compatibility_matrix_is_decoded_reconstructed_edited_and_encoded",
    );
}
