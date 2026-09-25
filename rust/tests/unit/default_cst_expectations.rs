use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use meta_language::{
    grammar_provenance, language_candidates_for_path, language_for_path, LinkId, LinkNetwork,
    LinkType, ParseConfiguration,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn parity_json(name: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity")
        .join(name);
    serde_json::from_str(&fs::read_to_string(path).expect("parity file is readable"))
        .expect("parity file is valid JSON")
}

/// Grammar rows of every inventory language: tree-sitter languages from
/// `default-cst-expected.json`, built-in grammar languages from
/// `builtin-cst-expected.json`.
fn expected_languages() -> serde_json::Map<String, Value> {
    let mut languages = parity_json("fixtures/default-cst-expected.json")["languages"]
        .as_object()
        .expect("default CST languages")
        .clone();
    languages.extend(
        parity_json("fixtures/builtin-cst-expected.json")["languages"]
            .as_object()
            .expect("built-in CST languages")
            .clone(),
    );
    languages
}

/// Projects the Syntax links of a public network to the grammar rows of
/// `parity/fixtures/default-cst-expected.json`, walking down from `root`, with
/// byte offsets relative to `base`.
fn syntax_rows(network: &LinkNetwork, root: LinkId, base: usize) -> Vec<Value> {
    let mut children: HashMap<LinkId, Vec<LinkId>> = HashMap::new();
    let mut fields: HashMap<(LinkId, LinkId), String> = HashMap::new();
    for link in network.links() {
        let metadata = link.metadata();
        match metadata.link_type() {
            Some(LinkType::Syntax) => {
                if let [parent] = link.references() {
                    children.entry(*parent).or_default().push(link.id());
                }
            }
            Some(LinkType::Field) if metadata.term().is_none() => {
                if let [parent, label, child] = link.references() {
                    let label = network
                        .link(*label)
                        .and_then(|label| label.metadata().term())
                        .expect("field label term");
                    fields.insert((*parent, *child), label.to_string());
                }
            }
            _ => {}
        }
    }
    let mut rows = Vec::new();
    let mut stack = vec![(root, 0_usize, None::<String>)];
    while let Some((id, depth, field)) = stack.pop() {
        let metadata = network.link(id).expect("syntax link").metadata();
        let flags = metadata.flags();
        let range = metadata.span().map(meta_language::SourceSpan::byte_range);
        rows.push(json!([
            depth,
            field,
            metadata.term(),
            i32::from(metadata.is_named()),
            range.map(|range| range.start() - base),
            range.map(|range| range.end() - base),
            format!(
                "{}{}{}",
                if flags.is_error() { "E" } else { "" },
                if flags.is_missing() { "M" } else { "" },
                if flags.is_extra() { "X" } else { "" },
            ),
        ]));
        for child in children.get(&id).into_iter().flatten().rev() {
            stack.push((*child, depth + 1, fields.get(&(id, *child)).cloned()));
        }
    }
    rows
}

fn document_root(network: &LinkNetwork) -> LinkId {
    let document = network
        .links()
        .find(|link| link.metadata().link_type() == Some(LinkType::Document))
        .expect("document link")
        .id();
    network
        .links()
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax) && link.references() == [document]
        })
        .expect("grammar root below the document")
        .id()
}

/// Every embedded region as `(language, start, end, grammar root)`, in source
/// order. A region of the document's own language over the whole document
/// (the natural-language annotation of a text) embeds nothing.
fn embedded_roots(
    network: &LinkNetwork,
    language: &str,
    source: &str,
) -> Vec<(String, usize, usize, LinkId)> {
    network
        .links()
        // Embedded regions reference the document and their Language link;
        // the self-description `region` type point has no span.
        .filter(|link| {
            link.metadata().link_type() == Some(LinkType::Region) && link.references().len() == 2
        })
        .filter(|region| {
            let metadata = region.metadata();
            let range = metadata.span().map(meta_language::SourceSpan::byte_range);
            !(metadata.language() == Some(language)
                && range.is_some_and(|range| range.start() == 0 && range.end() == source.len()))
        })
        .map(|region| {
            let range = region.metadata().span().expect("region span").byte_range();
            let root = network
                .links()
                .find(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.references() == [region.id()]
                })
                .expect("grammar root below the region")
                .id();
            (
                region
                    .metadata()
                    .language()
                    .expect("region language")
                    .to_string(),
                range.start(),
                range.end(),
                root,
            )
        })
        .collect()
}

/// Applies the documented public projections of the grammar rows: Lean's
/// public root `file` wraps the grammar `module`, and every Rocq `ident`
/// exposes its text as an `identifier` or `primitive_type` token child.
fn public_rows(language: &str, source: &str, rows: &[Value]) -> Vec<Value> {
    match language {
        "Lean" => {
            let flags = rows.first().map_or_else(|| json!(""), |row| row[6].clone());
            std::iter::once(json!([0, null, "file", 1, 0, source.len(), flags]))
                .chain(rows.iter().map(|row| {
                    let mut row = row.clone();
                    row[0] = json!(row[0].as_u64().expect("depth") + 1);
                    row
                }))
                .collect()
        }
        "Rocq" => rows
            .iter()
            .flat_map(|row| {
                let mut projected = vec![row.clone()];
                if row[2] == "ident" {
                    let start = usize::try_from(row[4].as_u64().expect("start")).expect("usize");
                    let end = usize::try_from(row[5].as_u64().expect("end")).expect("usize");
                    let term = match &source[start..end] {
                        "bool" | "nat" | "Prop" | "Set" | "SProp" | "Type" | "Z" => {
                            "primitive_type"
                        }
                        _ => "identifier",
                    };
                    let depth = row[0].as_u64().expect("depth") + 1;
                    projected.push(json!([depth, null, term, 1, start, end, ""]));
                }
                projected
            })
            .collect(),
        _ => rows.to_vec(),
    }
}

#[test]
fn rust_public_networks_match_grammar_derived_rows() {
    let inventory = parity_json("language-grammar-inventory.json");
    let expected = expected_languages();
    let mut differences = Vec::new();
    for language in inventory["languages"].as_array().expect("languages") {
        let name = language["name"].as_str().expect("name");
        let Some(want) = expected.get(name) else {
            continue;
        };
        for (kind, key) in [("positive", "source"), ("recovery", "recoverySource")] {
            let source = language[key].as_str().expect("source");
            let network = LinkNetwork::parse(source, name, ParseConfiguration::default());
            let rows = syntax_rows(&network, document_root(&network), 0);
            let want = public_rows(name, source, want[kind].as_array().expect("rows"));
            if rows != want {
                let index = rows
                    .iter()
                    .zip(&want)
                    .position(|(left, right)| left != right)
                    .unwrap_or_else(|| rows.len().min(want.len()));
                differences.push(format!(
                    "{name} {kind} row {index} ({} vs {}): actual {:?} expected {:?}",
                    rows.len(),
                    want.len(),
                    rows.get(index..(index + 2).min(rows.len())),
                    want.get(index..(index + 2).min(want.len())),
                ));
            }
        }
    }
    assert!(differences.is_empty(), "{}", differences.join("\n"));
}

#[test]
fn rust_embedded_regions_match_grammar_derived_rows() {
    let inventory = parity_json("language-grammar-inventory.json");
    let expected = expected_languages();
    let mut differences = Vec::new();
    for language in inventory["languages"].as_array().expect("languages") {
        let name = language["name"].as_str().expect("name");
        let Some(want) = expected.get(name) else {
            continue;
        };
        let source = language["source"].as_str().expect("source");
        let network = LinkNetwork::parse(source, name, ParseConfiguration::default());
        let regions = embedded_roots(&network, name, source);
        let want = want["embedded"].as_array().expect("embedded");
        assert_eq!(
            regions
                .iter()
                .map(|(language, start, end, _)| json!([language, start, end]))
                .collect::<Vec<_>>(),
            want.iter()
                .map(|region| json!([region["language"], region["startByte"], region["endByte"]]))
                .collect::<Vec<_>>(),
            "{name}"
        );
        for ((_, start, _, root), region) in regions.iter().zip(want) {
            let rows = syntax_rows(&network, *root, *start);
            if rows.as_slice() != region["rows"].as_array().expect("rows").as_slice() {
                differences.push(format!("{}: actual {rows:?}", region["path"]));
            }
        }
    }
    assert!(differences.is_empty(), "{}", differences.join("\n"));
}

const CST_POSITIVE_ASSERTIONS: [&str; 15] = [
    "ordinaryPublicParseApi",
    "realGrammarNodes",
    "independentExpectedStructure",
    "grammarVersionRecorded",
    "hierarchy",
    "namedFields",
    "childOrder",
    "tokens",
    "commentsAndTrivia",
    "exactUtf8Spans",
    "errorAndMissingNodes",
    "embeddedLanguageBoundaries",
    "exactReconstruction",
    "allAliases",
    "extensionDispatch",
];

const POSITIVE_TEST_NAME: &str =
    "every_rust_inventory_language_parses_to_its_complete_lossless_default_cst";

fn record_positive_cst_observations(language: &str, fixture_digest: &str) {
    let Some(path) = std::env::var_os("ISSUE_195_OBSERVATION_FILE") else {
        return;
    };
    let normalized = language
        .to_ascii_lowercase()
        .replace('+', "-plus")
        .replace('#', "-sharp");
    let slug = regex::Regex::new("[^a-z0-9]+")
        .expect("valid slug expression")
        .replace_all(&normalized, "-");
    let commit = std::env::var("ISSUE_195_COMMIT").expect("observation commit");
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("observation file opens");
    for assertion_id in CST_POSITIVE_ASSERTIONS {
        let record = json!({
            "testId": format!("i195-cst-{}-rust-positive", slug.trim_matches('-')),
            "assertionId": assertion_id,
            "fixtureId": format!("inventory:{language}"),
            "fixtureDigest": fixture_digest,
            "runtime": "rust",
            "commit": commit,
            "outcome": "passed",
            "testName": POSITIVE_TEST_NAME,
        });
        writeln!(file, "{record}").expect("observation record writes");
    }
}

/// The first differing row of `rows` against `want`, if any.
fn first_difference(rows: &[Value], want: &[Value]) -> Option<String> {
    if rows == want {
        return None;
    }
    let index = rows
        .iter()
        .zip(want)
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| rows.len().min(want.len()));
    Some(format!(
        "row {index} ({} vs {}): actual {:?} expected {:?}",
        rows.len(),
        want.len(),
        rows.get(index..(index + 2).min(rows.len())),
        want.get(index..(index + 2).min(want.len())),
    ))
}

fn rows_of(value: &Value) -> &[Value] {
    value.as_array().expect("rows")
}

fn has_flag(row: &Value, flags: &str) -> bool {
    row[6]
        .as_str()
        .is_some_and(|row_flags| row_flags.chars().any(|flag| flags.contains(flag)))
}

/// Checks the complete CST of one inventory language, returning the failed
/// checks.
#[allow(clippy::too_many_lines)]
fn positive_cst_problems(inventory: &Value, language: &Value, want: Option<&Value>) -> Vec<String> {
    let mut problems = Vec::new();
    let mut check = |condition: bool, message: String| {
        if !condition {
            problems.push(message);
        }
    };
    let name = language["name"].as_str().expect("name");
    let source = language["source"].as_str().expect("source");
    let recovery_source = language["recoverySource"]
        .as_str()
        .expect("recovery source");
    // ordinaryPublicParseApi, exactReconstruction
    let network = LinkNetwork::parse(source, name, ParseConfiguration::default());
    let recovery = LinkNetwork::parse(recovery_source, name, ParseConfiguration::default());
    check(
        network.reconstruct_text() == source,
        "positive reconstruction".into(),
    );
    check(
        recovery.reconstruct_text() == recovery_source,
        "recovery reconstruction".into(),
    );
    // independentExpectedStructure, realGrammarNodes, hierarchy, namedFields
    check(want.is_some(), "independent expectation".into());
    let Some(want) = want else {
        return problems;
    };
    let root = document_root(&network);
    let rows = syntax_rows(&network, root, 0);
    let want_rows = public_rows(name, source, rows_of(&want["positive"]));
    if let Some(difference) = first_difference(&rows, &want_rows) {
        check(false, format!("positive rows {difference}"));
    }
    check(
        rows.len() > 1
            && rows
                .windows(2)
                .all(|pair| pair[1][0].as_u64() <= pair[0][0].as_u64().map(|depth| depth + 1)),
        "hierarchy".into(),
    );
    check(
        rows.iter().filter(|row| !row[1].is_null()).count()
            == want_rows.iter().filter(|row| !row[1].is_null()).count(),
        "fields".into(),
    );
    // grammarVersionRecorded
    let recorded: Vec<_> = network
        .parse_grammars()
        .into_iter()
        .filter(|(language, _)| language == name)
        .map(|(_, grammar)| grammar)
        .collect();
    check(
        !recorded.is_empty() && recorded.as_slice() == grammar_provenance(name),
        "grammar provenance".into(),
    );
    for (id, grammar) in want["grammars"].as_object().expect("grammars") {
        check(
            recorded.iter().any(|recorded| {
                recorded.id == *id
                    && grammar["version"] == recorded.version.as_str()
                    && grammar["parserSha256"] == recorded.parser_sha256.as_str()
            }),
            format!("expected grammar {id}"),
        );
    }
    // childOrder, tokens, commentsAndTrivia, exactUtf8Spans
    let mut children: HashMap<LinkId, Vec<LinkId>> = HashMap::new();
    let mut tokens: HashMap<LinkId, Vec<String>> = HashMap::new();
    let mut trivia = HashSet::new();
    for link in network.links() {
        let metadata = link.metadata();
        match (metadata.link_type(), link.references()) {
            (Some(LinkType::Syntax), [parent]) => {
                children.entry(*parent).or_default().push(link.id());
            }
            (Some(LinkType::Token), [owner]) => tokens
                .entry(*owner)
                .or_default()
                .push(metadata.term().unwrap_or_default().to_string()),
            (Some(LinkType::Trivia), _) => {
                if let Some(span) = metadata.span() {
                    trivia.insert((span.byte_range().start(), span.byte_range().end()));
                }
            }
            _ => {}
        }
    }
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let metadata = network.link(id).expect("syntax link").metadata();
        let term = metadata.term().unwrap_or_default();
        let range = metadata.span().expect("syntax span").byte_range();
        let (start, end) = (range.start(), range.end());
        check(
            start <= end
                && end <= source.len()
                && source.is_char_boundary(start)
                && source.is_char_boundary(end),
            format!("UTF-8 span {term} {start}:{end}"),
        );
        let node_children = children.get(&id).cloned().unwrap_or_default();
        let mut previous_end = start;
        for child in &node_children {
            let child_range = network
                .link(*child)
                .and_then(|child| child.metadata().span())
                .expect("child span")
                .byte_range();
            check(
                child_range.start() >= previous_end && child_range.end() <= end,
                format!("child order {term} {}", child_range.start()),
            );
            previous_end = child_range.end();
        }
        if node_children.is_empty() && start < end {
            let text = &source[start..end];
            check(
                tokens
                    .get(&id)
                    .is_some_and(|tokens| tokens.iter().any(|token| token == text)),
                format!("token {term} {start}:{end}"),
            );
            if metadata.flags().is_extra() {
                check(
                    trivia.contains(&(start, end)),
                    format!("trivia {term} {start}:{end}"),
                );
            }
        }
        stack.extend(node_children);
    }
    check(
        rows.iter().filter(|row| has_flag(row, "X")).count()
            == want_rows.iter().filter(|row| has_flag(row, "X")).count(),
        "extras".into(),
    );
    // errorAndMissingNodes
    check(
        network.verify_full_match(None).is_clean(),
        "positive source is clean".into(),
    );
    check(
        !recovery.verify_full_match(None).is_clean(),
        "recovery source is diagnosed".into(),
    );
    let recovery_rows = syntax_rows(&recovery, document_root(&recovery), 0);
    let want_recovery = public_rows(name, recovery_source, rows_of(&want["recovery"]));
    if let Some(difference) = first_difference(&recovery_rows, &want_recovery) {
        check(false, format!("recovery rows {difference}"));
    }
    check(
        recovery_rows.iter().any(|row| has_flag(row, "EM")),
        "recovery has error or missing nodes".into(),
    );
    // embeddedLanguageBoundaries
    let regions = embedded_roots(&network, name, source);
    let want_regions = want["embedded"].as_array().expect("embedded");
    check(
        regions
            .iter()
            .map(|(language, start, end, _)| json!([language, start, end]))
            .collect::<Vec<_>>()
            == want_regions
                .iter()
                .map(|region| json!([region["language"], region["startByte"], region["endByte"]]))
                .collect::<Vec<_>>(),
        "embedded boundaries".into(),
    );
    for ((_, start, _, region_root), region) in regions.iter().zip(want_regions) {
        let region_rows = syntax_rows(&network, *region_root, *start);
        if let Some(difference) = first_difference(&region_rows, rows_of(&region["rows"])) {
            check(false, format!("embedded {} {difference}", region["path"]));
        }
    }
    // allAliases
    for alias in language["aliases"].as_array().expect("aliases") {
        let alias = alias.as_str().expect("alias");
        let aliased = LinkNetwork::parse(source, alias, ParseConfiguration::default());
        check(
            first_difference(&syntax_rows(&aliased, document_root(&aliased), 0), &rows).is_none()
                && aliased.reconstruct_text() == source,
            format!("alias {alias}"),
        );
    }
    // extensionDispatch: every extension offers the language, and a path that
    // selects it parses to the same tree.
    let extensions = inventory["extensionDispatch"][name]
        .as_array()
        .expect("extensions");
    let mut selected = extensions.is_empty();
    for extension in extensions {
        let path = format!("fixture{}", extension.as_str().expect("extension"));
        check(
            language_candidates_for_path(&path).contains(&name),
            format!("extension {path}"),
        );
        if language_for_path(&path) == Some(name) {
            selected = true;
            let via_path = LinkNetwork::parse(source, name, ParseConfiguration::default());
            check(
                first_difference(&syntax_rows(&via_path, document_root(&via_path), 0), &rows)
                    .is_none(),
                format!("extension {path} rows"),
            );
        }
    }
    check(selected, "an extension selects the language".into());
    problems
}

#[test]
fn every_rust_inventory_language_parses_to_its_complete_lossless_default_cst() {
    let inventory_bytes = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../parity/language-grammar-inventory.json"),
    )
    .expect("inventory is readable");
    let inventory: Value = serde_json::from_slice(&inventory_bytes).expect("inventory is JSON");
    let digest = format!("{:x}", Sha256::digest(&inventory_bytes));
    let expected = expected_languages();
    let mut failures = Vec::new();
    for language in inventory["languages"].as_array().expect("languages") {
        let name = language["name"].as_str().expect("name");
        let problems = positive_cst_problems(&inventory, language, expected.get(name));
        if problems.is_empty() {
            record_positive_cst_observations(name, &digest);
        } else {
            failures.push(format!("{name}: {}", problems.join("; ")));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
