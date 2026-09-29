use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use meta_language::{LinkFlags, LinkId, LinkNetwork, LinkType, ParseConfiguration};
use serde::Deserialize;
use serde_json::{Value, json};

/// A Syntax node of a public `LiNo` network.
struct Node {
    term: String,
    field: Option<String>,
    named: bool,
    flags: LinkFlags,
    start: usize,
    end: usize,
    children: Vec<Self>,
}

fn syntax_tree(network: &LinkNetwork) -> Node {
    let mut children: HashMap<LinkId, Vec<LinkId>> = HashMap::new();
    let mut fields: HashMap<(LinkId, LinkId), String> = HashMap::new();
    for link in network.links() {
        match link.metadata().link_type() {
            Some(LinkType::Syntax) => {
                if let [parent] = link.references() {
                    children.entry(*parent).or_default().push(link.id());
                }
            }
            Some(LinkType::Field) => {
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
    let document = network
        .links()
        .find(|link| link.metadata().link_type() == Some(LinkType::Document))
        .expect("document link")
        .id();
    let root = children[&document]
        .iter()
        .copied()
        .find(|id| network.link(*id).unwrap().metadata().link_type() == Some(LinkType::Syntax))
        .expect("grammar root below the document");
    build(network, &children, &fields, root, None)
}

fn build(
    network: &LinkNetwork,
    children: &HashMap<LinkId, Vec<LinkId>>,
    fields: &HashMap<(LinkId, LinkId), String>,
    id: LinkId,
    field: Option<String>,
) -> Node {
    let metadata = network.link(id).expect("syntax link").metadata();
    let range = metadata.span().expect("syntax span").byte_range();
    Node {
        term: metadata.term().unwrap_or_default().to_string(),
        field,
        named: metadata.is_named(),
        flags: metadata.flags(),
        start: range.start(),
        end: range.end(),
        children: children
            .get(&id)
            .into_iter()
            .flatten()
            .filter(|child| {
                network.link(**child).unwrap().metadata().link_type() == Some(LinkType::Syntax)
            })
            .map(|child| {
                build(
                    network,
                    children,
                    fields,
                    *child,
                    fields.get(&(id, *child)).cloned(),
                )
            })
            .collect(),
    }
}

/// The value of a whole quoted reference: an opening run of N delimiters, runs
/// inside in which each 2N delimiters are an escaped N, and a closing run whose
/// last N delimiters close it. N even delimiters alone are the empty reference.
fn decode_quoted(text: &str) -> String {
    let quote = text.chars().next().expect("quoted reference");
    let count = text
        .chars()
        .take_while(|character| *character == quote)
        .count();
    if count == text.len() {
        return String::new();
    }
    let characters: Vec<char> = text.chars().collect();
    let mut value = String::new();
    let mut position = count;
    while position < characters.len() {
        let run = characters[position..]
            .iter()
            .take_while(|character| **character == quote)
            .count();
        if run == 0 {
            value.push(characters[position]);
            position += 1;
            continue;
        }
        let closes = if position + run == characters.len() {
            count
        } else {
            0
        };
        let kept = run - run / (2 * count) * count - closes;
        value.push_str(&quote.to_string().repeat(kept));
        position += run;
    }
    value
}

/// Projects a `link`, `group` or reference node to the official shape:
/// {id, values, children} for links and references, {nested, children} for
/// parenthesized groups.
fn official_shape(node: &Node, source: &str) -> Value {
    let text = &source[node.start..node.end];
    let with_field = |field: &str| {
        node.children
            .iter()
            .filter(|child| child.field.as_deref() == Some(field))
            .map(|child| official_shape(child, source))
            .collect::<Vec<_>>()
    };
    match node.term.as_str() {
        "reference" => json!({"id": text, "values": [], "children": []}),
        "quoted_reference" => json!({"id": decode_quoted(text), "values": [], "children": []}),
        "group" => json!({"nested": with_field("link"), "children": with_field("child")}),
        term => {
            assert_eq!(term, "link");
            json!({
                "id": with_field("id").first().map(|id| id["id"].clone()),
                "values": with_field("value"),
                "children": with_field("child"),
            })
        }
    }
}

fn leaves(node: &Node) -> Vec<&Node> {
    if node.children.is_empty() && node.term != "lino_document" {
        vec![node]
    } else {
        node.children.iter().flat_map(leaves).collect()
    }
}

#[test]
fn lino_grammar_cst_carries_the_links_of_the_official_links_notation_parser() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/lino-grammar-cases.json");
    let text = fs::read_to_string(path).expect("LiNo grammar cases are readable");
    // The deep-nesting cases hold links nested further than serde_json's
    // default recursion limit.
    let mut deserializer = serde_json::Deserializer::from_str(&text);
    deserializer.disable_recursion_limit();
    let fixture = Value::deserialize(&mut deserializer).expect("LiNo grammar cases are valid JSON");
    for case in fixture["cases"].as_array().expect("cases") {
        let source = case["source"].as_str().expect("source");
        let network = LinkNetwork::parse(source, "LiNo", ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source);
        let tree = syntax_tree(&network);
        assert_eq!(tree.term, "lino_document");
        assert_eq!(
            tree.flags.has_error(),
            case["links"].is_null(),
            "error state of {source:?}"
        );
        if !case["links"].is_null() {
            let links = tree
                .children
                .iter()
                .filter(|child| child.named && !child.flags.is_extra())
                .map(|child| official_shape(child, source))
                .collect::<Vec<_>>();
            assert_eq!(Value::from(links), case["links"], "links of {source:?}");
        }
        let mut covered = 0;
        for leaf in leaves(&tree) {
            assert_eq!(leaf.start, covered, "leaves of {source:?} are contiguous");
            covered = leaf.end;
            if leaf.term == "whitespace" {
                assert!(leaf.flags.is_extra() && !leaf.named);
                assert!(
                    source[leaf.start..leaf.end]
                        .bytes()
                        .all(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
                );
            }
            if leaf.term == "comment" {
                let text = &source[leaf.start..leaf.end];
                assert!(leaf.flags.is_extra() && leaf.named);
                assert!(text.starts_with('#') && !text.contains(['\r', '\n']));
            }
        }
        assert_eq!(covered, source.len());
    }
}

/// Rows of the non-whitespace nodes, indented by depth and prefixed by field.
fn field_rows(node: &Node, depth: usize, rows: &mut Vec<String>) {
    let field = node
        .field
        .as_ref()
        .map_or_else(String::new, |field| format!("{field}: "));
    rows.push(format!("{}{field}{}", "  ".repeat(depth), node.term));
    for child in node
        .children
        .iter()
        .filter(|child| child.term != "whitespace")
    {
        field_rows(child, depth + 1, rows);
    }
}

#[test]
fn lino_grammar_cst_records_id_value_and_child_fields_and_recovers_per_line() {
    let network = LinkNetwork::parse(
        "greeting:\n  hello (x: y)\n(broken\nnext: line\n",
        "LiNo",
        ParseConfiguration::default(),
    );
    let tree = syntax_tree(&network);
    let mut rows = Vec::new();
    field_rows(&tree, 0, &mut rows);
    assert_eq!(
        rows,
        [
            "lino_document",
            "  link",
            "    id: reference",
            "    :",
            "    child: link",
            "      value: reference",
            "      value: group",
            "        (",
            "        link: link",
            "          id: reference",
            "          :",
            "          value: reference",
            "        )",
            "  ERROR",
            "    (",
            "    reference",
            "  link",
            "    id: reference",
            "    :",
            "    value: reference",
        ]
    );
    assert!(tree.flags.has_error() && !tree.flags.is_error());
    assert!(
        tree.children
            .iter()
            .any(|child| child.term == "ERROR" && child.flags.is_error())
    );
}
