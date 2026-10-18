//! Test-only transport of public CST fields and official `LiNo` syntax.
//! This module has no tests, observation calls or mapping decision algorithms.
use links_notation::LiNo;
use meta_language::{
    BinaryLinksOptions, LeafText, LinkNetwork, OperationValue, ParseOutcome, SyntaxAttributes,
    SyntaxTree, decode_link_network_binary, decode_structured_data_links,
    encode_link_network_binary, encode_structured_data_links,
};
use serde_json::{Value, json};

fn included_range_attributes(attributes: Option<&SyntaxAttributes>) -> Value {
    attributes.map_or(Value::Null, |fields| {
        Value::Object(
            fields
                .iter()
                .map(|(key, value)| {
                    let value = match value {
                        OperationValue::Integer(value) => json!(value),
                        OperationValue::Text(value) => json!(value),
                    };
                    (key.clone(), value)
                })
                .collect(),
        )
    })
}

fn included_range_field_header(
    tree: &SyntaxTree,
    value: &mut Value,
    field: Option<&str>,
    trivia: bool,
) {
    if let Some(field) = field {
        value["field"] = json!(field);
    }
    if trivia {
        value["trivia"] = json!(true);
    }
    value["start"] = json!(tree.start());
    value["end"] = json!(tree.end());
}

pub fn included_range_tree_data(tree: &SyntaxTree) -> Value {
    let mut value = json!({});
    match tree {
        SyntaxTree::Node {
            kind,
            field,
            trivia,
            children,
            attributes,
            ..
        } => {
            value["type"] = json!("node");
            value["kind"] = json!(kind);
            included_range_field_header(tree, &mut value, field.as_deref(), *trivia);
            value["children"] = json!(
                children
                    .iter()
                    .map(included_range_tree_data)
                    .collect::<Vec<_>>()
            );
            if attributes.is_some() {
                value["attributes"] = included_range_attributes(attributes.as_ref());
            }
        }
        SyntaxTree::Token {
            kind,
            field,
            trivia,
            attributes,
            lexical,
            ..
        } => {
            value["type"] = json!("token");
            value["kind"] = json!(kind);
            included_range_field_header(tree, &mut value, field.as_deref(), *trivia);
            if attributes.is_some() {
                value["attributes"] = included_range_attributes(attributes.as_ref());
            }
            if let Some(lexical) = lexical {
                value["lexical"] = lexical.clone();
            }
        }
        SyntaxTree::Error { .. } => {
            value["type"] = json!("error");
            value["start"] = json!(tree.start());
            value["end"] = json!(tree.end());
        }
        SyntaxTree::Missing { kind, literal, .. } => {
            value["type"] = json!("missing");
            value["kind"] = json!(kind);
            value["start"] = json!(tree.start());
            value["end"] = json!(tree.end());
            if *literal {
                value["literal"] = json!(true);
            }
        }
        SyntaxTree::Embed {
            language,
            root,
            source_children,
            selection,
            ..
        } => {
            value["type"] = json!("embed");
            value["language"] = json!(language);
            value["start"] = json!(tree.start());
            value["end"] = json!(tree.end());
            value["root"] = included_range_tree_data(root);
            if let Some(children) = source_children {
                value["sourceChildren"] = json!(
                    children
                        .iter()
                        .map(included_range_tree_data)
                        .collect::<Vec<_>>()
                );
            }
            if let Some(selected) = selection {
                let mut data = json!({"mode":selected.mode,"mapping":selected.mapping,
                    "start":selected.start,"end":selected.end,"candidates":selected.candidates,
                    "provenance":selected.provenance,"ambiguous":selected.ambiguous,
                    "alternatives":selected.alternatives.iter().map(|item| json!({
                        "language":item.language,"root":included_range_tree_data(&item.root),"provenance":item.provenance,
                    })).collect::<Vec<_>>()});
                if let Some(mapping) = &selected.source_mapping {
                    data["sourceMapping"].clone_from(mapping);
                }
                value["selection"] = data;
            }
        }
    }
    if let SyntaxTree::Token { text, .. } | SyntaxTree::Error { text, .. } = tree {
        match text {
            LeafText::Text(text) => value["text"] = json!(text),
            LeafText::Hex(hex) => {
                value["text"] = Value::Null;
                value["hex"] = json!(hex);
            }
        }
    }
    if let SyntaxTree::Error {
        reason: Some(reason),
        ..
    } = tree
        && !reason.is_empty()
    {
        value["reason"] = json!(reason);
    }
    value
}

pub fn included_range_bundle(outcome: &ParseOutcome) -> Value {
    json!({"tree":outcome.tree.as_ref().map(included_range_tree_data),"sourcePlanes":outcome.source_planes})
}

pub fn included_range_physical(tree: &Value) -> Vec<u8> {
    match tree["type"].as_str().unwrap() {
        "missing" => Vec::new(),
        "token" | "error" => tree["text"].as_str().map_or_else(
            || {
                tree["hex"]
                    .as_str()
                    .unwrap()
                    .as_bytes()
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|word| u8::from_str_radix(std::str::from_utf8(word).unwrap(), 16).unwrap())
                    .collect()
            },
            |text| text.as_bytes().to_vec(),
        ),
        _ => tree
            .get("sourceChildren")
            .or_else(|| tree.get("children"))
            .map_or_else(
                || included_range_physical(&tree["root"]),
                |children| {
                    children
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(included_range_physical)
                        .collect()
                },
            ),
    }
}

pub fn included_range_stored(value: &Value) -> [Value; 2] {
    let graph = encode_structured_data_links(value).unwrap();
    let text = graph.network.to_lino();
    let restored = LinkNetwork::from_lino(&text).unwrap();
    let binary = encode_link_network_binary(&restored, BinaryLinksOptions::default()).unwrap();
    let binary = decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap();
    assert_eq!(
        binary.to_lino(),
        text,
        "exact link identities survive both archives"
    );
    [
        decode_structured_data_links(&restored, graph.root).unwrap(),
        decode_structured_data_links(&binary, graph.root).unwrap(),
    ]
}

pub fn included_range_link_data(node: &LiNo<String>) -> Value {
    match node {
        LiNo::Ref(word) => json!(word),
        LiNo::Link { id, values } => {
            json!({"id":id,"values":values.iter().map(included_range_link_data).collect::<Vec<_>>()})
        }
    }
}

pub fn included_range_link_text(node: &Value) -> String {
    node.as_str().map_or_else(
        || {
            let prefix = node["id"]
                .as_str()
                .map_or(String::new(), |id| format!("{id}: "));
            format!(
                "({prefix}{})",
                node["values"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(included_range_link_text)
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        },
        str::to_owned,
    )
}
