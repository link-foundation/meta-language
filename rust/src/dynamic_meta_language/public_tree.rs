//! Exact host serialization of the existing native public CST.
use crate::grammar::feature_runtime::{
    GrammarSelection, LeafText, OperationValue, ParseOutcome, SyntaxAttributes, SyntaxTree,
};
use serde_json::{Map, Value, json};

fn attributes(value: &SyntaxAttributes) -> Value {
    Value::Object(
        value
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    match value {
                        OperationValue::Integer(value) => json!(value),
                        OperationValue::Text(value) => json!(value),
                    },
                )
            })
            .collect(),
    )
}
fn leaf(value: &LeafText, target: &mut Map<String, Value>) {
    match value {
        LeafText::Text(value) => {
            target.insert("text".to_owned(), json!(value));
        }
        LeafText::Hex(value) => {
            target.insert("text".to_owned(), Value::Null);
            target.insert("hex".to_owned(), json!(value));
        }
    }
}
fn selection(value: &GrammarSelection, depth: usize, limit: usize) -> Result<Value, String> {
    let mut result = json!({"mode":value.mode,"mapping":value.mapping,"start":value.start,"end":value.end,
        "candidates":value.candidates,"provenance":value.provenance,"alternatives":[],"ambiguous":value.ambiguous});
    if let Some(mapping) = &value.source_mapping {
        result["sourceMapping"] = mapping.clone();
    }
    result["alternatives"] = Value::Array(value.alternatives.iter().map(|alternative| {
        Ok(json!({"language":alternative.language,"root":tree(&alternative.root, depth + 1, limit)?,
            "provenance":alternative.provenance}))
    }).collect::<Result<_, String>>()?);
    Ok(result)
}

pub(super) fn tree(value: &SyntaxTree, depth: usize, limit: usize) -> Result<Value, String> {
    if depth > limit {
        return Err("dynamic meta-language: retained tree depth limit exceeded".to_owned());
    }
    let mut result = Map::new();
    match value {
        SyntaxTree::Node {
            kind,
            field,
            trivia,
            start,
            end,
            children,
            attributes: attrs,
        } => {
            result.insert("type".to_owned(), json!("node"));
            result.insert("kind".to_owned(), json!(kind));
            if let Some(field) = field {
                result.insert("field".to_owned(), json!(field));
            }
            if *trivia {
                result.insert("trivia".to_owned(), json!(true));
            }
            result.insert("start".to_owned(), json!(start));
            result.insert("end".to_owned(), json!(end));
            result.insert(
                "children".to_owned(),
                Value::Array(
                    children
                        .iter()
                        .map(|value| tree(value, depth + 1, limit))
                        .collect::<Result<_, _>>()?,
                ),
            );
            if let Some(attrs) = attrs {
                result.insert("attributes".to_owned(), attributes(attrs));
            }
        }
        SyntaxTree::Token {
            kind,
            field,
            trivia,
            start,
            end,
            attributes: attrs,
            text,
            lexical,
        } => {
            result.insert("type".to_owned(), json!("token"));
            result.insert("kind".to_owned(), json!(kind));
            if let Some(field) = field {
                result.insert("field".to_owned(), json!(field));
            }
            if *trivia {
                result.insert("trivia".to_owned(), json!(true));
            }
            result.insert("start".to_owned(), json!(start));
            result.insert("end".to_owned(), json!(end));
            if let Some(attrs) = attrs {
                result.insert("attributes".to_owned(), attributes(attrs));
            }
            if let Some(lexical) = lexical {
                result.insert("lexical".to_owned(), lexical.clone());
            }
            leaf(text, &mut result);
        }
        SyntaxTree::Error {
            start,
            end,
            text,
            reason,
        } => {
            result.insert("type".to_owned(), json!("error"));
            result.insert("start".to_owned(), json!(start));
            result.insert("end".to_owned(), json!(end));
            leaf(text, &mut result);
            if let Some(reason) = reason {
                result.insert("reason".to_owned(), json!(reason));
            }
        }
        SyntaxTree::Missing {
            kind,
            start,
            end,
            literal,
        } => {
            result.insert("type".to_owned(), json!("missing"));
            result.insert("kind".to_owned(), json!(kind));
            result.insert("start".to_owned(), json!(start));
            result.insert("end".to_owned(), json!(end));
            if *literal {
                result.insert("literal".to_owned(), json!(true));
            }
        }
        SyntaxTree::Embed {
            language,
            start,
            end,
            root,
            selection: selected,
            source_children,
        } => {
            result.insert("type".to_owned(), json!("embed"));
            result.insert("language".to_owned(), json!(language));
            result.insert("start".to_owned(), json!(start));
            result.insert("end".to_owned(), json!(end));
            result.insert("root".to_owned(), tree(root, depth + 1, limit)?);
            if let Some(children) = source_children {
                result.insert(
                    "sourceChildren".to_owned(),
                    Value::Array(
                        children
                            .iter()
                            .map(|value| tree(value, depth + 1, limit))
                            .collect::<Result<_, _>>()?,
                    ),
                );
            }
            if let Some(selected) = selected {
                result.insert(
                    "selection".to_owned(),
                    selection(selected, depth + 1, limit)?,
                );
            }
        }
    }
    Ok(Value::Object(result))
}

pub(super) fn outcome(value: &ParseOutcome, limit: usize) -> Result<Value, String> {
    let mut rejection = Value::Null;
    if let Some(reason) = &value.rejection {
        rejection = json!({"reason":reason.reason});
        if let Some(offset) = reason.offset {
            rejection["offset"] = json!(offset);
        }
        if let Some(line) = reason.line {
            rejection["line"] = json!(line);
        }
        if let Some(column) = reason.column {
            rejection["column"] = json!(column);
        }
        if let Some(expected) = &reason.expected {
            rejection["expected"] = json!(expected);
        }
        if let Some(limit) = reason.limit {
            rejection["limit"] = json!(limit);
        }
        if let Some(diagnostics) = &reason.diagnostics {
            rejection["diagnostics"] = json!(diagnostics);
        }
        if let Some(semantics) = &reason.offset_semantics {
            rejection["offsetSemantics"] = json!(semantics);
        }
    }
    let ambiguities: Vec<_> = value
        .ambiguities
        .iter()
        .map(|item| {
            let mut value = json!({"rule":item.rule,"start":item.start,"end":item.end});
            if let Some(languages) = &item.languages {
                value["languages"] = json!(languages);
            }
            value
        })
        .collect();
    let mut result = json!({"ok":value.ok,"tree":value.tree.as_ref().map(|value| tree(value, 0, limit)).transpose()?,
        "ambiguities":ambiguities,"rejection":rejection});
    if let Some(planes) = &value.source_planes {
        result["sourcePlanes"] = json!(planes);
    }
    Ok(result)
}
