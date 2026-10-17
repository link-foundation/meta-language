//! Test-only transport of public syntax fields and official `LiNo` syntax.
//! No grammar/source-mapping decoding or validation algorithms live here.
use links_notation::LiNo;
use meta_language::{
    BinaryLinksOptions, FeatureParseOptions, Grammar, GrammarExpr, GrammarRule, LeafText,
    LinkNetwork, OperationValue, ParseOutcome, SyntaxAttributes, SyntaxTree,
    compile_feature_grammar, decode_link_network_binary, decode_structured_data_links,
    encode_link_network_binary, encode_structured_data_links, parse_grammar_links,
    parse_links_expression, percent_encode_links_text,
};
use serde_json::{Value, json};
use std::collections::HashMap;

pub fn mapped_fixture(path: &str) -> Value {
    meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity")
            .join(path),
    )
    .unwrap()
}

pub fn mapped_json_grammar() -> Grammar {
    parse_grammar_links(include_str!("../../src/data/native-grammars/json.lino")).unwrap()
}

pub fn mapped_python_grammar() -> Grammar {
    parse_grammar_links(include_str!("../../src/data/native-grammars/python.lino")).unwrap()
}

pub fn mapped_json_host(names: &[(&str, &str)], mode: &str, prefix: &str) -> Grammar {
    let mut grammar = mapped_json_grammar();
    assert!(grammar.rule("mapped_document").is_none());
    let candidates = names
        .iter()
        .map(|(name, mapping)| {
            format!(
                "(grammar-candidate {} {mapping})",
                percent_encode_links_text(name),
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    let selection = parse_links_expression(&format!(
        "({mode} (ref {}) {candidates})",
        grammar.start().unwrap(),
    ))
    .unwrap();
    let expression = if prefix.is_empty() {
        selection
    } else {
        GrammarExpr::Sequence(vec![GrammarExpr::Terminal(prefix.to_owned()), selection])
    };
    let rule = GrammarRule::new("mapped_document", expression).with_source_names(vec![
        meta_language::GrammarSourceName {
            source: "authored".to_owned(),
            name: "mapped_document".to_owned(),
        },
    ]);
    grammar.add_rule(rule);
    grammar.set_start("mapped_document");
    grammar
}

pub fn mapped_literal(source: &str) -> Grammar {
    Grammar::new()
        .with_start("decoded")
        .with_rule(GrammarRule::new(
            "decoded",
            GrammarExpr::Terminal(source.to_owned()),
        ))
}

pub fn mapped_parse(
    grammar: &Grammar,
    grammars: &HashMap<String, Grammar>,
    source: &str,
) -> ParseOutcome {
    let resolve = |name: &str| grammars.get(name).cloned();
    compile_feature_grammar(grammar, Some(&resolve), FeatureParseOptions::default())
        .unwrap()
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap()
}

fn mapped_attributes(attributes: Option<&SyntaxAttributes>) -> Value {
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

// Exhaustive test-only transport of public fields, not a decoder or validator.
pub fn mapped_tree_data(tree: &SyntaxTree) -> Value {
    let mut value = json!({"start":tree.start(),"end":tree.end()});
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
            value["field"] = json!(field);
            value["trivia"] = json!(trivia);
            value["attributes"] = mapped_attributes(attributes.as_ref());
            value["children"] = json!(children.iter().map(mapped_tree_data).collect::<Vec<_>>());
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
            value["field"] = json!(field);
            value["trivia"] = json!(trivia);
            value["attributes"] = mapped_attributes(attributes.as_ref());
            value["lexical"] = json!(lexical);
        }
        SyntaxTree::Error { reason, .. } => {
            value["type"] = json!("error");
            value["reason"] = json!(reason);
        }
        SyntaxTree::Missing { kind, literal, .. } => {
            value["type"] = json!("missing");
            value["kind"] = json!(kind);
            value["literal"] = json!(literal);
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
            value["root"] = mapped_tree_data(root);
            if let Some(children) = source_children {
                value["sourceChildren"] =
                    json!(children.iter().map(mapped_tree_data).collect::<Vec<_>>());
            }
            value["selection"] = selection.as_ref().map_or(Value::Null, |selected| {
                let mut data = json!({"mode":selected.mode,"mapping":selected.mapping,
                    "start":selected.start,"end":selected.end,"candidates":selected.candidates,
                    "provenance":selected.provenance,"ambiguous":selected.ambiguous,
                    "alternatives":selected.alternatives.iter().map(|item| json!({
                        "language":item.language,"root":mapped_tree_data(&item.root),"provenance":item.provenance,
                    })).collect::<Vec<_>>()});
                if let Some(mapping) = &selected.source_mapping { data["sourceMapping"].clone_from(mapping); }
                data
            });
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
    value
}

pub fn mapped_bundle(outcome: &ParseOutcome) -> Value {
    json!({"tree":outcome.tree.as_ref().map(mapped_tree_data),"sourcePlanes":outcome.source_planes})
}

pub fn mapped_embeds<'a>(tree: &'a Value, found: &mut Vec<&'a Value>) {
    if tree["type"] == "embed" && tree["selection"]["mapping"] == "json-string" {
        found.push(tree);
    }
    for field in ["children", "sourceChildren"] {
        if let Some(children) = tree[field].as_array() {
            for child in children {
                mapped_embeds(child, found);
            }
        }
    }
    if tree.get("root").is_some() {
        mapped_embeds(&tree["root"], found);
    }
    if let Some(witnesses) = tree["selection"]["alternatives"].as_array() {
        for witness in witnesses {
            mapped_embeds(&witness["root"], found);
        }
    }
}

pub fn mapped_first(tree: &Value) -> &Value {
    let mut found = Vec::new();
    mapped_embeds(tree, &mut found);
    found[0]
}

pub fn mapped_physical(tree: &Value) -> Vec<u8> {
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
                || mapped_physical(&tree["root"]),
                |children| {
                    children
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(mapped_physical)
                        .collect()
                },
            ),
    }
}

pub fn mapped_stored_variants(value: &Value) -> [Value; 2] {
    let graph = encode_structured_data_links(value).unwrap();
    let text = graph.network.to_lino();
    let restored = LinkNetwork::from_lino(&text).unwrap();
    let binary = encode_link_network_binary(&restored, BinaryLinksOptions::default()).unwrap();
    let binary = decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap();
    assert_eq!(
        binary.to_lino(),
        text,
        "all actual link identities and metadata survive"
    );
    [
        decode_structured_data_links(&restored, graph.root).unwrap(),
        decode_structured_data_links(&binary, graph.root).unwrap(),
    ]
}

// The official LiNo syntax is inspectable data. The official grammar parser,
// rather than a test-owned domain decoder, reconstructs all grammar semantics.
pub fn mapped_link_data(node: &LiNo<String>) -> Value {
    match node {
        LiNo::Ref(word) => json!(word),
        LiNo::Link { id, values } => {
            json!({"id":id,"values":values.iter().map(mapped_link_data).collect::<Vec<_>>()})
        }
    }
}

pub fn mapped_link_text(node: &Value) -> String {
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
                    .map(mapped_link_text)
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        },
        str::to_owned,
    )
}

pub fn mapped_root_expression(grammar: &Grammar, expression: GrammarExpr) -> Grammar {
    let mut restored = Grammar::new();
    let root = grammar.start().unwrap();
    restored.set_start(root);
    if let Some(format) = grammar.source_format() {
        restored.set_source_format(format);
    }
    restored.set_declarations(grammar.declarations().clone());
    restored.set_kinds(grammar.kinds().to_vec());
    for mut rule in grammar.rules().iter().cloned() {
        if rule.name == root {
            rule.expr.clone_from(&expression);
        }
        restored.add_rule(rule);
    }
    restored
}

#[test]
fn mapped_scalar_profile_does_not_relabel_invalid_utf8_or_admit_lexical_escapes() {
    let bytes = [255, 34, 120, 34];
    let grammars = HashMap::from([
        ("decoded".to_owned(), mapped_literal("x")),
        ("raw".to_owned(), mapped_literal("\"x\"")),
    ]);
    let resolve = |name: &str| grammars.get(name).cloned();
    for (name, mapping, accepted) in [("decoded", "json-string", false), ("raw", "identity", true)]
    {
        let host = mapped_json_host(&[(name, mapping)], "any-grammar", "");
        let expression = GrammarExpr::Sequence(vec![
            GrammarExpr::AnyChar,
            host.rule("mapped_document").unwrap().expr.clone(),
        ]);
        let host = mapped_root_expression(&host, expression);
        let result = compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default())
            .unwrap()
            .parse_tree(&bytes, &FeatureParseOptions::default())
            .unwrap();
        assert_eq!(result.ok, accepted, "{mapping}: {:?}", result.rejection);
        assert!(
            result.source_planes.is_none(),
            "no invalid UTF-8 plane is declared"
        );
        if accepted {
            let tree = mapped_tree_data(result.tree.as_ref().unwrap());
            assert_eq!(mapped_physical(&tree), bytes);
            let forged = json!({"tree":tree,"sourcePlanes":[{
                "kind":"grammar-source","identity":0,"encoding":"UTF-8","byteLength":4,"derivation":null,
            }]});
            let error = meta_language::check_grammar_selection_mapped_tree(
                &forged,
                &FeatureParseOptions::default(),
            )
            .unwrap_err();
            assert_eq!(error.reason, "selection");
            assert!(error.message.contains("UTF-8"));
        } else {
            assert_eq!(
                result.rejection.unwrap().reason,
                "unsupportedEmbeddingMapping"
            );
        }
    }
    let expression = parse_links_expression(
        "(any-grammar (literal %22x%22) (grammar-candidate decoded json-string))",
    )
    .unwrap();
    let lexical = Grammar::new().with_start("lexical").with_rule(
        GrammarRule::new("lexical", expression).with_kind(meta_language::RuleKind::Token),
    );
    let result = mapped_parse(&lexical, &grammars, "\"x\"");
    assert!(!result.ok);
    assert_eq!(
        result.rejection.unwrap().reason,
        "unsupportedEmbeddingMapping"
    );
    for raw in [r#""\q""#, r#""\u{41}""#, r#""\u12""#, "\"line\nbreak\""] {
        // Authored literal admission isolates the mapping's syntax refusal;
        // the shipped JSON/Python comparisons live in the parent suite.
        let expression = parse_links_expression(&format!(
            "(any-grammar (literal {}) (grammar-candidate decoded json-string))",
            percent_encode_links_text(raw),
        ))
        .unwrap();
        let host = Grammar::new()
            .with_start("document")
            .with_rule(GrammarRule::new("document", expression));
        let result = mapped_parse(&host, &grammars, raw);
        assert!(!result.ok, "{raw}");
        assert_eq!(
            result.rejection.unwrap().reason,
            "invalidEmbeddingMapping",
            "{raw}"
        );
    }
}
