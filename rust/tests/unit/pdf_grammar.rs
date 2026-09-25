use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use meta_language::{LinkFlags, LinkId, LinkNetwork, LinkType, ParseConfiguration};
use serde_json::{json, Value};

/// A Syntax node of a public PDF network.
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

fn text_of<'a>(node: &Node, source: &'a str) -> &'a str {
    &source[node.start..node.end]
}

fn syntax_children(node: &Node) -> impl Iterator<Item = &Node> {
    node.children
        .iter()
        .filter(|child| child.named && !child.flags.is_extra())
}

fn field<'a>(node: &'a Node, name: &str) -> Option<&'a Node> {
    node.children
        .iter()
        .find(|child| child.field.as_deref() == Some(name))
}

fn number(text: &str) -> Value {
    json!({ "number": text.parse::<f64>().expect("PDF number") })
}

/// A name without its solidus, with `#XX` escapes decoded, as UTF-8 text.
fn decode_name(node: &Node, source: &str) -> String {
    let raw = text_of(node, source).as_bytes();
    let mut decoded = Vec::new();
    let mut index = 1;
    while index < raw.len() {
        let escaped = raw
            .get(index + 1..index + 3)
            .filter(|hex| raw[index] == b'#' && hex.iter().all(u8::is_ascii_hexdigit))
            .map(|hex| u8::from_str_radix(std::str::from_utf8(hex).unwrap(), 16).unwrap());
        if let Some(byte) = escaped {
            decoded.push(byte);
            index += 3;
        } else {
            decoded.push(raw[index]);
            index += 1;
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

fn entries(dictionary: &Node, source: &str) -> Value {
    let mut projected: Vec<(String, Value)> = Vec::new();
    for entry in syntax_children(dictionary) {
        let key = decode_name(field(entry, "key").expect("key"), source);
        let value = project(field(entry, "value").expect("value"), source);
        if let Some(existing) = projected.iter_mut().find(|(name, _)| *name == key) {
            existing.1 = value;
        } else {
            projected.push((key, value));
        }
    }
    projected
        .into_iter()
        .map(|(key, value)| json!([key, value]))
        .collect()
}

/// Projects a PDF object node to the shape of the pdf-lib oracle.
fn project(node: &Node, source: &str) -> Value {
    let text = text_of(node, source);
    match node.term.as_str() {
        "null" => Value::Null,
        "boolean" => Value::Bool(text == "true"),
        "integer" | "real" => number(text),
        "name" => json!({ "name": decode_name(node, source) }),
        "literal_string" => json!({ "string": &text[1..text.len() - 1] }),
        "hex_string" => json!({ "hex": &text[1..text.len() - 1] }),
        "indirect_reference" => json!({ "ref": [
            number(text_of(field(node, "object_number").unwrap(), source))["number"],
            number(text_of(field(node, "generation").unwrap(), source))["number"],
        ] }),
        "array" => syntax_children(node)
            .map(|child| project(child, source))
            .collect(),
        "dictionary" => json!({ "dictionary": entries(node, source) }),
        "stream" => json!({
            "stream": entries(field(node, "dictionary").unwrap(), source),
            "data": field(node, "data").map_or("", |data| text_of(data, source)),
        }),
        term => panic!("unexpected PDF object {term}"),
    }
}

/// The indirect objects of a PDF CST, later definitions replacing earlier ones, by object number.
fn indirect_objects(tree: &Node, source: &str) -> Value {
    let mut objects: Vec<((u64, u64), Value)> = Vec::new();
    for object in tree
        .children
        .iter()
        .filter(|child| child.term == "indirect_object")
    {
        let part = |name| {
            text_of(field(object, name).unwrap(), source)
                .parse::<u64>()
                .unwrap()
        };
        let reference = (part("object_number"), part("generation"));
        let value = json!({
            "ref": [reference.0, reference.1],
            "value": project(field(object, "value").unwrap(), source),
        });
        if let Some(existing) = objects.iter_mut().find(|(key, _)| *key == reference) {
            existing.1 = value;
        } else {
            objects.push((reference, value));
        }
    }
    objects.sort_by_key(|((number, _), _)| *number);
    objects.into_iter().map(|(_, value)| value).collect()
}

/// `value` with every number as a float, so integers and floats compare by value.
fn normalize(value: &Value) -> Value {
    match value {
        Value::Number(number) => json!(number.as_f64().expect("finite number")),
        Value::Array(items) => items.iter().map(normalize).collect(),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| (key.clone(), normalize(value)))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn leaves(node: &Node) -> Vec<&Node> {
    if node.children.is_empty() && node.term != "pdf_file" {
        vec![node]
    } else {
        node.children.iter().flat_map(leaves).collect()
    }
}

fn terms<'a>(node: &'a Node, seen: &mut HashSet<&'a str>) {
    seen.insert(&node.term);
    for child in &node.children {
        terms(child, seen);
    }
}

#[test]
fn pdf_grammar_cst_describes_the_indirect_objects_pdf_lib_reads() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../parity/fixtures/pdf-grammar-cases.json");
    let fixture: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("PDF grammar cases are readable"))
            .expect("PDF grammar cases are valid JSON");
    let mut seen = HashSet::new();
    let mut clean = 0;
    let cases = fixture["cases"].as_array().expect("cases");
    let trees = cases
        .iter()
        .map(|case| {
            let source = case["source"].as_str().expect("source");
            let network = LinkNetwork::parse(source, "PDF", ParseConfiguration::default());
            assert_eq!(network.reconstruct_text(), source);
            syntax_tree(&network)
        })
        .collect::<Vec<_>>();
    for (case, tree) in cases.iter().zip(&trees) {
        let source = case["source"].as_str().expect("source");
        assert_eq!(tree.term, "pdf_file");
        if case["objects"].is_null() {
            assert!(
                tree.flags.has_error(),
                "pdf-lib rejects {source:?}, so its CST has errors"
            );
        }
        if !tree.flags.has_error() {
            clean += 1;
            assert!(
                !case["objects"].is_null(),
                "pdf-lib reads the clean {source:?}"
            );
            assert_eq!(
                normalize(&indirect_objects(tree, source)),
                normalize(&case["objects"]),
                "objects of {source:?}"
            );
            terms(tree, &mut seen);
        }
        let mut covered = 0;
        for leaf in leaves(tree) {
            assert_eq!(leaf.start, covered, "leaves of {source:?} are contiguous");
            covered = leaf.end;
            let text = text_of(leaf, source);
            if leaf.term == "whitespace" {
                assert!(leaf.flags.is_extra() && !leaf.named);
                assert!(text
                    .bytes()
                    .all(|byte| matches!(byte, 0 | b'\t' | b'\n' | 0x0c | b'\r' | b' ')));
            }
            if leaf.term == "comment" {
                assert!(leaf.flags.is_extra() && leaf.named);
                assert!(text.starts_with('%') && !text.contains(['\r', '\n']));
            }
        }
        assert_eq!(covered, source.len());
    }
    assert!(clean >= 150, "{clean} clean cases");
    for term in [
        "header",
        "indirect_object",
        "stream",
        "content_stream",
        "operation",
        "operator",
        "inline_image",
        "image_data",
        "stream_data",
        "dictionary",
        "dictionary_entry",
        "array",
        "indirect_reference",
        "integer",
        "real",
        "boolean",
        "null",
        "name",
        "literal_string",
        "hex_string",
        "cross_reference_table",
        "cross_reference_subsection",
        "cross_reference_entry",
        "trailer",
        "start_cross_reference",
        "end_of_file",
        "comment",
        "whitespace",
    ] {
        assert!(seen.contains(term), "a clean case has a {term} node");
    }
}

/// Rows of the non-extra nodes, indented by depth, prefixed by field and
/// suffixed by the error or missing flag.
fn field_rows(node: &Node, depth: usize, rows: &mut Vec<String>) {
    let field = node
        .field
        .as_ref()
        .map_or_else(String::new, |field| format!("{field}: "));
    let flag = if node.flags.is_error() {
        " (error)"
    } else if node.flags.is_missing() {
        " (missing)"
    } else {
        ""
    };
    rows.push(format!("{}{field}{}{flag}", "  ".repeat(depth), node.term));
    for child in node.children.iter().filter(|child| !child.flags.is_extra()) {
        field_rows(child, depth + 1, rows);
    }
}

#[test]
fn pdf_grammar_cst_records_fields_and_keeps_malformed_input_in_error_and_missing_nodes() {
    let network = LinkNetwork::parse(
        "%PDF-1.7\n1 0 obj\n<< /A [1 ) /B >>\n2 0 obj\n<< /Length 2 >>\nstream\nq Q\nendstream\n",
        "PDF",
        ParseConfiguration::default(),
    );
    let tree = syntax_tree(&network);
    let mut rows = Vec::new();
    field_rows(&tree, 0, &mut rows);
    assert_eq!(
        rows,
        [
            "pdf_file",
            "  header",
            "  indirect_object",
            "    object_number: integer",
            "    generation: integer",
            "    obj",
            "    value: dictionary",
            "      <<",
            "      dictionary_entry",
            "        key: name",
            "        value: array",
            "          [",
            "          integer",
            "          ERROR (error)",
            "            )",
            "          name",
            "          ] (missing)",
            "      >>",
            "    endobj (missing)",
            "  indirect_object",
            "    object_number: integer",
            "    generation: integer",
            "    obj",
            "    value: stream",
            "      dictionary: dictionary",
            "        <<",
            "        dictionary_entry",
            "          key: name",
            "          value: integer",
            "        >>",
            "      stream",
            "      data: content_stream",
            "        operation",
            "          operator: operator",
            "        operation",
            "          operator: operator",
            "      endstream",
            "    endobj (missing)",
            "  end_of_file (missing)",
        ]
    );
    assert!(tree.flags.has_error() && !tree.flags.is_error());
}
