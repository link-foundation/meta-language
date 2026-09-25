use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use meta_language::{LinkId, LinkNetwork, LinkType, ParseConfiguration};
use serde_json::{json, Value};

fn parity_json(name: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity")
        .join(name);
    serde_json::from_str(&fs::read_to_string(path).expect("parity file is readable"))
        .expect("parity file is valid JSON")
}

/// Projects the Syntax links of a public network to the grammar rows of
/// `parity/fixtures/default-cst-expected.json`, walking down from `root`.
fn syntax_rows(network: &LinkNetwork, root: LinkId) -> Vec<Value> {
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
            range.map(meta_language::ByteRange::start),
            range.map(meta_language::ByteRange::end),
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
    let expected = parity_json("fixtures/default-cst-expected.json");
    let mut differences = Vec::new();
    for language in inventory["languages"].as_array().expect("languages") {
        let name = language["name"].as_str().expect("name");
        let Some(want) = expected["languages"].get(name) else {
            continue;
        };
        for (kind, key) in [("positive", "source"), ("recovery", "recoverySource")] {
            let source = language[key].as_str().expect("source");
            let network = LinkNetwork::parse(source, name, ParseConfiguration::default());
            let rows = syntax_rows(&network, document_root(&network));
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
